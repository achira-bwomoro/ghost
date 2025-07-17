use crate::db::InventoryItem;
use js_sys::Promise;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::future_to_promise;

use wasm_bindgen::prelude::*;
mod db;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}! 🚀")
}

#[wasm_bindgen]
pub fn addition(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn another_addition(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn create_inventory_item(item: JsValue) -> Promise {
    let item: InventoryItem = match from_value(item) {
        Ok(i) => i,
        Err(e) => return future_to_promise(async move { Err(JsValue::from_str(&e.to_string())) }),
    };
    future_to_promise(async move {
        match InventoryItem::create(&item).await {
            Ok(created) => to_value(&created).map_err(|e| JsValue::from_str(&e.to_string())),
            Err(e) => Err(JsValue::from_str(&e)),
        }
    })
}

#[wasm_bindgen]
pub fn read_inventory_item(id: String) -> Promise {
    future_to_promise(async move {
        match InventoryItem::read(&id).await {
            Ok(item) => to_value(&item).map_err(|e| JsValue::from_str(&e.to_string())),
            Err(e) => Err(JsValue::from_str(&e)),
        }
    })
}

#[wasm_bindgen]
pub fn update_inventory_item(item: JsValue) -> Promise {
    let item: InventoryItem = match from_value(item) {
        Ok(i) => i,
        Err(e) => return future_to_promise(async move { Err(JsValue::from_str(&e.to_string())) }),
    };
    future_to_promise(async move {
        match InventoryItem::update(&item).await {
            Ok(updated) => to_value(&updated).map_err(|e| JsValue::from_str(&e.to_string())),
            Err(e) => Err(JsValue::from_str(&e)),
        }
    })
}

#[wasm_bindgen]
pub fn delete_inventory_item(id: String) -> Promise {
    future_to_promise(async move {
        match InventoryItem::delete(&id).await {
            Ok(success) => Ok(JsValue::from_bool(success)),
            Err(e) => Err(JsValue::from_str(&e)),
        }
    })
}
