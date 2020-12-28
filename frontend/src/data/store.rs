use crate::data::*;
use data::user::User;
use std::rc::Rc;
use std::cell::{Cell, RefCell};

#[derive(Clone)]
pub struct UnwrappedStore {
    next_store_id : Cell<i32>,
    pub user: StoreItem<User>,
    // pub todo_tasks: StoreItem<TaskList>,
}

impl UnwrappedStore {
    pub fn new() -> Self {
        Self {
            next_store_id: Cell::new(0),
            user: StoreItem::default(),
            // todo_tasks: StoreItem::default(),
        }
    }
}

pub type Store = Rc<RefCell<UnwrappedStore>>;