use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, num_bigint_to_native_integer,
                             rugint_integer_to_native, GenerationMode};
use malachite_test::integer::conversion::from_u32::select_inputs;
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

#[test]
fn from_u32_properties() {
    // from(u: u32) is valid.
    // from(u: u32) is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // from(u: u32).to_u32() == Some(u)
    let one_u32 = |u: u32| {
        let n = native::Integer::from(u);
        let raw_gmp_n = gmp::Integer::from(u);
        assert!(raw_gmp_n.is_valid());
        let gmp_n = gmp_integer_to_native(&raw_gmp_n);
        let num_n = num_bigint_to_native_integer(&num::BigInt::from(u));
        let rugint_n = rugint_integer_to_native(&rugint::Integer::from(u));
        assert!(n.is_valid());
        assert_eq!(n.to_u32(), Some(u));
        assert_eq!(n, gmp_n);
        assert_eq!(n, num_n);
        assert_eq!(n, rugint_n);
    };

    for u in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u32(u);
    }

    for u in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32(u);
    }
}
