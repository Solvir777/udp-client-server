use crate::{Serializable, TestStruct};
use crate::sendable::Sendable;
pub struct UDPMessage {
	content: Box<dyn Sendable>
}

impl UDPMessage {
	pub fn new(content: Box<dyn Sendable>) -> Self {
		Self { content }
	}
	
	pub fn serialize(&self) -> Vec<u8> {
		let mut v = vec![self.content.unique_type_id()];
		v.extend(self.content.serialize());
		v
	}
	
	pub fn deserialize(bytes: &mut Vec<u8>) -> MessageTypeUDP {
		match bytes.remove(0) {
			0 => {
				MessageTypeUDP::TestStruct(TestStruct::deserialize(bytes))
			},
			1 => {
				MessageTypeUDP::String(String::deserialize(bytes))
			}
			_ => { panic!("Unrecognized Type Id received!") }
		}
	}
}

pub enum MessageTypeUDP {
	TestStruct(TestStruct),
	String(String),
}