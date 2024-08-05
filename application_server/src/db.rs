use rusqlite::Connection;

/// Establish a connection to the SQLite database
pub fn get_db_conn() -> Connection {
    let conn = Connection::open("restaurant.db").expect("Failed to open SQLite connection");
    conn
}

/// Initialize the database and create necessary tables
pub fn initialize_db() {
    println!("Initializing the database...");
    let conn = Connection::open("restaurant.db").expect("Failed to open SQLite connection");

    // Enable foreign key support
    conn.execute("PRAGMA foreign_keys = ON;", [])
        .expect("Failed to enable foreign key support");

    println!("Creating 'tables' table");
    create_table_table_if_not_exists(&conn).expect("Failed to create 'tables' table");

    println!("Creating 'menus' table");
    create_menu_table_if_not_exists(&conn).expect("Failed to create 'menus' table");

    println!("Creating 'orders' table");
    create_order_table_if_not_exists(&conn).expect("Failed to create 'orders' table");

    println!("Creating 'order_items' table");
    create_order_item_table_if_not_exists(&conn).expect("Failed to create 'order_items' table");
}

/// Create the 'tables' table if it doesn't exist
fn create_table_table_if_not_exists(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tables (id INTEGER PRIMARY KEY, code TEXT NOT NULL UNIQUE)",
        [],
    )?;
    Ok(())
}

/// Create the 'menus' table if it doesn't exist
fn create_menu_table_if_not_exists(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS menus (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
        [],
    )?;
    Ok(())
}

/// Create the 'orders' table if it doesn't exist
fn create_order_table_if_not_exists(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("CREATE TABLE IF NOT EXISTS orders (id INTEGER PRIMARY KEY, table_id INTEGER NOT NULL, FOREIGN KEY (table_id) REFERENCES tables(id), UNIQUE (table_id))",[])?;
    Ok(())
}

/// Create the 'order_items' table if it doesn't exist
fn create_order_item_table_if_not_exists(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("CREATE TABLE IF NOT EXISTS order_items (id INTEGER PRIMARY KEY, order_id INTEGER NOT NULL, menu_id INTEGER NOT NULL, cooking_time INTEGER NOT NULL, quantity INTEGER NOT NULL default 1, FOREIGN KEY (order_id) REFERENCES orders(id), FOREIGN KEY (menu_id) REFERENCES menus(id))",[])?;
    Ok(())
}
