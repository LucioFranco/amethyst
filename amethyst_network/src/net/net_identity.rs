use uuid::Uuid;
use amethyst_core::specs::{Component, VecStorage};

/// A network identity. It can represent either a client or a server.
/// It represents anything that can own an entity or a component.
/// Think of it as an identity card.
/// When used as a resource, it designates the local network uuid.
#[derive(Eq, PartialEq, Hash)]
pub struct NetIdentity {
    /// The uuid identifying this NetIdentity.
    pub uuid: Uuid,
}

impl Default for NetIdentity {
    fn default() -> Self {
        NetIdentity {
            uuid: Uuid::new_v4(),
        }
    }
}

impl Component for NetIdentity {
    type Storage = VecStorage<NetIdentity>;
}