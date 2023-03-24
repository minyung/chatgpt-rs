use serde::{Deserialize, Serialize};

// https://platform.openai.com/docs/api-reference/chat/create
#[derive(Serialize)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub n: Option<u64>,
    pub stream: Option<bool>,
    pub max_tokens: Option<u64>,
}

#[derive(Deserialize)]
pub struct CompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: Message,
    index: i32,
    finish_reason: String,
}

#[derive(Deserialize)]
struct Usage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}
