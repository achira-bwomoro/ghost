use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

// namespace and database configuration
const NAMESPACE: &str = "ghost";
const DATABASE: &str = "ghost_db";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InventoryItem {
    pub item_id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub quantity: u32,
    pub unit_price: f64,
    pub total_value: f64,
    pub supplier: String,
    pub sku: String,
    pub location: String,
    pub reorder_level: u32,
    pub expiration_date: Option<String>,
    pub weight: Option<f64>,
    pub dimensions: Option<(f64, f64, f64)>,
    pub condition: String,
    pub barcode: Option<String>,
}

impl InventoryItem {
    pub async fn create(item: &InventoryItem) -> Result<InventoryItem, String> {
        let dimensions_str = item
            .dimensions
            .map(|(l, w, h)| format!("[{}, {}, {}]", l, w, h))
            .unwrap_or("null".to_string());

        let expiration_date_str = item
            .expiration_date
            .as_ref()
            .map(|d| format!(r#""{}""#, d))
            .unwrap_or("null".to_string());

        let weight_str = item
            .weight
            .map(|w| w.to_string())
            .unwrap_or("null".to_string());

        let barcode_str = item
            .barcode
            .as_ref()
            .map(|b| format!(r#""{}""#, b))
            .unwrap_or("null".to_string());

        let query = format!(
            r#"
            mutation {{
                create_inventory_item(data: {{
                    item_id: "{item_id}",
                    name: "{name}",
                    description: "{description}",
                    category: "{category}",
                    quantity: {quantity},
                    unit_price: {unit_price},
                    total_value: {total_value},
                    supplier: "{supplier}",
                    sku: "{sku}",
                    location: "{location}",
                    reorder_level: {reorder_level},
                    expiration_date: {expiration_date_str},
                    weight: {weight_str},
                    dimensions: {dimensions_str},
                    condition: "{condition}",
                    barcode: {barcode_str}
                }}) {{
                    item_id
                    name
                    description
                    category
                    quantity
                    unit_price
                    total_value
                    supplier
                    sku
                    location
                    reorder_level
                    expiration_date
                    weight
                    dimensions
                    condition
                    barcode
                }}
            }}
            "#,
            item_id = item.item_id,
            name = item.name,
            description = item.description,
            category = item.category,
            quantity = item.quantity,
            unit_price = item.unit_price,
            total_value = item.total_value,
            supplier = item.supplier,
            sku = item.sku,
            location = item.location,
            reorder_level = item.reorder_level,
            expiration_date_str = expiration_date_str,
            weight_str = weight_str,
            dimensions_str = dimensions_str,
            condition = item.condition,
            barcode_str = barcode_str,
        );

        let resp = Request::post("http://localhost:3001/graphql")
            .header("Accept", "application/json")
            .header("surreal-ns", NAMESPACE)
            .header("surreal-db", DATABASE)
            .body(format!(r#"{{"query":"{}"}}"#, query.replace('\n', " ")))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json = resp
            .json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())?;
        let data = json["data"]["create_inventory_item"].clone();
        serde_json::from_value(data).map_err(|e| e.to_string())
    }

    pub async fn read(item_id: &str) -> Result<InventoryItem, String> {
        let query = format!(
            r#"
            query {{
                inventory_item_by_id(id: "{item_id}") {{
                    item_id
                    name
                    description
                    category
                    quantity
                    unit_price
                    total_value
                    supplier
                    sku
                    location
                    reorder_level
                    expiration_date
                    weight
                    dimensions
                    condition
                    barcode
                }}
            }}
            "#,
            item_id = item_id
        );

        let resp = Request::post("http://localhost:3001/graphql")
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"query":"{}"}}"#, query.replace('\n', " ")))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json = resp
            .json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())?;
        let data = json["data"]["inventory_item_by_id"].clone();
        serde_json::from_value(data).map_err(|e| e.to_string())
    }

    pub async fn update(item: &InventoryItem) -> Result<InventoryItem, String> {
        let dimensions_str = item
            .dimensions
            .map(|(l, w, h)| format!("[{}, {}, {}]", l, w, h))
            .unwrap_or("null".to_string());

        let expiration_date_str = item
            .expiration_date
            .as_ref()
            .map(|d| format!(r#""{}""#, d))
            .unwrap_or("null".to_string());

        let weight_str = item
            .weight
            .map(|w| w.to_string())
            .unwrap_or("null".to_string());

        let barcode_str = item
            .barcode
            .as_ref()
            .map(|b| format!(r#""{}""#, b))
            .unwrap_or("null".to_string());

        let query = format!(
            r#"
            mutation {{
                update_inventory_item(id: "{item_id}", data: {{
                    name: "{name}",
                    description: "{description}",
                    category: "{category}",
                    quantity: {quantity},
                    unit_price: {unit_price},
                    total_value: {total_value},
                    supplier: "{supplier}",
                    sku: "{sku}",
                    location: "{location}",
                    reorder_level: {reorder_level},
                    expiration_date: {expiration_date_str},
                    weight: {weight_str},
                    dimensions: {dimensions_str},
                    condition: "{condition}",
                    barcode: {barcode_str}
                }}) {{
                    item_id
                    name
                    description
                    category
                    quantity
                    unit_price
                    total_value
                    supplier
                    sku
                    location
                    reorder_level
                    expiration_date
                    weight
                    dimensions
                    condition
                    barcode
                }}
            }}
            "#,
            item_id = item.item_id,
            name = item.name,
            description = item.description,
            category = item.category,
            quantity = item.quantity,
            unit_price = item.unit_price,
            total_value = item.total_value,
            supplier = item.supplier,
            sku = item.sku,
            location = item.location,
            reorder_level = item.reorder_level,
            expiration_date_str = expiration_date_str,
            weight_str = weight_str,
            dimensions_str = dimensions_str,
            condition = item.condition,
            barcode_str = barcode_str,
        );

        let resp = Request::post("http://localhost:3001/graphql")
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"query":"{}"}}"#, query.replace('\n', " ")))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json = resp
            .json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())?;
        let data = json["data"]["update_inventory_item"].clone();
        serde_json::from_value(data).map_err(|e| e.to_string())
    }

    pub async fn delete(item_id: &str) -> Result<bool, String> {
        let query = format!(
            r#"
            mutation {{
                delete_inventory_item(id: "{item_id}")
            }}
            "#,
            item_id = item_id
        );

        let resp = Request::post("http://localhost:3001/graphql")
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"query":"{}"}}"#, query.replace('\n', " ")))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json = resp
            .json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())?;
        Ok(json["data"]["delete_inventory_item"]
            .as_bool()
            .unwrap_or(false))
    }
}
