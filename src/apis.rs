use yew::services::storage::{StorageService,Area};
use yew::services::console::{ConsoleService};
use crate::types::{Task};

pub fn get_tasks() -> Vec<Task> {
    // TODO: More safe way to handle than unwrap??
    let storage_service = StorageService::new(Area::Local).unwrap();
    let from_storage = StorageService::restore(&storage_service, "bspts_tasks");

    let default_tasks = vec![
        Task {
            id: 1,
            name: "Music".to_string(),
            description: "".to_string(),
            checked: false,
            daily_loss: 4
        },
        Task {
            id: 2,
            name: "Reading".to_string(),
            description: "At least one chapter of physical media".to_string(),
            checked: false,
            daily_loss: 8
        }
    ];

    let tasks = match from_storage {
        Ok(serialized_tasks) => {
            let deserialize_result: Result<Vec<Task>, _> = serde_json::from_str(&serialized_tasks);
            return match deserialize_result {
                Ok(tasks) => tasks,
                _ => {
                    ConsoleService::info("Failed to deserialize");
                    default_tasks
                }
            }
        },
        _ => {
            ConsoleService::info("Failed to get from memory");
            default_tasks
        }
    };

    return tasks;
}


pub fn set_tasks(tasks: &Vec<Task>) -> () {
    // TODO: More safe way to handle than unwrap??
    let mut storage_service = StorageService::new(Area::Local).unwrap();
    let serialized_tasks_result = serde_json::to_string(&tasks);
    match serialized_tasks_result {
        Ok(serialized_tasks) => {
            StorageService::store(&mut storage_service, "bspts_tasks", Ok(serialized_tasks));
        },
        _ => { 
            ConsoleService::info("Error storing tasks")
        }
    }
}