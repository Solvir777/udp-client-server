use std::net::{SocketAddr, UdpSocket};
use shared::{BUFFER_SIZE, SERVER_ADDRESS};
use shared::message::{Message, MessageType};

fn main() {
    let addresses: Vec<SocketAddr> = (8080..=9000)
        .map(|port| format!("127.0.0.1:{}", port)
            .parse()
            .unwrap())
        .collect();
    
    let socket = UdpSocket::bind(&addresses[..]).expect("Could not bind socket");
    socket.connect(SERVER_ADDRESS).expect("Could not connect to server");
    
    let buf = b"Hello World";
    socket.send(buf).expect("Could not send data");
    
    loop {
        let mut buf = [0u8; BUFFER_SIZE];
        socket.recv(&mut buf).expect("Could not receive message");
        let mut bytes = buf.to_vec();
        match Message::deserialize(&mut bytes) {
            MessageType::TestStruct(ts) => {
                println!("Received TestStruct {:?}", ts);
            },
            MessageType::String(str) => {
                println!("Received String {:?}", str);
            }
        }
        
        
    }
}
