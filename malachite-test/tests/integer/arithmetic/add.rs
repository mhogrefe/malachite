use common::LARGE_LIMIT;
use malachite_base::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, integer_to_bigint, integer_to_rugint_integer,
                             rugint_integer_to_integer, GenerationMode};
use malachite_test::integer::arithmetic::add::select_inputs;
use num::BigInt;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_triples_from_single,
                                     random_pairs, random_triples_from_single};
use std::str::FromStr;

#[test]
fn test_add() {
    let test = |u, v, out| {
        let mut n = Integer::from_str(u).unwrap();
        n += Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Integer::from_str(u).unwrap();
        n += &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() + Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() + Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() + &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() + &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigInt::from_str(u).unwrap() + BigInt::from_str(v).unwrap();
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
    test("0", "-123", "-123");
    test("123", "-123", "0");
    test("123", "-456", "-333");
    test("1000000000000", "-123", "999999999877");
    test("123", "-1000000000000", "-999999999877");
    test("12345678987654321", "-314159265358979", "12031519722295342");
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
    let two_integers = |x: Integer, y: Integer| {
        let num_sum = bigint_to_integer(&(integer_to_bigint(&x) + integer_to_bigint(&y)));
        let rugint_sum = rugint_integer_to_integer(
            &(integer_to_rugint_integer(&x) + integer_to_rugint_integer(&y)),
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

        let mut mut_x = integer_to_rugint_integer(&x);
        mut_x += integer_to_rugint_integer(&y);
        assert_eq!(rugint_integer_to_integer(&mut_x), sum);

        let reverse_sum = &y + &x;
        let inv_1 = &sum - &x;
        let inv_2 = &sum - &y;
        assert_eq!(num_sum, sum);
        assert_eq!(rugint_sum, sum);
        assert_eq!(reverse_sum, sum);
        assert_eq!(inv_1, y);
        assert_eq!(inv_2, x);
    };

    // x + (y: i32) == x + from(y)
    // (y: i32) + x == x + from(y)
    let integer_and_i32 = |x: Integer, y: i32| {
        let primitive_sum_1 = &x + y;
        let primitive_sum_2 = y + &x;
        let sum = x + Integer::from(y);
        assert_eq!(primitive_sum_1, sum);
        assert_eq!(primitive_sum_2, sum);
    };

    // x + 0 == x
    // 0 + x == x
    // x + x == x << 1
    // x + -x == 0
    let one_integer = |x: Integer| {
        let id_1 = &x + Integer::ZERO;
        let id_2 = Integer::ZERO + &x;
        let double = &x + &x;
        assert_eq!(id_1, x);
        assert_eq!(id_2, x);
        assert_eq!(double, &x << 1);
        assert_eq!(&x + (-&x), 0)
    };

    // (x + y) + z == x + (y + z)
    let three_integers = |x: Integer, y: Integer, z: Integer| {
        assert_eq!((&x + &y) + &z, x + (y + z));
    };

    for (x, y) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for (x, y) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for (x, y) in exhaustive_pairs(exhaustive_integers(), exhaustive_i()).take(LARGE_LIMIT) {
        integer_and_i32(x, y);
    }

    for (x, y) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(LARGE_LIMIT)
    {
        integer_and_i32(x, y);
    }

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for (x, y, z) in exhaustive_triples_from_single(exhaustive_integers()).take(LARGE_LIMIT) {
        three_integers(x, y, z);
    }

    for (x, y, z) in
        random_triples_from_single(random_integers(&EXAMPLE_SEED, 32)).take(LARGE_LIMIT)
    {
        three_integers(x, y, z);
    }
}
