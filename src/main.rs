mod network;

use std::time::{Instant};

fn main() {
    println!("Hello, world!");

    let start = Instant::now();
    let mut network = network::Network::new(1000000);
    let end = Instant::now();

    println!("{:?}", end.duration_since(start));

    network.print_info();

    network.random_activations();

}
