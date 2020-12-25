use crate::pages::MsgFromTask;
use crate::components::TaskItem;
use std::collections::VecDeque;
use data::task::Task;
use yew::prelude::*;
// use yew::services::ConsoleService;

type BoxT = Box<Task>;

/// A list of tasks with some methods to control it and return html
#[derive(Clone)]
pub struct TaskList {
    tasks_o: Option<VecDeque<BoxT>>
}

impl TaskList {
    pub fn new() -> Self {
        Self {tasks_o: None}
    }

    /// Instantiate a new Tasks from a vec of tasks
    pub fn from_vec(tasks_vec: Vec<Task>) -> Self {
        let boxed: Vec<BoxT> = tasks_vec.iter().map(|task| -> BoxT {Box::new(task.clone())}).collect();
        Self {tasks_o: Some(boxed.into())}
    }

    /// Add a task to the front of the list of tasks
    pub fn push(self: &mut Self, task: BoxT) {
        match &mut self.tasks_o {
            Some(tasks) => tasks.push_front(task),
            None => self.tasks_o = Some(vec![task].into()),
        }
    }

    /// Removes the task with the specified id.
    /// Returns the removed task on success
    pub fn remove(self: &mut Self, task_id: i32) -> Option<BoxT> {
        let tasks = self.tasks_o.as_mut()?;
        let task_index = tasks.iter().position(|t| t.id == task_id)?;
        tasks.remove(task_index)
    }

    pub fn is_unset(self: &Self) -> bool {
        self.tasks_o.is_none()
    }

    /// Converts these tasks to html 
    pub fn to_html<>(self: &Self, msg_handler: &Callback<MsgFromTask>) -> Html
    {
        match &self.tasks_o {
            Some(tasks) => tasks.iter().map(|task| {
                    html!{
                    <TaskItem
                        task={task}
                        msg_up={msg_handler}
                    />
                }
            }).collect(),
            None => html!{<></>},
        }
    }
}