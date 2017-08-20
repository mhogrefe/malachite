use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_gmp,
                             native_natural_to_num_biguint, native_natural_to_rugint_integer,
                             num_biguint_to_native_natural, rugint_integer_to_native_natural};
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};
use std::str::FromStr;

#[test]
fn test_shl_u32() {
    let test = |u, v: u32, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n <<= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n <<= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n <<= v;
        assert_eq!(n.to_string(), out);

        let n = native::Natural::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Natural::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rugint::Integer::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);

        let n = num::BigUint::from_str(u).unwrap() << v as usize;
        assert_eq!(n.to_string(), out);

        let n = &native::Natural::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &gmp::Natural::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &num::BigUint::from_str(u).unwrap() << v as usize;
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 10, "0");
    test("123", 0, "123");
    test("123", 1, "246");
    test("123", 2, "492");
    test("123", 25, "4127195136");
    test("123", 26, "8254390272");
    test("123", 100, "155921023828072216384094494261248");
    test("2147483648", 1, "4294967296");
    test("1000000000000", 0, "1000000000000");
    test("1000000000000", 3, "8000000000000");
    test("1000000000000", 24, "16777216000000000000");
    test("1000000000000", 25, "33554432000000000000");
    test("1000000000000", 31, "2147483648000000000000");
    test("1000000000000", 32, "4294967296000000000000");
    test("1000000000000", 33, "8589934592000000000000");
    test(
        "1000000000000",
        100,
        "1267650600228229401496703205376000000000000",
    );
}

#[test]
fn shl_u32_properties() {
    // n <<= u is equivalent for malachite-gmp, malachite-native, and rugint.
    // n << u is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // &n << u is equivalent for malachite-gmp, malachite-native, and num.
    // n <<= u; n is valid.
    // n << u is valid.
    // &n << u is valid.
    // n <<= u, n << u, and &n << u give the same result.
    // n << u >= n
    // n << u == n * (1 << u)
    // TODO >>
    let natural_and_u32 = |mut gmp_n: gmp::Natural, u: u32| {
        let mut n = gmp_natural_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n <<= u;
        assert!(gmp_n.is_valid());

        n <<= u;
        assert!(n.is_valid());
        assert_eq!(gmp_natural_to_native(&gmp_n), n);

        let mut rugint_n = native_natural_to_rugint_integer(&old_n);
        rugint_n <<= u;
        assert_eq!(rugint_integer_to_native_natural(&rugint_n), n);

        let n2 = old_n.clone();
        let result = &n2 << u;
        assert_eq!(result, n);
        assert!(result.is_valid());
        let result = n2 << u;
        assert!(result.is_valid());
        assert_eq!(result, n);

        let gmp_n2 = native_natural_to_gmp(&old_n);
        let result = &gmp_n2 << u;
        assert!(result.is_valid());
        assert_eq!(gmp_natural_to_native(&result), n);
        let result = gmp_n2 << u;
        assert!(result.is_valid());
        assert_eq!(gmp_natural_to_native(&result), n);

        let num_n2 = native_natural_to_num_biguint(&old_n);
        assert_eq!(num_biguint_to_native_natural(&(&num_n2 << u as usize)), n);
        assert_eq!(num_biguint_to_native_natural(&(num_n2 << u as usize)), n);

        let rugint_n2 = native_natural_to_rugint_integer(&old_n);
        assert_eq!(rugint_integer_to_native_natural(&(rugint_n2 << u)), n);

        assert!(&old_n << u >= old_n);
        assert_eq!(&old_n << u, old_n * (native::Natural::from(1u32) << u));
    };

    // n << 0 == n
    #[allow(identity_op)]
    let one_natural = |gmp_n: gmp::Natural| {
        let n = gmp_natural_to_native(&gmp_n);
        assert_eq!(&n << 0, n);
    };

    // 0 << n == 0
    // 1 << n is a power of 2
    let one_u32 = |u: u32| {
        assert_eq!(native::Natural::from(0u32) << u, 0);
        assert!((native::Natural::from(1u32) << u).is_power_of_two());
    };

    for (n, u) in log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(LARGE_LIMIT)
    {
        natural_and_u32(n, u);
    }

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in exhaustive_u().take(LARGE_LIMIT) {
        one_u32(n);
    }

    for n in natural_u32s_geometric(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_u32(n);
    }
}
