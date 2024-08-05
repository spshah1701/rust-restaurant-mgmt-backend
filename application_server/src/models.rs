// src/models.rs
use rusqlite::params;
use rusqlite::Connection;
use serde;
use serde::{Deserialize, Serialize};

/// Represents the state of the Restaurant
#[derive(Debug, Serialize, Deserialize)]
pub struct RestaurantState {
    pub tables: Vec<TableResponse>,
    pub menus: Vec<MenuResponse>,
    pub orders: Vec<OrderResponse>,
}

/// Represents a table creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    #[serde(skip)]
    pub id: i64,
    pub code: String,
}

/// Represents a response containing table details
#[derive(Debug, Serialize, Deserialize)]
pub struct TableResponse {
    pub id: i64,
    pub code: String,
}

/// Represents a menu creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct Menu {
    #[serde(skip)]
    pub id: i64,
    pub name: String,
}

/// Represents a response containing menu details
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuResponse {
    pub id: i64,
    pub name: String,
}

/// Represents an order creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRequestBody {
    pub table_id: i64,
    pub menu_ids: Vec<i64>,
}

/// Represents a response containing order details
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: i64,
    pub table_id: i64,
    pub table_name: String,
    pub total_cooking_time: i32, // Total cooking time calculated from order items
    pub menus: Vec<OrderItemResponse>,
}

/// Represents an order item creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItem {
    #[serde(skip)]
    pub id: i64,
    pub order_id: i64,
    pub menu_id: i64,
    pub cooking_tme: i64,
}

/// Represents a response containing order item details
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemResponse {
    pub id: i64,
    pub order_id: i64,
    pub menu_id: i64,
    pub menu_name: String,
    pub cooking_time: i64,
    pub quantity: i64,
}

/// Function to get the current state of the Restaurant
pub fn get_current_state(conn: &Connection) -> Result<RestaurantState, rusqlite::Error> {
    let tables = Table::list(conn)?;
    let menus = Menu::list(conn)?;
    let orders = OrderResponse::list(conn)?;

    Ok(RestaurantState {
        tables,
        menus,
        orders,
    })
}

/// Functions for managing Table records
impl Table {
    /// Create a new table
    pub fn create(conn: &rusqlite::Connection, table: &Table) -> rusqlite::Result<i64> {
        conn.execute("INSERT INTO tables (code) VALUES (?1)", params![table.code])?;
        // Retrieve the ID of the last inserted row
        let last_inserted_id = conn.last_insert_rowid();
        Ok(last_inserted_id)
    }

    /// List all tables
    pub fn list(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<TableResponse>> {
        let mut stmt = conn.prepare("SELECT * FROM tables")?;
        let rows = stmt.query_map(params![], |row| {
            Ok(TableResponse {
                id: row.get(0)?,
                code: row.get(1)?,
            })
        })?;
        // Collect and return the results as a vector
        Ok(rows.map(|result| result.unwrap()).collect())
    }

    /// Get the ID of an existing table by its code
    pub fn get_existing_table_id(
        conn: &Connection,
        table: &Table,
    ) -> Result<Option<i64>, rusqlite::Error> {
        let query = "SELECT id FROM tables WHERE code = ?1";
        let mut stmt = conn.prepare(query)?;
        let mut rows = stmt.query(params![table.code])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }
}

/// Functions for managing Menu records
impl Menu {
    /// Create a new menu item
    pub fn create(conn: &rusqlite::Connection, menu: &Menu) -> rusqlite::Result<i64> {
        conn.execute("INSERT INTO menus (name) VALUES (?1)", params![menu.name])?;
        // Retrieve the ID of the last inserted row
        let last_inserted_id = conn.last_insert_rowid();
        Ok(last_inserted_id)
    }

