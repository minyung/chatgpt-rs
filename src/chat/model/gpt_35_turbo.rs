use crate::chat::Chat;

pub struct Gpt35TurboChat;

impl Gpt35TurboChat {
    pub fn new(api_key: &str) -> Chat {
        Gpt35TurboChat::new_with_history(api_key, &vec![])
    }

    pub fn new_with_history(api_key: &str, history: &[String]) -> Chat {
        Chat::new(api_key, "gpt-3.5-turbo", history)
    }
}
