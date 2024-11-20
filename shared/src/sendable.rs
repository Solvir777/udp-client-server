use crate::Serializable;

pub trait Sendable: Serializable {
	fn unique_type_id(&self) -> u8;
}