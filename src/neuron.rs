use crate::event_deque;
use crate::synapse;

pub struct Neuron {
    pub id: usize,
    pub coords: [f64; 3],

    pub threshold: u8, // Serves as bias
    pub half_life: f32, // ms
    pub refractory_period: u64, // ms
    pub connections : Vec<synapse::Synapse>, 

    pub last_activation: u8,
    pub last_activation_time: u64,
    pub last_action_potential: u64,
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
            threshold: rand::random::<u8>()/2+128,
            half_life: 10.0,
            refractory_period: rand::random::<u64>() % 20 + 5,
            connections: Vec::new(),
            last_activation: 0,
            last_activation_time: 0,
            last_action_potential: 0, // This is ignored by random_activation
        };
        neuron
    }

    pub fn random_activation(&mut self) -> bool {
        self.last_activation = rand::random::<u8>();
        let above_threshold = self.last_activation > self.threshold;
        above_threshold
    }
    
    // Return true if above threshold
    pub fn simulate_voltage(&mut self, input_voltage: u8, time: u64) -> bool { 
        let delta_time = time - self.last_activation_time;
        let decay_factor = 0.5_f32.powf(delta_time as f32 / self.half_life);
        self.last_activation = (self.last_activation as f32 * decay_factor) as u8;
        self.last_activation += input_voltage;
        self.last_activation_time = time;

        let above_threshold = self.last_activation >= self.threshold;
        let recharging = self.last_activation_time - self.last_action_potential < self.refractory_period;

        above_threshold && !recharging
    }

    pub fn schedule_post_synpatic_action_potentials(&mut self,
                                                    events: &mut event_deque::EventDeque,
                                                    time: u64) {
        self.last_action_potential = time;
        for synapse in &self.connections {
            let voltage = synapse.weight + self.last_activation as u8;
            events.add_action_potential(time + synapse.delay as u64, 
                                          self.id, synapse.to,
                                          voltage);
        }

    }
}
