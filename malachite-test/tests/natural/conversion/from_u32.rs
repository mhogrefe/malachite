use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num;

#[test]
fn test_from_u32() {
    let test = |u: u32, out| {
        let x = native::Natural::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = gmp::Natural::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(num::BigUint::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u32::max_value(), "4294967295");
}
