# Rust Restaurant Management Backend API

## Project Description

This Rust-based Restaurant Management Backend API is designed to streamline restaurant operations through a server-client architecture. The application server, built with the Warp framework, handles HTTP routes, database interactions using SQLite, and includes comprehensive unit tests to ensure reliability. The client server simulates multiple asynchronous requests to test the system's responsiveness. By leveraging Rust's safety and concurrency features, this project aims to replicate restaurant operations.

## Libraries Used

- **tokio**: An asynchronous runtime for Rust, enabling concurrent tasks and I/O operations with support for async/await syntax.
- **rusqlite**: A SQLite database library for Rust, providing a bundled version for ease of use and integration.
- **warp**: A web server framework for Rust, designed for building fast and reliable HTTP APIs.
- **serde**: A serialization and deserialization library for Rust, allowing easy conversion of data structures to and from JSON with derive macros.
- **serde_json**: A JSON parsing and serialization library for Rust, working seamlessly with Serde for efficient JSON handling.



## Project Structure
### Application Server
- **main.rs**: Sets up a basic web server by initializing the database, combining HTTP routes from the routes module, starting the Warp server, and adding request tracing for incoming requests.
- **models.rs**: Defines the data models and their associated functions
- **routes.rs**: Defines the HTTP routes for a restaurant management API
- **db.rs**: Includes functions for database initialization and regular DB connection usage
- **handlers.rs**: Defines the handlers for various operations and also includes unit tests.


### Client Server
- **main.rs**: This file simulates a client generating multiple asynchronous requests to the restaurant (i.e. application server)


## Getting Started (Application Server)

1. **Change to Project Directory:**
 ```bash
 cd application_server
 ```
2. **Install Dependencies:**  
```bash
cargo build
```
3. **Run the unit test cases:**
```bash
cargo test
```
4. **Run the project:**
```bash
cargo run
```

## Getting Started (Client Server)

1. **Change to Project Directory:**
 ```bash
 cd client_server
 ```
2. **Install Dependencies:**  
```bash
cargo build
```
3. **Run the project:**
```bash
cargo run
```
