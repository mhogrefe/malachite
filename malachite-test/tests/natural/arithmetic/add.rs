use common::LARGE_LIMIT;
use malachite_base::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, natural_to_biguint, natural_to_rugint_integer,
                             rugint_integer_to_natural, GenerationMode};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned, pairs_of_naturals,
                                      triples_of_naturals};
use num::BigUint;
use rugint;
use std::str::FromStr;

#[test]
fn test_add() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n += Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n += &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() + Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() + Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() + &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() + &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigUint::from_str(u).unwrap() + BigUint::from_str(v).unwrap();
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
    // x + y is equivalent for malachite, num, and rugint.
    // x += y, x += &y, x + y, x + &y, &x + y, and &x + &y give the same result.
    // x + y == y + x
    // x + y - y == x and x + y - x == y
    // x + y >= x and x + y >= y
    let two_naturals = |x: Natural, y: Natural| {
        let num_sum = biguint_to_natural(&(natural_to_biguint(&x) + natural_to_biguint(&y)));
        let rugint_sum = rugint_integer_to_natural(
            &(natural_to_rugint_integer(&x) + natural_to_rugint_integer(&y)),
        );

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

        let mut mut_x = natural_to_rugint_integer(&x);
        mut_x += natural_to_rugint_integer(&y);
        assert_eq!(rugint_integer_to_natural(&mut_x), sum);

        let reverse_sum = &y + &x;
        let inv_1 = (&sum - &x).unwrap();
        let inv_2 = (&sum - &y).unwrap();
        assert_eq!(num_sum, sum);
        assert_eq!(rugint_sum, sum);
        assert_eq!(reverse_sum, sum);
        assert_eq!(inv_1, y);
        assert_eq!(inv_2, x);

        assert!(sum >= x);
        assert!(sum >= y);
    };

    // x + (y: u32) == x + from(y)
    // (y: u32) + x == x + from(y)
    let natural_and_u32 = |x: Natural, y: u32| {
        let primitive_sum_1 = &x + y;
        let primitive_sum_2 = y + &x;
        let sum = x + Natural::from(y);
        assert_eq!(primitive_sum_1, sum);
        assert_eq!(primitive_sum_2, sum);
    };

    // x + 0 == x
    // 0 + x == x
    // x + x == x << 1
    let one_natural = |x: Natural| {
        let x_old = x.clone();
        let id_1 = &x + Natural::ZERO;
        let id_2 = Natural::ZERO + &x;
        let double = &x + &x;
        assert_eq!(id_1, x_old);
        assert_eq!(id_2, x_old);
        assert_eq!(double, x_old << 1);
    };

    // (x + y) + z == x + (y + z)
    let three_naturals = |x: Natural, y: Natural, z: Natural| {
        assert_eq!((&x + &y) + &z, x + (y + z));
    };

    for (x, y) in pairs_of_naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in pairs_of_naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in pairs_of_natural_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u32(x, y);
    }

    for (x, y) in pairs_of_natural_and_unsigned(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u32(x, y);
    }

    for n in naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for (x, y, z) in triples_of_naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        three_naturals(x, y, z);
    }

    for (x, y, z) in triples_of_naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        three_naturals(x, y, z);
    }
}
