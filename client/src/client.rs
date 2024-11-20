use std::net::{SocketAddr, UdpSocket};
use std::ops::Range;
use shared::{BUFFER_SIZE, SERVER_ADDRESS};
use shared::udp_message::{UDPMessage, MessageTypeUDP};

pub struct Client {
	socket: UdpSocket,
}

impl Client {
	pub(crate) fn run(mut self) -> !{
		loop {
			let mut buf = [0u8; BUFFER_SIZE];
			if let Ok(_) = self.socket.recv(&mut buf) {
				match UDPMessage::deserialize(&mut buf.to_vec()) {
					MessageTypeUDP::TestStruct(ts) => {
						println!("Received TestStruct {:?}", ts);
					},
					MessageTypeUDP::String(str) => {
						println!("Received String {:?}", str);
					}
				}
			}
		}
	}
}

impl Client {
	pub fn new(ports: Range<u32>) -> Result<Self, std::io::Error> {
		let addresses: Vec<SocketAddr> = (ports)
			.map(|port| format!("127.0.0.1:{}", port)
				.parse()
				.unwrap())
			.collect();
		
		let socket = UdpSocket::bind(&addresses[..])?;
		socket.connect(SERVER_ADDRESS)?;
		socket.set_nonblocking(true)?;
		socket.send(b"")?;
		Ok(Self{
			socket,
		})
	}
}