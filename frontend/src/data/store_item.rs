use yew::prelude::*;
use yew::services::ConsoleService;
use std::collections::HashMap;
use std::cell::{RefCell, Ref};
use std::rc::{Rc, Weak};

pub type ItemPtr<T> = Rc<RefCell<T>>;
pub type StoreListener<T> = Rc<Callback<ItemPtr<T>>>;
type WeakStoreListener<T> = Weak<Callback<ItemPtr<T>>>;

/// An item in the store that can be acted on
#[derive(Clone)]
pub struct StoreItem<T> {
    /// The item itself
    item: ItemPtr<T>,
    /// A list of callbacks to run when the item is mutated
    listeners: Vec<WeakStoreListener<T>>,
}

impl<T> StoreItem<T>
where
    T: Default
{
    pub fn new(data: T) -> StoreItem<T> {
        StoreItem {
            item: Rc::new(RefCell::new(data)),
            listeners: vec![],
        }
    }

    pub fn default() -> StoreItem<T> {
        StoreItem {
            item: StoreItem::new_ptr(),
            listeners: vec![],
        }
    }

    pub fn new_ptr() -> ItemPtr<T> {
        Rc::new(RefCell::new(T::default()))
    }

    /// Call this to update the underlying item
    /// * run_update: The function that will update the item and return true
    /// if an update occurred, false in any other case.
    pub fn update<F>(self: &mut Self, run_update: F)
    where
        F: FnOnce(&mut T) -> bool
    {
        ConsoleService::log(format!("update refs: {}", Rc::strong_count(&self.item)).as_str());
        let item_was_updated = if let Ok(mut mut_item) = self.item.try_borrow_mut() {
            run_update(&mut mut_item)
        } else {
            ConsoleService::error("Failed to take a mutable reference on the item");
            false
        };

        if item_was_updated {
            // Call each listener
            let filtered_listeners = self.listeners.iter().filter_map(|weak_listener| {
                if let Some(listener) = weak_listener.upgrade() {
                    listener.emit(Rc::clone(&self.item));
                    Some(Weak::clone(weak_listener))
                } else {
                    // if a listener has been destroyed, remove it from the list
                    None
                }
            }).collect();
            self.listeners = filtered_listeners;
            // for weak_listener in self.listeners {
            //     let listener = weak_listener.upgrade()
            //     listener.1.emit(Rc::clone(&self.item));
            // }
        }
    }

    /// Call this to set the value of the underlying item
    /// * value: The new value
    /// if an update occurred, false in any other case.
    pub fn set(self: &Self, value: T) {
        ConsoleService::log(format!("set refs: {}", Rc::strong_count(&self.item)).as_str());
        self.item.replace(value);
    }

    /// Call to subscribe to changes, returns a pointer to the underlying
    /// data which is non-mutable as it is owned by the store.
    /// * callback: A callback for when the item is mutated
    /// * call_now: true if the callback should be called immediately with
    /// the current value of the item
    pub fn subscribe(self: &mut Self, callback: &StoreListener<T>, call_now: bool) {
        ConsoleService::log(format!("sub refs: {}", Rc::strong_count(&self.item)).as_str());
        if call_now {
            callback.emit(Rc::clone(&self.item));
        }
        let weak_listener: WeakStoreListener<T> = Rc::downgrade(callback);
        self.listeners.push(weak_listener);
    }
}