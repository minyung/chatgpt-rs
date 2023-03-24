pub mod model;
mod protocol;
mod errors;

pub use errors::*;

use std::time::Duration;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap};
use reqwest::blocking::Client;
use crate::chat::protocol::{CompletionRequest, CompletionResponse, Message};

const CHAT_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

pub struct Chat {
    api_key: String,
    model: String,
    pub history: Vec<String>,
}

impl Chat {
    pub fn new(api_key: &str, model: &str, history: &[String]) -> Self {
        Self {
            api_key: api_key.to_string(),
            model: model.to_string(),
            history: history.to_vec(),
        }
    }

    pub fn run(&mut self, message: &str) -> Result<String, Error> {
        let result = self.run_with_n(message, 1)?;
        Ok(result[0].to_string())
    }

    // n??
    pub fn run_with_n(&mut self, message: &str, n: u64) -> Result<Vec<String>, Error> {
        self.history.push(message.to_string());

        let request = CompletionRequest {
            model: self.model.to_string(),
            messages: Chat::get_messages(self.history.clone()),
            max_tokens: Some(2048),
            temperature: Some(0.1),
            top_p: Some(0.1),
            n: Some(n),
            stream: None,
        };
        Ok(self.run_with_completion_request(&request)?)
    }

    fn run_with_completion_request(&mut self, request: &CompletionRequest) -> Result<Vec<String>, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, format!("Bearer {}", self.api_key).parse()?);
        headers.insert(CONTENT_TYPE, "application/json".parse()?);


        let res = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()?
            .post(CHAT_ENDPOINT.to_string())
            .headers(headers)
            .json(request)
            .send()?;
        if !res.status().is_success() {
            return Err(Error::ChatGpt(res.text()?));
        }
        let completion_response: CompletionResponse = res.json()?;
        let result: Vec<String> = completion_response.choices.iter().map(|val| val.message.content.to_string()).collect();

        self.history.append(&mut result.clone());
        Ok(result)
    }

    pub fn clear_history(&mut self) {
        self.history.clear()
    }

    fn get_messages(strings: Vec<String>) -> Vec<Message> {
        strings.iter().map(|val| {
            Message {
                role: "user".to_string(),
                content: val.to_string(),
            }
        }).collect()
    }
}
