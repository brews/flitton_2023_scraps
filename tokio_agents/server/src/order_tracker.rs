use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};

/// Order commands that can be sent to Tracker Actor
#[derive(Debug, Clone)]
pub enum Order {
    BUY(String, f32),
    GET,
}

/// Messages sent to the TrackerActor
pub struct TrackerMessage {
    pub command: Order,
    pub respond_to: oneshot::Sender<String>,
}

/// Actor that sends GET query to TrackerActor for order information
pub struct GetTrackerActor {
    pub sender: mpsc::Sender<TrackerMessage>,
}
impl GetTrackerActor {
    /// Send GET query message to TrackerActor
    pub async fn send(self) -> String {
        println!("GET function firing");
        let (send, recv) = oneshot::channel();
        let message = TrackerMessage {
            command: Order::GET,
            respond_to: send,
        };
        let _ = self.sender.send(message).await;
        match recv.await {
            Err(e) => panic!("{}", e),
            Ok(outcome) => return outcome,
        }
    }
}

/// Actor that tracks placed orders.
pub struct TrackerActor {
    pub receiver: mpsc::Receiver<TrackerMessage>,
    pub db: HashMap<String, f32>,
}
impl TrackerActor {
    /// Create new TrackerActor given a rx it listens to for messages.
    pub fn new(receiver: mpsc::Receiver<TrackerMessage>) -> Self {
        TrackerActor {
            receiver,
            db: HashMap::new(),
        }
    }

    fn send_state(&self, respond_to: oneshot::Sender<String>) {
        // Turn state into 'key:value;key2:value2;\n'
        let mut buffer = self
            .db
            .iter()
            .map(|(k, v)| format!("{k}:{v};"))
            .collect::<Vec<String>>();
        buffer.push("\n".to_string());
        let out = buffer.join("");

        println!("sending state: {out}");
        let _ = respond_to.send(out);
    }

    fn handle_message(&mut self, message: TrackerMessage) {
        match message.command {
            Order::BUY(ticker, amount) => {
                // If an order already exists for the ticker we need to add to the existing amount and not just overwrite it.
                match self.db.get(&ticker) {
                    Some(ticker_amount) => {
                        self.db.insert(ticker, ticker_amount + amount);
                    }
                    None => {
                        self.db.insert(ticker, amount);
                    }
                }
                println!("db: {:?}", self.db);
            }
            Order::GET => {
                println!("getting state");
                self.send_state(message.respond_to);
            }
        };
    }

    /// Have TrackerActor run its main loop, awaiting messages.
    pub async fn run(mut self) {
        println!("tracker actor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }
}
