pub use serializable::Serializable;
pub mod message;
mod serializable;
mod test_struct;
pub use test_struct::TestStruct;
pub const SERVER_ADDRESS: &str = "127.0.0.1:6000";
pub const BUFFER_SIZE: usize = 2048;



pub fn sleep(t: u64) {
    std::thread::sleep(std::time::Duration::from_millis(t));
}