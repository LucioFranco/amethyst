use std::io;
use std::net::UdpSocket as NetSocket;

use packet::{Packet, CompletePacket};
use net::NetConnection;
use connection::SocketState;
use super::GAFFER_MTU;
use ToSingleSocketAddr;

/// Socket that will block the current thread when receiving or sending data.
pub struct UdpSocket<T: 'static + Send + Sync> {
    // the socket used for sending and receiving data.
    pub udp_socket: NetSocket,
    // the state of the socket, this will store the acknowledgments and stuff.
    state: SocketState<T>,
    // temp receive buffer for storing data.
    recv_buffer: [u8; GAFFER_MTU],
}

impl<T: 'static + Send + Sync> UdpSocket<T> {
    pub fn bind<A: ToSingleSocketAddr>(addr: A) -> io::Result<Self> {
        let first_addr = addr.to_single_socket_addr().unwrap();
        NetSocket::bind(&first_addr).map(|sock| {
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
    pub fn recv(&mut self) -> io::Result<Packet> {
        self.udp_socket.recv_from(&mut self.recv_buffer)
            // TODO: Fix to_vec, it is suboptimal here
            .and_then(|(len, addr)| CompletePacket::deserialize(self.recv_buffer[..len].to_vec()).map(|res| (addr, res)) )
            .map(|(addr, packet)|
                {
                    self.state.receive(addr, packet)
                })
    }

    /// Send a normal message
    ///
    /// - Send dropped packets
    /// - Send packet
    pub fn send(&mut self, p: Packet) -> io::Result<usize> {
        let dropped_packets = self.state.dropped_packets(p.addr);
        for packet in dropped_packets.into_iter() {
            // TODO: if this fails, a bunch of packets are dropped
            panic!("dropped");
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
    fn single_send(&mut self, p: Packet) -> io::Result<usize> {
        let (destination, payload) = self.state.preprocess_packet(p);
        self.udp_socket.send_to(payload.as_ref(), &destination)
    }
}

