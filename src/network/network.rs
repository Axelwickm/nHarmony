pub mod neuron;
use rand::{thread_rng, Rng};

extern crate rayon;
use rayon::prelude::*;

extern crate kdtree;
use kdtree::KdTree;
use kdtree::distance::squared_euclidean;

pub struct Network {
    pub neurons: Vec<neuron::Neuron>,
}

impl Network {
    pub fn new(n_neurons: usize) -> Network {
        const TARGET_DENSITY: f64 = 0.5;
        let fnn : f64 = n_neurons as f64;

        let size = f64::powf(fnn / TARGET_DENSITY, 1.0 / 3.0);
        let half = size / 2.0;

        // Run parallel for each neuron
        let mut neurons : Vec<neuron::Neuron> = (0..n_neurons).into_par_iter().map(|i| neuron::Neuron::new(i, half)).collect();

        let connection_inds = Network::form_connections(&neurons);
        for (i, connections) in connection_inds.iter().enumerate() {
            neurons[i].connections = connections.iter().map(|(id, dist)| (*id, (f64::max(1.0, dist/5.0)*255.0) as u8, 128)).collect();
        }

        Network { neurons }
    }

    fn form_connections(neurons: &Vec<neuron::Neuron>) -> Vec<Vec<(usize, f64)>> {
        let mut kdtree = KdTree::new(3);
        for neuron in neurons {
            kdtree.add(neuron.coords, neuron).unwrap();
        }

        // In parallel for each neuron, find the clostest neighbours
        let connection_refs : Vec<(&neuron::Neuron, Vec<(f64, &&neuron::Neuron)>)> =
            neurons.into_par_iter().map(|neuron| {
                let nearest = kdtree.nearest(&neuron.coords, 15, &squared_euclidean).unwrap();
                // Filter out too far away
                let neighbours : Vec<(f64, &&neuron::Neuron)>  = nearest.into_iter().filter(|(dist, _)| {
                    dist < &5.0
                }).collect();

                (neuron, neighbours)
            }).collect();

        // Convert references to indexes
        let connection_inds = connection_refs.into_iter().map(|(_, neighbours)| {
            let mut connections = Vec::new();
            for (dist, neighbour) in neighbours {
                connections.push((neighbour.id, dist));
            }
            connections
        }).collect();

        connection_inds
    }

    pub fn random_activations(&mut self) {
        // Go through every neuron and set a random u8 activation value
        for neuron in &mut self.neurons {
            neuron.random_activation();
        }

    }

    pub fn print_info(&self) {
        println!("Network info:");
        println!("\tNeurons: {}", self.neurons.len());
    }
}
