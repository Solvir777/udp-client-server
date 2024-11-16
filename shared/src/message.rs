use crate::{Serializable, TestStruct};

pub struct Message {
	content: Box<dyn Sendable>
}


impl Message {
	pub fn new(content: Box<dyn Sendable>) -> Self {
		Self { content }
	}
	
	pub fn serialize(&self) -> Vec<u8> {
		let mut v = vec![self.content.id()];
		v.extend(self.content.serialize());
		v
	}
	
	pub fn deserialize(bytes: &mut Vec<u8>) -> MessageType {
		match bytes.remove(0) {
			0 => {
				MessageType::TestStruct(TestStruct::deserialize(bytes))
			},
			1 => {
				MessageType::String(String::deserialize(bytes))
			}
			_ => { panic!("Unrecognized Type Id received!") }
		}
	}
}

pub enum MessageType{
	TestStruct(TestStruct),
	String(String),
}

pub trait Sendable: Serializable{
	fn id(&self) -> u8;
}