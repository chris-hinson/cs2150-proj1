use std::io::Write;
use std::process::{Command, Stdio};
pub fn main() {
    let mut server = Command::new("./target/debug/chatroom-server")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut server_stdin = server.stdin.take().expect("Failed to open stdin");
    /*std::thread::spawn(move || {
        server_stdin
            .write_all("Hello, world!".as_bytes())
            .expect("Failed to write to stdin");
    });*/
    // sleep a bit so that child can process the input
    std::thread::sleep(std::time::Duration::from_millis(500));

    // send SIGINT to the child
    nix::sys::signal::kill(
        nix::unistd::Pid::from_raw(server.id() as i32),
        nix::sys::signal::Signal::SIGINT,
    )
    .expect("cannot send ctrl-c");

    // wait for child to terminate
    //child.wait().unwrap();
    let server_output = server.wait_with_output().expect("Failed to read stdout");
    println!("{:?}", server_output)
}
