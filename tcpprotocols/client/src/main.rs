use bincode;
use bytes::Bytes;
use futures::sink::SinkExt;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::io;
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, Decoder};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    pub ticker: String,
    pub amount: f32,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut framed = BytesCodec::new().framed(stream);

    let message = Message {
        ticker: String::from("BYND"),
        amount: 3.2,
    };
    let message_bin = bincode::serialize(&message).unwrap();
    let sending_message = Bytes::from(message_bin);

    framed.send(sending_message).await.unwrap();
    let message = framed.next().await.unwrap().unwrap();
    let message = bincode::deserialize::<Message>(&message).unwrap();
    println!("{:?}", message);

    Ok(())
}
