use core::cmp::Ordering;

use embassy_stm32::can::Frame;
use embassy_time::Instant;
use embedded_can::Id;

/// Envelope for CAN messages
#[derive(Debug, Clone)]
pub struct CanEnvelope {
    pub envelope: embassy_stm32::can::frame::Envelope,
}

impl defmt::Format for CanEnvelope {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{:?}", &self.envelope.frame);
    }
}

impl CanEnvelope {
    /// Makes a `CanEnvelope` from a `Frame`
    pub fn new_from_frame(frame: Frame) -> Self {
        Self {
            envelope: embassy_stm32::can::frame::Envelope {
                frame,
                ts: Instant::now(),
            },
        }
    }

    /// Makes a `CanEnvelope` from an ID with the provided payload
    pub fn new_with_id(id: u16, payload: &[u8]) -> Self {
        Self::new_from_frame(Frame::new_standard(id, payload).unwrap())
    }

    /// Returns the ID of the envelope
    pub fn id(&self) -> &Id {
        self.envelope.frame.id()
    }

    /// Returns the payload of the envelope
    pub fn payload(&self) -> &[u8] {
        self.envelope.frame.data()
    }

    /// Returns the timestamp of the envelope
    pub fn timestamp(&self) -> Instant {
        self.envelope.ts
    }
}

impl core::cmp::PartialEq for CanEnvelope {
    fn eq(&self, other: &Self) -> bool {
        self.envelope.frame.id() == other.envelope.frame.id()
    }
}

impl Ord for CanEnvelope {
    fn cmp(&self, other: &Self) -> Ordering {
        self.envelope
            .frame
            .id()
            .cmp(other.envelope.frame.id())
    }
}

impl core::cmp::Eq for CanEnvelope {}

impl PartialOrd for CanEnvelope {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.envelope.frame.id().cmp(other.envelope.frame.id()))
    }
}
