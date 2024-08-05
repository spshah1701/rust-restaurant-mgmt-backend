// src/routes.rs

use crate::db::get_db_conn;
use crate::handlers::{
    create_menu_handler, create_order_handler, create_table_handler, delete_order_item_handler,
    get_order_item_for_table_handler, list_menu_handler, list_order_handler,
    list_order_items_for_table_handler, list_table_handler,
};
use rusqlite::Connection;
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

/// Middleware for handling errors and converting them into JSON responses
/// Handles Route Not Found and Deserialization Errors.

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    // Handle route not found error
    if err.is_not_found() {
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Error: Resource not found {:?}", err)),
            warp::http::StatusCode::NOT_FOUND,
        ))
    // Handle deserialization error
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Error: Failed to deserialize request body")),
            warp::http::StatusCode::BAD_REQUEST,
        ))
    // Handle other errors
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Error: {:?}", err)),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

/// Helper function to provide a database connection to route handlers
/// Supplies a new database connection for each route
fn with_db() -> impl Filter<Extract = (Connection,), Error = Infallible> + Clone {
    warp::any().map(|| get_db_conn())
}

/// Route to list all orders. GET request
pub fn list_all_orders_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("orders")
        .and(warp::get())
        .and(with_db())
        .and_then(|conn| list_order_handler(conn))
}

/// Route to create a new order.
/// POST request that expects `table_id` as an i64 and `menu_ids` as a Vec<i64>.
/// Returns BAD REQUEST if `menu_ids` is empty.
/// If there's an existing active order for the given `table_id`, it adds new items to it.
/// Otherwise, creates a new order and returns the order ID.
pub fn create_order_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("orders" / "create")
        .and(warp::post())
        .and(with_db())
        .and(warp::body::json())
        .and_then(|conn, req_body| create_order_handler(conn, req_body))
}

/// Route to delete a specific menu item from a table.
/// DELETE request at /orders/{table_id}/items/{item_id}.
/// Deletes the item and returns a success/error message.
/// If the deleted item was the last one, updates the order status to complete.
pub fn delete_item_from_order_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    warp::path!("orders" / i64 / "items" / i64)
        .and(warp::delete())
        .and(with_db())
        .and_then(|table_id, menu_id, conn| delete_order_item_handler(conn, table_id, menu_id))
}

/// Route to list all tables
pub fn list_tables_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tables")
        .and(warp::get())
        .and(with_db())
        .and_then(|conn| list_table_handler(conn))
}

/// Route to create a table.
/// POST request that expects a `code` in the request body and returns the table's ID upon creation.
pub fn create_table_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tables" / "create")
        .and(warp::post())
        .and(with_db())
        .and(warp::body::json())
        .and_then(|conn, req_body| create_table_handler(conn, req_body))
}

/// Route to list all order items for a specific table. /tables/{table_id}/items
pub fn list_order_items_for_table_route(
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tables" / i64 / "items")
        .and(warp::get())
        .and(with_db())
        .and_then(|table_id, conn| list_order_items_for_table_handler(conn, table_id))
}

/// Route to get a specific menu item from a table. /tables/{table_id}/items/{item_id}
pub fn get_item_from_order_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("tables" / i64 / "items" / i64)
        .and(warp::get())
        .and(with_db())
        .and_then(|table_id, menu_id, conn| {
            get_order_item_for_table_handler(conn, table_id, menu_id)
        })
}

/// Route to list all menus
pub fn list_menus_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("menus")
        .and(warp::get())
        .and(with_db())
        .and_then(|conn| list_menu_handler(conn))
}

/// Route to create a menu.
/// POST request that expects a `name` in the request body.
pub fn create_menu_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("menus" / "create")
        .and(warp::post())
        .and(with_db())
        .and(warp::body::json())
        .and_then(|conn, req_body| create_menu_handler(conn, req_body))
}

/// Route to get state of restaurant.
// pub fn restaurant_state_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
//     warp::path!("state")
//         .and(warp::get())
//         .and(with_db())
//         .and_then(|conn| get_state_handler(conn))
// }

/// Combine all routes
pub fn restaurant_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let routes = create_order_route()
        .or(create_table_route())
        .or(create_menu_route())
        .or(list_tables_route())
        .or(list_menus_route())
        .or(list_all_orders_route())
        .or(delete_item_from_order_route())
        .or(list_order_items_for_table_route())
        .or(get_item_from_order_route());

    routes.recover(handle_rejection)
}
