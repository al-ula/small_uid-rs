//!
#![doc = include_str!("../README.md")]
//!
mod error;
mod test;
mod generation;
mod checking;

pub use error::SmallUidError;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
type Error = SmallUidError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmallUid(pub u64);

impl SmallUid {
    /// Creates a new small unique identifier.
    pub fn new() -> Result<SmallUid, Error> {
        generation::gen()
    }

    /// Creates a SmallUid from the provided timestamp and random number.
    pub fn from_parts(timestamp: u64, random: u64) -> Result<SmallUid, Error> {
        compose(timestamp, random)
    }

    /// Creates a SmallUid from the provided timestamp.
    pub fn from_timestamp(timestamp: u64) -> Result<SmallUid, Error> {
        let timestamp = checking::timestamp_check(timestamp)?;
        let random = generation::random_gen()?;
        compose(timestamp, random)
    }

    /// Creates a SmallUid from the provided random number.
    pub fn from_random(random: u64) -> Result<SmallUid, Error> {
        let random = checking::rng_size_check(random)?;
        let timestamp = generation::timestamp_gen()?;
        compose(timestamp, random)
    }
}

impl From<u64> for SmallUid {
    fn from(value: u64) -> Self {
        SmallUid(value)
    }
}

impl TryFrom<String> for SmallUid {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let smalluidstr = &value;
        let mut smalluidvec = Vec::new();
        base64_url::decode_to_vec(smalluidstr, &mut smalluidvec)?;
        let smalluidarr: [u8; 8] = smalluidvec
            .try_into()
            .map_err(|_| SmallUidError::VecToArray)?;
        let smalluidu64 = u64::from_be_bytes(smalluidarr);
        Ok(SmallUid(smalluidu64))
    }
}

impl From<SmallUid> for String {
    fn from(value: SmallUid) -> Self {
        value.to_string()
    }
}

impl From<SmallUid> for u64 {
    fn from(value: SmallUid) -> Self {
        value.0
    }
}

impl Display for SmallUid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let smalluid = base64_url::encode(&self.0.to_be_bytes());
        write!(f, "{}", smalluid)
    }
}


/// Composes a timestamp and a random number into a SmallUid.
fn compose(timestamp: u64, random: u64) -> Result<SmallUid, Error> {
    let timestamp = checking::timestamp_check(timestamp)?;
    let random = checking::rng_size_check(random)?;
    Ok(SmallUid((timestamp << 20) | random))
}