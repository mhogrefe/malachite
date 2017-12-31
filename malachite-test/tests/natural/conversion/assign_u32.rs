use common::LARGE_LIMIT;
use malachite_base::traits::Assign;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_num_biguint,
                             native_natural_to_rugint_integer, num_biguint_to_native_natural,
                             rugint_integer_to_native_natural, GenerationMode};
use malachite_test::natural::conversion::assign_u32::{select_inputs, num_assign_u32};
use num;
use rugint;
use rugint::Assign as rugint_assign;
use std::str::FromStr;

#[test]
fn test_assign_u32() {
    let test = |u, v: u32, out| {
        let mut x = native::Natural::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Natural::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = num::BigUint::from_str(u).unwrap();
        num_assign_u32(&mut x, v);
        assert_eq!(x.to_string(), out);

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
    };
    test("123", 456, "456");
    test("123", u32::max_value(), "4294967295");
    test("1000000000000", 123, "123");
}

#[test]
fn assign_u32_properties() {
    // n.assign(u) is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // n.assign(u) is valid.
    // n.assign(u); n == u
    // n.assign(Natural::from(u)) is equivalent to n.assign(u)
    let natural_and_u32 = |mut gmp_n: gmp::Natural, u: u32| {
        let mut n = gmp_natural_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n.assign(u);
        assert!(gmp_n.is_valid());
        assert_eq!(gmp_n, u);
        n.assign(u);
        assert!(n.is_valid());
        assert_eq!(n, u);
        let mut alt_n = old_n.clone();
        alt_n.assign(native::Natural::from(u));
        assert_eq!(alt_n, n);

        let mut num_n = native_natural_to_num_biguint(&old_n);
        num_assign_u32(&mut num_n, u);
        assert_eq!(num_biguint_to_native_natural(&num_n), u);

        let mut rugint_n = native_natural_to_rugint_integer(&old_n);
        rugint_n.assign(u);
        assert_eq!(rugint_integer_to_native_natural(&rugint_n), u);
    };

    for (n, u) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }
}
