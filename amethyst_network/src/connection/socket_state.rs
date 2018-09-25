use amethyst_core::specs::{Component, VecStorage};

use std::collections::{HashMap};
use std::collections::hash_map::Entry;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use packet::{Packet, CompletePacket};
use {NetIdentity, NetConnection};
use uuid::Uuid;
use itertools::Itertools;
use std::borrow::BorrowMut;

pub struct SocketState<E: 'static + Send + Sync> {
    connections: HashMap<SocketAddr, NetConnection<E>>
}

impl<E> SocketState<E>  where E: 'static + Send + Sync {
    pub fn new() -> SocketState<E> {
        SocketState { connections: HashMap::new() }
    }

    pub fn preprocess_packet(&mut self, p: Packet) -> (SocketAddr, Vec<u8>) {
        let connection = self.connections.entry(p.addr).or_insert(NetConnection::new(p.addr));
        connection.waiting_packets.enqueue(connection.seq_num, p.clone());
        let final_packet = ::utils::assemble_packet(connection.seq_num, p.clone(), connection);
        connection.seq_num = connection.seq_num.wrapping_add(1);
        (p.addr, final_packet.serialized())
    }

    pub fn dropped_packets(&mut self, addr: SocketAddr) -> Vec<Packet> {
        let connection = self.connections.entry(addr).or_insert(NetConnection::new(addr));
        connection.dropped_packets.drain(..).collect()
    }

    pub fn receive(&mut self, addr: SocketAddr, packet: CompletePacket) -> Packet {
        let connection = self.connections.entry(addr).or_insert(NetConnection::new(addr));
        connection.their_acks.ack(packet.seq);
        let dropped_packets = connection.waiting_packets.ack(packet.ack_seq, packet.ack_field);
        connection.dropped_packets = dropped_packets.into_iter().map(|(_, p)| p).collect();
        Packet { addr: addr, payload: packet.payload }
    }
}