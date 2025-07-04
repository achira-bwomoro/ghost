use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}! 🚀")
}

#[wasm_bindgen]
pub fn addition(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn current_time() -> String {
    use chrono::prelude::*;
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}
