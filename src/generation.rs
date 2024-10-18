use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use crate::{Error, SmallUid};

pub fn timestamp_gen() -> Result<u64, Error> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)?;
    let timestamp = since_the_epoch.as_millis() as u64;
    crate::checking::timestamp_check(timestamp)
}

pub fn random_gen() -> Result<u64, Error> {
    let random_20bit: u64 = rand::thread_rng().gen_range(0..(1 << 20));
    crate::checking::rng_size_check(random_20bit)
}

pub fn gen() -> Result<SmallUid, Error> {
    let timestamp = timestamp_gen()?;
    let random = random_gen()?;
    Ok(SmallUid((timestamp << 20) | random))
}