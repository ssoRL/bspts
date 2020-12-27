use yew::prelude::*;
use yew::services::ConsoleService;
use std::collections::HashMap;
use crate::data::StoreID;
use std::cell::{RefCell, Ref};
use std::rc::Rc;

pub type ItemPtr<T> = Rc<RefCell<T>>;

/// An item in the store that can be acted on
#[derive(Clone)]
pub struct StoreItem<T> {
    /// The item itself
    item: ItemPtr<T>,
    /// A list of callbacks to run when the item is mutated
    listeners: RefCell<HashMap<StoreID, Callback<ItemPtr<T>>>>,
}

impl<T> StoreItem<T>
where
    T: Default
{
    pub fn new(data: T) -> StoreItem<T> {
        StoreItem {
            item: Rc::new(RefCell::new(data)),
            listeners: RefCell::new(HashMap::new()),
        }
    }

    pub fn default() -> StoreItem<T> {
        StoreItem {
            item: StoreItem::new_ptr(),
            listeners: RefCell::new(HashMap::new()),
        }
    }

    pub fn new_ptr() -> ItemPtr<T> {
        Rc::new(RefCell::new(T::default()))
    }

    /// Call this to update the underlying item
    /// * run_update: The function that will update the item and return true
    /// if an update occurred, false in any other case.
    pub fn update<F>(self: &Self, run_update: F)
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
            let listeners = self.listeners.borrow_mut();
            listeners.iter().for_each(|listener| listener.1.emit(Rc::clone(&self.item)));
            // for listener in listeners.into_iter() {
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
    pub fn subscribe(self: &Self, id: StoreID, callback: Callback<ItemPtr<T>>, call_now: bool) {
        ConsoleService::log(format!("sub refs: {}", Rc::strong_count(&self.item)).as_str());
        if call_now {
            callback.emit(Rc::clone(&self.item));
        }
        let mut mut_listeners = self.listeners.borrow_mut();
        mut_listeners.insert(id, callback);
    }

    /// remove self from the list of callbacks
    pub fn unsubscribe(self: &Self, store_id: StoreID) {
        ConsoleService::log(format!("unsub refs: {}", Rc::strong_count(&self.item)).as_str());
        let mut mut_listeners = self.listeners.borrow_mut();
        mut_listeners.remove(&store_id);
    }
}