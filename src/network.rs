pub mod neuron;
use rand::{thread_rng, Rng};

extern crate rayon;
use rayon::prelude::*;


pub struct Network <'a> {
    pub neurons: Vec<neuron::Neuron<'a>>,
}

impl<'a> Network <'a> {
    pub fn new(n_neurons: usize) -> Network<'a> {
        const TARGET_DENSITY: f64 = 0.5;
        let fnn : f64 = n_neurons as f64;

        let size = (f64::powf(fnn / TARGET_DENSITY, 1.0 / 3.0)).ceil() as i64;
        let half = size / 2;

        // Run parallel for each neuron
        let neurons = (0..n_neurons).into_par_iter().map(|_| {
            let mut rng = thread_rng();
            let x = rng.gen_range(-half, half);
            let y = rng.gen_range(-half, half);
            let z = rng.gen_range(-half, half);
            let neuron = neuron::Neuron {
                weight: 255,
                x, y, z,
                connected_neurons: Vec::new(),
            };
            neuron
        }).collect();

        Network { neurons }
    }

    pub fn print_info(&self) {
        println!("Network info:");
        println!("\tNeurons: {}", self.neurons.len());
    }
}
