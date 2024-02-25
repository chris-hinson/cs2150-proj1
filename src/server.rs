use std::pin::Pin;
use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::{mpsc,RwLock};
use std::sync::Arc;

use proj1::chatroom_data::auth_server::{Auth};
use proj1::chatroom_data::{CreationResult, LoginRequest, LoginResult, LogoutRequest, LogoutResult, User};
use tokio_stream::Stream;

use proj1::chatroom_data::chat_server::{Chat, ChatServer};
use proj1::chatroom_data::MessagePacket;
use proj1::chatroom_data::Req;
use proj1::chatroom_data::Nothing;
use std::collections::HashMap;


#[derive(Debug, Default)]
pub struct ChatServerImpl {
    //history: Vec<MessagePacket>
    connections: Arc<RwLock<HashMap<String, mpsc::Sender<MessagePacket>>>>,
}

impl ChatServerImpl{
    /*pub fn insert(&self,msg: MessagePacket){
        let mut gaurd = self.history.lock().unwrap();
        gaurd.push(msg);
    }*/
}

#[tonic::async_trait]
impl Auth for ChatServerImpl {
    #[allow(unused_variables)]
    async fn login(
        &self,
        request: Request<LoginRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<LoginResult>, Status> {
        Ok(Response::new(LoginResult {}))
    }
    #[allow(unused_variables)]
    async fn create_user(
        &self,
        request: Request<User>,
    ) -> Result<Response<CreationResult>, Status> {
        Ok(Response::new(CreationResult {}))
    }
    #[allow(unused_variables)]
    async fn logout(
        &self,
        request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResult>, Status> {
        Ok(Response::new(LogoutResult {}))
    }
}
#[tonic::async_trait]

impl Chat for ChatServerImpl {
    
    #[allow(non_camel_case_types)]
    type chatlinkStream =
        Pin<Box<dyn Stream<Item = Result<MessagePacket, Status>> + Send + 'static>>;

    async fn chatlink(
        &self,
        request: Request<Req>,
    ) -> Result<Response<Self::chatlinkStream>, Status> {

        let name = request.into_inner().username;
        let (stream_tx, stream_rx) = mpsc::channel(1);
        // When connecting, create related sender and reciever
        let (tx, mut rx) = mpsc::channel(1);
        {
            self.connections.write().await.insert(name.clone(), tx);
        }

        let connections_clone = self.connections.clone();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match stream_tx.send(Ok(msg)).await {
                    Ok(_) => {}
                    Err(_) => {
                        // If sending failed, then remove the user from shared data
                        println!(
                            "[Remote] stream tx sending error. Remote {}",
                            &name
                        );
                        connections_clone.write().await.remove(&name);
                    }
                }
            }
        });
        println!("returning stream to client");
        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(stream_rx),
        )))
    }

    async fn send_message(&self, request: Request<MessagePacket>) -> Result<Response<Nothing>,Status>{
        let req_data = request.into_inner();
        println!("got send message command: {:?}",req_data);
        let connections = self.connections.read().await;
        for (key,value) in &*connections{
            println!("key: {}",key);
            value.send(req_data.clone()).await.unwrap();
        }
        

        Ok(Response::new(Nothing {}))
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
