use crate::data::*;
use data::{
    user::User,
    task::Task,
    reward::Reward,
};
use std::rc::Rc;
use std::cell::{Cell};
use yew::services::ConsoleService;
use std::collections::VecDeque;

pub type Store = Rc<UnwrappedStore>;

#[derive(Clone)]
pub struct UnwrappedStore {
    next_store_id : Cell<i32>,
    pub session_user: StoreItem<Option<User>>,
    pub todo_tasks: StoreItem<TaskList>,
    pub done_tasks: StoreItem<TaskList>,
    pub rewards: StoreItem<VecDeque<Reward>>,
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
    /// Delete the task with the specified id
    DeleteTask(i32),
    SetRewards(Vec<Reward>),
    DeleteReward(i32),
}

impl UnwrappedStore {
    pub fn new() -> Self {
        Self {
            next_store_id: Cell::new(0),
            session_user: StoreItem::default(),
            todo_tasks: StoreItem::default(),
            done_tasks: StoreItem::default(),
            rewards: StoreItem::default(),
        }
    }

    pub fn act(self: &Self, action: StoreAction) {
        match action {
            StoreAction::StartSession(user) => {
                ConsoleService::log("Starting Session");
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
            StoreAction::DeleteTask(task_id) => {
                let remove_task = move |tasks: &mut TaskList| {
                    tasks.remove(task_id).is_some()
                };
                if self.todo_tasks.update(remove_task) {
                    // Deleted from the todo list
                    ()
                } else if self.done_tasks.update(remove_task) {
                    // Deleted from the done list
                    ()
                } else {
                    let err_msg = format!("Could not find task {} to remove", task_id);
                    ConsoleService::error(&err_msg);
                    ()
                }
            }
            StoreAction::SetRewards(rewards) => {
                let dq_rewards: VecDeque<Reward> = rewards.into();
                self.rewards.set(dq_rewards);
            }
            StoreAction::DeleteReward(reward_id) => {
                let remove_reward = move |rewards: &mut VecDeque<Reward>| {
                    match rewards.iter().position(|r| r.id == reward_id) {
                        Some(i) => rewards.remove(i).is_some(),
                        None => false,
                    }
                };
                if self.rewards.update(remove_reward) {
                    // Deleted from the todo list
                    ()
                } else {
                    let err_msg = format!("Could not find task {} to remove", reward_id);
                    ConsoleService::error(&err_msg);
                }
            }
        }
    }
}