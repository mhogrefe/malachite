use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_pairs_from_single, random_pairs,
                                     random_pairs_from_single};
use std::str::FromStr;

#[test]
fn test_add_assign() {
    let test = |u, v, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n += native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n += gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "579");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("12345678987654321", "314159265358979", "12659838253013300");
}

#[test]
fn test_add() {
    let test = |u, v, out| {
        let n = native::Natural::from_str(u).unwrap() + native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Natural::from_str(u).unwrap() + gmp::Natural::from_str(v).unwrap();
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
}

fn to_native(n: &gmp::Natural) -> native::Natural {
    let mut native = native::Natural::new();
    native.assign_limbs_le(n.limbs_le().as_slice());
    native
}

fn from_native(n: &native::Natural) -> gmp::Natural {
    let mut gmp = gmp::Natural::new();
    gmp.assign_limbs_le(n.limbs_le().as_slice());
    gmp
}

fn to_num(n: &gmp::Natural) -> num::BigUint {
    num::BigUint::from_str(n.to_string().as_ref()).unwrap()
}

fn from_num(n: &num::BigUint) -> gmp::Natural {
    gmp::Natural::from_str(n.to_string().as_ref()).unwrap()
}

fn to_rugint(n: &gmp::Natural) -> rugint::Integer {
    rugint::Integer::from_str(n.to_string().as_ref()).unwrap()
}

fn from_rugint(n: &rugint::Integer) -> gmp::Natural {
    gmp::Natural::from_str(n.to_string().as_ref()).unwrap()
}

#[test]
fn add_properties() {
    // x + y is valid.
    // x + y is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // x + y == y + x
    let two_naturals = |x: gmp::Natural, y: gmp::Natural| {
        let native_sum = from_native(&(to_native(&x) + to_native(&y)));
        let num_sum = from_num(&(to_num(&x) + to_num(&y)));
        let rugint_sum = from_rugint(&(to_rugint(&x) + to_rugint(&y)));
        let reverse_sum = y.clone() + x.clone();
        let sum: gmp::Natural = x + y;
        assert!(sum.is_valid());
        assert_eq!(native_sum, sum);
        assert_eq!(num_sum, sum);
        assert_eq!(rugint_sum, sum);
        assert_eq!(reverse_sum, sum);
    };

    // x + (y: u32) == x + from(y)
    // (y: u32) + x == x + from(y)
    let natural_and_u32 = |x: gmp::Natural, y: u32| {
        let primitive_sum_1 = x.clone() + y;
        let primitive_sum_2 = y + x.clone();
        let sum = x + gmp::Natural::from(y);
        assert_eq!(primitive_sum_1, sum);
        assert_eq!(primitive_sum_2, sum);
    };

    // x + 0 == x
    // 0 + x == x
    // x + x == x << 1
    let one_natural = |x: gmp::Natural| {
        let x_old = x.clone();
        let id_1 = x.clone() + gmp::Natural::from(0);
        let id_2 = gmp::Natural::from(0) + x.clone();
        let double = x.clone() + x;
        assert_eq!(id_1, x_old);
        assert_eq!(id_2, x_old);
        assert_eq!(double, x_old << 1);
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

    for (x, y) in random_pairs(&EXAMPLE_SEED,
                               &(|seed| random_naturals(seed, 32)),
                               &(|seed| random_x(seed)))
                .take(LARGE_LIMIT) {
        natural_and_u32(x, y);
    }

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }

    //TODO inverse of sub

    //TODO associativity
}
