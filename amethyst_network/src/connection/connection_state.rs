///The state of the connection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    /// The connection is established.
    Connected,
    /// The connection is being established.
    Connecting,
    /// The connection has been dropped.
    Disconnecting,
    /// The connection has been dropped.
    Disconnected,
}