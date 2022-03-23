use std::fmt;

pub struct ActionPotential {
    pub time: u64,
    pub to: usize,
    pub from: usize,
    pub voltage: u8,
}

impl fmt::Debug for ActionPotential {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActionPotential {{ time: {}, to: {}, from: {}, voltage: {} }}", self.time, self.to, self.from, self.voltage)
    }
}

impl PartialOrd for ActionPotential {
    fn partial_cmp(&self, other: &ActionPotential) -> Option<std::cmp::Ordering> {
        other.time.partial_cmp(&self.time)
    }
}

impl PartialEq for ActionPotential {
    fn eq(&self, other: &ActionPotential) -> bool {
        self.time == other.time
    }
}

impl Ord for ActionPotential {
    fn cmp(&self, other: &ActionPotential) -> std::cmp::Ordering {
        other.time.cmp(&self.time)
    }
}

impl Eq for ActionPotential {}
