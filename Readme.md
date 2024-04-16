## Notes

## Grading Instructions
Using docker
```
docker build -t my-rust-app .

sudo docker run -it --rm --net=host --name my-running-app my-rust-app
```

To play with it manually:

start server using 
`cargo run --bin chatroom-server 50051` where the first command line argument is the port on which to spawn the server

start as many client instances as you want in seperate shells by doing `cargo run --bin chatroom-client name 50051` where name is the username to connect under and the port is the second argument. clients can be gracefully killed by sending a "/kill" message but because this grpc library is hot trash, theres not a well documented way to gracefully kill the server, so just crtl+c it