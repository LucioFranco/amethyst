use std::io;

use bincode::{deserialize, serialize, Infinite};
use bincode::internal::ErrorKind;
use serde::de::DeserializeOwned;
use serde::Serialize;

use sockets::udp::non_blocking::UdpSocket;
use NetEvent;

pub struct NetEntity;

impl NetEntity
{
    pub fn new() -> Self
    {
        NetEntity {}
    }

    pub fn serialize<T>(event: &NetEvent<T>) -> Vec<u8>
        where
            T: Serialize,
    {
        let ser = serialize(event, Infinite);

        match ser {
            Ok(s) => {
                return s.as_slice().to_vec();
            }
            Err(e) => { return Vec::new(); /*error!("Failed to serialize the event: {}", e)*/ },
        }
    }

    pub fn deserialize<T>(data: &[u8]) -> Result<NetEvent<T>, Box<ErrorKind>> where
        T: DeserializeOwned,
    {
        deserialize::<NetEvent<T>>(data)
    }
}