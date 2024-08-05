// src/main.rs
mod db;
mod handlers;
mod models;
mod routes;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Set up the database
    db::initialize_db();

    // Combine all defined routes
    let routes = routes::restaurant_routes();

    println!("Starting the application server");
    warp::serve(routes.with(warp::trace::request()))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
