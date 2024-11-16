mod client;

use crate::client::Client;

fn main() {
    let mut client = Client::new(8080..9000).expect("Couldn't create client!");
    client.run();
}
