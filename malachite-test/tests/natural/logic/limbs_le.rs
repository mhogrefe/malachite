use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use std::str::FromStr;

#[test]
fn test_limbs_le() {
    let test = |n, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().limbs_le(), out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().limbs_le(), out);
    };
    test("0", Vec::new());
    test("123", vec![123]);
    test("1000000000000", vec![3567587328, 232]);
    test("1701411834921604967429270619762735448065",
         vec![1, 2, 3, 4, 5]);
    test("4294967295", vec![4294967295]);
    test("4294967296", vec![0, 1]);
    test("18446744073709551615", vec![4294967295, 4294967295]);
    test("18446744073709551616", vec![0, 0, 1]);
}
