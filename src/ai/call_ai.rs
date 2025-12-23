use crate::ai::gemini::call_gemini;
use crate::ai::open_router::call_open_router;
use crate::utils::config::HntConfig;
use crate::utils::loader::Loader;

pub async fn ai(prompt: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let api = HntConfig::load().api;

    let loader = Loader::start();

    let res = match api.default_ai.as_str() {
        "gemini" => call_gemini(prompt, &api.gemini_api_key).await?,
        "open_router" => call_open_router(prompt, &api.openrouter_api_key).await?,
        _ => {
            loader.stop();
            return Err("Unsupported AI model".into());
        }
    };

    loader.stop();
    Ok(res)
}
