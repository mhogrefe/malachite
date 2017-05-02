use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use std::str::FromStr;

#[test]
fn test_limb_count() {
    let test = |n, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().limb_count(), out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().limb_count(), out);
    };
    test("0", 0);
    test("123", 1);
    test("1000000000000", 2);
}
