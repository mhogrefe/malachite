use common::LARGE_LIMIT;
use malachite_base::traits::One;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, GenerationMode};
use malachite_test::natural::logic::limb_count::select_inputs;
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
    test("4294967295", 1);
    test("4294967296", 2);
    test("18446744073709551615", 2);
    test("18446744073709551616", 3);
}

#[test]
fn limb_count_properties() {
    // x.limb_count() is equivalent for malachite-gmp and malachite-native.
    // (x < 2^32) == (x.limb_count() <= 1)
    // if x != 0, (x.limb_count() == n) == (2^(32*(n-1)) <= x < 2^(32*n))
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let limb_count = x.limb_count();
        assert_eq!(gmp_x.limb_count(), limb_count);
        assert_eq!(x <= u32::max_value(), x.limb_count() <= 1);
        if x != 0 {
            let n = limb_count as u32;
            assert!(native::Natural::ONE << ((n - 1) << 5) <= x);
            assert!(x < native::Natural::ONE << (n << 5));
        }
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
