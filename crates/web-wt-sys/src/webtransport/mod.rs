//! Bindings for the [`WebTransport`] class and related items.
//!
//! <https://www.w3.org/TR/webtransport/>
//!
//! <https://w3c.github.io/webtransport/#web-transport>
//!
//! Hand-crafted based on the [WebIDL][1] with
//! `https://github.com/w3c/webtransport` @
//! `61b5cdae67c7442911cdb29c65bbff61301f46ca`.
//!
//! [1]: https://w3c.github.io/webtransport/#idl-index

mod web_transport;
mod web_transport_bidirectional_stream;
mod web_transport_close_info;
mod web_transport_congestion_control;
mod web_transport_connection_stats;
mod web_transport_datagram_duplex_stream;
mod web_transport_datagram_stats;
mod web_transport_datagrams_writable;
mod web_transport_error;
mod web_transport_error_options;
mod web_transport_error_source;
mod web_transport_hash;
mod web_transport_options;
mod web_transport_receive_stream;
mod web_transport_receive_stream_stats;
mod web_transport_reliability_mode;
mod web_transport_send_group;
mod web_transport_send_options;
mod web_transport_send_stream;
mod web_transport_send_stream_options;
mod web_transport_send_stream_stats;
mod web_transport_writer;

pub use web_transport::*;
pub use web_transport_bidirectional_stream::*;
pub use web_transport_close_info::*;
pub use web_transport_congestion_control::*;
pub use web_transport_connection_stats::*;
pub use web_transport_datagram_duplex_stream::*;
pub use web_transport_datagram_stats::*;
pub use web_transport_datagrams_writable::*;
pub use web_transport_error::*;
pub use web_transport_error_options::*;
pub use web_transport_error_source::*;
pub use web_transport_hash::*;
pub use web_transport_options::*;
pub use web_transport_receive_stream::*;
pub use web_transport_receive_stream_stats::*;
pub use web_transport_reliability_mode::*;
pub use web_transport_send_group::*;
pub use web_transport_send_options::*;
pub use web_transport_send_stream::*;
pub use web_transport_send_stream_options::*;
pub use web_transport_send_stream_stats::*;
pub use web_transport_writer::*;
