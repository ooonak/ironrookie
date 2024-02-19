mod signing;

extern crate serde;
extern crate rmp_serde as rmps;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Message {
    msg_id: u64,
    beacon_id: u64,
    nonce: u32,
    cmd: String
}
