//! The network send and receive System
use std::clone::Clone;
use std::io::{Error, ErrorKind};
use std::net::{IpAddr, SocketAddr};
use std::str::{self, FromStr};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use amethyst_core::specs::{
    Entities, Entity, Join, Read, ReadStorage, Resources, System, SystemData, Write, WriteStorage,
};
use mio::Token;
use serde::de::DeserializeOwned;
use serde::Serialize;
use shrev::*;
use bincode::deserialize;

use sockets::udp::blocking::UdpSocket;
use packet::Packet;
use connection::ConnectionState;
use *;

enum InternalSocketEvent<E> {
    SendEvents {
        target: SocketAddr,
        events: Vec<NetEvent<E>>,
    },
    Stop,
}

// If a client sends both a connect event and other events,
// only the connect event will be considered valid and all others will be lost.

/// The System managing the network state and connections.
/// The T generic parameter corresponds to the network event type.
/// Receives events and filters them.
/// Received events will be inserted into the NetReceiveBuffer resource.
/// To send an event, add it to the NetSendBuffer resource.
///
/// If both a connection (Connect or Connected) event is received at the same time as another event from the same connection,
/// only the connection event will be considered and rest will be filtered out.
// TODO: add Unchecked Event type list. Those events will be let pass the client connected filter (Example: NetEvent::Connect).
// Current behaviour: hardcoded passthrough of Connect and Connected events.
pub struct NetSocketSystem<E: 'static + Send + Sync + PartialEq>
{
    /// The list of filters applied on the events received.
    pub filters: Vec<Box<NetFilter<E>>>,

    tx: Sender<InternalSocketEvent<E>>,
    rx: Receiver<Packet>,
}

impl<E> NetSocketSystem<E> where E: Serialize + PartialEq + Send + Sync + 'static,
{
    /// Creates a `NetSocketSystem` and binds the Socket on the ip and port added in parameters.
    pub fn new(ip: &str,  udp_port: u16, filters: Vec<Box<NetFilter<E>>>) -> Result<NetSocketSystem<E>, Error>
    {
        let mut socket: UdpSocket<E> = UdpSocket::bind(&SocketAddr::new(
            IpAddr::from_str(ip).expect("Unreadable input IP."),
            udp_port,
        ))?;

        socket.udp_socket.set_nonblocking(true).unwrap();

        // this -> thread
        let (tx1, rx1) = channel();
        // thread -> this
        let (tx2, rx2) = channel();

        thread::spawn(move || {
            //rx1,tx2
            let send_queue = rx1;
            let receive_queue = tx2;
            let mut socket = socket;

            'outer: loop {
                // send
                for control_event in send_queue.try_iter() {
                    match control_event {
                        InternalSocketEvent::SendEvents { target, events } => {
                            for ev in events {
                                let data = NetEntity::serialize(&ev);

                                match socket.send(Packet::new(target, data)) {
                                    Ok(qty) => {},
                                    Err(e) => { error!("Failed to send data to network socket: {}", e) }
                                }
                            }
                        }
                        InternalSocketEvent::Stop => break 'outer,
                    }
                }

                loop {
                    // receive
                    let received_data = socket.recv();

                    // try receive something
                    match received_data {
                        Ok(received_packet) =>
                            {
                                receive_queue.send(received_packet).unwrap();
                            },
                        Err(e) => {

                            if e.kind() == ErrorKind::WouldBlock {
                                break;
                            } else {
                                 error!("Could not receive datagram: {}", e);
                            }
                        }
                    }
                }
            }
        });

        Ok(NetSocketSystem {
            filters,
            tx: tx1,
            rx: rx2,
        })
    }
}

impl<'a, E> System<'a> for NetSocketSystem<E> where E: Send + Sync + Serialize + Clone + DeserializeOwned + PartialEq + 'static,
{
    type SystemData = (Entities<'a>, WriteStorage<'a, NetConnection<E>>);

    fn run(&mut self, (entities, mut net_connections): (Entities<'a>,  WriteStorage<'a, NetConnection<E>>)) {
        // handle messages to send
        for (entity, mut net_connection) in (&*entities, &mut net_connections).join() {
            // TODO: find out why the read needs this

            let target = net_connection.target.clone();

            if net_connection.state == ConnectionState::Connected || net_connection.state == ConnectionState::Connecting {
                self.tx.send(InternalSocketEvent::SendEvents {
                    target,
                    events: net_connection.send_buffer_early_read().cloned().collect(),
                }).unwrap();
            }else if net_connection.state == ConnectionState::Disconnected {
                self.tx.send(InternalSocketEvent::Stop).unwrap();
            }
        }

        // handle all received packet
        for packet in self.rx.try_iter() {
            let mut matched = false;

            for mut net_connection in (&mut net_connections).join() {

                // We found the origin
                if net_connection.target == packet.addr {
                    matched = true;

                    let result = deserialize::<NetEvent<E>>(packet.payload.as_slice());

                    match result {
                        Ok(net_event) => {
                            // Filter events
                            let mut filtered = false;

                            if !filtered {
                                net_connection.receive_buffer.single_write(net_event);
                            } else {
                                info!(
                                    "Filtered an incoming network packet from source {:?}",
                                    packet.payload
                                );
                            }
                        },
                        Err(e) => {
                            error!("Failed to deserialize an incoming network event: {} From source: {:?}", e, packet.payload);
                        }
                    }
                }

                if !matched {
                    info!("Received packet from unknown source");
                }
            }
        }
    }
}


