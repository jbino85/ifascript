use std::time::Duration;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EboTrigger {
    StackUnderflow,
    DivisionByZero,
    InvalidCast,
    HeapOverflow,
    ForbiddenBranch,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ebo {
    TimeDelay(Duration),
    ProofOfWork(u32),
    TokenBurn(String),
    IntentionString(String),
}

pub struct EboHistory {
    counts: HashMap<EboTrigger, u32>,
}

impl EboHistory {
    pub fn new() -> Self {
        Self { counts: HashMap::new() }
    }

    pub fn record(&mut self, trigger: EboTrigger) {
        *self.counts.entry(trigger).or_insert(0) += 1;
    }

    pub fn required_ebo(&self, trigger: &EboTrigger) -> Ebo {
        let count = self.counts.get(trigger).copied().unwrap_or(0);
        match (trigger, count) {
            (EboTrigger::StackUnderflow, 0..=2) => Ebo::TimeDelay(Duration::from_secs(1)),
            (EboTrigger::StackUnderflow, _) => Ebo::ProofOfWork(20),
            (EboTrigger::DivisionByZero, _) => Ebo::ProofOfWork(20),
            (EboTrigger::ForbiddenBranch, _) => Ebo::IntentionString("I vow clarity and no harm".to_string()),
            _ => Ebo::TimeDelay(Duration::from_secs(5)),
        }
    }

    pub fn has_trigger(&self, trigger: &EboTrigger) -> bool {
        // Public accessor for testing
        self.counts.contains_key(trigger)
    }
}

impl EboTrigger {
    pub fn accepts(&self, ebo: &Ebo) -> bool {
        match (self, ebo) {
            (EboTrigger::StackUnderflow, Ebo::TimeDelay(d)) => d.as_secs() >= 1,
            (EboTrigger::DivisionByZero, Ebo::ProofOfWork(diff)) => *diff >= 20,
            (EboTrigger::ForbiddenBranch, Ebo::IntentionString(s)) => {
                let lower = s.to_lowercase();
                lower.contains("clarity") || lower.contains("no harm")
            }
            _ => false,
        }
    }
}

impl Default for EboHistory {
    fn default() -> Self {
        Self::new()
    }
}
