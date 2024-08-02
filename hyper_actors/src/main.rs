mod actors;

use http_body_util::BodyExt;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use serde::Deserialize;
use serde_json;
use std::convert::Infallible;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, mpsc::Sender};

use actors::messages::MessageType;
use actors::messages::StateActorMessage;
use actors::runner::RunnerActor;
use actors::state::StateActor;

#[derive(Deserialize, Debug)]
struct IncomingBody {
    pub chat_id: i32,
    pub turn: i32,
    pub input: String,
    pub output: String,
}

async fn handle(
    req: Request<hyper::body::Incoming>,
    channel_sender: Sender<StateActorMessage>,
) -> Result<Response<String>, Infallible> {
    println!("incoming message from the outside");

    let method = req.method().clone();
    println!("{}", method);
    let uri = req.uri();
    println!("{}", uri);

    let bytes = req.into_body().collect().await.unwrap().to_bytes();
    let string_body = String::from_utf8(bytes.to_vec()).expect("response was not valid utf-8");
    let value: IncomingBody = serde_json::from_str(&string_body.as_str()).unwrap();

    let message = StateActorMessage {
        message_type: MessageType::INPUT,
        chat_id: Some(value.chat_id),
        single_data: Some(format!(
            "{}>>{}>>{}>>",
            value.input, value.output, value.turn
        )),
        block_data: None,
    };
    channel_sender.send(message).await.unwrap();
    Ok(Response::new(format!("{:?}", value).into()))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000);
    let listener = TcpListener::bind(addr).await.unwrap();

    let (state_tx, state_rx) = mpsc::channel::<StateActorMessage>(1);
    let (runner_tx, runner_rx) = mpsc::channel::<StateActorMessage>(1);

    tokio::spawn(async move {
        let state_actor = StateActor::new(state_rx, runner_tx);
        state_actor.run().await;
    });

    tokio::spawn({
        let state_tx = state_tx.clone();
        async move {
            let lib_runner_actor = RunnerActor::new(runner_rx, state_tx, 30);
            lib_runner_actor.run().await;
        }
    });

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);

        tokio::task::spawn({
            let state_tx = state_tx.clone();

            async {
                let http = http1::Builder::new();
                let conn = http.serve_connection(
                    io,
                    service_fn(move |req| {
                        let state_tx = state_tx.clone();
                        async { handle(req, state_tx).await }
                    }),
                );

                if let Err(err) = conn.await {
                    eprintln!("Error serving connection: {:?}", err);
                }
            }
        });
    }
}
