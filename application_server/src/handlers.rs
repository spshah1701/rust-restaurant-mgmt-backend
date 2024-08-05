use crate::models::{
    Menu, MenuResponse, OrderItem, OrderItemResponse, OrderRequestBody, OrderResponse, Table,
    TableResponse,
};
use rand::Rng;
use rusqlite::params;
use rusqlite::Connection;
use serde_json::json;
use warp;

// Handlers for Table operations

/// List all tables
pub async fn list_table_handler(conn: Connection) -> Result<impl warp::Reply, warp::Rejection> {
    match Table::list(&conn) {
        Ok(tables) => Ok(warp::reply::with_status(
            warp::reply::json(&tables),
            warp::http::StatusCode::OK,
        )),
        Err(_err) => {
            // If an error occurs while fetching the tables, return an empty array with an internal server error status
            Ok(warp::reply::with_status(
                warp::reply::json::<Vec<TableResponse>>(&vec![]),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

/// Create a new table
pub async fn create_table_handler(
    conn: Connection,
    data: Table,
) -> Result<impl warp::Reply, warp::Rejection> {
    match Table::get_existing_table_id(&conn, &data) {
        Ok(Some(table_id)) => {
            // If the table already exists, return the existing table ID with a created status
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({ "id": table_id })),
                warp::http::StatusCode::CREATED,
            ))
        }
        Ok(None) => {
            // If the table does not exist, create a new one
            match Table::create(&conn, &data) {
                Ok(table_id) => Ok(warp::reply::with_status(
                    warp::reply::json(&json!({ "id": table_id })),
                    warp::http::StatusCode::CREATED,
                )),
                Err(_err) => {
                    // If table creation fails, return an internal server error status with an error message
                    Ok(warp::reply::with_status(
                        warp::reply::json(&json!({"error":"Error creating table"})),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            }
        }
        Err(_err) => {
            // If there is an error checking for the existing table, return an internal server error status with an error message
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({"error":"Error creating table"})),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

// Handlers for Menu operations

/// List all menus
pub async fn list_menu_handler(conn: Connection) -> Result<impl warp::Reply, warp::Rejection> {
    match Menu::list(&conn) {
        Ok(menus) => Ok(warp::reply::with_status(
            warp::reply::json(&menus),
            warp::http::StatusCode::OK,
        )),
        Err(_err) => {
            // If an error occurs while fetching the menus, return an empty array with an internal server error status
            Ok(warp::reply::with_status(
                warp::reply::json::<Vec<MenuResponse>>(&vec![]),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

/// Create a new menu
pub async fn create_menu_handler(
    conn: Connection,
    data: Menu,
) -> Result<impl warp::Reply, warp::Rejection> {
    match Menu::get_existing_menu_id(&conn, &data) {
        Ok(Some(menu_id)) => {
            // If the menu already exists, return the existing menu ID with a created status
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({ "id": menu_id })),
                warp::http::StatusCode::CREATED,
            ))
        }
        Ok(None) => {
            // If the menu does not exist, create a new one
            match Menu::create(&conn, &data) {
                Ok(menu_id) => Ok(warp::reply::with_status(
                    warp::reply::json(&json!({ "id": menu_id })),
                    warp::http::StatusCode::CREATED,
                )),
                Err(_err) => {
                    // If menu creation fails, return an internal server error status with an error message
                    Ok(warp::reply::with_status(
                        warp::reply::json(&json!({ "error": "Error creating Menu" })),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            }
        }
        Err(_err) => {
            // If there is an error checking for the existing menu, return an internal server error status with an error message
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({ "error": "Error creating Menu" })),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

// Handlers for Order operations

/// Create a new order
pub async fn create_order_handler(
    conn: Connection,
    req_body: OrderRequestBody,
) -> Result<impl warp::Reply, warp::Rejection> {
    let table_id = req_body.table_id;
    let menu_ids = req_body.menu_ids;
    if menu_ids.is_empty() {
        // Return BAD REQUEST if no menu items are provided
        return Ok(warp::reply::with_status(
            warp::reply::json(&json!({"error":"Please Add Items"})),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }

    match OrderResponse::get_existing_order_id(&conn, table_id) {
        Ok(Some(order_id)) => {
            // If an active order exists, update the order items
            for menu_id in menu_ids {
                // Generate a random cooking time for the order item
                let cooking_time = rand::thread_rng().gen_range(5..=15);
                match OrderItem::get_existing_order_item_id(&conn, order_id, menu_id) {
                    Ok(Some(order_item_id)) => {
                        // If order item exists, update its quantity
                        match OrderItem::add_quantity_of_existing_order_item(&conn, order_item_id) {
                            Ok(_) => continue,
                            Err(_) => {
                                // Respond with an error if updating the order item fails
                                return Ok(warp::reply::with_status(
                                    warp::reply::json(
                                        &json!({"error":"Error updating order Item"}),
                                    ),
                                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                                ));
                            }
                        }
                    }
                    Ok(None) => {
                        // If order item does not exist, create a new one
                        match OrderItem::create(&conn, order_id, menu_id, cooking_time) {
                            Ok(_) => continue,
                            Err(_err) => {
                                // Respond with an error if creating the order item fails
                                eprintln!("{}", _err);
                                return Ok(warp::reply::with_status(
                                    warp::reply::json(
                                        &json!({"error":"Error creating order Item"}),
                                    ),
                                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                                ));
                            }
                        }
                    }
                    Err(_err) => {
                        // Respond with an error if there is an issue checking for the existing order item
                        return Ok(warp::reply::with_status(
                            warp::reply::json(
                                &json!({"error":"Error checking for existing order Item"}),
                            ),
                            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                        ));
                    }
                }
            }

            // If all order items were successfully handled, return a success message
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({"success":"All order items updated successfully"})),
                warp::http::StatusCode::OK,
            ))
        }
        Ok(None) => {
            // If no active order exists, create a new order and order items
            match OrderResponse::create(&conn, table_id) {
                Ok(last_inserted_id) => {
                    for menu_id in menu_ids {
                        // Generate a random cooking time for each order item
                        let cooking_time = rand::thread_rng().gen_range(5..=15);
                        match OrderItem::create(&conn, last_inserted_id, menu_id, cooking_time) {
                            Ok(_) => continue,
                            Err(_err) => {
                                // Respond with an error if creating an order item fails
                                eprintln!("{}", _err);
                                return Ok(warp::reply::with_status(
                                    warp::reply::json(
                                        &json!({"error":"Error creating order Item"}),
                                    ),
                                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                                ));
                            }
                        }
                    }

                    // If the order and all order items were successfully created, return a success message with the new order ID
                    Ok(warp::reply::with_status(
                        warp::reply::json(
                            &json!({"id":last_inserted_id, "success":"Order and all order items created successfully"}),
                        ),
                        warp::http::StatusCode::CREATED,
                    ))
                }
                Err(_err) => {
                    // Respond with an error if creating the order fails
                    Ok(warp::reply::with_status(
                        warp::reply::json(
                            &json!({"error":format!("Error creating order {}", _err)}),
                        ),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            }
        }
        Err(_err) => {
            // Respond with an error if there is an issue checking for the existing order
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({"error":"Error checking for existing order"})),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

/// List all orders
pub async fn list_order_handler(conn: Connection) -> Result<impl warp::Reply, warp::Rejection> {
    match OrderResponse::list(&conn) {
        Ok(menus) => Ok(warp::reply::with_status(
            warp::reply::json(&menus),
            warp::http::StatusCode::OK,
        )),
        Err(_err) => {
            // If an error occurs while fetching the orders, return an empty array with an internal server error status
            Ok(warp::reply::with_status(
                warp::reply::json::<Vec<OrderResponse>>(&vec![]),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

/// Delete a specific order item from an order by table ID
pub async fn delete_order_item_handler(
    conn: Connection,
    table_id: i64,
    menu_id: i64,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Decrease the item quantity if greater than 1
    let result = conn.execute(
        "UPDATE order_items 
        SET cooking_time = cooking_time - (cooking_time/quantity), quantity = quantity - 1
        WHERE order_items.order_id IN (
            SELECT orders.id
            FROM orders
            JOIN tables ON orders.table_id = tables.id
            WHERE tables.id = ?1
        ) AND order_items.menu_id = ?2 AND order_items.quantity > 1",
        params![table_id, menu_id],
    );

    match result {
        Ok(updated) => {
            if updated > 0 {
                // If quantity was greater than 1, update and return success
                Ok(warp::reply::with_status(
                    warp::reply::json(&json!({"success": "Menu quantity updated successfully"})),
                    warp::http::StatusCode::OK,
                ))
            } else {
                // If quantity is 1, delete the order item
                let delete_result = conn.execute(
                    "DELETE FROM order_items 
                    WHERE order_items.order_id IN (
                        SELECT orders.id
                        FROM orders
                        JOIN tables ON orders.table_id = tables.id
                        WHERE tables.id = ?1
                    ) AND order_items.menu_id = ?2",
                    params![table_id, menu_id],
                );

                match delete_result {
                    Ok(_) => {
                        let order_id_result = OrderResponse::get_existing_order_id(&conn, table_id);

                        match order_id_result {
                            Ok(Some(order_id)) => {
                                let has_items = OrderResponse::has_items(&conn, order_id);

                                match has_items {
                                    Ok(false) => {
                                        // If there are no more items, delete the order as well
                                        let _ = conn.execute(
                                            "DELETE from orders WHERE id = ?",
                                            params![order_id],
                                        );

                                        Ok(warp::reply::with_status(
                                            warp::reply::json(
                                                &json!({"success": "Menu deleted successfully and order deleted"}),
                                            ),
                                            warp::http::StatusCode::OK,
                                        ))
                                    }
                                    Ok(true) => {
                                        // If there are still items, return success without deleting the order
                                        Ok(warp::reply::with_status(
                                            warp::reply::json(
                                                &json!({"success": "Menu deleted successfully"}),
                                            ),
                                            warp::http::StatusCode::OK,
                                        ))
                                    }
                                    Err(_err) => {
                                        // If an error occurs while checking if the order has items, return an error
                                        Ok(warp::reply::with_status(
                                            warp::reply::json(
                                                &json!({"error": "Menu delete failed"}),
                                            ),
                                            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                                        ))
                                    }
                                }
                            }
                            _ => {
                                // If an error occurs while retrieving the order ID, return an error
                                Ok(warp::reply::with_status(
                                    warp::reply::json(
                                        &json!({"error": "Failed to retrieve order ID"}),
                                    ),
                                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                                ))
                            }
                        }
                    }
                    Err(_) => {
                        // If deleting the order item fails, return an error
                        Ok(warp::reply::with_status(
                            warp::reply::json(&json!({"error": "Menu delete failed"})),
                            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                        ))
                    }
                }
            }
        }
        Err(_err) => {
            // If updating the quantity fails, return an error
            eprintln!("Failed to update quantity: {:?}", _err);
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({"error": "Failed to update quantity"})),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

/// List all order items for a specific table
pub async fn list_order_items_for_table_handler(
    conn: Connection,
    table_id: i64,
) -> Result<impl warp::Reply, warp::Rejection> {
    match OrderItem::list_order_items(&conn, table_id) {
        Ok(items) => Ok(warp::reply::with_status(
            warp::reply::json(&items),
            warp::http::StatusCode::OK,
        )),
        Err(_err) => {
            // If an error occurs while fetching the order items, return an empty array with an internal server error status
            eprintln!("{}", _err);
            Ok(warp::reply::with_status(
                warp::reply::json::<Vec<OrderItemResponse>>(&vec![]),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

/// Retrieve a specific item from a specific table
pub async fn get_order_item_for_table_handler(
    conn: Connection,
    table_id: i64,
    menu_id: i64,
) -> Result<impl warp::Reply, warp::Rejection> {
    match OrderItem::get_item(&conn, table_id, menu_id) {
        Ok(Some(item)) => Ok(warp::reply::with_status(
            warp::reply::json(&item),
            warp::http::StatusCode::OK,
        )),
        Ok(None) => {
            // If no item is found, return a NOT FOUND status with an error message
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({"error": "No Item Found"})),
                warp::http::StatusCode::NOT_FOUND,
            ))
        }
        Err(_err) => {
            // If an error occurs while retrieving the item, return an internal server error status with an error message
            eprintln!("{}", _err);
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({"error": "Something went wrong!"})),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use warp::{hyper::Body, Reply};

    // Set up an in-memory test database
    fn setup_test_db() -> Connection {
        println!("Initializing the test database...");
        let conn = Connection::open_in_memory().expect("Failed to create test database");
        conn.execute("PRAGMA foreign_keys = ON;", [])
            .expect("Failed to enable foreign key support");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tables (id INTEGER PRIMARY KEY, code TEXT NOT NULL UNIQUE)",
            [],
        )
        .expect("Failed to create tables table");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS menus (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
            [],
        )
        .expect("Failed to create menus table");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS orders (id INTEGER PRIMARY KEY, table_id INTEGER NOT NULL, FOREIGN KEY (table_id) REFERENCES tables(id), UNIQUE (table_id))",
            [],
        )
        .expect("Failed to create orders table");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS order_items (id INTEGER PRIMARY KEY, order_id INTEGER NOT NULL, menu_id INTEGER NOT NULL, cooking_time INTEGER NOT NULL, quantity INTEGER NOT NULL default 1, FOREIGN KEY (order_id) REFERENCES orders(id), FOREIGN KEY (menu_id) REFERENCES menus(id))",
            [],
        )
        .expect("Failed to create order_items table");
        conn
    }

    // Insert static table and menu data into the test database
    fn setup_static_data(conn: &Connection) {
        let table_codes = vec!["T-01", "T-02", "T-03"];
        for code in table_codes {
            conn.execute("INSERT INTO tables (code) VALUES (?1)", &[code])
                .expect("Failed to insert table data");
        }

        let menu_names = vec!["M-01", "M-02", "M-03", "M-04", "M-05"];
        for name in menu_names {
            conn.execute("INSERT INTO menus (name) VALUES (?1)", &[name])
                .expect("Failed to insert menu data");
        }
    }

    // Convert a warp Response to a serde JSON Value
    async fn convert_response_to_json(resp: warp::http::Response<Body>) -> serde_json::Value {
        let body_bytes = warp::hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let body_string = String::from_utf8_lossy(&body_bytes);
        serde_json::from_str(&body_string).unwrap()
    }

    // Test Case: Menu Creation
    #[tokio::test]
    async fn test_create_menu_handler() {
        let conn = setup_test_db();
        let menu = Menu {
            id: 0,
            name: "Menu-01".to_string(),
        };
        let result = create_menu_handler(conn, menu).await;
        match result {
            Ok(rep) => {
                let resp = rep.into_response();
                assert_eq!(resp.status(), warp::http::StatusCode::CREATED);
                let json_data = convert_response_to_json(resp).await;
                assert_eq!(json_data["id"].as_i64(), Some(1));
            }
            Err(_) => {
                panic!("Unhandled Error");
            }
        }
    }

    // Test Case: Table Creation
    #[tokio::test]
    async fn test_create_table_handler() {
        let conn = setup_test_db();
        let table = Table {
            id: 0,
            code: "Table-01".to_string(),
        };
        let result = create_table_handler(conn, table).await;
        match result {
            Ok(rep) => {
                let resp = rep.into_response();
                assert_eq!(resp.status(), warp::http::StatusCode::CREATED);
                let json_data = convert_response_to_json(resp).await;
                assert_eq!(json_data["id"].as_i64(), Some(1));
            }
            Err(_) => {
                panic!("Unhandled Error");
            }
        }
    }

    // Test Case: Order creation fails with invalid data
    #[tokio::test]
    async fn test_create_order_handler_wrong_data() {
        let conn = setup_test_db();
        let order = OrderRequestBody {
            table_id: 1,
            menu_ids: vec![1, 2],
        };
        let result = create_order_handler(conn, order).await;
        // Expecting error due to missing table and menu entries
        match result {
            Ok(rep) => {
                let resp = rep.into_response();
                assert_eq!(resp.status(), warp::http::StatusCode::INTERNAL_SERVER_ERROR);
                let json_data = convert_response_to_json(resp).await;
                assert_eq!(
                    json_data["error"].as_str(),
                    Some("Error creating order FOREIGN KEY constraint failed")
                );
            }
            Err(_) => {
                panic!("Unhandled Error");
            }
        }
    }

    // Test Case: Order creation fails with empty menu_ids
    #[tokio::test]
    async fn test_create_order_handler_wrong_data2() {
        let mut conn = setup_test_db();
        setup_static_data(&mut conn);
        let order = OrderRequestBody {
            table_id: 1,
            menu_ids: vec![],
        };
        let result = create_order_handler(conn, order).await;
        // Expecting error due to empty menu_ids
        match result {
            Ok(rep) => {
                let resp = rep.into_response();
                assert_eq!(resp.status(), warp::http::StatusCode::BAD_REQUEST);
                let json_data = convert_response_to_json(resp).await;
                assert_eq!(json_data["error"].as_str(), Some("Please Add Items"));
            }
            Err(_) => {
                panic!("Unhandled Error");
            }
        }
    }

    // Test Case: Order creation succeeds with valid data
    #[tokio::test]
    async fn test_create_order_handler_correct_data() {
        let conn = setup_test_db();
        setup_static_data(&conn);
        let order = OrderRequestBody {
            table_id: 1,
            menu_ids: vec![1, 2],
        };

        let result = create_order_handler(conn, order).await;
        // Expecting successful order creation for table_id 1 with menu_ids 1 and 2
        match result {
            Ok(rep) => {
                let resp = rep.into_response();
                assert_eq!(resp.status(), warp::http::StatusCode::CREATED);
                let json_data = convert_response_to_json(resp).await;
                assert_eq!(json_data["id"].as_i64(), Some(1));
            }
            Err(_) => {
                panic!("Unhandled Error");
            }
        }
    }

    // Test Case: Remove an item from an order
    #[tokio::test]
    async fn test_remove_item_from_table_handler() {
        let mut conn = setup_test_db();
        setup_static_data(&conn);
        // Start a transaction for creating order and order items
        let tx = conn.transaction().expect("Transaction creation failed");

        // Insert into the orders table
        tx.execute("INSERT INTO orders (table_id) VALUES (?1)", [1])
            .expect("Order creation failed");

        // Get the last inserted order_id
        let order_id = tx.last_insert_rowid();

        // Insert into the order_items table using the obtained order_id
        tx.execute(
            "INSERT INTO order_items (order_id, menu_id, cooking_time) VALUES (?1, ?2, ?3)",
            [order_id, 1, 6],
        )
        .expect("OrderItems creation failed");

        tx.execute(
            "INSERT INTO order_items (order_id, menu_id, cooking_time) VALUES (?1, ?2, ?3)",
            [order_id, 2, 7],
        )
        .expect("OrderItems creation failed");

        // Commit the transaction
        tx.commit().expect("Commit failed");
        let result = delete_order_item_handler(conn, 1, 2).await;
        // Expecting to remove menu 2 from the order while keeping menu 1
        match result {
            Ok(rep) => {
                let resp = rep.into_response();
                assert_eq!(resp.status(), warp::http::StatusCode::OK);
                let json_data = convert_response_to_json(resp).await;
                assert_eq!(
                    json_data["success"].as_str(),
                    Some("Menu deleted successfully")
                );
            }
            Err(_) => {
                panic!("Unhandled Error");
            }
        }
    }

    // Test Case: Removing all items from an order deletes the order
    #[tokio::test]
    async fn test_all_order_item_remove_handler() {
        let mut conn = setup_test_db();
        setup_static_data(&conn);
        // Start a transaction for creating order and order items
        let tx = conn.transaction().expect("Transaction creation failed");

        // Insert into the orders table
        tx.execute("INSERT INTO orders (table_id) VALUES (?1)", [1])
            .expect("Order creation failed");

        // Get the last inserted order_id
        let order_id = tx.last_insert_rowid();

        // Insert into the order_items table using the obtained order_id
        tx.execute(
            "INSERT INTO order_items (order_id, menu_id, cooking_time) VALUES (?1, ?2, ?3)",
            [order_id, 1, 6],
        )
        .expect("OrderItems creation failed");

        // Commit the transaction
        tx.commit().expect("Commit failed");
        let result = delete_order_item_handler(conn, 1, 1).await;
        // Expecting to remove menu 1 from the order and delete the order since no items remain
        match result {
            Ok(rep) => {
                let resp = rep.into_response();
                assert_eq!(resp.status(), warp::http::StatusCode::OK);
                let json_data = convert_response_to_json(resp).await;
                assert_eq!(
                    json_data["success"].as_str(),
                    Some("Menu deleted successfully and order deleted")
                );
            }
            Err(_) => {
                panic!("Unhandled Error");
            }
        }
    }

    // Test Case: Removing an item with quantity greater than 1 reduces the quantity of the item
    #[tokio::test]
    async fn test_order_item_quantity_reduce_handler() {
        let mut conn = setup_test_db();
        setup_static_data(&conn);
        // Start a transaction for creating order and order items
        let tx = conn.transaction().expect("Transaction creation failed");

        // Insert into the orders table
        tx.execute("INSERT INTO orders (table_id) VALUES (?1)", [1])
            .expect("Order creation failed");

        // Get the last inserted order_id
        let order_id = tx.last_insert_rowid();

        // Insert into the order_items table using the obtained order_id
        tx.execute(
            "INSERT INTO order_items (order_id, menu_id, cooking_time, quantity) VALUES (?1, ?2, ?3, ?4)",
            [order_id, 1, 6, 2],
        ).expect("OrderItems creation failed");

        // Commit the transaction
        tx.commit().expect("Commit failed");
        let result = delete_order_item_handler(conn, 1, 1).await;
        // Expecting to update the quantity of menu 1
        match result {
            Ok(rep) => {
                let resp = rep.into_response();
                assert_eq!(resp.status(), warp::http::StatusCode::OK);
                let json_data = convert_response_to_json(resp).await;
                assert_eq!(
                    json_data["success"].as_str(),
                    Some("Menu quantity updated successfully")
                );
            }
            Err(_) => {
                panic!("Unhandled Error");
            }
        }
    }

    // Test Case: Retrieve a specific item from a table
    #[tokio::test]
    async fn test_get_item_from_table_handler() {
        let mut conn = setup_test_db();
        setup_static_data(&conn);
        // Start a transaction for creating order and order items
        let tx = conn.transaction().expect("Transaction creation failed");

        // Insert into the orders table
        tx.execute("INSERT INTO orders (table_id) VALUES (?1)", [1])
            .expect("Order creation failed");

        // Get the last inserted order_id
        let order_id = tx.last_insert_rowid();

        // Insert into the order_items table using the obtained order_id
        tx.execute(
            "INSERT INTO order_items (order_id, menu_id, cooking_time) VALUES (?1, ?2, ?3)",
            [order_id, 1, 6],
        )
        .expect("OrderItems creation failed");

        tx.execute(
            "INSERT INTO order_items (order_id, menu_id, cooking_time) VALUES (?1, ?2, ?3)",
            [order_id, 2, 7],
        )
        .expect("OrderItems creation failed");

        // Commit the transaction
        tx.commit().expect("Commit failed");

        let result = get_order_item_for_table_handler(conn, 1, 2).await;
        // Expecting to retrieve menu 2 from the table
        match result {
            Ok(rep) => {
                let resp = rep.into_response();
                match resp.status() {
                    // If item found, return the item
                    warp::http::StatusCode::OK => {
                        let json_data = convert_response_to_json(resp).await;
                        assert_eq!(json_data["menu_name"].as_str(), Some("M-02"));
                    }
                    // If item not found, return an error
                    warp::http::StatusCode::NOT_FOUND => {
                        let json_data = convert_response_to_json(resp).await;
                        assert_eq!(json_data["error"].as_str(), Some("No Item Found"));
                    }
                    _ => {}
                }
            }
            Err(_) => {
                panic!("Unhandled Error");
            }
        }
    }
}
