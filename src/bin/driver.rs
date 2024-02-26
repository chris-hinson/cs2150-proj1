use std::io::Write;
use std::process::{Command, Stdio};
pub fn main() {
    let mut server = Command::new("./target/debug/chatroom-server")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");
    let mut server_stdin = server.stdin.take().expect("Failed to open server stdin");

    std::thread::sleep(std::time::Duration::from_millis(2000));

    let mut client_pool:Vec<(std::process::Child,std::process::ChildStdin)> = Vec::new();

    for i in 0..2{
        let mut client = Command::new("./target/debug/chatroom-client")
        .arg(format!("{}",i))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn client 1");
        
        let mut client_stdin = client.stdin.take().expect("Failed to open client 1 stdin");

        client_pool.push((client,client_stdin));
    }


    send_message_to_client(&mut client_pool[0].1, "client 0 says hello".to_string());
    send_message_to_client(&mut client_pool[1].1, "client 1 says hello".to_string());



    // sleep a bit so that child can process the input
    std::thread::sleep(std::time::Duration::from_millis(1000));

    //kill server
    kill_process(server.id() as i32);

    //inspect cihld and server output
    let server_output = server.wait_with_output().expect("Failed to read stdout");
    println!("server output: {:?}", server_output);
    //for i in 0..client_pool.len(){
    for (i,mut client) in client_pool.into_iter().enumerate(){
        send_message_to_client(&mut client.1,"/kill".to_string());
        let client_output = client.0.wait_with_output().expect("Failed to read client stdout");
        println!("client {} output: {:?}",i,client_output);
    }

}


pub fn send_message_to_client( client_handle:&mut std::process::ChildStdin, msg: String){
    client_handle.write_all(&format!("{}\n",msg).into_bytes()).expect("failed to write to client");
    client_handle.flush().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));
}

pub fn kill_process(id: i32){
    nix::sys::signal::kill(
        nix::unistd::Pid::from_raw(id),
        nix::sys::signal::Signal::SIGINT,
    ).expect(&format!("failed to kill process with PID: {}",id))
}