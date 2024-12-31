use reqwest::Client;
use serde_json::Value;
use std::collections::{HashSet};
use std::{fs, io::Write, path::Path, time::Duration};
use tokio::time;
use std::io::{stdout, Write as IoWrite};

const GRAPHQL_URL: &str = "http://35.187.231.22:8080/v1/graphql";

#[derive(Debug, Clone)]
struct EventLogItem {
    address_items: Vec<String>,
    bool_items: Vec<String>,
}

async fn fetch_graphql_data(client: &Client, offset: usize) -> Option<Vec<EventLogItem>> {
    let graphql_query = serde_json::json!({
        "query": format!("query {{
            EventLog1(offset: {}) {{
                eventData_addressItems_items
                eventData_boolItems_items
                blockNumber
            }}
        }}", offset)
    });

    let response = client.post(GRAPHQL_URL)
        .json(&graphql_query)
        .send()
        .await
        .expect("Failed to send GraphQL request");

    if response.status().is_success() {
        let json_response: Value = response.json().await.expect("Failed to parse response");
        if let Some(event_log_items) = json_response["data"]["EventLog1"].as_array() {
            let mut event_logs = Vec::new();
            for item in event_log_items {
                let event_data = EventLogItem {
                    address_items: item["eventData_addressItems_items"]
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect(),
                    bool_items: item["eventData_boolItems_items"]
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect(),
                };
                event_logs.push(event_data);
            }
            return Some(event_logs);
        }
    }
    None
}


fn save_to_json_file(file_name: &str, data: &HashSet<String>) {
    let folder_path = Path::new("filteredEventLogs");
    if !folder_path.exists() {
        fs::create_dir(folder_path).unwrap();
    }

    let file_path = folder_path.join(file_name);
    let json_data = serde_json::to_string_pretty(&data).unwrap();

    let mut file = fs::File::create(file_path.clone()).unwrap();
    file.write_all(json_data.as_bytes()).unwrap();

    println!("📁 Data written to: {:?}", file_path);
}


fn append_new_data(new_data: &HashSet<String>, file_name: &str) -> HashSet<String> {
    let mut existing_data: HashSet<String> = HashSet::new();
    let file_path = Path::new("filteredEventLogs").join(file_name);

   
    if file_path.exists() {
        if let Ok(file_content) = fs::read_to_string(&file_path) {
            if let Ok(parsed_data) = serde_json::from_str::<Vec<String>>(&file_content) {
                existing_data.extend(parsed_data);
            }
        }
    }

   
    for item in new_data {
        existing_data.insert(item.clone());
    }

   
    save_to_json_file(file_name, &existing_data);

    existing_data
}

async fn periodically_update_data(client: &Client) {
    let mut offset = 0;

    loop {
        display_loading_animation();

       
        let mut user_list = HashSet::new();
        let mut long_positions = HashSet::new();
        let mut short_positions = HashSet::new();
        let mut market_long_positions = HashSet::new();
        let mut market_short_positions = HashSet::new();
        let mut new_user_count = 0;
        let mut new_long_count = 0;
        let mut new_short_count = 0;
        let mut new_market_long_count = 0;
        let mut new_market_short_count = 0;

        if let Some(event_log_items) = fetch_graphql_data(client, offset).await {
            let batch_size = event_log_items.len();
            if batch_size == 0 {
                println!("🟢 No more data available to fetch.");
                break;
            }

            let market_list = vec![
                "0x46E715C0826123824352F4f0BCD279c815A0945E".to_string(),
                "0xeE553341d93bcF93e77E101e15bCbe07aF7E488f".to_string(),
                "0xD0875336db5a5b6FD70081918c559284Dc8434fA".to_string(),
            ];

            for event in event_log_items {
                if !event.address_items.is_empty() {
                    // User List (Unique addresses)
                    user_list.insert(event.address_items[0].clone());

                    // Long/Short Position classification
                    let is_long = event.bool_items.get(0) == Some(&"true".to_string());
                    if is_long {
                        long_positions.insert(event.address_items[0].clone());
                    } else {
                        short_positions.insert(event.address_items[0].clone());
                    }

                    // Market-based classification (Assuming index 1 exists for market address)
                    if event.address_items.len() > 1 && market_list.contains(&event.address_items[1]) {
                        let market_address = event.address_items[1].clone();
                        if is_long {
                            market_long_positions.insert(market_address);
                        } else {
                            market_short_positions.insert(market_address);
                        }
                    }
                }
            }

          
            new_user_count = append_new_data(&user_list, "userList.json").len() - user_list.len();
            new_long_count = append_new_data(&long_positions, "longPositionAccounts.json").len() - long_positions.len();
            new_short_count = append_new_data(&short_positions, "shortPositionAccounts.json").len() - short_positions.len();
            new_market_long_count = append_new_data(&market_long_positions, "accountForMarketForLong.json").len() - market_long_positions.len();
            new_market_short_count = append_new_data(&market_short_positions, "accountForMarketForShort.json").len() - market_short_positions.len();

            offset += batch_size;  

            println!("✅ Processed {} entries (offset: {})", batch_size, offset);
            println!("📈 New User Addresses Added: {}", new_user_count);
            println!("💹 New Long Positions Added: {}", new_long_count);
            println!("📉 New Short Positions Added: {}", new_short_count);
            println!("💼 New Market Long Positions Added: {}", new_market_long_count);
            println!("💼 New Market Short Positions Added: {}", new_market_short_count);
            println!("-------------------------------------");
        }


        time::sleep(Duration::from_secs(60)).await;
    }
}


fn display_loading_animation() {
    let symbols = vec!["⏳", "🔄", "⌛", "⏳", "🔄", "⌛"];
    for symbol in symbols.iter() {
        print!("\r{} Fetching new data...", symbol);
        std::io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_millis(500));  
    }
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    println!("🚀 Starting periodic data fetching with pagination...");
    periodically_update_data(&client).await;
}