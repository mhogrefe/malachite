use common::LARGE_LIMIT;
use malachite_base::round::RoundingMode;
use malachite_base::num::{One, ShrRound, Zero};
use malachite_nz::natural::Natural;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_small_u32,
                                      pairs_of_natural_and_small_u32_var_1,
                                      pairs_of_natural_and_small_u32_var_2,
                                      triples_of_natural_small_u32_and_small_u32};
use std::cmp::min;
use std::str::FromStr;

#[test]
fn test_mod_power_of_2() {
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_2_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().mod_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().mod_power_of_2_ref(v);
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
    test("4294967295", 31, "2147483647");
    test("4294967295", 32, "4294967295");
    test("4294967295", 33, "4294967295");
    test("4294967296", 31, "0");
    test("4294967296", 32, "0");
    test("4294967296", 33, "4294967296");
    test("4294967297", 31, "1");
    test("4294967297", 32, "1");
    test("4294967297", 33, "4294967297");
}

#[test]
fn test_neg_mod_power_of_2() {
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.neg_mod_power_of_2_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().neg_mod_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().neg_mod_power_of_2_ref(v);
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
    test("4294967295", 31, "1");
    test("4294967295", 32, "1");
    test("4294967295", 33, "4294967297");
    test("4294967296", 31, "0");
    test("4294967296", 32, "0");
    test("4294967296", 33, "4294967296");
    test("4294967297", 31, "2147483647");
    test("4294967297", 32, "4294967295");
    test("4294967297", 33, "4294967295");
}

