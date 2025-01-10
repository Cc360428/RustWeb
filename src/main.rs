use actix_web::{ App, HttpServer};
use std::env;
use utils::utils::send_telegram; // Import send_telegram
mod utils;

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

    println!("-------------------Starting server on {}", address);

    // Start the web server
    let server = HttpServer::new(move || {
        App::new()
    });

    // Run the server
    server.bind(&address)?.run().await
}