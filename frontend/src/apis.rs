use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::console::{ConsoleService};
use types::task::{Task};

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get_tasks(callback: FetchCallback<Vec<Task>>) -> FetchTask {
    // Send out the fetch to populate the word from hi
    let get = Request::get("/task").body(Nothing).unwrap();
    FetchService::fetch(get, callback).unwrap()
}


// pub fn set_tasks(tasks: &Vec<Task>) -> () {
//     // TODO: More safe way to handle than unwrap??
//     let mut storage_service = StorageService::new(Area::Local).unwrap();
//     let serialized_tasks_result = serde_json::to_string(&tasks);
//     match serialized_tasks_result {
//         Ok(serialized_tasks) => {
//             StorageService::store(&mut storage_service, "bspts_tasks", Ok(serialized_tasks));
//         },
//         _ => { 
//             ConsoleService::info("Error storing tasks")
//         }
//     }
// }