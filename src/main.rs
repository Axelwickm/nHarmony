mod network;

use std::time::{Instant};

fn main() {
    println!("Hello, world!");

    let start = Instant::now();
    let network = network::Network::new(100000);
    let end = Instant::now();

    network.print_info();

    println!("{:?}", end.duration_since(start));
}
