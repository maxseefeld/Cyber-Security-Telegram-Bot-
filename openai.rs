use reqwest;
use serde_json::json;

pub async fn get_openai_response(prompt: &str) -> String {
    let api_key = "<YOUR_OPENAI_API_KEY>";
    let url = "https://api.openai.com/v1/engines/davinci-codex/completions";
    let prompt_data = format!("Give me some cybersecurity advice: {}", prompt);
    let payload = json!({
        "prompt": prompt_data,
        "max_tokens": 50,
        "temperature": 0.5,
        "n": 1,
        "stop": "\n"
    });
    let client = reqwest::Client::new();
    let response = client.post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(payload.to_string())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    response
}
