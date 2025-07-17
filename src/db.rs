use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InventoryItem {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub quantity: i32,
    pub price: f64,
}
impl InventoryItem {
    pub async fn create(item: &InventoryItem) -> Result<InventoryItem, String> {
        let query = format!(
            r#"
            mutation {{
                createInventoryItem(input: {{
                    name: "{}",
                    description: "{}",
                    quantity: {},
                    price: {}
                }}) {{
                    id
                    name
                    description
                    quantity
                    price
                }}
            }}
            "#,
            item.name, item.description, item.quantity, item.price
        );

        let resp = Request::post("http://localhost:3000/graphql")
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"query":"{}"}}"#, query.replace('\n', " ")))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json = resp
            .json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())?;
        let data = json["data"]["createInventoryItem"].clone();
        serde_json::from_value(data).map_err(|e| e.to_string())
    }

    pub async fn read(id: &str) -> Result<InventoryItem, String> {
        let query = format!(
            r#"
            query {{
                inventoryItem(id: "{}") {{
                    id
                    name
                    description
                    quantity
                    price
                }}
            }}
            "#,
            id
        );

        let resp = Request::post("http://localhost:3000/graphql")
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"query":"{}"}}"#, query.replace('\n', " ")))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json = resp
            .json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())?;
        let data = json["data"]["inventoryItem"].clone();
        serde_json::from_value(data).map_err(|e| e.to_string())
    }

    pub async fn update(item: &InventoryItem) -> Result<InventoryItem, String> {
        let id = item.id.as_ref().ok_or("Missing id")?;
        let query = format!(
            r#"
            mutation {{
                updateInventoryItem(id: "{}", input: {{
                    name: "{}",
                    description: "{}",
                    quantity: {},
                    price: {}
                }}) {{
                    id
                    name
                    description
                    quantity
                    price
                }}
            }}
            "#,
            id, item.name, item.description, item.quantity, item.price
        );

        let resp = Request::post("http://localhost:3000/graphql")
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"query":"{}"}}"#, query.replace('\n', " ")))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json = resp
            .json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())?;
        let data = json["data"]["updateInventoryItem"].clone();
        serde_json::from_value(data).map_err(|e| e.to_string())
    }

    pub async fn delete(id: &str) -> Result<bool, String> {
        let query = format!(
            r#"
            mutation {{
                deleteInventoryItem(id: "{}") {{
                    id
                }}
            }}
            "#,
            id
        );

        let resp = Request::post("http://localhost:3000/graphql")
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"query":"{}"}}"#, query.replace('\n', " ")))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json = resp
            .json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())?;
        Ok(json["data"]["deleteInventoryItem"]["id"].is_string())
    }
}
