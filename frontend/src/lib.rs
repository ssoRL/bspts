#![recursion_limit="1024"]

mod pages;
mod apis;
mod app;
mod components;
mod data;
mod fontable;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}