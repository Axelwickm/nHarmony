use std::collections::BinaryHeap;

use crate::action_potential::ActionPotential;

pub struct EventDeque {
    events: BinaryHeap<ActionPotential>, // time, event
}

impl EventDeque {
    pub fn new() -> EventDeque {
        EventDeque {
            events: BinaryHeap::new(),
        }
    }

    pub fn pop(&mut self) -> Option<ActionPotential> {
        let first = self.events.pop();
        first
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn add_action_potential(&mut self, time: u64, from: usize, to: usize, voltage: u8) {
        self.events.push(ActionPotential {
            time,
            to,
            from,
            voltage,
        });
    }
}

