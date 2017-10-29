use common::LARGE_LIMIT;
use malachite_base::traits::Assign;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_num_bigint,
                             native_integer_to_rugint, num_bigint_to_native_integer,
                             rugint_integer_to_native};
use malachite_test::integer::conversion::assign_i32::num_assign_i32;
use num;
use rugint;
use rugint::Assign as rugint_assign;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::str::FromStr;

#[test]
fn test_assign_i32() {
    let test = |u, v: i32, out| {
        let mut x = native::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = num::BigInt::from_str(u).unwrap();
        num_assign_i32(&mut x, v);
        assert_eq!(x.to_string(), out);

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
    };
    test("123", -456, "-456");
    test("-123", i32::max_value(), "2147483647");
    test("123", i32::min_value(), "-2147483648");
    test("1000000000000", 123, "123");
}

#[test]
fn assign_i32_properties() {
    // n.assign(i) is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // n.assign(i) is valid.
    // n.assign(i); n == u
    // n.assign(Integer::from(i)) is equivalent to n.assign(i)
    let integer_and_i32 = |mut gmp_n: gmp::Integer, i: i32| {
        let mut n = gmp_integer_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n.assign(i);
        assert!(gmp_n.is_valid());
        assert_eq!(gmp_n, i);
        n.assign(i);
        assert!(n.is_valid());
        assert_eq!(n, i);
        let mut alt_n = old_n.clone();
        alt_n.assign(native::Integer::from(i));
        assert_eq!(alt_n, n);

        let mut num_n = native_integer_to_num_bigint(&old_n);
        num_assign_i32(&mut num_n, i);
        assert_eq!(num_bigint_to_native_integer(&num_n), i);

        let mut rugint_n = native_integer_to_rugint(&old_n);
        rugint_n.assign(i);
        assert_eq!(rugint_integer_to_native(&rugint_n), i);
    };

    for (n, i) in exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }

    for (n, i) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i32>(seed)),
    ).take(LARGE_LIMIT)
    {
        integer_and_i32(n, i);
    }
}
