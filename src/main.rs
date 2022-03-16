mod neuron;

fn main() {
    println!("Hello, world!");
    let neuron = neuron::Neuron {weight: 255};
    println!("{}", neuron.weight);
}
