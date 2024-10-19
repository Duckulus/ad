/// This function returns data mod m
///
/// data can be an arbitrarily large number specified in big endian order
pub fn mod_hash(data: Vec<u8>, m: u32) -> u32 {
    let mut sum: u32 = 0;
    for byte in data {
        sum = ((sum << 8) + byte as u32) % m;
    }
    sum
}

#[test]
pub fn mod_hash_test() {
    assert_eq!(mod_hash(vec![97], 5), 97 % 5);
    assert_eq!(mod_hash(123_456_789u32.to_be_bytes().into(), 27), 123_456_789u32 % 27);
    assert_eq!(mod_hash(vec![0x01, 0x1C, 0x4D, 0xFA, 0xF1], 97), 75);
}