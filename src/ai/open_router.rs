use reqwest::Client;
use serde_json::json;

pub async fn call_open_router(
    prompt: &str,
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    if api_key.is_empty() {
        return Err("No API key found".into());
    }

    let payload = json!({
        "model": "qwen/qwen2.5-7b-instruct",
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    let client = Client::new();
    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", "http://localhost")
        .header("X-Title", "commit-message")
        .json(&payload)
        .send()
        .await?;

    let result: serde_json::Value = response.json().await?;
    Ok(result)
}
