mod signing;

extern crate rmp_serde as rmps;
extern crate serde;

use rand::Rng;
use rustflake::Snowflake;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub   struct Message {
    msg_id: i64,
    beacon_id: i64,
    nonce: i32,
    cmd: String,
}

impl Message {
    pub fn new(beacon: i64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            msg_id: Snowflake::default().generate(),
            beacon_id: beacon,
            nonce: rng.gen::<i32>(),
            cmd: "Hello".to_string(),
        }
    }
}
