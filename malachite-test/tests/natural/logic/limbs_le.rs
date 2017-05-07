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
}
