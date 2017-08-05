use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_num_biguint,
                             native_natural_to_rugint_integer, num_biguint_to_native_natural,
                             rugint_integer_to_native_natural};
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_pairs_from_single,
                                     exhaustive_triples_from_single, random_pairs,
                                     random_pairs_from_single, random_triples_from_single};
use std::str::FromStr;

#[test]
fn test_add_assign() {
    #[allow(cyclomatic_complexity)]
    let test = |u, v, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n += native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = native::Natural::from_str(u).unwrap();
        n += &native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n += gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n += &gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = native::Natural::from_str(u).unwrap() + native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &native::Natural::from_str(u).unwrap() + native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = native::Natural::from_str(u).unwrap() + &native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &native::Natural::from_str(u).unwrap() + &native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Natural::from_str(u).unwrap() + gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Natural::from_str(u).unwrap() + &gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &gmp::Natural::from_str(u).unwrap() + gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &gmp::Natural::from_str(u).unwrap() + &gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num::BigUint::from_str(u).unwrap() + num::BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rugint::Integer::from_str(u).unwrap() + rugint::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "579");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("12345678987654321", "314159265358979", "12659838253013300");
    test(
        "1000000000000",
        "1000000000000000000000000",
        "1000000000001000000000000",
    );
}

#[test]
fn add_properties() {
    // x + y is valid.
    // x + &y is valid.
    // &x + y is valid.
    // &x + &y is valid.
    // x + y is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // x += y, x += &y, x + y, x + &y, &x + y, and &x + &y give the same result.
    // x + y == y + x
    #[allow(cyclomatic_complexity)]
    let two_naturals = |gmp_x: gmp::Natural, gmp_y: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let y = gmp_natural_to_native(&gmp_y);
        let raw_gmp_sum = gmp_x.clone() + gmp_y.clone();
        assert!(raw_gmp_sum.is_valid());
        let gmp_sum = gmp_natural_to_native(&raw_gmp_sum);
        let num_sum = num_biguint_to_native_natural(
            &(native_natural_to_num_biguint(&x) + native_natural_to_num_biguint(&y)),
        );
        let rugint_sum = rugint_integer_to_native_natural(
            &(native_natural_to_rugint_integer(&x) +
                  native_natural_to_rugint_integer(&y)),
        );

        let sum_val_val = gmp_x.clone() + gmp_y.clone();
        let sum_val_ref = gmp_x.clone() + &gmp_y;
        let sum_ref_val = &gmp_x + gmp_y.clone();
        assert!(sum_val_val.is_valid());
        assert!(sum_val_ref.is_valid());
        assert!(sum_ref_val.is_valid());
        assert_eq!(sum_val_val, raw_gmp_sum);
        assert_eq!(sum_val_ref, raw_gmp_sum);
        assert_eq!(sum_ref_val, raw_gmp_sum);

        let sum_val_val = x.clone() + y.clone();
        let sum_val_ref = x.clone() + &y;
        let sum_ref_val = &x + y.clone();
        let sum = &x + &y;
        assert!(sum_val_val.is_valid());
        assert!(sum_val_ref.is_valid());
        assert!(sum_ref_val.is_valid());
        assert!(sum.is_valid());
        assert_eq!(sum_val_val, sum);
        assert_eq!(sum_val_ref, sum);
        assert_eq!(sum_ref_val, sum);

        let mut mut_x = x.clone();
        mut_x += y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x += &y;
        assert_eq!(mut_x, sum);
        assert!(mut_x.is_valid());

        let mut mut_x = gmp_x.clone();
        mut_x += gmp_y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, raw_gmp_sum);
        let mut mut_x = gmp_x.clone();
        mut_x += &gmp_y;
        assert_eq!(mut_x, raw_gmp_sum);
        assert!(mut_x.is_valid());

        let mut mut_x = native_natural_to_rugint_integer(&x);
        mut_x += native_natural_to_rugint_integer(&y);
        assert_eq!(rugint_integer_to_native_natural(&mut_x), sum);

        let reverse_sum = &y + &x;
        let inv_1 = (&sum - &x).unwrap();
        let inv_2 = (&sum - &y).unwrap();
        assert_eq!(gmp_sum, sum);
        assert_eq!(num_sum, sum);
        assert_eq!(rugint_sum, sum);
        assert_eq!(reverse_sum, sum);
        assert_eq!(inv_1, y);
        assert_eq!(inv_2, x);
    };

    // x + (y: u32) == x + from(y)
    // (y: u32) + x == x + from(y)
    let natural_and_u32 = |gmp_x: gmp::Natural, y: u32| {
        let x = gmp_natural_to_native(&gmp_x);
        let primitive_sum_1 = &x + y;
        let primitive_sum_2 = y + &x;
        let sum = x + native::Natural::from(y);
        assert_eq!(primitive_sum_1, sum);
        assert_eq!(primitive_sum_2, sum);
    };

    // x + 0 == x
    // 0 + x == x
    // x + x == x << 1
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let x_old = x.clone();
        let id_1 = &x + native::Natural::from(0u32);
        let id_2 = native::Natural::from(0u32) + &x;
        let double = &x + &x;
        assert_eq!(id_1, x_old);
        assert_eq!(id_2, x_old);
        assert_eq!(double, x_old << 1);
    };

    // (x + y) + z == x + (y + z)
    let three_naturals = |gmp_x: gmp::Natural, gmp_y: gmp::Natural, gmp_z: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let y = gmp_natural_to_native(&gmp_y);
        let z = gmp_natural_to_native(&gmp_z);
        assert_eq!((&x + &y) + &z, x + (y + z));
    };

    for (x, y) in exhaustive_pairs_from_single(exhaustive_naturals()).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in random_pairs_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in exhaustive_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(LARGE_LIMIT) {
        natural_and_u32(x, y);
    }

    for (x, y) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(LARGE_LIMIT)
    {
        natural_and_u32(x, y);
    }

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for (x, y, z) in exhaustive_triples_from_single(exhaustive_naturals()).take(LARGE_LIMIT) {
        three_naturals(x, y, z);
    }

    for (x, y, z) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32))
        .take(LARGE_LIMIT)
    {
        three_naturals(x, y, z);
    }
}
