//! Networking library for `amethyst`

use std::vec;
use std::net::ToSocketAddrs;
use ecs::{Component, VecStorage};
use ecs::{Entities, Entity, Join, ReadStorage, System, WriteStorage};
use ecs::transform::{LocalTransform, Transform};

mod server;
mod message;

/// Represents syncing from local
struct LocalSync(u64);

impl Component for LocalSync {
    type Storage = VecStorage<Self>;
}

/// Represents syncing remotly
struct RemoteSync(u64);

impl Component for RemoteSync {
    type Storage = VecStorage<Self>;
}

/// Represents the player
pub struct Player {
    id: u8,
}
