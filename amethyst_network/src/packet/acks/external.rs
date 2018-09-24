/// Third party's ack information
///
/// Holds the latest seq_num we've seen from them and the 32 bit bitfield
/// for extra redundancy
#[derive(Debug)]
pub struct ExternalAcks {
    pub last_seq: u16,
    pub field: u32,
    initialized: bool
}

impl ExternalAcks {
    pub fn new() -> ExternalAcks {
        ExternalAcks { last_seq: 0, field: 0, initialized: false }
    }

    pub fn ack(&mut self, seq_num: u16) {
        if !self.initialized {
            self.last_seq = seq_num;
            self.initialized = true;
            return;
        }

        let pos_diff = seq_num.wrapping_sub(self.last_seq);
        let neg_diff = self.last_seq.wrapping_sub(seq_num);

        if pos_diff == 0 {
            return;
        }

        if pos_diff < 32000 {
            if pos_diff <= 32 {
                self.field = ((self.field << 1 ) | 1) << (pos_diff - 1);
            } else {
                self.field = 0;
            }
            self.last_seq = seq_num;
        } else if neg_diff <= 32 {
            self.field = self.field | (1 << neg_diff - 1);
        }
    }
}
