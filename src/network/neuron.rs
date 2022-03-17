pub struct Neuron<'a> {
    pub weight: u8,

    pub x: i64,
    pub y: i64,
    pub z: i64,

    pub connected_neurons: Vec<&'a Neuron<'a>>,
}
