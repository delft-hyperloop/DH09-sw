use embassy_stm32::can::frame::FdFrame;
use embassy_time::Instant;
use embedded_can::Id;

/// CanEvnelope used for making CAN messages
#[derive(Debug, Clone)]
pub struct CanEnvelope {
    pub envelope: embassy_stm32::can::frame::FdEnvelope,
}

impl CanEnvelope {
    /// Makes a new `CanEnvelope` object from an `FdFrame`
    pub fn new_from_frame(frame: FdFrame) -> Self {
        Self {
            envelope: embassy_stm32::can::frame::FdEnvelope {
                frame,
                ts: Instant::now(),
            },
        }
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

impl core::cmp::PartialOrd for CanEnvelope {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.envelope
            .frame
            .id()
            .partial_cmp(&other.envelope.frame.id())
    }
}

impl core::cmp::Eq for CanEnvelope {}

impl core::cmp::Ord for CanEnvelope {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.envelope.frame.id().cmp(&other.envelope.frame.id())
    }
}
