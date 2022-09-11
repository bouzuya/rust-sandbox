use event_store_core::event_stream_seq::EventStreamSeq;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Version(EventStreamSeq);

impl From<EventStreamSeq> for Version {
    fn from(event_stream_seq: EventStreamSeq) -> Self {
        Self(event_stream_seq)
    }
}

impl From<Version> for EventStreamSeq {
    fn from(version: Version) -> Self {
        version.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_stream_seq_conversion_test() {
        let seq = EventStreamSeq::from(1_u32);
        assert_eq!(EventStreamSeq::from(Version::from(seq)), seq);
    }
}
