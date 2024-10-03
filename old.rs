// old code for rush to get data :

// use std::collections::HashSet;
// use std::fs;
// use std::path::Path;
// use std::io::Write;
// use serde::{Serialize, Deserialize};
// use std::fs::File;
// use std::io::Read;
// use std::path::PathBuf;

// // Struct for event log items
// #[derive(Serialize, Deserialize, Debug)]
// struct EventLogItem {
//     eventData_addressItems_items: Vec<String>,
//     eventData_boolItems_items: Vec<String>,
// }

// // Struct for the event log data
// #[derive(Serialize, Deserialize, Debug)]
// struct EventLogData {
//     EventLog1: Vec<EventLogItem>,
// }

// // Struct for the top-level data wrapper
// #[derive(Serialize, Deserialize, Debug)]
// struct EventLogWrapper {
//     data: EventLogData,
// }

// // Function to write results to JSON files
// fn save_to_json_file(file_name: &str, data: Vec<String>) {
//     let folder_path = Path::new("filteredEventLogs");
//     if !folder_path.exists() {
//         fs::create_dir(folder_path).unwrap(); // Create folder if it doesn't exist
//     }

//     let file_path: PathBuf = folder_path.join(file_name);
//     let json_data = serde_json::to_string_pretty(&data).unwrap();

//     let mut file = fs::File::create(file_path.clone()).unwrap(); // Clone `file_path` here
//     file.write_all(json_data.as_bytes()).unwrap();

//     println!("Data written to file: {:?}", file_path);
// }

// // Function to read filtered event log JSON file
// fn read_filtered_data() -> EventLogData {
//     let file_path = Path::new("filteredEventLog.json");
//     let mut file = File::open(file_path).expect("File not found");
//     let mut data = String::new();
//     file.read_to_string(&mut data).expect("Unable to read file");

//     let event_log_wrapper: EventLogWrapper = serde_json::from_str(&data).expect("Error parsing JSON");
//     event_log_wrapper.data // Return the `data` field that contains `EventLog1`
// }

// // Function to get user list
// fn get_user_list() -> Vec<String> {
//     let mut account_list: HashSet<String> = HashSet::new();

//     // Deserialize the filtered JSON file
//     let accounts: EventLogData = read_filtered_data();

//     for event in accounts.EventLog1 {
//         if !event.eventData_addressItems_items.is_empty() {
//             account_list.insert(event.eventData_addressItems_items[0].clone());
//         }
//     }

//     let account_list_vec = account_list.clone().into_iter().collect(); // Clone before using into_iter
//     save_to_json_file("userList.json", account_list_vec);

//     account_list.into_iter().collect()
// }

// // Function to get long position accounts
// fn get_long_position_accounts() -> Vec<String> {
//     let mut account_list_for_long: HashSet<String> = HashSet::new();

//     let accounts: EventLogData = read_filtered_data();

//     for event in accounts.EventLog1 {
//         if !event.eventData_addressItems_items.is_empty() {
//             let is_long_position = event.eventData_boolItems_items.get(0) == Some(&"true".to_string());
//             if is_long_position {
//                 account_list_for_long.insert(event.eventData_addressItems_items[0].clone());
//             }
//         }
//     }

//     let account_list_for_long_vec = account_list_for_long.clone().into_iter().collect(); // Clone before using into_iter
//     save_to_json_file("longPositionAccounts.json", account_list_for_long_vec);

//     account_list_for_long.into_iter().collect()
// }

// // Function to get short position accounts
// fn get_short_position_accounts() -> Vec<String> {
//     let mut account_list_for_short: HashSet<String> = HashSet::new();

//     let accounts: EventLogData = read_filtered_data();

//     for event in accounts.EventLog1 {
//         if !event.eventData_addressItems_items.is_empty() {
//             let is_long_position = event.eventData_boolItems_items.get(0) == Some(&"true".to_string());
//             if !is_long_position {
//                 account_list_for_short.insert(event.eventData_addressItems_items[0].clone());
//             }
//         }
//     }

//     let account_list_for_short_vec = account_list_for_short.clone().into_iter().collect(); // Clone before using into_iter
//     save_to_json_file("shortPositionAccounts.json", account_list_for_short_vec);

//     account_list_for_short.into_iter().collect()
// }

// // Function to get accounts for a specific market with long positions
// fn get_account_for_market_for_long() -> Vec<String> {
//     let mut account_list_for_long: HashSet<String> = HashSet::new();
//     let market_list = vec![
//         "0x46E715C0826123824352F4f0BCD279c815A0945E".to_string(),
//         "0xeE553341d93bcF93e77E101e15bCbe07aF7E488f".to_string(),
//         "0xD0875336db5a5b6FD70081918c559284Dc8434fA".to_string(),
//     ];

//     let accounts: EventLogData = read_filtered_data();

//     for event in accounts.EventLog1 {
//         if event.eventData_addressItems_items.len() > 1 && market_list.contains(&event.eventData_addressItems_items[1]) {
//             let is_long_position = event.eventData_boolItems_items.get(0) == Some(&"true".to_string());
//             if is_long_position {
//                 account_list_for_long.insert(event.eventData_addressItems_items[0].clone());
//             }
//         }
//     }

//     let account_list_for_long_vec = account_list_for_long.clone().into_iter().collect(); // Clone before using into_iter
//     save_to_json_file("accountForMarketForLong.json", account_list_for_long_vec);

//     account_list_for_long.into_iter().collect()
// }

// // Function to get accounts for a specific market with short positions
// fn get_account_for_market_for_short() -> Vec<String> {
//     let mut account_list_for_short: HashSet<String> = HashSet::new();
//     let market_list = vec![
//         "0x46E715C0826123824352F4f0BCD279c815A0945E".to_string(),
//         "0xeE553341d93bcF93e77E101e15bCbe07aF7E488f".to_string(),
//         "0xD0875336db5a5b6FD70081918c559284Dc8434fA".to_string(),
//     ];

//     let accounts: EventLogData = read_filtered_data();

//     for event in accounts.EventLog1 {
//         if event.eventData_addressItems_items.len() > 1 && market_list.contains(&event.eventData_addressItems_items[1]) {
//             let is_long_position = event.eventData_boolItems_items.get(0) == Some(&"true".to_string());
//             if !is_long_position {
//                 account_list_for_short.insert(event.eventData_addressItems_items[0].clone());
//             }
//         }
//     }

//     let account_list_for_short_vec = account_list_for_short.clone().into_iter().collect(); // Clone before using into_iter
//     save_to_json_file("accountForMarketForShort.json", account_list_for_short_vec);

//     account_list_for_short.into_iter().collect()
// }

// // Main function for testing
// fn main() {
//     println!("Fetching user list...");
//     get_user_list();

//     println!("Fetching long position accounts...");
//     get_long_position_accounts();

//     println!("Fetching short position accounts...");
//     get_short_position_accounts();

//     println!("Fetching long position accounts for specific markets...");
//     get_account_for_market_for_long();

//     println!("Fetching short position accounts for specific markets...");
//     get_account_for_market_for_short();
// }