use reqwest::Client;
use serde_json::json;
use crate::utils::config::HntConfig;

pub async fn ai(prompt: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let payload = json!({
        "contents": [
            { "parts": [{ "text": prompt }] }
        ]
    });

    let api_key = HntConfig::load().ai_api_key;
    let client = Client::new();
    let response = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent")
        .header("Content-Type", "application/json")
        .header("X-goog-api-key", api_key)
        .json(&payload)
        .send()
        .await?;

    let result: serde_json::Value = response.json().await?;
    Ok(result)
}