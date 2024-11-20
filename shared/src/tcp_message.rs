use crate::{Serializable, TestStruct};
use crate::sendable::Sendable;

pub trait TCP: Sendable{}

pub struct TCPMessage {
	content: Box<dyn Sendable>
}


impl TCPMessage {
	pub fn new(content: Box<dyn Sendable>) -> Self {
		Self { content }
	}
	
	pub fn serialize(&self) -> Vec<u8> {
		let mut v = vec![self.content.unique_type_id()];
		v.extend(self.content.serialize());
		v
	}
	
	pub fn deserialize(bytes: &mut Vec<u8>) -> MessageTypeTCP {
		match bytes.remove(0) {
			0 => {
				MessageTypeTCP::TestStruct(TestStruct::deserialize(bytes))
			},
			1 => {
				MessageTypeTCP::String(String::deserialize(bytes))
			}
			_ => { panic!("Unrecognized Type Id received!") }
		}
	}
}

pub enum MessageTypeTCP {
	TestStruct(TestStruct),
	String(String),
}