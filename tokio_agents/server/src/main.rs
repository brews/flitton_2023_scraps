mod actors;
mod order_tracker;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::mpsc;

use actors::{BuyOrder, Message, OrderBookActor};
use order_tracker::{GetTrackerActor, TrackerActor, TrackerMessage};

#[tokio::main]
async fn main() {
    let addr = String::from("127.0.0.1:8080");

    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    let (tx, rx) = mpsc::channel::<Message>(1);
    let (tracker_tx, tracker_rx) = mpsc::channel::<TrackerMessage>(1);

    tokio::spawn(async {
        TrackerActor::new(tracker_rx).run().await;
    });
    tokio::spawn({
        let tracker_tx = tracker_tx.clone();
        async move {
            OrderBookActor::new(rx, tracker_tx, 20.0).run().await;
        }
    });
    println!("order book running now");

    // Ideally, should limit number of sockets with an upper bound with a <Arc<Semiphore>> or maybe tokio's 'backoff' arg in socket.listen()?
    while let Ok((mut stream, peer)) = listener.accept().await {
        println!("Incoming connection from: {}", peer.to_string());
        let tx = tx.clone();
        let tracker_tx = tracker_tx.clone();

        tokio::spawn(async move {
            println!("thread starting {} starting", peer.to_string());
            let (reader, mut writer) = stream.split();

            let mut buf_reader = BufReader::new(reader);
            let mut buf = vec![];

            loop {
                match buf_reader.read_until(b'\n', &mut buf).await {
                    Ok(n) => {
                        if n == 0 {
                            println!("EOF received");
                            break;
                        }

                        let buf_string = String::from_utf8_lossy(&buf);
                        let data: Vec<String> = buf_string
                            .split(";")
                            .map(|x| x.to_string().replace("\n", ""))
                            .collect();
                        println!("here is the data {:?}", data);

                        // Parse command input command.
                        // Assuming first element is the actual command.
                        let command = data[0].clone();
                        match command.as_str() {
                            "BUY" => {
                                println!("buy order command processed");
                                let amount = data[1].parse::<f32>().unwrap();
                                let ticker = data[2].clone();
                                let order_actor = BuyOrder::new(amount, ticker, tx.clone());
                                println!("{}: {}", order_actor.ticker, order_actor.amount);
                                order_actor.send().await;
                            }
                            "GET" => {
                                println!("get order command processed");
                                let get_actor = GetTrackerActor {
                                    sender: tracker_tx.clone(),
                                };
                                let state = get_actor.send().await;
                                println!("sending back: {:?}", state);
                                writer.write_all(state.as_bytes()).await.unwrap();
                            }
                            _ => {
                                panic!("{} command not supported", command);
                            }
                        }
                        buf.clear();
                    }
                    Err(e) => println!("Error receiving message: {}", e),
                }
            }
            println!("thread {} finishing", peer.to_string());
        });
    }
}
