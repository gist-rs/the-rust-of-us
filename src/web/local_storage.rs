use wasm_bindgen::prelude::*;

pub fn get_local_storage() -> web_sys::Storage {
    web_sys::window()
        .expect("No window")
        .local_storage()
        .expect("Failed to get local storage")
        .expect("No local storage")
}

pub fn get_local_storage_value(key: &str) -> Option<String> {
    let local_storage = get_local_storage();
    local_storage.get_item(key).ok().flatten()
}

pub fn set_local_storage_value(key: &str, value: &str) {
    let local_storage = get_local_storage();
    local_storage
        .set_item(key, value)
        .expect("failed to set item");
}
