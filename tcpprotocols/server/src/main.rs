use tokio::net::TcpListener;
use tokio_util::codec::{BytesCodec, Decoder};

use bytes::Bytes;
use futures::sink::SinkExt;
use futures::StreamExt;

use bincode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    pub ticker: String,
    pub amount: f32,
}

#[tokio::main]
async fn main() {
    let addr = String::from("127.0.0.1:8080");
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {addr}");

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut framed = BytesCodec::new().framed(stream);
            let message = framed.next().await.unwrap();
            match message {
                Ok(bytes) => {
                    let message = bincode::deserialize::<Message>(&bytes).unwrap();
                    println!("{:?}", message);
                    let message_bin = bincode::serialize(&message).unwrap();
                    let sending_message = Bytes::from(message_bin);
                    framed.send(sending_message).await.unwrap();
                }
                Err(err) => println!("Socket closed with error: {:?}", err),
            }
            println!("Socket received FIN packet and closed connection");
        });
    }
}
