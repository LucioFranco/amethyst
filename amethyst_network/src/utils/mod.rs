mod addr;

use std::net::SocketAddr;

use net::NetConnection;
use packet::{
    CompletePacket,
    Packet
};

pub use self::addr::ToSingleSocketAddr;

pub fn assemble_packet<T: Send + Sync>( seq_num: u16, p: Packet, connection: &NetConnection<T>) -> CompletePacket {
    CompletePacket {
        seq: seq_num,
        ack_seq: connection.their_acks.last_seq,
        ack_field: connection.their_acks.field,
        payload: p.payload
    }
}