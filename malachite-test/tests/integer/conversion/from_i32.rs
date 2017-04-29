use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use num;
use rugint;

#[test]
fn test_from_i32() {
    let test = |i: i32, out| {
        let x = native::Integer::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = gmp::Integer::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(num::BigInt::from(i).to_string(), out);

        assert_eq!(rugint::Integer::from(i).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "-123");
    test(i32::min_value(), "-2147483648");
    test(i32::max_value(), "2147483647");
}
