use std::collections::{HashMap, VecDeque};
use std::mem;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::actors::messages::{MessageType, StateActorMessage};

/// State actor sends and receives messages. Also has state to store/reference chat logs.
#[derive(Debug)]
pub struct StateActor {
    // chat_queue is VecDeque because popping off the first element doesn't require reallocating all other elements.
    /// FIFO chat queue. Assumes older chat logs will have more chats than newer chat logs.
    pub chat_queue: VecDeque<i32>,
    /// Logs stored with chat ID as key and value gives vector of Q&A as a String.
    pub chat_logs: HashMap<i32, Vec<String>>, // ChatID is key, value is Q&A String.
    pub receiver: Receiver<StateActorMessage>,
    pub sender: Sender<StateActorMessage>,
}

impl StateActor {
    /// Constructs a new StateActor with empty chat log and queue, given channels to send and receive messages.
    pub fn new(
        receiver: Receiver<StateActorMessage>,
        sender: Sender<StateActorMessage>,
    ) -> StateActor {
        let chat_queue: VecDeque<i32> = VecDeque::new();
        let chat_logs: HashMap<i32, Vec<String>> = HashMap::new();
        return StateActor {
            chat_queue,
            chat_logs,
            receiver,
            sender,
        };
    }

    /// Pull chat messages from chat log cache or storage.
    pub fn get_message_data(&mut self, chat_id: i32) -> Vec<String> {
        return self.chat_logs.remove(&chat_id).unwrap();
    }

    /// Insert chat messages into state..
    pub fn insert_message(&mut self, chat_id: i32, message_data: String) {
        match self.chat_logs.get_mut(&chat_id) {
            Some(patient_log) => {
                // Log for Chat ID already exists so append to it.
                patient_log.push(message_data);
            }
            None => {
                // Log under chat ID doesn't exist so need to create need entry in BOTH log and queue.
                self.chat_queue.push_back(chat_id);
                self.chat_logs.insert(chat_id, vec![message_data]);
            }
        }
    }

    /// Handle input message and dispatch work accordingly based on message type.
    async fn handle_message(&mut self, message: StateActorMessage) {
        println!("state actor is receiving a message");

        match message.message_type {
            MessageType::INPUT => {
                // Simply insert the message.
                self.insert_message(message.chat_id.unwrap(), message.single_data.unwrap());
            }
            MessageType::OUTPUT => {
                // Must get the oldest chats from the bottom of the queue.
                match self.chat_queue.pop_front() {
                    Some(chat_id) => {
                        // Got message data. Send it to the runner actor.
                        let data = self.get_message_data(chat_id);
                        let message = StateActorMessage {
                            message_type: MessageType::OUTPUT,
                            chat_id: Some(chat_id),
                            single_data: None,
                            block_data: Some(data),
                        };
                        let _ = self.sender.send(message).await.unwrap();
                    }
                    None => {
                        // If there is nothing in the queue then send empty message to runner actor.
                        let message = StateActorMessage {
                            message_type: MessageType::EMPTY,
                            chat_id: None,
                            single_data: None,
                            block_data: None,
                        };
                        let _ = self.sender.send(message).await.unwrap();
                    }
                }
            }
            MessageType::EMPTY => {
                // This should not happen.
                // Kinda makes me feel like the message system should be redesigned to avoid this.
                panic!("empty messages should not be sent to the state actor");
            }
        }
        println!("{:?}", self.chat_logs);
        println!("{:?}", self.chat_queue);
    }

    // Run main loop of actor, await and handle messages.
    pub async fn run(mut self) {
        println!("state actor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }
}
