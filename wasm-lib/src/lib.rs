mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: Option<String>) -> String {
    format!("Hello from Rust, {}!", name.unwrap_or_default())
}
