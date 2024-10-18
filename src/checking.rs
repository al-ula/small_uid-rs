use crate::{Error, SmallUidError};

/// Checks if the timestamp is within the 44-bit range.
pub fn timestamp_check(timestamp: u64) -> Result<u64, Error> {
    let timestamp_str = format!("{:0b}", timestamp);
    if timestamp_str.len() <= 44 {
        Ok(timestamp)
    } else {
        Err(SmallUidError::TimestampLimit)
    }
}

/// Checks if the random number is within the 20-bit range.
pub fn rng_size_check(rn: u64) -> Result<u64, Error> {
    let random_20bit_str = format!("{:0b}", rn);
    if random_20bit_str.len() <= 20 {
        Ok(rn)
    } else {
        Err(SmallUidError::RandomSizeLimit)
    }
}