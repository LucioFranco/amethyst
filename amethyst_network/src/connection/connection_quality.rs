// This defines whether the connection is good or bad.
///
// TODO: When network conditions are `Good` we send 30 packets per-second, and when network conditions are `Bad` we drop to 10 packets per-second.
pub enum ConnectionQuality
{
    Good,
    Bad
}