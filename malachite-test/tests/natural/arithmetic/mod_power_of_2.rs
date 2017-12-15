use common::LARGE_LIMIT;
use malachite_base::round::RoundingMode;
use malachite_base::traits::{One, ShrRound, Zero};
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_gmp};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, log_pairs, random_pairs,
                                     random_triples};
use std::cmp::min;
use std::str::FromStr;

#[test]
fn test_mod_power_of_2() {
    let test = |u, v: u32, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n.mod_power_of_2_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n.mod_power_of_2_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = native::Natural::from_str(u).unwrap().mod_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Natural::from_str(u).unwrap().mod_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = native::Natural::from_str(u).unwrap().mod_power_of_2_ref(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Natural::from_str(u).unwrap().mod_power_of_2_ref(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 0, "0");
    test("260", 8, "4");
    test("1611", 4, "11");
    test("123", 100, "123");
    test("1000000000000", 0, "0");
    test("1000000000000", 12, "0");
    test("1000000000001", 12, "1");
    test("999999999999", 12, "4095");
    test("1000000000000", 15, "4096");
    test("1000000000000", 100, "1000000000000");
    test("1000000000000000000000000", 40, "1020608380928");
    test("1000000000000000000000000", 64, "2003764205206896640");
}

#[test]
fn test_complement_mod_power_of_2() {
    let test = |u, v: u32, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n.complement_mod_power_of_2_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n.complement_mod_power_of_2_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = native::Natural::from_str(u)
            .unwrap()
            .complement_mod_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Natural::from_str(u)
            .unwrap()
            .complement_mod_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = native::Natural::from_str(u)
            .unwrap()
            .complement_mod_power_of_2_ref(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Natural::from_str(u)
            .unwrap()
            .complement_mod_power_of_2_ref(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 0, "0");
    test("260", 8, "252");
    test("1611", 4, "5");
    test("123", 100, "1267650600228229401496703205253");
    test("1000000000000", 0, "0");
    test("1000000000000", 12, "0");
    test("1000000000001", 12, "4095");
    test("999999999999", 12, "1");
    test("1000000000000", 15, "28672");
    test("1000000000000", 100, "1267650600228229400496703205376");
    test("1000000000000000000000000", 40, "78903246848");
    test("1000000000000000000000000", 64, "16442979868502654976");
}

#[test]
fn mod_power_of_2_properties() {
    // n.mod_power_of_2(u) is equivalent for malachite-gmp and malachite-native.
    // n.mod_power_of_2(u) is equivalent for malachite-gmp and malachite-native.
    // n.mod_power_of_2_ref(u) is equivalent for malachite-gmp and malachite-native.
    // n.mod_power_of_2_assign(u); n is valid.
    // n.mod_power_of_2(u) is valid.
    // n.mod_power_of_2_ref(u) is valid.
    // n.mod_power_of_2_assign(u), n.mod_power_of_2(u), and n.mod_power_of_2_ref(u) give the same
    //      result.
    // (n >> u << u) + n.mod_power_of_2(u) == n
    // n.mod_power_of_2(u) < (1 << u)
    // (n.mod_power_of_2(u) == 0) == (n.divisible_by_power_of_2(u))
    // n.mod_power_of_2(u).mod_power_of_2(u) == n.mod_power_of_2(u)
    let natural_and_u32 = |mut gmp_n: gmp::Natural, u: u32| {
        let mut n = gmp_natural_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n.mod_power_of_2_assign(u);
        assert!(gmp_n.is_valid());

        n.mod_power_of_2_assign(u);
        assert!(n.is_valid());
        assert_eq!(gmp_natural_to_native(&gmp_n), n);

        let n2 = old_n.clone();
        let result = n2.mod_power_of_2_ref(u);
        assert_eq!(result, n);
        assert!(result.is_valid());
        let result = n2.mod_power_of_2(u);
        assert!(result.is_valid());
        assert_eq!(result, n);

        let gmp_n2 = native_natural_to_gmp(&old_n);
        let result = gmp_n2.mod_power_of_2_ref(u);
        assert!(result.is_valid());
        assert_eq!(gmp_natural_to_native(&result), n);
        let result = gmp_n2.mod_power_of_2(u);
        assert!(result.is_valid());
        assert_eq!(gmp_natural_to_native(&result), n);

        assert_eq!((&old_n >> u << u) + &n, old_n);
        assert!(n < (native::Natural::ONE << u));
        assert_eq!(n == 0, old_n.divisible_by_power_of_2(u));
        assert_eq!(n.mod_power_of_2_ref(u), n);
    };

    // If n is divisible by 2^u, n.mod_power_of_2(u) == 0
    let natural_and_u32_divisible = |gmp_n: gmp::Natural, u: u32| {
        let n = gmp_natural_to_native(&gmp_n);
        assert_eq!(n.mod_power_of_2(u), 0);
    };

    // If n is not divisible by 2^u, n.mod_power_of_2(u) != 0
    // If n is not divisible by 2^u, n.mod_power_of_2(u) + n.complement_mod_power_of_2(u) == 1 << u
    let natural_and_u32_non_divisible = |gmp_n: gmp::Natural, u: u32| {
        let n = gmp_natural_to_native(&gmp_n);
        assert_ne!(n.mod_power_of_2_ref(u), 0);
        assert_eq!(
            n.mod_power_of_2_ref(u) + n.complement_mod_power_of_2(u),
            native::Natural::ONE << u
        );
    };

    // n.mod_power_of_2(u).mod_power_of_2(v) == n.mod_power_of_2(min(u, v))
    let natural_and_two_u32s = |gmp_n: gmp::Natural, u: u32, v: u32| {
        let n = gmp_natural_to_native(&gmp_n);
        assert_eq!(
            n.mod_power_of_2_ref(u).mod_power_of_2(v),
            n.mod_power_of_2(min(u, v))
        );
    };

    // n.mod_power_of_2(0) == 0
    let one_natural = |gmp_n: gmp::Natural| {
        let n = gmp_natural_to_native(&gmp_n);
        assert_eq!(n.mod_power_of_2_ref(0), 0);
    };

    // 0.mod_power_of_2(n) == 0
    let one_u32 = |u: u32| {
        assert_eq!(native::Natural::ZERO.mod_power_of_2(u), 0);
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

    for (n, u) in log_pairs(exhaustive_naturals(), exhaustive_u::<u32>())
        .map(|(n, u)| (n << u, u))
        .take(LARGE_LIMIT)
    {
        natural_and_u32_divisible(n, u);
    }

    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).map(|(n, u)| (n << u, u))
        .take(LARGE_LIMIT)
    {
        natural_and_u32_divisible(n, u);
    }

    for (n, u) in log_pairs(exhaustive_naturals(), exhaustive_u::<u32>())
        .filter(|&(ref n, u)| !n.divisible_by_power_of_2(u))
        .take(LARGE_LIMIT)
    {
        natural_and_u32_non_divisible(n, u);
    }

    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).filter(|&(ref n, u)| !n.divisible_by_power_of_2(u))
        .take(LARGE_LIMIT)
    {
        natural_and_u32_non_divisible(n, u);
    }

    for (n, (u, v)) in log_pairs(
        exhaustive_naturals(),
        exhaustive_pairs_from_single(exhaustive_u::<u32>()),
    ).take(LARGE_LIMIT)
    {
        natural_and_two_u32s(n, u, v);
    }

    for (n, u, v) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(LARGE_LIMIT)
    {
        natural_and_two_u32s(n, u, v);
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

#[test]
fn complement_mod_power_of_2_properties() {
    // n.complement_mod_power_of_2(u) is equivalent for malachite-gmp and malachite-native.
    // n.complement_mod_power_of_2(u) is equivalent for malachite-gmp and malachite-native.
    // n.complement_mod_power_of_2_ref(u) is equivalent for malachite-gmp and malachite-native.
    // n.complement_mod_power_of_2_assign(u); n is valid.
    // n.complement_mod_power_of_2(u) is valid.
    // n.complement_mod_power_of_2_ref(u) is valid.
    // n.complement_mod_power_of_2_assign(u), n.complement_mod_power_of_2(u), and
    //      (&n).complement_mod_power_of_2(u) give the same result.
    // (n.shr_round(u, Ceiling) << u) - n.complement_mod_power_of_2(u) == n
    // n.complement_mod_power_of_2(u) < (1 << u)
    // (n.complement_mod_power_of_2(u) == 0) == (n.divisible_by_power_of_2(u))
    // n.complement_mod_power_of_2(u).complement_mod_power_of_2(u) == n.mod_power_of_2(u)
    let natural_and_u32 = |mut gmp_n: gmp::Natural, u: u32| {
        let mut n = gmp_natural_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n.complement_mod_power_of_2_assign(u);
        assert!(gmp_n.is_valid());

        n.complement_mod_power_of_2_assign(u);
        assert!(n.is_valid());
        assert_eq!(gmp_natural_to_native(&gmp_n), n);

        let n2 = old_n.clone();
        let result = n2.complement_mod_power_of_2_ref(u);
        assert_eq!(result, n);
        assert!(result.is_valid());
        let result = n2.complement_mod_power_of_2(u);
        assert!(result.is_valid());
        assert_eq!(result, n);

        let gmp_n2 = native_natural_to_gmp(&old_n);
        let result = gmp_n2.complement_mod_power_of_2_ref(u);
        assert!(result.is_valid());
        assert_eq!(gmp_natural_to_native(&result), n);
        let result = gmp_n2.complement_mod_power_of_2(u);
        assert!(result.is_valid());
        assert_eq!(gmp_natural_to_native(&result), n);

        assert_eq!(
            (((&old_n).shr_round(u, RoundingMode::Ceiling) << u) - &n),
            Some(old_n.clone())
        );
        assert!(n < (native::Natural::ONE << u));
        assert_eq!(n == 0, old_n.divisible_by_power_of_2(u));
        assert_eq!(n.complement_mod_power_of_2_ref(u), old_n.mod_power_of_2(u));
    };

    // If n is divisible by 2^u, n.complement_mod_power_of_2(u) == 0
    let natural_and_u32_divisible = |gmp_n: gmp::Natural, u: u32| {
        let n = gmp_natural_to_native(&gmp_n);
        assert_eq!(n.complement_mod_power_of_2(u), 0);
    };

    let natural_and_u32_non_divisible = |gmp_n: gmp::Natural, u: u32| {
        let n = gmp_natural_to_native(&gmp_n);
        let m = n.complement_mod_power_of_2_ref(u);
        assert_ne!(m, 0);
        assert_eq!(((((&n >> u) + 1) << u) - &m), Some(n.clone()));
        assert_eq!(n.mod_power_of_2(u) + m, native::Natural::ONE << u);
    };

    // n.complement_mod_power_of_2(0) == 0
    let one_natural = |gmp_n: gmp::Natural| {
        let n = gmp_natural_to_native(&gmp_n);
        assert_eq!(n.complement_mod_power_of_2_ref(0), 0);
    };

    // 0.complement_mod_power_of_2(n) == 0
    let one_u32 = |u: u32| {
        assert_eq!(native::Natural::ZERO.complement_mod_power_of_2(u), 0);
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

    for (n, u) in log_pairs(exhaustive_naturals(), exhaustive_u::<u32>())
        .map(|(n, u)| (n << u, u))
        .take(LARGE_LIMIT)
    {
        natural_and_u32_divisible(n, u);
    }

    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).map(|(n, u)| (n << u, u))
        .take(LARGE_LIMIT)
    {
        natural_and_u32_divisible(n, u);
    }

    for (n, u) in log_pairs(exhaustive_naturals(), exhaustive_u::<u32>())
        .filter(|&(ref n, u)| !n.divisible_by_power_of_2(u))
        .take(LARGE_LIMIT)
    {
        natural_and_u32_non_divisible(n, u);
    }

    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).filter(|&(ref n, u)| !n.divisible_by_power_of_2(u))
        .take(LARGE_LIMIT)
    {
        natural_and_u32_non_divisible(n, u);
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