    /// List all menu items
    pub fn list(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<MenuResponse>> {
        let mut stmt = conn.prepare("SELECT * FROM menus")?;
        let rows = stmt.query_map(params![], |row| {
            Ok(MenuResponse {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;
        // Collect and return the results as a vector
        Ok(rows.map(|result| result.unwrap()).collect())
    }

    /// Get the ID of an existing menu item by its name
    pub fn get_existing_menu_id(
        conn: &Connection,
        menu: &Menu,
    ) -> Result<Option<i64>, rusqlite::Error> {
        let query = "SELECT id FROM menus WHERE name = ?1";
        let mut stmt = conn.prepare(query)?;
        let mut rows = stmt.query(params![menu.name])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }
}

/// Functions for managing Order records
impl OrderResponse {
    /* CRUD Functions for Order Model */

    /// Create a new order
    pub fn create(conn: &rusqlite::Connection, table_id: i64) -> rusqlite::Result<i64> {
        conn.execute(
            "INSERT INTO orders (table_id) VALUES (?1)",
            params![table_id],
        )?;
        // Retrieve the ID of the last inserted row
        let last_inserted_id = conn.last_insert_rowid();
        Ok(last_inserted_id)
    }

    /// List all orders
    pub fn list(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<OrderResponse>> {
        let mut stmt = conn.prepare("SELECT orders.id, orders.table_id, t.code FROM orders JOIN tables as t on orders.table_id=t.id")?;
        let rows = stmt.query_map(params![], |row| {
            let order_response = OrderResponse {
                id: row.get(0)?,
                table_id: row.get(1)?,
                table_name: row.get(3)?,
                total_cooking_time: OrderResponse::calculate_total_cooking_time(conn, row.get(0)?)?, // Calculate total cooking time
                menus: OrderItem::list_all_order_items(conn, row.get(0)?)?,
            };
            Ok(order_response)
        })?;

        // Collect and return the results as a vector
        Ok(rows.map(|result| result.unwrap()).collect())
    }

    /* Utility Functions for Order Model. This block contains utility functions for the Order model */

    /// Get the existing order ID for a specific table, checking if there is an active order
    pub fn get_existing_order_id(
        conn: &Connection,
        table_id: i64,
    ) -> Result<Option<i64>, rusqlite::Error> {
        let query = "SELECT id FROM orders WHERE table_id = ?1";
        let mut stmt = conn.prepare(query)?;
        let mut rows = stmt.query(params![table_id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    /// Calculate the total cooking time dynamically from the current order items
    pub fn calculate_total_cooking_time(
        conn: &rusqlite::Connection,
        order_id: i64,
    ) -> rusqlite::Result<i32> {
        let query = "
        SELECT SUM(oi.cooking_time)
        FROM orders
        JOIN order_items oi ON oi.order_id = orders.id
        WHERE orders.id = ?1
    ";

        conn.query_row(query, params![order_id], |row| row.get(0))
    }

    /// Check if the order has any remaining items
    pub fn has_items(conn: &rusqlite::Connection, order_id: i64) -> rusqlite::Result<bool> {
        let query = "SELECT COUNT(*) FROM order_items WHERE order_id = ?";
        let count: i64 = conn.query_row(query, params![order_id], |row| row.get(0))?;
        Ok(count > 0)
    }
}

/// Functions for managing OrderItem records
impl OrderItem {
    /// Create a new order item
    pub fn create(
        conn: &rusqlite::Connection,
        order_id: i64,
        menu_id: i64,
        cooking_time: i64,
    ) -> rusqlite::Result<i64> {
        conn.execute(
            "INSERT INTO order_items (order_id, menu_id, cooking_time, quantity) VALUES (?1, ?2, ?3, ?4)",
            params![order_id, menu_id, cooking_time, 1],
        )?;
        // Retrieve the ID of the last inserted row
        let last_inserted_id = conn.last_insert_rowid();
        Ok(last_inserted_id)
    }

    /// List all order items for a specific order
    pub fn list_all_order_items(
        conn: &rusqlite::Connection,
        order_id: i64,
    ) -> rusqlite::Result<Vec<OrderItemResponse>> {
        let mut stmt = conn.prepare("SELECT order_items.id, order_items.order_id, order_items.menu_id, m.name, order_items.quantity, order_items.cooking_time FROM order_items JOIN menus as m on order_items.menu_id=m.id WHERE order_id = ?1")?;
        let rows = stmt.query_map(params![order_id], |row| {
            Ok(OrderItemResponse {
                id: row.get(0)?,
                order_id: row.get(1)?,
                menu_id: row.get(2)?,
                menu_name: row.get(3)?,
                quantity: row.get(4)?,
                cooking_time: row.get(5)?,
            })
        })?;
        // Collect and return the results as a vector
        let result: Result<Vec<_>, _> = rows.collect();
        result
    }

    /// List all order items for a specific table
    pub fn list_order_items(
        conn: &rusqlite::Connection,
        table_id: i64,
    ) -> rusqlite::Result<Vec<OrderItemResponse>> {
        let query = "SELECT order_items.id, order_items.order_id, order_items.menu_id, m.name, order_items.quantity, order_items.cooking_time
        FROM order_items
        JOIN orders ON orders.id = order_items.order_id
        JOIN menus as m on order_items.menu_id=m.id
        WHERE orders.table_id = ?1";
        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map(params![table_id], |row| {
            Ok(OrderItemResponse {
                id: row.get(0)?,
                order_id: row.get(1)?,
                menu_id: row.get(2)?,
                menu_name: row.get(3)?,
                quantity: row.get(4)?,
                cooking_time: row.get(5)?,
            })
        })?;
        // Collect and return the results as a vector
        let result: Result<Vec<_>, _> = rows.collect();
        result
    }

    /// Get a specific item from a specific table by menu ID
    pub fn get_item(
        conn: &rusqlite::Connection,
        table_id: i64,
        menu_id: i64,
    ) -> rusqlite::Result<Option<OrderItemResponse>> {
        let query = "
        SELECT order_items.id, order_items.order_id, order_items.menu_id, m.name, order_items.quantity, order_items.cooking_time
        FROM order_items
        JOIN orders ON orders.id = order_items.order_id
        JOIN menus as m on order_items.menu_id=m.id
        WHERE orders.table_id = ?1 AND order_items.menu_id = ?2";
        let mut stmt = conn.prepare(query)?;
        let result = stmt.query_row(params![table_id, menu_id], |row| {
            Ok(OrderItemResponse {
                id: row.get(0)?,
                order_id: row.get(1)?,
                menu_id: row.get(2)?,
                menu_name: row.get(3)?,
                quantity: row.get(4)?,
                cooking_time: row.get(5)?,
            })
        });
        // Return the result if found, otherwise handle the error
        match result {
            Ok(item) => Ok(Some(item)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(err) => Err(err),
        }
    }

    /* Utility functions for the OrderItem model */

    /// Get the existing order item ID for a given order and menu
    pub fn get_existing_order_item_id(
        conn: &Connection,
        order_id: i64,
        menu_id: i64,
    ) -> Result<Option<i64>, rusqlite::Error> {
        let query = "SELECT id FROM order_items WHERE order_id = ?1 AND menu_id = ?2";
        let mut stmt = conn.prepare(query)?;
        let mut rows = stmt.query(params![order_id, menu_id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    /// Increase the quantity of an existing order item
    pub fn add_quantity_of_existing_order_item(
        conn: &Connection,
        order_item_id: i64,
    ) -> Result<bool, rusqlite::Error> {
        let query = "UPDATE order_items
        SET cooking_time = (cooking_time / quantity) * (quantity + 1),
        quantity = quantity + 1
        WHERE id = ?1";
        let result = conn.execute(query, params![order_item_id])?;
        Ok(result > 0)
    }
}
