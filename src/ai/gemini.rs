use reqwest::Client;
use serde_json::json;

pub async fn call(
    prompt: &str,
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let payload = json!({
        "contents": [
            { "parts": [{ "text": prompt }] }
        ]
    });

    let client = Client::new();
    let response = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-8b:generateContent")
        .header("Content-Type", "application/json")
        .header("X-goog-api-key", api_key)
        .json(&payload)
        .send()
        .await?;

    let result: serde_json::Value = response.json().await?;
    Ok(result)
}
