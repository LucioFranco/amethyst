//! Network Connection and states.

use amethyst_core::shrev::{EventChannel, EventIterator, ReaderId};
use amethyst_core::specs::{Component, VecStorage};

use packet::{ExternalAcks, AckRecord};
use connection::ConnectionState;
use std::net::SocketAddr;
use packet::{Packet};
use uuid::Uuid;
use NetEvent;

// TODO: Think about relationship between NetConnection and NetIdentity.

/// A network connection target data.
pub struct NetConnection<E: 'static + Send + Sync> {
    pub seq_num: u16,
    // dropped packages of this connection
    pub dropped_packets: Vec<Packet>,
    // packages that could be send
    pub waiting_packets: AckRecord,
    // acknowledges from the other side.
    pub their_acks: ExternalAcks,
    /// The remote socket address of this connection.
    pub target: SocketAddr,

    pub send_buffer: EventChannel<NetEvent<E>>,
    pub receive_buffer: EventChannel<NetEvent<E>>,
    /// Private. Used by `NetSocketSystem` to be able to immediately send events upon receiving a new NetConnection.
    send_reader: ReaderId<NetEvent<E>>,

    /// The state of the connection.
    pub state: ConnectionState,
}

impl<E: 'static + Send + Sync> NetConnection<E> {
    pub fn new(target: SocketAddr) -> Self {

        let mut send_buffer = EventChannel::new();
        let send_reader = send_buffer.register_reader();

        NetConnection {
            seq_num: 0,
            dropped_packets: Vec::new(),
            waiting_packets: AckRecord::new(),
            their_acks: ExternalAcks::new(),
            target,
            state: ConnectionState::Connecting,
            send_buffer: send_buffer,
            send_reader: send_reader,
            receive_buffer: EventChannel::<NetEvent<E>>::new(),
        }
    }

//    /// Function used ONLY by NetSocketSystem.
//    /// Since most users will want to both create the connection and send messages on the same frame,
//    /// we need a way to read those. Since the NetSocketSystem runs after the creation of the NetConnection,
//    /// it cannot possibly have registered his reader early enough to catch the initial messages that the user wants to send.
//    ///
//    /// The downside of this is that you are forced to take NetConnection mutably inside of NetSocketSystem.
//    /// If someone finds a better solution, please open a PR.
    pub fn send_buffer_early_read(&mut self) -> EventIterator<NetEvent<E>> {
        self.send_buffer.read(&mut self.send_reader)
    }
}

impl<E: 'static + Send + Sync>  PartialEq for NetConnection<E> {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target && self.state == other.state
    }
}

impl<E: 'static + Send + Sync>  Eq for NetConnection<E>  {}

impl<E: 'static + Send + Sync> Component for NetConnection<E>  {
    type Storage = VecStorage<Self>;
}