use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub const BASE_URL: &str = "http://127.0.0.1:18082";

pub async fn post<T, R>(url: &str, body: &T) -> Result<R, String>
where
    T: Serialize,
    R: for<'de> Deserialize<'de>,
{
    let full_url = format!("{}{}", BASE_URL, url);
    let response = Request::post(&full_url)
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("Request failed with status: {}", response.status()));
    }

    response.json::<R>().await.map_err(|e| e.to_string())
}

pub async fn get<R>(url: &str) -> Result<R, String>
where
    R: for<'de> Deserialize<'de>,
{
    let full_url = format!("{}{}", BASE_URL, url);
    let response = Request::get(&full_url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("Request failed with status: {}", response.status()));
    }

    response.json::<R>().await.map_err(|e| e.to_string())
}

pub async fn post_chat(message: &str) -> Result<String, String> {
    let full_url = format!("{}/agent/chat", BASE_URL);
    let response = Request::post(&full_url)
        .header("Content-Type", "text/plain")
        .body(message)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("Chat failed with status: {}", response.status()));
    }

    response.text().await.map_err(|e| e.to_string())
}

pub async fn get_history() -> Result<Vec<ChatMessage>, String> {
    let session_id = get_or_create_session_id();
    let full_url = format!("{}/agent/history", BASE_URL);
    
    let response = Request::get(&full_url)
        .header("X-Session-Id", &session_id)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("History fetch failed: {}", response.status()));
    }

    response.json::<Vec<ChatMessage>>().await.map_err(|e| e.to_string())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub fn get_or_create_session_id() -> String {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    match storage.get_item("hsa_session_id").unwrap() {
        Some(id) => id,
        None => {
            let new_id = uuid::Uuid::new_v4().to_string();
            storage.set_item("hsa_session_id", &new_id).unwrap();
            new_id
        }
    }
}

use wasm_streams::ReadableStream;
use futures::StreamExt;
use js_sys::Uint8Array;
use wasm_bindgen::JsCast;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
    #[serde(rename = "tools_support")]
    pub tools_support: bool,
    pub priority: i32,
}

pub async fn get_models() -> Result<Vec<ModelInfo>, String> {
    get("/agent/models").await
}

pub async fn post_chat_stream(message: &str, model_id: Option<String>) -> Result<impl futures::Stream<Item = Result<String, String>>, String> {
    let full_url = format!("{}/agent/chat", BASE_URL);
    
    // 构造 JSON 报文以支持参数扩展
    let body_json = json!({
        "input": message,
        "modelId": model_id
    });

    let session_id = get_or_create_session_id();

    let response = Request::post(&full_url)
        .header("Content-Type", "application/json")
        .header("X-Session-Id", &session_id)
        .json(&body_json)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("Chat failed with status: {}", response.status()));
    }

    let body = response.body().ok_or_else(|| "No body in response".to_string())?;
    let stream = ReadableStream::from_raw(body.unchecked_into())
        .into_stream()
        .map(|result| {
            result
                .map(|js_val| {
                    let uint8_array = Uint8Array::new(&js_val);
                    let vec = uint8_array.to_vec();
                    String::from_utf8_lossy(&vec).into_owned()
                })
                .map_err(|e| format!("{:?}", e))
        });

    Ok(stream)
}
