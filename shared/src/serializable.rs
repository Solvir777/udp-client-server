use crate::message::Sendable;

pub trait Serializable {
	fn serialize(&self) -> Vec<u8>;
	fn deserialize(bytes: &mut Vec<u8>) -> Self where Self: Sized;
}
macro_rules! impl_serializable{
	($($t:ty),*) => {
		$(
		impl Serializable for $t {
			fn serialize(&self) -> Vec<u8>{
				self.to_be_bytes().to_vec()
			}
			
			fn deserialize(all_bytes: &mut Vec<u8>) -> Self {
				let mut array = [0u8; std::mem::size_of::<Self>()];
				array.copy_from_slice(&all_bytes[..std::mem::size_of::<Self>()]);
				all_bytes.drain(..std::mem::size_of::<Self>());
				Self::from_be_bytes(array)
			}
		}
		)*
	}
}

impl_serializable!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl Serializable for String {
	fn serialize(&self) -> Vec<u8> {
		self.as_bytes().to_vec()
	}
	
	fn deserialize(bytes: &mut Vec<u8>) -> String {
		let end = bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len());
		let c = bytes.drain(..end).collect();
		String::from_utf8(c).expect("Failed to parse string")
	}
}
impl Sendable for String{
	fn id(&self) -> u8 {1}
}