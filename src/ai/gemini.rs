use reqwest::Client;
use serde_json::json;

pub async fn call_gemini(
    prompt: &str,
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    if api_key.is_empty() {
        return Err("No API key found".into());
    }

    let payload = json!({
        "contents": [
            { "parts": [{ "text": prompt }] }
        ]
    });

    let client = Client::new();
    let response = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent")
        .header("Content-Type", "application/json")
        .header("X-goog-api-key", api_key)
        .json(&payload)
        .send()
        .await?;

    let result: serde_json::Value = response.json().await?;
    Ok(result)
}
