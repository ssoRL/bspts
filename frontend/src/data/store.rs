use crate::data::*;
use data::{
    task::Task,
    user::User,
};
use std::rc::Rc;
use std::cell::{Cell};
use yew::services::ConsoleService;

pub type Store = Rc<UnwrappedStore>;

#[derive(Clone)]
pub struct UnwrappedStore {
    next_store_id : Cell<i32>,
    pub session_user: StoreItem<Option<User>>,
    pub todo_tasks: StoreItem<TaskList>,
    pub done_tasks: StoreItem<TaskList>,
}

/// The actions that the store can provide
pub enum StoreAction {
    /// Adds the user to the table
    StartSession(User),
    // Ends the session, logging out
    // EndSession,
    /// Takes a vec of tasks and stores it
    SetTasks{tasks: Vec<Task>, are_done: bool},
    /// Set the specified task as complete
    CompleteTask(i32),
    /// Delete the task with the specified id
    DeleteTask(i32),
}

impl UnwrappedStore {
    pub fn new() -> Self {
        Self {
            next_store_id: Cell::new(0),
            // session_user: RefCell::new(StoreItem::default()),
            // todo_tasks: RefCell::new(StoreItem::default()),
            // done_tasks: RefCell::new(StoreItem::default()),
            session_user: StoreItem::default(),
            todo_tasks: StoreItem::default(),
            done_tasks: StoreItem::default(),
        }
    }

    pub fn act(self: &Self, action: StoreAction) {
        match action {
            StoreAction::StartSession(user) => {
                self.session_user.set(Some(user))
            }
            // StoreAction::EndSession => {
            //     self.session_user.set(None)
            // }
            StoreAction::SetTasks{tasks, are_done} => {
                let task_list = TaskList::from_vec(tasks);
                if are_done {
                    ConsoleService::log("Setting done tasks");
                    self.done_tasks.set(task_list)
                } else {
                    ConsoleService::log("Setting todo tasks");
                    self.todo_tasks.set(task_list)
                }
            }
            StoreAction::CompleteTask(task_id) => {
                let remove_task = move |tasks: &mut TaskList| {
                    tasks.remove(task_id)
                };
                match self.todo_tasks.update(remove_task) {
                    Some(task) => {
                        self.done_tasks.update(move |tasks: &mut TaskList| {
                            tasks.push(task.clone());
                            Some(())
                        });
                        ()
                    }
                    None => {
                        let err_msg = format!("Could not find task {} to complete", task_id);
                        ConsoleService::error(&err_msg);
                    }
                }
            }
            StoreAction::DeleteTask(task_id) => {
                let remove_task = move |tasks: &mut TaskList| {
                    tasks.remove(task_id)
                };
                if self.todo_tasks.update(remove_task).is_some() {
                    // Deleted from the todo list
                    ()
                } else if self.done_tasks.update(remove_task).is_some() {
                    // Deleted from the done list
                    ()
                } else {
                    let err_msg = format!("Could not find task {} to remove", task_id);
                    ConsoleService::error(&err_msg);
                    ()
                }
            }
        }
    }
}