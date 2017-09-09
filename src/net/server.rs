use std::io;
use std::vec;
use std::sync::mpsc;

use futures::{Future, Stream, Sink};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::Core;

use net::Player;

/// The low-level networking server
///
/// # Examples
///
/// ```
///  world.add_resource(Server::listen())
/// ```
pub struct ServerManager {
    pub players: Vec<Player>,
    messages_in: mpsc::Receiver<u8>,
    messages_out: mpsc::Sender<u8>,
}

impl ServerManager {
    /// Create server
    fn listen() -> Self {
        let messages_in = mpsc::channel();
        let messages_out = mpsc::channel();

        ServerManager {
            players: Vec::new(),
            messages_in: messages_in.1,
            messages_out: messages_out.0,
        }


    }

    fn send_message() {
        unimplemented!()
    }

    fn recv_message() {
        unimplemented!()
    }
}

fn create_server() {

}
