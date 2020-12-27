use crate::data::*;
use data::user::User;
use std::rc::Rc;
use std::cell::{Cell, RefCell};

#[derive(Clone)]
pub struct UnwrappedStore {
    next_store_id : Cell<i32>,
    pub user: StoreItem<User>,
    pub todo_tasks: StoreItem<TaskList>,
}

impl UnwrappedStore {
    pub fn new() -> Self {
        Self {
            next_store_id: Cell::new(0),
            user: StoreItem::default(),
            todo_tasks: StoreItem::default(),
        }
    }
    /// Gets an id for elements to use when subscribing to store items
    pub fn get_store_id(self: &Self) -> StoreID {
        let id = self.next_store_id.get();
        self.next_store_id.replace(id+1);
        id
    }
}

pub type Store = Rc<RefCell<UnwrappedStore>>;