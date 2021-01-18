use yew::prelude::*;
use yew::services::ConsoleService;
use std::cell::{RefCell};
use std::rc::{Rc, Weak};

pub type ItemPtr<T> = Rc<RefCell<T>>;
/// This is the listener that will be sent out and held by the subscriber.
/// As long as they hold this strong reference, the callback will get called
/// any time that the item changes. If this goes out of scope, then the next
/// time that the store item is changed, it will notice that the strong count
/// has dropped to zero and remove this listener
pub type StoreListener<T> = Rc<Callback<ItemPtr<T>>>;
/// This is a weak reference to the callback that the store item keeps on
/// hand. Any time it want to call the callback it must first upgrade this
/// reference to ensure the callback is still in scope.
type WeakStoreListener<T> = Weak<Callback<ItemPtr<T>>>;

/// An item in the store that can be acted on
#[derive(Clone)]
pub struct StoreItem<T> {
    /// The item itself
    item: ItemPtr<T>,
    /// A list of callbacks to run when the item is mutated
    listeners: RefCell<Vec<WeakStoreListener<T>>>,
}

impl<T> StoreItem<T>
where
    T: Default
{
    pub fn default() -> StoreItem<T> {
        StoreItem {
            item: StoreItem::new_ptr(),
            listeners: RefCell::new(vec![]),
        }
    }

    pub fn new_ptr() -> ItemPtr<T> {
        Rc::new(RefCell::new(T::default()))
    }

    /// Calls all of the listeners with a new value, if a listener has been dropped
    /// and can't upgrade, remove it from the list of listeners
    fn call_listeners(self: &Self) {
        let mut dropped_listener: bool = false;
        // Create a list of the listeners that haven't been dropped
        let filtered_listeners: Vec<WeakStoreListener<T>> = if let Ok(listeners) = self.listeners.try_borrow() {
            ConsoleService::log(format!("Calling {} listeners", listeners.len()).as_str());
            listeners.iter().filter_map(|weak_listener| {
                if weak_listener.upgrade().is_some() {
                    Some(Weak::clone(weak_listener))
                } else {
                    // if a listener has been destroyed, don't add it to the filtered list
                    dropped_listener = true;
                    None
                }
            }).collect()
        } else {
            ConsoleService::error("Could not borrow listeners to call");
            panic!();
        };
        // If any listeners were dropped, replace the old list with the updated one
        if dropped_listener {
            ConsoleService::log("Replacing listeners");
            self.listeners.replace(filtered_listeners.clone());
        };
        // Now call those listeners, note that the borrow of the master list has been
        // relinquished at this point so that the if any of the emits cause a new
        // subscribe/update call, they will be able to borrow successfully
        ConsoleService::log("Calling listeners");
        for weak_listener in filtered_listeners {
            match weak_listener.upgrade() {
                Some(listener) => listener.emit(Rc::clone(&self.item)),
                None => ConsoleService::error("A listener was dropped before it could be called"),
            }
        }
        ConsoleService::log("Listeners called");
    }

    /// Call this to update the underlying item
    /// * run_update: The function that will update the item and return a some
    /// if an update occurred, or none in any other case.
    pub fn update<F>(self: &Self, run_update: F) -> bool
    where
        F: FnOnce(&mut T) -> bool
    {
        ConsoleService::log(format!("update refs: {}", Rc::strong_count(&self.item)).as_str());
        let item_was_updated = if let Ok(mut mut_item) = self.item.try_borrow_mut() {
            run_update(&mut mut_item)
        } else {
            ConsoleService::error("Could not borrow item to update");
            false
        };

        if item_was_updated {
            self.call_listeners();
        };
        item_was_updated
    }

    /// Call this to set the value of the underlying item
    /// * value: The new value
    /// if an update occurred, false in any other case.
    pub fn set(self: &Self, value: T) {
        ConsoleService::log(format!("set refs: {}", Rc::strong_count(&self.item)).as_str());
        self.item.replace(value);
        self.call_listeners();
    }

    /// Call to subscribe to changes, returns a pointer to the underlying
    /// data which is non-mutable as it is owned by the store.
    /// * callback: A callback for when the item is mutated
    /// * call_now: true if the callback should be called immediately with
    /// the current value of the item
    pub fn subscribe(self: &Self, callback: Callback<ItemPtr<T>>, call_now: bool) -> StoreListener<T> {
        ConsoleService::log("Subscribing");
        let store_listener: StoreListener<T> = Rc::new(callback);
        ConsoleService::log(format!("sub refs: {}", Rc::strong_count(&self.item)).as_str());
        if call_now {
            store_listener.emit(Rc::clone(&self.item));
        }
        let weak_listener: WeakStoreListener<T> = Rc::downgrade(&store_listener);
        match self.listeners.try_borrow_mut() {
            Ok(mut listeners) => {
                listeners.push(weak_listener);
            }
            Err(e) => {
                let msg = format!("Could not borrow listeners to add callback: {}", e);
                ConsoleService::error(&msg);
                panic!()
            }
        };
        ConsoleService::log("Subscribed");
        store_listener
    }
}