use reqwest::{Client};
use serde::Serialize;

#[derive(Serialize)]
struct TelegramMessage {
    chat_id: i64,
    text: String,
}

pub async fn send_telegram(chat_id: i64, token: &str, text: &str) -> Result<(), String> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let client = Client::new();

    let message = TelegramMessage {
        chat_id,
        text: text.to_string(),
    };

    // Send the request and await the response
    let response = client.post(&url).json(&message).send().await.map_err(|e| e.to_string())?;

    // Check if the response was successful
    if !response.status().is_success() {
        // Log the response text for debugging
        let response_text = response.text().await.map_err(|e| e.to_string())?;
        eprintln!("Failed to send message: {:?}", response_text);

        // Return an error with a custom message
        return Err(format!("Failed to send message: {}", response_text));
    }

    Ok(())
}