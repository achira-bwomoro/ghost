use chrono::NaiveDateTime;
use rusqlite::{Connection, Result, params};

pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub barcode: String,
    pub category: String,
    pub subcategories: Vec<String>,
    pub price: f64,
    pub quantity: i32,
    pub location: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

// Connect to the SQLite database and create the products table if it doesn't exist
pub fn connect() -> Result<Connection> {
    let conn = Connection::open("ghost.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            barcode TEXT NOT NULL,
            category TEXT NOT NULL,
            subcategories TEXT,
            price REAL NOT NULL,
            quantity INTEGER NOT NULL,
            location TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT
        )",
        [],
    )?;
    Ok(conn)
}

// Insert a product into the database
pub fn insert_product(conn: &Connection, product: &Product) -> Result<()> {
    conn.execute(
        "INSERT INTO products (name, description, barcode, category, subcategories, price, quantity, location, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            product.name,
            product.description,
            product.barcode,
            product.category,
            product.subcategories.join(","),
            product.price,
            product.quantity,
            product.location,
            product.created_at.to_string(),
            product.updated_at.as_ref().map(|dt| dt.to_string()),
        ],
    )?;
    Ok(())
}

// Read all products from the database
pub fn read_products(conn: &Connection) -> Result<Vec<Product>> {
    let mut stmt = conn.prepare("SELECT id, name, description, barcode, category, subcategories, price, quantity, location, created_at, updated_at FROM products")?;
    let product_iter = stmt.query_map([], |row| {
        let subcategories: String = row.get(5)?;
        Ok(Product {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            barcode: row.get(3)?,
            category: row.get(4)?,
            subcategories: subcategories.split(',').map(|s| s.to_string()).collect(),
            price: row.get(6)?,
            quantity: row.get(7)?,
            location: row.get(8)?,
            created_at: NaiveDateTime::parse_from_str(
                &row.get::<_, String>(9)?,
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
            updated_at: match row.get::<_, Option<String>>(10)? {
                Some(dt) => Some(NaiveDateTime::parse_from_str(&dt, "%Y-%m-%d %H:%M:%S").unwrap()),
                None => None,
            },
        })
    })?;
    let mut products = Vec::new();
    for product in product_iter {
        products.push(product?);
    }
    Ok(products)
}

// Read a single product by ID
pub fn read_product_by_id(conn: &Connection, product_id: i32) -> Result<Option<Product>> {
    let mut stmt = conn.prepare("SELECT id, name, description, barcode, category, subcategories, price, quantity, location, created_at, updated_at FROM products WHERE id = ?1")?;
    let mut rows = stmt.query(params![product_id])?;
    if let Some(row) = rows.next()? {
        let subcategories: String = row.get(5)?;
        Ok(Some(Product {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            barcode: row.get(3)?,
            category: row.get(4)?,
            subcategories: subcategories.split(',').map(|s| s.to_string()).collect(),
            price: row.get(6)?,
            quantity: row.get(7)?,
            location: row.get(8)?,
            created_at: chrono::NaiveDateTime::parse_from_str(
                &row.get::<_, String>(9)?,
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
            updated_at: match row.get::<_, Option<String>>(10)? {
                Some(dt) => {
                    Some(chrono::NaiveDateTime::parse_from_str(&dt, "%Y-%m-%d %H:%M:%S").unwrap())
                }
                None => None,
            },
        }))
    } else {
        Ok(None)
    }
}

// Update a product's quantity in the database
pub fn update_product_quantity(
    conn: &Connection,
    product_id: i32,
    new_quantity: i32,
) -> Result<()> {
    conn.execute(
        "UPDATE products SET quantity = ?1, updated_at = datetime('now') WHERE id = ?2",
        params![new_quantity, product_id],
    )?;
    Ok(())
}

// Update all fields of a product by ID
pub fn update_product(conn: &Connection, product: &Product) -> Result<()> {
    conn.execute(
        "UPDATE products SET name = ?1, description = ?2, barcode = ?3, category = ?4, subcategories = ?5, price = ?6, quantity = ?7, location = ?8, updated_at = datetime('now') WHERE id = ?9",
        params![
            product.name,
            product.description,
            product.barcode,
            product.category,
            product.subcategories.join(","),
            product.price,
            product.quantity,
            product.location,
            product.id,
        ],
    )?;
    Ok(())
}

// Delete a product by ID
pub fn delete_product(conn: &Connection, product_id: i32) -> Result<()> {
    conn.execute("DELETE FROM products WHERE id = ?1", params![product_id])?;
    Ok(())
}
