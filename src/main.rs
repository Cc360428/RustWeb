use crate::pkg::log::log::init_logger;
use crate::pkg::redis::kv::KvItem;
use crate::pkg::telegram::telegram::send_telegram;
use actix_web::{web, App, HttpServer};
// use configs::redis::start_monitoring;
use log::info;
use pkg::redis::kv::KvStore;
use routers::product::init_routes;
use routers::routers::index;
use std::env;

// mod configs;
mod pkg {
    pub mod log;
    pub mod redis;
    pub mod telegram;
}

mod routers;
mod utils;

fn configure_app(app: &mut web::ServiceConfig) {
    app.route("/", web::get().to(index));
    app.configure(init_routes);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    // start_monitoring();
    // let config = configs::redis::CONFIG.lock().unwrap();
    // println!(
    // "Initial config: {:?}",
    // *config.malaysia.env.get("dev").unwrap()
    // );

    // TODO 为了不报错
    // Set a key-value pair
    KvStore::set_kv("my_key1", "my_value").expect("Failed to set key-value pair");

    // Get the value associated with the key
    match KvStore::get_kv("my_key").expect("Failed to get value") {
        Some(value) => println!("Value: {}", value),
        None => println!("Key not found"),
    }

    // Delete the key-value pair
    KvStore::del_kv("my_key").expect("Failed to delete key");

    // Create a KvItem
    let kv_item = KvItem {
        key: "another_key".to_string(),
        value: "another_value".to_string(),
    };

    // Set the KvItem
    KvStore::set_kv_item(&kv_item).expect("Failed to set KvItem");

    // Get the KvItem
    match KvStore::get_kv_item("another_key").expect("Failed to get KvItem") {
        Some(item) => println!("Retrieved KvItem: {:?}", item),
        None => println!("KvItem not found"),
    }

    // Delete the KvItem
    KvStore::del_kv_item(&kv_item).expect("Failed to delete KvItem");

    info!("Starting server at: http://localhost:8080/");
    let address = format!(
        "{}:{}",
        env::var("ADDR").unwrap_or_else(|_| "0.0.0.0".to_string()),
        "8080".to_string()
    );

    let variables = vec!["send_tg", "chat_id", "token"];
    let mut config: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for var in &variables {
        match env::var(var) {
            Ok(value) => {
                println!("{}: {}", var, value);
                config.insert(var.to_string(), value);
            }
            Err(_) => {
                println!("{} is not set", var);
            }
        }
    }

    if let Some(send_tg) = config.get("send_tg") {
        if send_tg == "1" {
            let chat_id = config.get("chat_id").and_then(|s| s.parse::<i64>().ok());
            let token = config.get("token");
            if let (Some(chat_id), Some(token)) = (chat_id, token) {
                if let Err(err) = send_telegram(chat_id, token, "Start Run OK!").await {
                    eprintln!("Error sending Telegram message: {:?}", err);
                }
            } else {
                eprintln!("chat_id or token is missing or invalid.");
            }
        }
    }

    info!("log Starting server on:{}", address); // 记录日志
    println!("Starting server on {}", address);

    HttpServer::new(|| App::new().configure(configure_app))
        .bind(address)?
        .run()
        .await
}
