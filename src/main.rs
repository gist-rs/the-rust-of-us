#![recursion_limit = "1024"]
mod web;

use bevy::log::debug;
use console_error_panic_hook::set_once as set_panic_hook;
use the_rust_of_us::{entry, get_public_key};
use wasm_bindgen::prelude::*;
use web::local_storage::{get_local_storage_value, set_local_storage_value};
use web_sys::window;

#[wasm_bindgen(inline_js = "export function snippetTest() { console.log('Hello from JS FFI!'); }")]
extern "C" {
    fn snippetTest();
}

#[wasm_bindgen]
pub fn wasm_ffi() {
    web_sys::console::log_1(&"Hello from WASM!".into());
}

// #[wasm_bindgen]
// pub fn set_public_key(public_key: &str) {
//     web_sys::console::log_1(&format!("public_key: {:?}", public_key).into());
//     debug!("public_key: {:?}", public_key);
// }

fn start_app() {
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access document");
    let body = document.body().expect("Could not access document.body");
    let text_node = document.create_text_node("Hello, world from Vanilla Rust!");
    body.append_child(text_node.as_ref())
        .expect("Failed to append text");
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        set_panic_hook();
        snippetTest();
        // start_app();
        // set_public_key("baz");
        debug!("public_key:...");
        debug!("public_key:{:?}", get_public_key());
    }
    entry();
}
