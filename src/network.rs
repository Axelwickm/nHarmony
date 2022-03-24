extern crate rayon;
use rayon::prelude::*;

extern crate kdtree;
use kdtree::KdTree;
use kdtree::distance::squared_euclidean;

use crate::neuron;
use crate::synapse;
use crate::event_deque;

pub struct Network {
    pub neurons: Vec<neuron::Neuron>,
    event_deques: event_deque::EventDeque,
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
            neurons[i].connections = connections.iter().map(|(id, dist)|
                                                            synapse::Synapse::new(*id, dist) ).collect();
        }

        Network { neurons, event_deques: event_deque::EventDeque::new()}
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
        let connection_inds = connection_refs.into_iter().map(|(neuron, neighbours)| {
            let mut connections = Vec::new();
            for (dist, neighbour) in neighbours {
                if neighbour.id != neuron.id {
                    connections.push((neighbour.id, dist));
                }
            }
            connections
        }).collect();

        connection_inds
    }

    pub fn random_activations(&mut self) {
        // Go through every neuron and set a random u8 activation value
        for neuron in &mut self.neurons {
            let spiked = neuron.random_activation();
            if spiked {
                neuron.schedule_post_synpatic_action_potentials(
                    &mut self.event_deques,
                    0
                );
            }
        }

        // Print how many scheduled spikes
        println!("Initially scheduled {} spikes",
                 self.event_deques.len());

    }

    pub fn simulate_next_event(&mut self) {
        let first = self.event_deques.pop();
        if first.is_none() {
            return;
        }

        let first = first.unwrap();
        let spiked = self.neurons[first.to].simulate_voltage(first.voltage, first.time);
        if spiked {
            self.neurons[first.to].schedule_post_synpatic_action_potentials(
                &mut self.event_deques,
                first.time,
            );
        }

        println!("Time: {}, Scheduled: {}", first.time, self.event_deques.len());

    }


    pub fn print_info(&self) {
        println!("Network info:");
        println!("\tNeurons: {}", self.neurons.len());
        println!("\tEvent deques: {}", self.event_deques.len());
    }
}
