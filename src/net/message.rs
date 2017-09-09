use std::net::SocketAddr;
use std::io;
use tokio_core::net::UdpCodec;

/// Message codec that uses bincode to serialize messages
pub struct MessageCodec;

impl UdpCodec for MessageCodec {
    type In = (SocketAddr, Vec<u8>);
    type Out = (SocketAddr, Vec<u8>);

    fn decode(&mut self, addr: &SocketAddr, buf: &[u8]) -> io::Result<Self::In> {
        Ok((*addr, buf.to_vec()))
    }

    fn encode(&mut self, (addr, buf): Self::Out, into: &mut Vec<u8>) -> SocketAddr {
        into.extend(buf);
        addr
    }
}
