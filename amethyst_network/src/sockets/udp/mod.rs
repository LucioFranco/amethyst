pub mod blocking;
pub mod non_blocking;

/**
 * Maximum transmission unit of a gaffer payload
 *
 * Derived from ethernet_mtu - ipv6_header_size - udp_header_size - gaffer_header_size
 *       1452 = 1500         - 40               - 8               - 8
 *
 * This is not strictly guaranteed -- there may be less room in an ethernet frame than this due to
 * variability in ipv6 header size.
 */
pub const GAFFER_MTU: usize = 1452; /* bytes */