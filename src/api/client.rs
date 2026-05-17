use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub const BASE_URL: &str = "http://127.0.0.1:18882";

pub async fn post<T, R>(url: &str, body: &T) -> Result<R, String>
where
    T: Serialize,
    R: for<'de> Deserialize<'de>,
{
    let full_url = format!("{}{}", BASE_URL, url);
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let tenant_id = storage.get_item("hsa_tenant_id").unwrap().unwrap_or_else(|| "default".to_string());

    let response = Request::post(&full_url)
        .header("X-Tenant-Id", &tenant_id)
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
    let tenant_id = get_current_tenant_id();
    let response = Request::get(&full_url)
        .header("X-Tenant-Id", &tenant_id)
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
    let tenant_id = get_current_tenant_id();
    let response = Request::post(&full_url)
        .header("Content-Type", "text/plain")
        .header("X-Tenant-Id", &tenant_id)
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
    let tenant_id = get_current_tenant_id();
    let full_url = format!("{}/agent/history", BASE_URL);
    
    let response = Request::get(&full_url)
        .header("X-Session-Id", &session_id)
        .header("X-Tenant-Id", &tenant_id)
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
            let now = js_sys::Date::now().to_string();
            let new_id = format!("S-{}-{}", now, uuid::Uuid::new_v4().to_string());
            storage.set_item("hsa_session_id", &new_id).unwrap();
            new_id
        }
    }
}

/// [NEW] 强制重置会话 ID，实现“新对话”功能
pub fn reset_session_id() -> String {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let now = js_sys::Date::now().to_string();
    let new_id = format!("S-{}-{}", now, uuid::Uuid::new_v4().to_string());
    storage.set_item("hsa_session_id", &new_id).unwrap();
    new_id
}

pub fn get_current_tenant_id() -> String {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    storage.get_item("hsa_tenant_id").unwrap().unwrap_or_else(|| "default".to_string())
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
    #[serde(default)]
    pub is_available: bool,
    #[serde(default)]
    pub status_msg: String,
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
    let tenant_id = get_current_tenant_id();
    web_sys::console::log_1(&format!(">>> [DEBUG] API 调用开始，当前 Session ID: {}, Tenant ID: {}", session_id, tenant_id).into());

    let response = Request::post(&full_url)
        .header("Content-Type", "application/json")
        .header("X-Session-Id", &session_id)
        .header("X-Tenant-Id", &tenant_id)
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

pub async fn update_state(findings: &str) -> Result<(), String> {
    let body = json!({ "findings": [findings] });
    
    let _: serde_json::Value = post("/agent/update_state", &body).await?;
    web_sys::console::log_1(&"State update successful".into());
    Ok(())
}

pub async fn fork_session(source: &str, target: &str) -> Result<String, String> {
    let body = json!({
        "sourceSessionId": source,
        "targetSessionId": target
    });
    
    let res: serde_json::Value = post("/agent/fork", &body).await?;
    Ok(res["targetSession"].as_str().unwrap_or(target).to_string())
}

// [V4.8.0] 可视化工具链 API

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelMetrics {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub daily_used: u64,
    pub daily_quota: u64,
    pub usage_pct: f64,
    pub estimated_cost: f64,
    pub lifetime_tokens: u64,
    // [V4.9.6] Google billing dimensions
    #[serde(default)] pub daily_requests: u64,
    #[serde(default)] pub rpd_limit: u64,
    #[serde(default)] pub current_rpm: u64,
    #[serde(default)] pub rpm_limit: u64,
    #[serde(default)] pub current_tpm: u64,
    #[serde(default)] pub tpm_limit: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetricsResponse {
    pub date: Option<String>,
    #[serde(default)] pub current_minute: Option<String>,
    pub models: Vec<ModelMetrics>,
    pub total_daily_tokens: u64,
    pub total_lifetime_tokens: u64,
    #[serde(default)] pub total_daily_requests: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraceNode {
    pub node: String,
    pub duration_ms: Option<u64>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraceResponse {
    pub total_nodes: u32,
    pub total_ms: u64,
    pub nodes: Vec<TraceNode>,
}

pub async fn get_metrics() -> Result<MetricsResponse, String> {
    get("/agent/metrics").await
}

pub async fn get_trace() -> Result<TraceResponse, String> {
    get("/agent/trace").await
}

pub async fn probe_models() -> Result<serde_json::Value, String> {
    post("/agent/models/probe", &json!({})).await
}

// [V4.9.6] MemPalace 证据图谱 API

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvidenceNode {
    pub id: String,
    pub name: String,
    pub category: u32,
    pub value: f64,
    #[serde(rename = "symbolSize", default)] pub symbol_size: f64,
    #[serde(default)] pub tooltip: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvidenceEdge {
    pub source: String,
    pub target: String,
    #[serde(default)] pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvidenceGraph {
    pub nodes: Vec<EvidenceNode>,
    pub edges: Vec<EvidenceEdge>,
    pub total: u32,
    #[serde(default)] pub message: Option<String>,
}

pub async fn get_palace_graph() -> Result<EvidenceGraph, String> {
    get("/palace/graph").await
}

// [V4.9.6] MemPalace 时间轴 API

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryEvent {
    pub ts: String,
    pub node: String,
    pub event: String,
    pub finding: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimelineResponse {
    pub session_id: String,
    pub events: Vec<MemoryEvent>,
    #[serde(default)] pub error: Option<String>,
}

pub async fn get_memory_timeline(session_id: &str) -> Result<TimelineResponse, String> {
    get(&format!("/palace/timeline?session_id={}", session_id)).await
}

// [V4.9.6] MemPalace 记忆冲突 API

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConflictItem {
    pub id: String,
    pub item_a: String,
    pub item_b: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConflictResponse {
    pub conflicts: Vec<ConflictItem>,
}

pub async fn get_conflicts() -> Result<ConflictResponse, String> {
    get("/palace/conflicts").await
}

pub async fn resolve_conflict(finding_a: &str, finding_b: &str, keep_a: bool, keep_b: bool) -> Result<(), String> {
    let body = json!({
        "finding_a": finding_a,
        "finding_b": finding_b,
        "keep_a": keep_a,
        "keep_b": keep_b
    });
    let _: serde_json::Value = post("/palace/conflicts/resolve", &body).await?;
    Ok(())
}
