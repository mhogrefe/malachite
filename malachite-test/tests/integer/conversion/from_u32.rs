use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use num;
use rugint;

#[test]
fn test_from_u32() {
    let test = |u: u32, out| {
        let x = native::Integer::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = gmp::Integer::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(num::BigInt::from(u).to_string(), out);

        assert_eq!(rugint::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u32::max_value(), "4294967295");
}
