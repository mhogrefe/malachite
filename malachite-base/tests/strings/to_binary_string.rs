use malachite_base::strings::ToBinaryString;

#[test]
pub fn test_to_binary_string() {
    assert_eq!(5u32.to_binary_string(), "101");
    assert_eq!(
        (-5i32).to_binary_string(),
        "11111111111111111111111111111011"
    );
}
