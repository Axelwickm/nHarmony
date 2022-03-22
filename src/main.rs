
use std::time::{Instant};

mod network;
mod neuron;
mod action_potential;
mod event_deque;

fn main() {
    println!("Building network");

    let start = Instant::now();
    let mut network = network::Network::new(1000000);
    let end = Instant::now();

    println!("{:?}", end.duration_since(start));

    network.print_info();


    println!("Running network");

    network.random_activations();

    // Run 10000 times
    for _ in 0..10000 {
        network.simulate_next_event();
    }

}
