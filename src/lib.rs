use crate::chatroom_data::MessagePacket;

//this mod "holds" all the proto defs that get generated by the include_proto!
//proc macro at compile time. this is why the use statements above it start with this name
pub mod chatroom_data {
    tonic::include_proto!("chatroom"); // The string specified here must match the proto package name
}

impl MessagePacket {
    pub fn new(msg: String, user: String) -> MessagePacket {
        Self {
            msg,
            ts: {
                let start = std::time::SystemTime::now();
                let since_the_epoch = start
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Time went backwards");
                since_the_epoch.as_secs()
            },
            user,
        }
    }
}
