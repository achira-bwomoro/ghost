use chrono::prelude::*;
use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
mod db;
use db::{
    Product, connect as db_connect, delete_product, insert_product, read_product_by_id,
    read_products, update_product, update_product_quantity,
};

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
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[derive(Serialize, Deserialize)]
pub struct JsProduct {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub barcode: String,
    pub category: String,
    pub subcategories: Vec<String>,
    pub price: f64,
    pub quantity: i32,
    pub location: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<Product> for JsProduct {
    fn from(p: Product) -> Self {
        JsProduct {
            id: p.id,
            name: p.name,
            description: p.description,
            barcode: p.barcode,
            category: p.category,
            subcategories: p.subcategories,
            price: p.price,
            quantity: p.quantity,
            location: p.location,
            created_at: p.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: p
                .updated_at
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

impl From<JsProduct> for Product {
    fn from(j: JsProduct) -> Self {
        Product {
            id: j.id,
            name: j.name,
            description: j.description,
            barcode: j.barcode,
            category: j.category,
            subcategories: j.subcategories,
            price: j.price,
            quantity: j.quantity,
            location: j.location,
            created_at: NaiveDateTime::parse_from_str(&j.created_at, "%Y-%m-%d %H:%M:%S").unwrap(),
            updated_at: j
                .updated_at
                .and_then(|dt| NaiveDateTime::parse_from_str(&dt, "%Y-%m-%d %H:%M:%S").ok()),
        }
    }
}

#[wasm_bindgen]
pub fn create_product(product: JsValue) -> Result<(), JsValue> {
    let js_product: JsProduct =
        from_value(product).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let conn = db_connect().map_err(|e| JsValue::from_str(&e.to_string()))?;
    insert_product(&conn, &Product::from(js_product))
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(())
}

#[wasm_bindgen]
pub fn get_all_products() -> Result<JsValue, JsValue> {
    let conn = db_connect().map_err(|e| JsValue::from_str(&e.to_string()))?;
    let products = read_products(&conn).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let js_products: Vec<JsProduct> = products.into_iter().map(JsProduct::from).collect();
    to_value(&js_products).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn get_product_by_id(product_id: i32) -> Result<JsValue, JsValue> {
    let conn = db_connect().map_err(|e| JsValue::from_str(&e.to_string()))?;
    let product =
        read_product_by_id(&conn, product_id).map_err(|e| JsValue::from_str(&e.to_string()))?;
    match product {
        Some(p) => to_value(&JsProduct::from(p)).map_err(|e| JsValue::from_str(&e.to_string())),
        None => Err(JsValue::from_str("Product not found")),
    }
}

#[wasm_bindgen]
pub fn update_product_wasm(product: JsValue) -> Result<(), JsValue> {
    let js_product: JsProduct =
        from_value(product).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let conn = db_connect().map_err(|e| JsValue::from_str(&e.to_string()))?;
    update_product(&conn, &Product::from(js_product))
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(())
}

#[wasm_bindgen]
pub fn update_product_quantity_wasm(product_id: i32, new_quantity: i32) -> Result<(), JsValue> {
    let conn = db_connect().map_err(|e| JsValue::from_str(&e.to_string()))?;
    update_product_quantity(&conn, product_id, new_quantity)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(())
}

#[wasm_bindgen]
pub fn delete_product_wasm(product_id: i32) -> Result<(), JsValue> {
    let conn = db_connect().map_err(|e| JsValue::from_str(&e.to_string()))?;
    delete_product(&conn, product_id).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(())
}
