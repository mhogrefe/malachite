use common::LARGE_LIMIT;
use malachite_base::traits::Assign;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_num_biguint,
                             num_biguint_to_native_natural, GenerationMode};
use malachite_test::natural::conversion::assign_u64::{select_inputs, num_assign_u64};
use num;
use std::str::FromStr;

#[test]
fn test_assign_u64() {
    let test = |u, v: u64, out| {
        let mut x = native::Natural::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Natural::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = num::BigUint::from_str(u).unwrap();
        num_assign_u64(&mut x, v);
        assert_eq!(x.to_string(), out);
    };
    test("123", 456, "456");
    test("123", u32::max_value().into(), "4294967295");
    test("123", u64::max_value(), "18446744073709551615");
    test("1000000000000000000000000", 123, "123");
}

#[test]
fn assign_u64_properties() {
    // n.assign(u) is equivalent for malachite-gmp, malachite-native, and num.
    // n.assign(u) is valid.
    // n.assign(u); n == u
    // n.assign(Natural::from(u)) is equivalent to n.assign(u)
    let natural_and_u64 = |mut gmp_n: gmp::Natural, u: u64| {
        let mut n = gmp_natural_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n.assign(u);
        assert!(gmp_n.is_valid());
        assert_eq!(gmp_n, gmp::Natural::from(u));
        n.assign(u);
        assert!(n.is_valid());
        let natural_u = native::Natural::from(u);
        assert_eq!(n, natural_u);
        let mut alt_n = old_n.clone();
        alt_n.assign(native::Natural::from(u));
        assert_eq!(alt_n, n);

        let mut num_n = native_natural_to_num_biguint(&old_n);
        num_assign_u64(&mut num_n, u);
        assert_eq!(num_biguint_to_native_natural(&num_n), natural_u);
    };

    for (n, u) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u64(n, u);
    }

    for (n, u) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u64(n, u);
    }
}
