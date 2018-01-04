use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, num_bigint_to_native_integer, GenerationMode};
use malachite_test::integer::conversion::from_i64::select_inputs;
use num;

#[test]
fn test_from_i64() {
    let test = |i: i64, out| {
        let x = native::Integer::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = gmp::Integer::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(num::BigInt::from(i).to_string(), out);
    };
    test(0i64, "0");
    test(123i64, "123");
    test(-123i64, "-123");
    test(1_000_000_000_000i64, "1000000000000");
    test(-1_000_000_000_000i64, "-1000000000000");
    test(i64::max_value(), "9223372036854775807");
    test(i64::min_value(), "-9223372036854775808");
}

#[test]
fn from_i64_properties() {
    // from(i: i64) is valid.
    // from(i: i64) is equivalent for malachite-gmp, malachite-native, and num.
    // from(i: i64).to_u64() == Some(i)
    let one_i64 = |i: i64| {
        let n = native::Integer::from(i);
        let raw_gmp_n = gmp::Integer::from(i);
        assert!(raw_gmp_n.is_valid());
        let gmp_n = gmp_integer_to_native(&raw_gmp_n);
        let num_n = num_bigint_to_native_integer(&num::BigInt::from(i));
        assert!(n.is_valid());
        assert_eq!(n.to_i64(), Some(i));
        assert_eq!(n, gmp_n);
        assert_eq!(n, num_n);
    };

    for i in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_i64(i);
    }

    for i in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_i64(i);
    }
}
