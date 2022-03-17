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
        let mut neurons : Vec<neuron::Neuron> = (0..n_neurons).into_par_iter().map(|i| {
            let mut rng = thread_rng();
            let x = rng.gen_range(-half..half);
            let y = rng.gen_range(-half..half);
            let z = rng.gen_range(-half..half);
            let neuron = neuron::Neuron {
                id: i,
                weight: 255,
                coords: [x, y, z],
                connections: Vec::new(),
            };

            neuron
        }).collect();

        let connection_inds = Network::form_connections(&neurons);
        for (i, connections) in connection_inds.iter().enumerate() {
            neurons[i].connections = connections.to_vec();
        }

        Network { neurons }
    }

    fn form_connections(neurons: &Vec<neuron::Neuron>) -> Vec<Vec<usize>> {
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
            for (_, neighbour) in neighbours {
                connections.push(neighbour.id);
            }
            connections
        }).collect();

        connection_inds
    }

    pub fn print_info(&self) {
        println!("Network info:");
        println!("\tNeurons: {}", self.neurons.len());
    }
}
