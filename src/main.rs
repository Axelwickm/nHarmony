use std::time::{Instant};

mod network;
mod neuron;
mod synapse;
mod action_potential;
mod event_deque;

mod network_renderer;

#[macro_use]
extern crate glium;
extern crate glutin;

fn main() {
    let nr = network_renderer::NetworkRenderer::new();
    for _ in 0..100 {
        nr.render();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    return;
    
    println!("Building network");

    let start = Instant::now();
    let mut network = network::Network::new(1000000);
    let end = Instant::now();

    println!("{:?}", end.duration_since(start));

    network.print_info();


    println!("Running network");

    network.random_activations();

    // Run many times
    for _ in 0..100000000 {
        network.simulate_next_event();
    }

}
