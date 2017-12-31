use common::LARGE_LIMIT;
use malachite_base::traits::Assign;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_gmp,
                             native_natural_to_num_biguint, native_natural_to_rugint_integer,
                             num_biguint_to_native_natural, rugint_integer_to_native_natural,
                             GenerationMode};
use malachite_test::natural::conversion::clone_and_assign::{select_inputs_1, select_inputs_2};
use num;
use rugint;
use rugint::Assign as rugint_assign;
use std::str::FromStr;

#[test]
fn test_clone() {
    let test = |u| {
        let x = native::Natural::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = gmp::Natural::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = num::BigUint::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);

        let x = rugint::Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
    };
    test("123");
    test("1000000000000");
}

#[test]
fn test_clone_from_assign_and_assign_ref() {
    let test = |u, v, out| {
        // clone_from
        let mut x = native::Natural::from_str(u).unwrap();
        x.clone_from(&native::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Natural::from_str(u).unwrap();
        x.clone_from(&gmp::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = num::BigUint::from_str(u).unwrap();
        x.clone_from(&num::BigUint::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.clone_from(&rugint::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);

        // assign Natural by value
        let mut x = native::Natural::from_str(u).unwrap();
        x.assign(native::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Natural::from_str(u).unwrap();
        x.assign(gmp::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(rugint::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);

        // assign Natural by reference
        let mut x = native::Natural::from_str(u).unwrap();
        x.assign(&native::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Natural::from_str(u).unwrap();
        x.assign(&gmp::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(&rugint::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
    };
    test("123", "456", "456");
    test("123", "1000000000000", "1000000000000");
    test("1000000000000", "123", "123");
    test("1000000000000", "2000000000000", "2000000000000");
}

#[test]
fn clone_and_assign_properties() {
    // x.clone() is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // x.clone() is valid.
    // x.clone() == x
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let x_cloned = x.clone();
        assert!(x_cloned.is_valid());
        let gmp_x_cloned = gmp_x.clone();
        assert!(gmp_x_cloned.is_valid());
        assert_eq!(gmp_natural_to_native(&gmp_x_cloned), x_cloned);
        assert_eq!(
            num_biguint_to_native_natural(&native_natural_to_num_biguint(&x).clone()),
            x
        );
        assert_eq!(
            rugint_integer_to_native_natural(&native_natural_to_rugint_integer(&x).clone()),
            x
        );
        assert_eq!(x_cloned, x);
    };

    // x.clone_from(y) is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // x.clone_from(y) is valid.
    // x.clone_from(y); x == y
    // x.assign(y) is equivalent for malachite-gmp, malachite-native, and rugint.
    // x.assign(y) is valid.
    // x.assign(y); x == y
    // x.assign(&y) is equivalent for malachite-gmp, malachite-native, and rugint.
    // x.assign(&y) is valid.
    // x.assign(&y); x == y
    let two_naturals = |mut gmp_x: gmp::Natural, gmp_y: gmp::Natural| {
        let mut x = gmp_natural_to_native(&gmp_x);
        let y = gmp_natural_to_native(&gmp_y);
        let old_x = x.clone();
        gmp_x.clone_from(&gmp_y);
        assert!(gmp_x.is_valid());
        assert_eq!(gmp_x, gmp_y);
        x.clone_from(&y);
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut num_x = native_natural_to_num_biguint(&old_x);
        let num_y = native_natural_to_num_biguint(&y);
        num_x.clone_from(&num_y);
        assert_eq!(num_biguint_to_native_natural(&num_x), y);
        let mut rugint_x = native_natural_to_rugint_integer(&old_x);
        let rugint_y = native_natural_to_rugint_integer(&y);
        rugint_x.clone_from(&rugint_y);
        assert_eq!(rugint_integer_to_native_natural(&rugint_x), y);

        x = old_x.clone();
        gmp_x = native_natural_to_gmp(&old_x);
        gmp_x.assign(gmp_y.clone());
        assert!(gmp_x.is_valid());
        assert_eq!(gmp_x, gmp_y);
        x.assign(y.clone());
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rugint_x = native_natural_to_rugint_integer(&old_x);
        let rugint_y = native_natural_to_rugint_integer(&y);
        rugint_x.assign(rugint_y);
        assert_eq!(rugint_integer_to_native_natural(&rugint_x), y);

        x = old_x.clone();
        gmp_x = native_natural_to_gmp(&old_x);
        gmp_x.assign(&gmp_y);
        assert!(gmp_x.is_valid());
        assert_eq!(gmp_x, gmp_y);
        x.assign(&y);
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rugint_x = native_natural_to_rugint_integer(&old_x);
        let rugint_y = native_natural_to_rugint_integer(&y);
        rugint_x.assign(&rugint_y);
        assert_eq!(rugint_integer_to_native_natural(&rugint_x), y);
    };

    for n in select_inputs_1(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in select_inputs_1(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for (x, y) in select_inputs_2(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in select_inputs_2(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }
}
