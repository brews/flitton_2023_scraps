use serde::Serialize;
use std::env;

/// Types of messages sent between actors.
#[derive(Debug, Serialize)]
pub enum MessageType {
    INPUT,
    OUTPUT,
    EMPTY,
}

/// Messages sent to and from the state actor.
#[derive(Debug, Serialize)]
pub struct StateActorMessage {
    pub message_type: MessageType,
    pub chat_id: Option<i32>,
    pub single_data: Option<String>,
    pub block_data: Option<Vec<String>>, // Uses "$" as delimiter to separate pairs of questions and answers.
}

impl StateActorMessage {
    /// Send blocks of cached messages to the server.
    pub async fn send_to_server(&self) {
        let lib_url = env::var("SERVER_URL").unwrap();
        let joined = self.block_data.clone().unwrap().join("&");

        let body = PostBody {
            chat_id: self.chat_id.unwrap(),
            block_data: joined,
        };

        let client = reqwest::Client::new();
        let res = client.post(lib_url).json(&body).send().await.unwrap();
        println!("{:?}", res);
    }
}

#[derive(Debug, Serialize)]
struct PostBody {
    pub chat_id: i32,
    pub block_data: String,
}
