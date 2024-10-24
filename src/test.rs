use crate::SmallUid;

#[test]
fn test_new() {
    let smalluid1 = SmallUid::new();
    let smalluid2 = SmallUid::new();
    assert_ne!(smalluid1, smalluid2);
}

#[test]
fn test_from_parts() {
    let timestamp = 1234567890;
    let random = 123456;
    let smalluid1 = SmallUid::from_parts(timestamp, random);
    let smalluid2 = SmallUid::from_parts(timestamp, random);
    assert_eq!(smalluid1, smalluid2);
}

#[test]
fn test_from_timestamp() {
    let timestamp = 1234567890;
    let smalluid1 = SmallUid::from_timestamp(timestamp);
    let smalluid2 = SmallUid::from_timestamp(timestamp);
    assert_ne!(smalluid1, smalluid2);
}

#[test]
fn test_from_random() {
    let random = 123456;
    let smalluid1 = SmallUid::from_random(random);
    std::thread::sleep(std::time::Duration::from_millis(1));
    let smalluid2 = SmallUid::from_random(random);
    assert_ne!(smalluid1, smalluid2);
}

#[test]
fn test_try_from() {
    let input = "GSntNvOw6n8";
    let smalluid1 = SmallUid::try_from(input.to_string()).unwrap();
    let output = smalluid1.to_string();
    assert_eq!(input, output);
}

#[test]
fn test_from() {
    let smalluid = SmallUid::new();
    let string1: String = smalluid.into();
    let string2: String = smalluid.into();
    assert_eq!(string1, string2);
    let u64_1: u64 = smalluid.into();
    let u64_2: u64 = smalluid.into();
    assert_eq!(u64_1, u64_2);
}

#[test]
fn test_display() {
    let smalluid = SmallUid::new();
    let string1 = format!("{}", smalluid);
    let string2 = format!("{}", smalluid);
    assert_eq!(string1, string2);
}

#[test]
fn from_u64() {
    let smalluid = SmallUid::new();
    let u64: u64 = smalluid.into();
    let smalluid2: SmallUid = u64.into();
    assert_eq!(smalluid, smalluid2);
}

#[test]
fn test_get(){
    let smalluid = SmallUid::new();
    let timestamp = smalluid.get_timestamp();
    let random = smalluid.get_random();
    let smalluid2 = SmallUid::from_parts(timestamp, random);
    assert_eq!(smalluid, smalluid2);
}