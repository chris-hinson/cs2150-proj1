//this mod "holds" all the proto defs that get generated by the include_proto!
//proc macro at compile time. this is why the use statements above it start with this name
/*pub mod chat_client {
    tonic::include_proto!("chatroom"); // The string specified here must match the proto package name
}*/
use proj1::chatroom_data::chat_client::ChatClient;
use proj1::chatroom_data::MessagePacket;
use std::error::Error;
use tonic::transport::Channel;
use tonic::Request;

use std::sync::Arc;
use tokio::sync::RwLock;

//use colored::Colorize;
use std::env;

use async_stdin::recv_from_stdin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    //println!("{}",args[1]);
    //create client and connect to the central sever
    let mut client = ChatClient::connect("http://[::1]:50051").await?;

    //TODO: auth or smth idfk

    let rx = recv_from_stdin(10);
    run_chatlink(&mut client, rx, args[1].clone()).await?;

    //once we reach here, the client has attempted to disconnect, so make the deauth call for them implicitly

    Ok(())
}

#[allow(unreachable_code)]
async fn run_chatlink(
    client: &mut ChatClient<Channel>,
    mut rx: tokio::sync::mpsc::Receiver<String>,
    name: String,
) -> Result<(), Box<dyn Error>> {
    let response = client
        .chatlink(Request::new(proj1::chatroom_data::Req {
            username: name.clone(),
        }))
        .await?;
    let mut inbound = response.into_inner();

    let shared_buf: Arc<RwLock<Vec<MessagePacket>>> = Arc::new(RwLock::new(Vec::new()));
    let shared_buf_clone = shared_buf.clone();
    tokio::spawn(async move {
        loop {
            match inbound.message().await {
                Ok(val) => match val {
                    Some(val) => {
                        let mut gaurd = shared_buf_clone.write().await;
                        gaurd.push(val);
                        drop(gaurd);
                    }
                    None => {
                        println!("no incoming message found")
                    }
                },
                Err(_e) => {}
            }
        }
    });

    //TODO: make this loop breakable
    loop {
        let mut temp_buf = shared_buf.write().await;
        for i in &*temp_buf {
            println!("{:?}", i);
        }
        *temp_buf = Vec::new();

        match rx.try_recv() {
            Ok(msg) => {
                client
                    .send_message(proj1::chatroom_data::MessagePacket::new(msg, name.clone()))
                    .await?;
            }
            Err(_e) => {}
        }

        drop(temp_buf)
    }

    Ok(())
}

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
