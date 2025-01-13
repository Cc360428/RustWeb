use actix_web::{web, App, HttpServer};
use std::env;
use utils::utils::send_telegram;

use routers::product::init_routes;
use routers::routers::index;

mod routers;
mod utils;

fn configure_app(app: &mut web::ServiceConfig) {
    app.route("/", web::get().to(index));
    app.service(web::scope("product").configure(init_routes));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    println!("Starting server on {}", address);

    HttpServer::new(|| App::new().configure(configure_app))
        .bind(address)?
        .run()
        .await

    // HttpServer::new(|| App::new()
    //     .route("/",web::get().to(index))
    //     .service(web::scope("/product").configure(init_routes))
    //     .service(hello))
    //     .bind(address)?
    //     .run()
    //     .await
}
