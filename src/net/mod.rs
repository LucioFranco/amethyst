use std::vec;
use std::net::ToSocketAddrs;
use amethyst::ecs::{Component, VecStorage};
use amethyst::ecs::{Entities, Entity, Join, ReadStorage, System, WriteStorage};
use amethyst::ecs::transform::{LocalTransform, Transform};

struct LocalSync(u64);

impl Component for LocalSync {
    type Storage = VecStorage<Self>;
}

struct RemoteSync(u64);

impl Component for RemoteSync {
    type Storage = VecStorage<Self>;
}

struct Connection;

struct Server {
    pub players: Vec<(String, Connection)>,
}

impl Server {
    fn new() -> Server {
        Server {
            players: Vec::new(),
            // TOOD: add UDPSocket
        }
    }

    fn listen<A: ToSocketAddr>(addr: A) -> Self {
        unimplemented!()
    }
}
