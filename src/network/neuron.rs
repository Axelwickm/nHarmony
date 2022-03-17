pub struct Neuron {
    pub id: usize,
    pub weight: u8,

    pub coords: [f64; 3],
    pub connections : Vec<usize>,
}
