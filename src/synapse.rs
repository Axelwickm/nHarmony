
pub struct Synapse {
    pub to: usize,
    pub weight: u8,
    pub delay: u8
}

impl Synapse {
    pub fn new(to: usize, dist: &f64) -> Synapse {
        let weight = rand::random::<u8>()/2 + 128;
        let delay = (dist / 5.0 * 25.0).round() as u8;
        Synapse {
            to, weight, delay
        }
    }
}
