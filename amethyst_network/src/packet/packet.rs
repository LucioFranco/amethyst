use std::net::SocketAddr;
use std::io::{self, Cursor};

use ToSingleSocketAddr;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use bincode::{serialize, Infinite};

use NetEvent;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Packet {
    pub addr: SocketAddr,
    pub payload: Vec<u8>
}

impl Packet {
    pub fn dummy_packet() -> Packet {
        Packet::new("0.0.0.0:7878", Vec::new())
    }

    /// Create new packet by passing the receiving endpoint and the actual data.
    pub fn new<A: ToSingleSocketAddr>(addr: A, payload: Vec<u8>) -> Packet {
        let first_addr = addr.to_single_socket_addr().unwrap();
        Packet { addr: first_addr, payload: payload }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
/// packet that will be send over the network witch contains:
/// 1. the sequence number
/// 2. the last acknowledged sequence number
/// 3. last 32 acknowledged packages.
pub struct CompletePacket {
    // this is the sequence number so that we can know where in the sequence of packages this packet belongs.
    pub seq: u16,
    // this is the last acknowledged sequence number.
    pub ack_seq: u16,
    // this is an bitfield of all last 32 acknowledged packages
    pub ack_field: u32,
    // this is the payload in witch the packet data is stored.
    pub payload: Vec<u8>
}

impl CompletePacket {
    /// serialize package to raw data.
    pub fn serialized(&self) -> Vec<u8> {
        let mut wtr = Vec::new();
        wtr.write_u16::<BigEndian>(self.seq).unwrap();
        wtr.write_u16::<BigEndian>(self.ack_seq).unwrap();
        wtr.write_u32::<BigEndian>(self.ack_field).unwrap();
        wtr.append(&mut self.payload.clone());
        wtr
    }

    /// deserialize raw data to an instance of CompletePacket
    pub fn deserialize(mut bytes: Vec<u8>) -> io::Result<CompletePacket> {
        let payload = bytes.split_off(8);
        let mut rdr = Cursor::new(bytes);

        let seq = rdr.read_u16::<BigEndian>()?;
        let ack_seq = rdr.read_u16::<BigEndian>()?;
        let ack_field = rdr.read_u32::<BigEndian>()?;

        Ok(CompletePacket {
            seq,
            ack_seq,
            ack_field,
            payload
        })
    }
}