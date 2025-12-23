use crate::ai::gemini::call;
use crate::utils::config::HntConfig;
use crate::utils::loader::Loader;

pub async fn ai(prompt: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let api_key = HntConfig::load().api.gemini_api_key;
    if api_key.is_empty() {
        return Err("No API key found".into());
    }
    let loader = Loader::start();

    let res = match call(prompt, &api_key).await {
        Ok(result) => result,
        Err(e) => {
            loader.stop();
            return Err(e);
        }
    };
    loader.stop();
    Ok(res)
}
