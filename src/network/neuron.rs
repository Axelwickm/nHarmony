pub struct Neuron {
    pub id: usize,
    pub coords: [f64; 3],

    pub threshold: u8, // Serves as bias
    pub connections : Vec<(usize, u8, u8)>, 

    pub activation: u8,
}

impl Neuron {
    pub fn new(id: usize, coord_limit: f64) -> Neuron {
        let neuron = Neuron {
            id,
            coords: [
                // From -coord_limit to coord_limit
                rand::random::<f64>() * 2.0 * coord_limit - coord_limit,
                rand::random::<f64>() * 2.0 * coord_limit - coord_limit,
                rand::random::<f64>() * 2.0 * coord_limit - coord_limit,
            ],
            threshold: rand::random::<u8>(),
            connections: Vec::new(),
            activation: 0,
        };
        neuron
    }

    pub fn random_activation(&mut self) {
        self.activation = rand::random::<u8>();
    }
}
