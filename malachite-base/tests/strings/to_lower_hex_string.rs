use malachite_base::strings::ToLowerHexString;

#[test]
pub fn test_to_lower_hex_string() {
    assert_eq!(50u32.to_lower_hex_string(), "32");
    assert_eq!((-100i32).to_lower_hex_string(), "ffffff9c");
}
