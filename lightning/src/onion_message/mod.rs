// This file is Copyright its original authors, visible in version control
// history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

//! Onion Messages: sending, receiving, forwarding, and ancillary utilities live here
//!
//! Onion messages are multi-purpose messages sent between peers over the lightning network. In the
//! near future, they will be used to communicate invoices for [offers], unlocking use cases such as
//! static invoices, refunds and proof of payer. Further, you will be able to accept payments
//! without revealing your node id through the use of [blinded paths].
//!
//! LDK sends and receives onion messages via the [`OnionMessenger`]. See its documentation for more
//! information on its usage.
//!
//! [offers]: <https://github.com/lightning/bolts/pull/798>
//! [blinded paths]: crate::blinded_path::BlindedPath

mod messenger;
mod offers;
mod packet;
#[cfg(test)]
mod functional_tests;

// Re-export structs so they can be imported with just the `onion_message::` module prefix.
pub use self::messenger::{CustomOnionMessageHandler, DefaultMessageRouter, Destination, MessageRouter, OnionMessageContents, OnionMessagePath, OnionMessenger, PeeledOnion, PendingOnionMessage, SendError};
pub use self::messenger::{create_onion_message, peel_onion_message};
#[cfg(not(c_bindings))]
pub use self::messenger::{SimpleArcOnionMessenger, SimpleRefOnionMessenger};
pub use self::offers::{OffersMessage, OffersMessageHandler};
pub use self::packet::{Packet, ParsedOnionMessageContents};
pub(crate) use self::packet::ControlTlvs;
pub(crate) use self::messenger::new_pending_onion_message;
