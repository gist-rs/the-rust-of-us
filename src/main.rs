#![recursion_limit = "1024"]

#[cfg(target_arch = "wasm32")]
use console_error_panic_hook::set_once as set_panic_hook;

use the_rust_of_us::entry;
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen(inline_js = "export function snippetTest() { console.log('Hello from JS FFI!'); }")]
extern "C" {
    fn snippetTest();
}

// TOFIX
#[wasm_bindgen]
pub fn wasm_ffi() {
    web_sys::console::log_1(&"Hello from WASM!".into());
}

// #[wasm_bindgen]
// pub fn set_public_key(public_key: &str) {
//     web_sys::console::log_1(&format!("public_key: {:?}", public_key).into());
//     debug!("public_key: {:?}", public_key);
// }

#[cfg(target_arch = "wasm32")]
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
    }
    entry();
}
