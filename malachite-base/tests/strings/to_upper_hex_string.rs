use malachite_base::strings::ToUpperHexString;

#[test]
pub fn test_to_upper_hex_string() {
    assert_eq!(50u32.to_upper_hex_string(), "32");
    assert_eq!((-100i32).to_upper_hex_string(), "FFFFFF9C");
}
