use malachite_base::strings::ToOctalString;

#[test]
pub fn test_to_octal_string() {
    assert_eq!(50u32.to_octal_string(), "62");
    assert_eq!((-100i32).to_octal_string(), "37777777634");
}
