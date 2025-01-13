use actix_web::{web, App, HttpServer};
use log::info;
use routers::product::init_routes;
use routers::routers::index;
use std::env;

use utils::log::init_logger;
use utils::utils::send_telegram;

mod routers;
mod utils;

fn configure_app(app: &mut web::ServiceConfig) {
    app.route("/", web::get().to(index));
    app.configure(init_routes);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
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
