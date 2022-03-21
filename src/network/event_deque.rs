use std::collections::VecDeque;

pub enum ActionPotential {
    voltage: u8,
}

struct EventDeque {
    events: VecDeque<u64, ActionPotential> // time, event
}