#[test]
fn mod_power_of_2_properties() {
    // n.mod_power_of_2_assign(u); n is valid.
    // n.mod_power_of_2(u) is valid.
    // n.mod_power_of_2_ref(u) is valid.
    // n.mod_power_of_2_assign(u), n.mod_power_of_2(u), and n.mod_power_of_2_ref(u) give the same
    //      result.
    // (n >> u << u) + n.mod_power_of_2(u) == n
    // n.mod_power_of_2(u) < (1 << u)
    // (n.mod_power_of_2(u) == 0) == n.divisible_by_power_of_2(u)
    // n.mod_power_of_2(u).mod_power_of_2(u) == n.mod_power_of_2(u)
    let natural_and_u32 = |mut n: Natural, u: u32| {
        let old_n = n.clone();
        n.mod_power_of_2_assign(u);
        assert!(n.is_valid());

        let n2 = old_n.clone();
        let result = n2.mod_power_of_2_ref(u);
        assert_eq!(result, n);
        assert!(result.is_valid());
        let result = n2.mod_power_of_2(u);
        assert!(result.is_valid());
        assert_eq!(result, n);

        assert_eq!((&old_n >> u << u) + &n, old_n);
        assert!(n < (Natural::ONE << u));
        assert_eq!(n == 0, old_n.divisible_by_power_of_2(u));
        assert_eq!(n.mod_power_of_2_ref(u), n);
    };

    // If n is divisible by 2^u, n.mod_power_of_2(u) == 0
    let natural_and_u32_divisible = |n: Natural, u: u32| {
        assert_eq!(n.mod_power_of_2(u), 0);
    };

    // If n is not divisible by 2^u, n.mod_power_of_2(u) != 0
    // If n is not divisible by 2^u, n.mod_power_of_2(u) + n.neg_mod_power_of_2(u) == 1 << u
    let natural_and_u32_non_divisible = |n: Natural, u: u32| {
        assert_ne!(n.mod_power_of_2_ref(u), 0);
        assert_eq!(
            n.mod_power_of_2_ref(u) + n.neg_mod_power_of_2(u),
            Natural::ONE << u
        );
    };

    // n.mod_power_of_2(u).mod_power_of_2(v) == n.mod_power_of_2(min(u, v))
    let natural_and_two_u32s = |n: Natural, u: u32, v: u32| {
        assert_eq!(
            n.mod_power_of_2_ref(u).mod_power_of_2(v),
            n.mod_power_of_2(min(u, v))
        );
    };

    // n.mod_power_of_2(0) == 0
    let one_natural = |n: Natural| {
        assert_eq!(n.mod_power_of_2_ref(0), 0);
    };

    // 0.mod_power_of_2(n) == 0
    let one_u32 = |u: u32| {
        assert_eq!(Natural::ZERO.mod_power_of_2(u), 0);
    };

    for (n, u) in pairs_of_natural_and_small_u32(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in pairs_of_natural_and_small_u32(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in pairs_of_natural_and_small_u32_var_1(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        natural_and_u32_divisible(n, u);
    }

    for (n, u) in pairs_of_natural_and_small_u32_var_1(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        natural_and_u32_divisible(n, u);
    }

    for (n, u) in pairs_of_natural_and_small_u32_var_2(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        natural_and_u32_non_divisible(n, u);
    }

    for (n, u) in pairs_of_natural_and_small_u32_var_2(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        natural_and_u32_non_divisible(n, u);
    }

    for (n, u, v) in
        triples_of_natural_small_u32_and_small_u32(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        natural_and_two_u32s(n, u, v);
    }

    for (n, u, v) in
        triples_of_natural_small_u32_and_small_u32(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        natural_and_two_u32s(n, u, v);
    }

    for n in naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in unsigneds(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u32(n);
    }

    for n in unsigneds(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32(n);
    }
}

#[test]
fn neg_mod_power_of_2_properties() {
    // n.neg_mod_power_of_2_assign(u); n is valid.
    // n.neg_mod_power_of_2(u) is valid.
    // n.neg_mod_power_of_2_ref(u) is valid.
    // n.neg_mod_power_of_2_assign(u), n.neg_mod_power_of_2(u), and (&n).neg_mod_power_of_2(u) give
    //      the same result.
    // (n.shr_round(u, Ceiling) << u) - n.neg_mod_power_of_2(u) == n
    // n.neg_mod_power_of_2(u) < (1 << u)
    // (n.neg_mod_power_of_2(u) == 0) == n.divisible_by_power_of_2(u)
    // n.neg_mod_power_of_2(u).neg_mod_power_of_2(u) == n.mod_power_of_2(u)
    // n.neg_mod_power_of_2(u) = (-n).mod_power_of_2(u)
    let natural_and_u32 = |mut n: Natural, u: u32| {
        let old_n = n.clone();
        n.neg_mod_power_of_2_assign(u);
        assert!(n.is_valid());

        let n2 = old_n.clone();
        let result = n2.neg_mod_power_of_2_ref(u);
        assert_eq!(result, n);
        assert!(result.is_valid());
        let result = n2.neg_mod_power_of_2(u);
        assert!(result.is_valid());
        assert_eq!(result, n);

        assert_eq!(
            (((&old_n).shr_round(u, RoundingMode::Ceiling) << u) - &n),
            Some(old_n.clone())
        );
        assert!(n < (Natural::ONE << u));
        assert_eq!(n == 0, old_n.divisible_by_power_of_2(u));
        assert_eq!(n.neg_mod_power_of_2_ref(u), old_n.mod_power_of_2_ref(u));
        assert_eq!(n, (-old_n).mod_power_of_2(u));
    };

    // If n is divisible by 2^u, n.neg_mod_power_of_2(u) == 0
    let natural_and_u32_divisible = |n: Natural, u: u32| {
        assert_eq!(n.neg_mod_power_of_2(u), 0);
    };

    let natural_and_u32_non_divisible = |n: Natural, u: u32| {
        let m = n.neg_mod_power_of_2_ref(u);
        assert_ne!(m, 0);
        assert_eq!(((((&n >> u) + 1) << u) - &m), Some(n.clone()));
        assert_eq!(n.mod_power_of_2(u) + m, Natural::ONE << u);
    };

    // n.neg_mod_power_of_2(0) == 0
    let one_natural = |n: Natural| {
        assert_eq!(n.neg_mod_power_of_2_ref(0), 0);
    };

    // 0.neg_mod_power_of_2(n) == 0
    let one_u32 = |u: u32| {
        assert_eq!(Natural::ZERO.neg_mod_power_of_2(u), 0);
    };

    for (n, u) in pairs_of_natural_and_small_u32(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in pairs_of_natural_and_small_u32(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in pairs_of_natural_and_small_u32_var_1(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        natural_and_u32_divisible(n, u);
    }

    for (n, u) in pairs_of_natural_and_small_u32_var_1(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        natural_and_u32_divisible(n, u);
    }

    for (n, u) in pairs_of_natural_and_small_u32_var_2(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        natural_and_u32_non_divisible(n, u);
    }

    for (n, u) in pairs_of_natural_and_small_u32_var_2(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        natural_and_u32_non_divisible(n, u);
    }

    for n in naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in unsigneds(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u32(n);
    }

    for n in unsigneds(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32(n);
    }
}
