use yew::prelude::*;
use yew::services::ConsoleService;
use std::rc::Rc;
use std::collections::HashMap;
use crate::data_store::StoreID;

/// An item in the store that can be acted on
#[derive(Clone)]
pub struct StoreItem<T> {
    /// The item itself
    item: Rc<T>,
    /// A list of callbacks to run when the item is mutated
    listeners: HashMap<StoreID, Callback<Rc<T>>>,
    /// The next_id to use
    next_id: StoreID,
}

impl<T> StoreItem<T> {
    pub fn new(data: T) -> StoreItem<T> {
        StoreItem {
            item: Rc::new(data),
            listeners: HashMap::new(),
            next_id: 0,
        }
    }

    /// Call this to update the underlying item
    /// * run_update: The function that will update the item and return true
    /// if an update occurred, false in any other case.
    pub fn update<F>(self: &mut Self, run_update: F)
    where
        F: FnOnce(&mut T) -> bool
    {
        ConsoleService::log(format!("update refs: {}", Rc::strong_count(&self.item)).as_str());
        if let Some(mut_item) = Rc::get_mut(&mut self.item) {
            if run_update(mut_item) {
                for listener in &self.listeners {
                    listener.1.emit(Rc::clone(&self.item));
                }
            }
        } else {
            ConsoleService::error("Failed to take a mutable reference on the item")
        }
    }

    /// Call to subscribe to changes, returns a pointer to the underlying
    /// data which is non-mutable as it isowned by the store.
    /// * callback: A callback for when the item is mutated
    /// * call_now: true if the callback should be called immediately with
    /// the current value of the item
    /// * Return: The id of this item in the store. The listener is responsible
    /// for unsubscribing during its destroy method.
    pub fn subscribe(self: &mut Self, callback: Callback<Rc<T>>, call_now: bool) -> StoreID {
        ConsoleService::log(format!("sub refs: {}", Rc::strong_count(&self.item)).as_str());
        if call_now {
            callback.emit(Rc::clone(&self.item));
        }
        let id = self.next_id;
        self.next_id += 1;
        self.listeners.insert(id, callback);
        id
    }

    /// remove self from the list of callbacks
    pub fn unsubscribe(self: &mut Self, store_id: StoreID) {
        ConsoleService::log(format!("unsub refs: {}", Rc::strong_count(&self.item)).as_str());
        self.listeners.remove(&store_id);
    }
}