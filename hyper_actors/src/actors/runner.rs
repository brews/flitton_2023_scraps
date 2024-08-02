use crate::actors::messages::{MessageType, StateActorMessage};
use tokio::sync::mpsc::{Receiver, Sender};

use std::time;

/// Actor that constantly loops, sending messages to the state actor asking for batched data, and sending the batched data to the server batched data is present.
pub struct RunnerActor {
    /// Number of seconds actor will wait before making another data request.
    pub interval: i32,
    pub receiver: Receiver<StateActorMessage>,
    pub sender: Sender<StateActorMessage>,
}

impl RunnerActor {
    pub fn new(
        receiver: Receiver<StateActorMessage>,
        sender: Sender<StateActorMessage>,
        interval: i32,
    ) -> RunnerActor {
        // We don't really need this new() constructor now but it's nice to have if we need to add additional functionality in the future.
        return RunnerActor {
            interval,
            receiver,
            sender,
        };
    }

    /// Runs the actors main loop. Sending messages to state actor and server.
    pub async fn run(mut self) {
        println!("runner actor is running");
        let seconds = time::Duration::from_secs(self.interval as u64);

        loop {
            tokio::time::sleep(seconds).await;

            let message = StateActorMessage {
                message_type: MessageType::OUTPUT,
                chat_id: None,
                single_data: None,
                block_data: None,
            };

            match self.sender.send(message).await {
                Ok(_) => {
                    let message = self.receiver.recv().await.unwrap();
                    match message.message_type {
                        MessageType::OUTPUT => {
                            message.send_to_server().await;
                        }
                        _ => {
                            println!("state is empty");
                        }
                    }
                }
                Err(_) => {
                    println!("runner failed to send message");
                }
            };
        }
    }
}
