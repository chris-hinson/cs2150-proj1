use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;
use tonic::{transport::Server, Request, Response, Status};

use chat_server::auth_server::{Auth, AuthServer};
use chat_server::{CreationResult, LoginRequest, LoginResult, LogoutRequest, LogoutResult, User};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};

use chat_server::chat_server::{Chat, ChatServer};
//use chat_server::{HistoryRequest, HistoryResult, MessagePacket, MessageSendResult};
use chat_server::MessagePacket;

//this mod "holds" all the proto defs that get generated by the include_proto!
//proc macro at compile time. this is why the use statements above it start with this name
pub mod chat_server {
    tonic::include_proto!("chatroom"); // The string specified here must match the proto package name
}

#[derive(Debug, Default, Copy, Clone)]
pub struct ChatServerImpl {}

#[tonic::async_trait]
impl Auth for ChatServerImpl {
    async fn login(
        &self,
        request: Request<LoginRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<LoginResult>, Status> {
        /*// Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting*/
        Ok(Response::new(chat_server::LoginResult {}))
    }
    async fn create_user(
        &self,
        request: Request<User>,
    ) -> Result<Response<CreationResult>, Status> {
        Ok(Response::new(chat_server::CreationResult {}))
    }
    async fn logout(
        &self,
        request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResult>, Status> {
        Ok(Response::new(chat_server::LogoutResult {}))
    }
}
#[tonic::async_trait]
impl Chat for ChatServerImpl {
    type chatlinkStream =
        Pin<Box<dyn Stream<Item = Result<MessagePacket, Status>> + Send + 'static>>;

    /*async fn send_message(
        &self,
        request: Request<MessagePacket>,
    ) -> Result<Response<MessageSendResult>, Status> {
        Ok(Response::new(chat_server::MessageSendResult {}))
    }
    async fn get_history(
        &self,
        request: Request<HistoryRequest>,
    ) -> Result<Response<HistoryResult>, Status> {
        Ok(Response::new(chat_server::HistoryResult {}))
        }*/

    async fn chatlink(
        &self,
        request: Request<tonic::Streaming<MessagePacket>>,
    ) -> Result<Response<Self::chatlinkStream>, Status> {
        let mut stream = request.into_inner();

        let output = async_stream::try_stream! {
            while let Some(note) = stream.next().await {
                //let note = note?;

                //let location = note.location.clone().unwrap();

                //let location_notes = notes.entry(location).or_insert(vec![]);
                //location_notes.push(note);

                /*for note in location_notes {
                    yield note.clone();
                    }*/
                yield note.clone().unwrap();
            }
        };
        Ok(Response::new(Box::pin(output) as Self::chatlinkStream))
    }
}

impl MessagePacket {
    pub fn new() -> MessagePacket {
        Self {}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let chatroom = ChatServerImpl::default();

    Server::builder()
        .add_service(AuthServer::new(chatroom))
        .add_service(ChatServer::new(chatroom))
        .serve(addr)
        .await?;

    Ok(())
}
