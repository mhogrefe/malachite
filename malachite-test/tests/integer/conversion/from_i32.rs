use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, num_bigint_to_native_integer,
                             rugint_integer_to_native, GenerationMode};
use malachite_test::integer::conversion::from_i32::select_inputs;
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

#[test]
fn from_i32_properties() {
    // from(i: i32) is valid.
    // from(i: i32) is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // from(i: i32).to_i32() == Some(i)
    let one_i32 = |i: i32| {
        let n = native::Integer::from(i);
        let raw_gmp_n = gmp::Integer::from(i);
        assert!(raw_gmp_n.is_valid());
        let gmp_n = gmp_integer_to_native(&raw_gmp_n);
        let num_n = num_bigint_to_native_integer(&num::BigInt::from(i));
        let rugint_n = rugint_integer_to_native(&rugint::Integer::from(i));
        assert!(n.is_valid());
        assert_eq!(n.to_i32(), Some(i));
        assert_eq!(n, gmp_n);
        assert_eq!(n, num_n);
        assert_eq!(n, rugint_n);
    };

    for i in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_i32(i);
    }

    for i in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_i32(i);
    }
}
