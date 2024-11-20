use std::collections::HashSet;
use std::net::{SocketAddr, UdpSocket};
use std::time::Instant;
use shared::{SERVER_ADDRESS, BUFFER_SIZE, sleep, TestStruct, Serializable};
use shared::udp_message::UDPMessage;
use shared::sendable::Sendable;

pub struct Server {
	udp: UdpSocket,
	creation_time: Instant,
	active_connections: HashSet<SocketAddr>,
}


impl Server {
	pub(crate) fn new() -> Self {
		let udp = UdpSocket::bind(SERVER_ADDRESS).unwrap();
		udp.set_nonblocking(true).unwrap();
		Self{
			udp,
			creation_time: Instant::now(),
			active_connections: HashSet::new(),
		}
	}
	
	pub(crate) fn run(&mut self) -> !{
		let mut buf = [0; BUFFER_SIZE];
		loop{
			
			if let Ok((_, addr)) = self.udp.recv_from(&mut buf) {
				self.active_connections.insert(addr);
			}
			
			for addr in &self.active_connections {
				if(self.creation_time.elapsed().as_secs() % 4 == 0){
					let mut t = TestStruct::new();
					t.b = self.creation_time.elapsed().as_secs_f32();
					self.send(addr, Box::new(t));
				}
				
				else{
					let mut t = TestStruct::new();
					t.c = self.creation_time.elapsed().as_secs_f64();
					self.send(addr, Box::new(t));
				}
				
				
			}
			
			sleep(100);
		}
	}
	fn send(&self, addr: &SocketAddr, msg: Box<dyn Sendable>) {
		let message = UDPMessage::new(msg);
		
		self.udp.send_to(&*message.serialize(), addr).unwrap();
	}
}