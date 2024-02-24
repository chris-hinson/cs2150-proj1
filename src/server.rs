use std::pin::Pin;
use tonic::{transport::Server, Request, Response, Status};
use std::sync::{Arc,Mutex};

use proj1::chatroom_data::auth_server::{Auth};
use proj1::chatroom_data::{CreationResult, LoginRequest, LoginResult, LogoutRequest, LogoutResult, User};
use tokio_stream::{Stream, StreamExt};

use proj1::chatroom_data::chat_server::{Chat, ChatServer};
//use chat_server::{HistoryRequest, HistoryResult, MessagePacket, MessageSendResult};
use proj1::chatroom_data::MessagePacket;
//use async_mutex::Mutex;

#[derive(Debug, Default)]
pub struct ChatServerImpl {
    history: Vec<MessagePacket>
}

impl ChatServerImpl{
    /*pub fn insert(&self,msg: MessagePacket){
        let mut gaurd = self.history.lock().unwrap();
        gaurd.push(msg);
    }*/
}

#[tonic::async_trait]
impl Auth for ChatServerImpl {
    async fn login(
        &self,
        request: Request<LoginRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<LoginResult>, Status> {
        Ok(Response::new(LoginResult {}))
    }
    async fn create_user(
        &self,
        request: Request<User>,
    ) -> Result<Response<CreationResult>, Status> {
        Ok(Response::new(CreationResult {}))
    }
    async fn logout(
        &self,
        request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResult>, Status> {
        Ok(Response::new(LogoutResult {}))
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
        println!("entering chatlink function in server");
        let mut stream = request.into_inner();

        let output = async_stream::stream! {
            while let Some(note) = stream.next().await {
                println!("got a message");
                //let mut gaurd = self.history.lock().unwrap();s
                //gaurd.push(note.clone().unwrap());
                //self.history.drop();
                //std::mem::drop(gaurd);

                //let mut gaurd = self.history.lock().await;
                //gaurd.push(note.clone().unwrap());
                //drop(gaurd);

                //self.insert(note.clone().unwrap());
                //let mut fuckyou = self.history.clone();
                //fuckyou.push(note.clone().unwrap());
                //self.history = fuckyou;
                yield Ok(note.clone().unwrap());
            }


        };

        println!("returning stream to client");
        Ok(Response::new(Box::pin(output) as Self::chatlinkStream))
    }
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let chatroom = ChatServerImpl::default();

    Server::builder()
        //.add_service(AuthServer::new(chatroom))
        .add_service(ChatServer::new(chatroom))
        .serve(addr)
        .await?;

    Ok(())
}
