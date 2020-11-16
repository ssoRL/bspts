#![recursion_limit="256"]

mod pages;
mod apis;
mod components;

use pages::Home;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Home>::new().mount_to_body();
}