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
    let neuron_count = 1000000;
    let neurons_per_group = 800;
    if (neuron_count % neurons_per_group) != 0 {
        panic!("neuron_count must be divisible by neurons_per_group");
    }
    let group_count = neuron_count / neurons_per_group;

    let nr = network_renderer::NetworkRenderer::new(neurons_per_group, group_count);
    for t in 0..1000 {
        nr.render(t);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    return;
    
    println!("Building network");

    let start = Instant::now();
    let mut network = network::Network::new(neuron_count as usize);
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
