pub mod neuron;
use std::rand::{thread_rng, Rng};

pub struct Network {
    pub neurons: Vec<neuron::Neuron>,
}

impl Network {
    pub fn new(n_neurons: usize) -> Network {
        const target_density: f64 = 0.5;
        let fnn : f64 = n_neurons as f64;

        let size = pow(fnn / target_density, 1.0 / 3.0);

        let mut neurons = Vec::new();
        for _ in 0..n_neurons {
            // Random position
            neurons.push(neuron::Neuron::new());
        }
        Network { neurons }

    }
}
