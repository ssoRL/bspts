use yew::prelude::*;
use yew::services::ConsoleService;
use std::collections::HashMap;
use std::cell::{RefCell};
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
    pub fn default() -> StoreItem<T> {
        StoreItem {
            item: StoreItem::new_ptr(),
            listeners: vec![],
        }
    }

    pub fn new_ptr() -> ItemPtr<T> {
        Rc::new(RefCell::new(T::default()))
    }

    /// Calls all of the listeners with a new value, if a listener has been dropped
    /// and can't upgrade, remove it from the list of listeners
    fn call_listeners(self: &mut Self) {
        ConsoleService::log(format!("Calling {} listeners", self.listeners.len()).as_str());
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
    }

    /// Call this to update the underlying item
    /// * run_update: The function that will update the item and return a some
    /// if an update occurred, or none in any other case.
    pub fn update<F, R>(self: &mut Self, run_update: F) -> Option<R>
    where
        F: Fn(&mut T) -> Option<R>
    {
        ConsoleService::log(format!("update refs: {}", Rc::strong_count(&self.item)).as_str());
        let item_was_updated = if let Ok(mut mut_item) = self.item.try_borrow_mut() {
            run_update(&mut mut_item)
        } else {
            ConsoleService::error("Failed to take a mutable reference on the item");
            None
        };

        if item_was_updated.is_some() {
            self.call_listeners();
        };
        item_was_updated
    }

    /// Call this to set the value of the underlying item
    /// * value: The new value
    /// if an update occurred, false in any other case.
    pub fn set(self: &mut Self, value: T) {
        ConsoleService::log(format!("set refs: {}", Rc::strong_count(&self.item)).as_str());
        self.item.replace(value);
        self.call_listeners();
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