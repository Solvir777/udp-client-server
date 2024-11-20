use crate::Serializable;
use sendable_derive::Sendable;
use serializable_derive::Serializable;
use crate::sendable::Sendable;

#[derive(Debug, Serializable, Sendable)]
pub struct TestStruct {
	pub a: i32,
	pub b: f32,
	pub c: f64,
}
impl TestStruct {
	pub fn new() -> Self {
		Self{
			a: 900,
			b: 0.5,
			c: 500.,
		}
	}
}