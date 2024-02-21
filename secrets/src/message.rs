// https://docs.rs/rmp-serde/latest/rmp_serde/

extern crate rmp_serde as rmps;
extern crate serde;

use rand::Rng;
use rustflake::Snowflake;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Envelope {
    msg_id: i64,
    beacon_id: i64,
    nonce: i32,
    message: String,
}

impl Envelope {
    pub fn new(beacon: i64, message: &str) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            msg_id: Snowflake::default().generate(),
            beacon_id: beacon,
            nonce: rng.gen::<i32>(),
            message: message.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmp_serde::Serializer;

    #[test]
    fn test_envelope() {
        let beacon_id = Snowflake::default().generate();
        let envelope = Envelope::new(beacon_id, "Hello World!");

        let buf_out = rmp_serde::to_vec(&envelope).unwrap();

        let envelop_deserialized = rmp_serde::from_slice(&buf_out).unwrap();

        assert_eq!(envelope, envelop_deserialized);
        assert_eq!(beacon_id, envelop_deserialized.beacon_id);
    }
}
