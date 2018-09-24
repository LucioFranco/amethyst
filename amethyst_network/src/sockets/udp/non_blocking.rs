use std::io;

use packet::{Packet, CompletePacket};
use mio::udp::UdpSocket as MioUdpSocket;
use connection::SocketState;
use net::NetConnection;
use ToSingleSocketAddr;
use super::GAFFER_MTU;

#[allow(dead_code)]
/// Socket that will not block the current thread when receiving or sending data.
pub struct UdpSocket<T: 'static + Send + Sync> {
    // the socket used for sending and receiving data.
    udp_socket: MioUdpSocket,
    // the state of the socket, this will store the acknowledgments and stuff.
    state: SocketState<T>,
    // temp receive buffer for storing data.
    recv_buffer: [u8; GAFFER_MTU]
}

impl<T: 'static + Send + Sync> UdpSocket<T> {
    pub fn bind<A: ToSingleSocketAddr>(addr: A) -> io::Result<Self> {
        let first_addr = addr.to_single_socket_addr().unwrap();
        MioUdpSocket::bind(&first_addr).map(|sock| {
            UdpSocket {
                udp_socket: sock,
                state: SocketState::new(),
                recv_buffer: [0; GAFFER_MTU]
            }
        })
    }

    /// Receive a normal message
    ///
    /// - Get next message
    /// - Add its sequence # to our memory
    /// - Identify dropped packets from message header
    /// - Forget own acked packets
    /// - Enqueue Sure-Dropped packets into resubmit-queue
    pub fn recv(&mut self) -> io::Result<Option<Packet>> {

        self.udp_socket.recv_from(&mut self.recv_buffer)
            .and_then(|opt| {
                match opt {
                    Some((len, addr)) => {
                        // TODO: Fix to_vec, it is suboptimal here
                        CompletePacket::deserialize(self.recv_buffer[..len].to_vec())
                            .map(|packet| Some(self.state.receive(addr, packet)))
                    },
                    None => Ok(None)
                }
            })
    }

    /// Send a normal message
    ///
    /// - Send dropped packets
    /// - Send packet
    pub fn send(&mut self, p: Packet) -> io::Result<Option<usize>> {
        let dropped_packets = self.state.dropped_packets(p.addr);
        for packet in dropped_packets.into_iter() {
            // TODO: if this fails, a bunch of packets are dropped
            try!(self.single_send(packet));
        }
        self.single_send(p)
    }

    /// - Get and increment sequence number
    /// - Remember packet
    /// - Add all headers
    ///   - Sequence #
    ///   - Current ack
    ///   - Ack bitfield
    /// - Send packet
    fn single_send(&mut self, p: Packet) -> io::Result<Option<usize>> {
        let (destination, payload) = self.state.preprocess_packet(p);

        self.udp_socket.send_to(payload.as_ref(), &destination)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use packet::Packet;

    #[test]
    fn recv_doesnt_block() {
        let mut sock = UdpSocket::<()>::bind("0.0.0.0:45213").unwrap();

        let payload = sock.recv();
        assert!(payload.is_ok());
        assert_eq!(payload.unwrap(), None);
    }

    #[test]
    fn recv_can_recv() {
        let mut send_sock = UdpSocket::<()>::bind("127.0.0.1:45214").unwrap();
        let mut recv_sock = UdpSocket::<()>::bind("127.0.0.1:45215").unwrap();
        let send_res = send_sock.send(Packet::new("127.0.0.1:45215", vec![1, 2, 3]));
        println!("result: {:?}", send_res);

        assert!(send_res.is_ok());
        assert!(send_res.unwrap().is_some());

        let packet = recv_sock.recv();
        assert!(packet.is_ok());
        let packet_payload = packet.unwrap();
        assert!(packet_payload.is_some());
        let unwrap_pkt = packet_payload.unwrap();
        assert_eq!(unwrap_pkt.payload, vec![1, 2, 3]);
        let addr = unwrap_pkt.addr;
        assert_eq!(addr.to_string(), "127.0.0.1:45214");
    }
}
