use yew::services::storage::{StorageService,Area};
use yew::services::console::{ConsoleService};
use crate::types::{Task};

pub fn get_tasks() -> Result<Vec<Task>, ()> {
    // TODO: More safe way to handle than unwrap??
    let storage_service = StorageService::new(Area::Local).unwrap();
    let from_storage = StorageService::restore(&storage_service, "bspts_tasks");

    let tasks = match from_storage {
        Ok(serialized_tasks) => {
            let deserialize_result: Result<Vec<Task>, _> = serde_json::from_str(&serialized_tasks);
            return match deserialize_result {
                Ok(tasks) => Ok(tasks),
                _ => {
                    ConsoleService::info("Failed to deserialize");
                    Err(())
                }
            }
        },
        _ => {
            ConsoleService::info("Failed to get from memory");
            Err(())
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