use std::collections::HashMap;
use packet::Packet;
use itertools::Itertools;

/// Packets waiting for an ack
///
/// Holds up to 32 packets waiting for ack
///
/// Additionally, holds packets "forward" of the current ack packet
#[derive(Debug)]
pub struct AckRecord {
    packets: HashMap<u16, Packet>
}

impl AckRecord {
    pub fn new() -> AckRecord {
        AckRecord { packets: HashMap::new() }
    }

    pub fn is_empty(&mut self) -> bool {
        self.packets.is_empty()
    }

    pub fn len(&mut self) -> usize {
        self.packets.len()
    }

    /// Adds a packet to the waiting packets
    pub fn enqueue(&mut self, seq: u16, packet: Packet) {
        // TODO: Handle overwriting other packet?
        //   That really shouldn't happen, but it should be encoded here
        self.packets.insert(seq, packet);
    }

    /// Finds and removes acked packets, returning dropped packets
    #[allow(unused_parens)]
    pub fn ack(&mut self, seq: u16, seq_field: u32) -> Vec<(u16, Packet)> {
        let mut dropped_packets = Vec::new();
        let mut acked_packets = Vec::new();
        self.packets.keys().foreach(|k| {
            let diff = seq.wrapping_sub(*k);
            if diff == 0 {
                acked_packets.push(*k);
            } else if diff <= 32 {
                let field_acked = (seq_field & (1 << diff - 1) != 0);
                if field_acked {
                    acked_packets.push(*k);
                }
            } else if diff < 32000 {
                dropped_packets.push(*k);
            }
        });
        acked_packets.into_iter().foreach(|seq| { self.packets.remove(&seq); });
        dropped_packets.into_iter().map(|seq| (seq, self.packets.remove(&seq).unwrap())).collect()
    }
}
