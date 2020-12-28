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

pub enum StoreAction {
    SetUser(User),
}

impl UnwrappedStore {
    pub fn new() -> Self {
        Self {
            next_store_id: Cell::new(0),
            user: StoreItem::default(),
            // todo_tasks: StoreItem::default(),
        }
    }

    pub fn act(self: &Self, action: StoreAction) {
        match action {
            StoreAction::SetUser(user) => {
                self.user.set(user)
            },
        }
    }
}

pub type Store = Rc<RefCell<UnwrappedStore>>;