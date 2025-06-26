use std::fmt;

use crate::wakan::{Radio, ScheduledTransmitionTime};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Transmission<P> {
    pub when: ScheduledTransmitionTime,
    pub packet: P,
    pub radio: Radio,
}

impl<P> Transmission<P> {
    pub fn new(when: ScheduledTransmitionTime, packet: P, radio: Radio) -> Self {
        Self {
            when,
            packet,
            radio,
        }
    }
}

impl<P: std::fmt::Display> fmt::Display for Transmission<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transmission of {} at {} on {}",
            self.packet, self.when, self.radio
        )
    }
}

impl<P> From<(ScheduledTransmitionTime, P, Radio)> for Transmission<P> {
    fn from((when, packet, radio): (ScheduledTransmitionTime, P, Radio)) -> Self {
        Self {
            when,
            packet,
            radio,
        }
    }
}
