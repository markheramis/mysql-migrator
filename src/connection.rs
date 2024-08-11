
use std::io;
use std::fs;
use serde_json::Map;
use serde_json::json;
use serde::Serialize;
use serde::Deserialize;
use serde_json::Value;
use rpassword::read_password;
use arguments::Args;
use crate::arguments;

#[derive(Debug, Deserialize)]
pub struct ConnectionConfig {
    pub source: ConnectionDatabaseConfig,
    pub destination: ConnectionDatabaseConfig,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionDatabaseConfig {
    pub hostname: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

pub fn get_config(args: &Args) -> ConnectionConfig {
    let path: &str = args
        .connection_config
        .as_deref()
        .unwrap_or("connection.json");

    let mut conf: Value = match fs::read_to_string(path) {
        Ok(str) => serde_json::from_str(&str).unwrap(),
        Err(_) => serde_json::from_str("{}").unwrap()
    };

    let conn_map: &mut Map<String, Value> = conf.as_object_mut().unwrap();
    process_connection_configuration(&args, conn_map);

    return serde_json::from_value(json!(conn_map))
        .expect("Failed to deserialize ConnectionConfig");
}
pub fn process_connection_configuration(
    args: &Args,
    conn_map: &mut Map<String, Value>
) {
    if !conn_map.contains_key("source") {
        conn_map.insert("source".to_string(), json!(Map::new()));
    }
    if !conn_map.contains_key("destination") {
        conn_map.insert("destination".to_string(), json!(Map::new()));
    }
    process_source_configuration(&args, conn_map);
    process_destination_configuration(&args, conn_map);
}
fn input_string(
    root_key: String,
    key: String,
    conn_map: &mut Map<String, Value>,
) {
    println!("Please enter your {} {}: ", root_key, key);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if let Some(root) = conn_map.get_mut(&root_key) {
        if let Some(map) = root.as_object_mut() {
            map.insert(key.to_string(), json!(input.trim()));
        }
    }
}
fn input_password(
    root_key: String,
    key: String,
    conn_map:  &mut Map<String, Value>
) {
    println!("Please enter your {} {}: ", root_key, key);
    let password = read_password().unwrap();
    if let Some(root) = conn_map.get_mut(&root_key) {
        if let Some(map) = root.as_object_mut() {
            map.insert(key.to_string(), json!(password.trim()));
        }
    }
}
fn input_int(
    root_key: String,
    key: String,
    conn_map: &mut Map<String, Value>
) {
    println!("Please enter your {} {}: ", root_key, key);
    let mut input: String = String::new();
    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    match input.trim().parse::<i64>() {
        Ok(i) => {
            if let Some(root) = conn_map.get_mut(&root_key) {
                if let Some(map) = root.as_object_mut() {
                    map.insert(key.to_string(), json!(i));
                }
            }
        }
        Err(_) => println!("{} {} is not a valid number", root_key, key),
    }
}
fn process_source_configuration(args: &Args, conn_map: &mut Map<String, Value>) {
    if let Some(source_host) = &args.source_host {
        conn_map.get_mut("source").and_then(|source| {
            source.as_object_mut().map(|map| {
                map.insert("hostname".to_string(), json!(source_host));
            })
        });
    } else if !conn_map.contains_key("source") || conn_map["source"]["hostname"].is_null() {
        input_string("source".to_string(), "hostname".to_string(), conn_map);
    }
    if let Some(source_port) = args.source_port {
        conn_map.get_mut("source").and_then(|source| {
            source.as_object_mut().map(|map| {
                map.insert("port".to_string(), json!(source_port));
            })
        });
    } else if !conn_map.contains_key("source") || conn_map["source"]["port"].is_null() {
        input_int("source".to_string(), "port".to_string(), conn_map);
    }
    if let Some(source_database) = &args.source_database {
        conn_map.get_mut("source").and_then(|source| {
            source.as_object_mut().map(|map| {
                map.insert("database".to_string(), json!(source_database));
            })
        });
    } else if !conn_map.contains_key("source") || conn_map["source"]["database"].is_null() {
        input_string("source".to_string(), "database".to_string(), conn_map);
    }
    if let Some(source_username) = &args.source_username {
        conn_map.get_mut("source").and_then(|source| {
            source.as_object_mut().map(|map| {
                map.insert("username".to_string(), json!(source_username));
            })
        });
    } else if !conn_map.contains_key("source") || conn_map["source"]["username"].is_null() {
        input_string("source".to_string(), "username".to_string(), conn_map);
    }
    if let Some(source_password) = &args.source_password {
        conn_map.get_mut("source").and_then(|source| {
            source.as_object_mut().map(|map| {
                map.insert("password".to_string(), json!(source_password));
            })
        });
    } else if !conn_map.contains_key("source") || conn_map["source"]["password"].is_null() {
        input_password("source".to_string(), "password".to_string(), conn_map);
    }
}

fn process_destination_configuration(args: &Args, conn_map: &mut Map<String, Value>) {
    if let Some(destination_host) = &args.destination_host {
        conn_map.get_mut("destination").and_then(|destination| {
            destination.as_object_mut().map(|map| {
                map.insert("hostname".to_string(), json!(destination_host));
            })
        });
    } else if !conn_map.contains_key("destination") || conn_map["destination"]["hostname"].is_null() {
        input_string("destination".to_string(), "hostname".to_string(), conn_map);
    }
    if let Some(destination_port) = args.destination_port {
        conn_map.get_mut("destination").and_then(|destination| {
            destination.as_object_mut().map(|map| {
                map.insert("port".to_string(), json!(destination_port));
            })
        });
    } else if !conn_map.contains_key("destination") || conn_map["destination"]["port"].is_null() {
        input_int("destination".to_string(), "port".to_string(), conn_map);
    }
    if let Some(destination_database) = &args.destination_database {
        conn_map.get_mut("destination").and_then(|destination| {
            destination.as_object_mut().map(|map| {
                map.insert("database".to_string(), json!(destination_database));
            })
        });
    } else if !conn_map.contains_key("destination") || conn_map["destination"]["database"].is_null() {
        input_string("destination".to_string(), "database".to_string(), conn_map);
    }
    if let Some(destination_username) = &args.destination_username {
        conn_map.get_mut("destination").and_then(|destination| {
            destination.as_object_mut().map(|map| {
                map.insert("username".to_string(), json!(destination_username));
            })
        });
    } else if !conn_map.contains_key("destination") || conn_map["destination"]["username"].is_null() {
        input_string("destination".to_string(), "username".to_string(), conn_map);
    }
    if let Some(destination_password) = &args.destination_password {
        conn_map.get_mut("destination").and_then(|destination| {
            destination.as_object_mut().map(|map| {
                map.insert("password".to_string(), json!(destination_password));
            })
        });
    } else if !conn_map.contains_key("destination") || conn_map["destination"]["password"].is_null() {
        input_password("destination".to_string(), "password".to_string(), conn_map);
    }
}
