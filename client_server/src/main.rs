use rand::seq::SliceRandom;
use reqwest::Client;
use serde_json::Value;
use tokio::time::{timeout, Duration};

async fn create_tables() -> Vec<i64> {
    // Create a new HTTP client
    let client = Client::new();
    // Define table codes to be created
    let table_codes = vec!["T-01", "T-02", "T-03", "T-04", "T-05"];
    // Vector to store the IDs of created tables
    let mut table_ids = Vec::new();

    // Iterate over the table codes and create tables
    for index in 0..5 {
        // Make a POST request to create a table
        let response: Value = client
            .post("http://localhost:3030/tables/create")
            .json(&serde_json::json!({"code": table_codes[index]})) // Send table code in the request body
            .send()
            .await
            .expect("Failed to create table") // Handle request failure
            .json()
            .await
            .expect("Failed to parse response"); // Handle response parsing failure

        // Extract the table ID from the response and add it to the vector
        table_ids.push(response["id"].as_i64().expect("Missing or invalid id"));
    }

    // Return the vector of table IDs
    return table_ids;
}

async fn create_menus() -> Vec<i64> {
    // Create a new HTTP client
    let client = Client::new();
    // Define menu names to be created
    let menu_names = ["Menu-01", "Menu-02", "Menu-03", "Menu-04", "Menu-05"];
    // Vector to store the IDs of created menus
    let mut menu_ids = Vec::new();

    // Iterate over the menu names and create menus
    for index in 0..5 {
        // Make a POST request to create a menu
        let response: Value = client
            .post("http://localhost:3030/menus/create")
            .json(&serde_json::json!({"name": menu_names[index]})) // Send menu name in the request body
            .send()
            .await
            .expect("Failed to create menu") // Handle request failure
            .json()
            .await
            .expect("Failed to parse response"); // Handle response parsing failure

        // Extract the menu ID from the response and add it to the vector
        menu_ids.push(response["id"].as_i64().expect("Missing or invalid id"));
    }

    // Return the vector of menu IDs
    return menu_ids;
}

async fn order_simulation(client: &Client, table_ids: &[i64], menu_ids: &[i64]) {
    // Simulate concurrent requests by spawning multiple tasks
    let handles: Vec<_> = (0..2)
        .map(|_| {
            // Clone the client for each task
            let client = client.clone();
            // Randomly select a table ID from the provided list
            let table_id = *table_ids.choose(&mut rand::thread_rng()).unwrap();
            // Shuffle and select a subset of menu IDs
            let mut menu_subarray = menu_ids.to_vec();
            menu_subarray.shuffle(&mut rand::thread_rng());
            menu_subarray.truncate(3);

            tokio::spawn(async move {
                // 1. Create an order
                let response = client
                    .post("http://localhost:3030/orders/create")
                    .json(&serde_json::json!({
                        "table_id": table_id,
                        "menu_ids": menu_subarray,
                    })) // Send table_id and menu_ids in the request body
                    .send()
                    .await
                    .expect("Failed to create order") // Handle request failure
                    .json::<serde_json::Value>()
                    .await
                    .expect("Failed to parse response"); // Handle response parsing failure

                println!(
                    "Created Order for table {} with menus {:?}: {:?}",
                    table_id, menu_subarray, response
                );
                tokio::time::sleep(Duration::from_secs(1)).await; // Simulate processing delay

                // 2. Retrieve all items from the order by table ID
                let response = client
                    .get(&format!("http://localhost:3030/tables/{}/items", table_id))
                    .send()
                    .await
                    .expect("Failed to get all items") // Handle request failure
                    .json::<serde_json::Value>()
                    .await
                    .expect("Failed to parse response"); // Handle response parsing failure

                // Extract and print relevant fields from the response
                if let Some(items) = response.as_array() {
                    let mut new_array = Vec::new();

                    for item in items {
                        if let (Some(menu), Some(time), Some(quantity)) = (
                            item.get("menu_name").and_then(|v| v.as_str()),
                            item.get("cooking_time").and_then(|v| v.as_i64()),
                            item.get("quantity").and_then(|v| v.as_i64()),
                        ) {
                            let new_item = (menu, time, quantity);
                            new_array.push(new_item);
                        }
                    }

                    println!("All Items from Table {}: {:?}", table_id, new_array);
                }
                tokio::time::sleep(Duration::from_secs(1)).await; // Simulate processing delay

                // 3. Retrieve a specific item from the table by menu ID
                if let Some(menu_id) = menu_subarray.first() {
                    let response = client
                        .get(&format!(
                            "http://localhost:3030/tables/{}/items/{}",
                            table_id, *menu_id
                        ))
                        .send()
                        .await
                        .expect("Failed to get specific item") // Handle request failure
                        .json::<serde_json::Value>()
                        .await
                        .expect("Failed to parse response"); // Handle response parsing failure

                    println!(                        
                        "First item from table {} is: Menu: {:?}, Cooking Time: {:?}, Quantity: {:?}",                        
                        table_id,
                        response["menu_name"].as_str(),
                        response["cooking_time"].as_i64(),
                        response["quantity"].as_i64()
                    );
                    tokio::time::sleep(Duration::from_secs(1)).await; // Simulate processing delay
                }

                // 4. Remove one item from the table by menu ID
                if let Some(menu_id) = menu_subarray.first() {
                    let response = client
                        .delete(&format!(
                            "http://localhost:3030/orders/{}/items/{}",
                            table_id, *menu_id
                        ))
                        .send()
                        .await
                        .expect("Failed to remove item") // Handle request failure
                        .json::<serde_json::Value>()
                        .await
                        .expect("Failed to parse response"); // Handle response parsing failure

                    println!(
                        "Removed Menu {} from Table {}: {:?}",
                        menu_id, table_id, response
                    );
                    tokio::time::sleep(Duration::from_secs(1)).await; // Simulate processing delay
                }
            })
        })
        .collect();

    // Wait for all tasks to finish with a timeout of 30 seconds
    for handle in handles {
        if let Err(e) = timeout(Duration::from_secs(30), handle).await {
            eprintln!("Task timed out: {:?}", e);
        }
    }
}


#[tokio::main]
async fn main() {
    // Create tables and menus by making asynchronous requests to the server
    let table_ids = create_tables().await; // Create tables and get their IDs
    let menu_ids = create_menus().await; // Create menus and get their IDs

    // Create a new HTTP client to be used for making requests
    let client = Client::new();

    // Simulate the ordering process using the created tables and menus
    order_simulation(&client, &table_ids, &menu_ids).await;
}
