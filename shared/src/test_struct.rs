use crate::message::Sendable;
use crate::Serializable;

#[derive(Debug)]
pub struct TestStruct {
	pub a: i32,
	pub b: i128,
	pub c: f64,
}

impl TestStruct {
	pub fn new() -> Self {
		Self{
			a: 900,
			b: 100000,
			c: f64::INFINITY,
		}
	}
}

impl Serializable for TestStruct {
	fn serialize(&self) -> Vec<u8> {
		let mut ret = vec!();
		ret.extend(self.a.serialize());
		ret.extend(self.b.serialize());
		ret.extend(self.c.serialize());
		ret
	}
	
	fn deserialize(bytes: &mut Vec<u8>) -> Self {
		let a = i32::deserialize(bytes);
		let b = i128::deserialize(bytes);
		let c = f64::deserialize(bytes);
		TestStruct { a, b, c }
	}
}

impl Sendable for TestStruct {
	fn id(&self) -> u8 {0}
}