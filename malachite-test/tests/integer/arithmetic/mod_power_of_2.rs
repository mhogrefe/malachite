use common::LARGE_LIMIT;
use malachite_base::round::RoundingMode;
use malachite_base::num::{One, PartialOrdAbs, ShrRound, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::small_u32s;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_small_u32,
                                      pairs_of_integer_and_small_u32_var_1,
                                      pairs_of_integer_and_small_u32_var_2,
                                      triples_of_integer_small_u32_and_small_u32};
use std::cmp::min;
use std::str::FromStr;

#[test]
fn test_mod_power_of_2() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.mod_power_of_2_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap().mod_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap().mod_power_of_2_ref(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };

    test("0", 0, "0");
    test("2", 1, "0");
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
    test("2147483647", 30, "1073741823");
    test("2147483647", 31, "2147483647");
    test("2147483647", 32, "2147483647");
    test("2147483648", 30, "0");
    test("2147483648", 31, "0");
    test("2147483648", 32, "2147483648");
    test("2147483649", 30, "1");
    test("2147483649", 31, "1");
    test("2147483649", 32, "2147483649");
    test("4294967295", 31, "2147483647");
    test("4294967295", 32, "4294967295");
    test("4294967295", 33, "4294967295");
    test("4294967296", 31, "0");
    test("4294967296", 32, "0");
    test("4294967296", 33, "4294967296");
    test("4294967297", 31, "1");
    test("4294967297", 32, "1");
    test("4294967297", 33, "4294967297");

    test("-2", 1, "0");
    test("-260", 8, "252");
    test("-1611", 4, "5");
    test("-123", 100, "1267650600228229401496703205253");
    test("-1000000000000", 0, "0");
    test("-1000000000000", 12, "0");
    test("-1000000000001", 12, "4095");
    test("-999999999999", 12, "1");
    test("-1000000000000", 15, "28672");
    test("-1000000000000", 100, "1267650600228229400496703205376");
    test("-1000000000000000000000000", 40, "78903246848");
    test("-1000000000000000000000000", 64, "16442979868502654976");
    test("-2147483647", 30, "1");
    test("-2147483647", 31, "1");
    test("-2147483647", 32, "2147483649");
    test("-2147483648", 30, "0");
    test("-2147483648", 31, "0");
    test("-2147483648", 32, "2147483648");
    test("-2147483649", 30, "1073741823");
    test("-2147483649", 31, "2147483647");
    test("-2147483649", 32, "2147483647");
    test("-4294967295", 31, "1");
    test("-4294967295", 32, "1");
    test("-4294967295", 33, "4294967297");
    test("-4294967296", 31, "0");
    test("-4294967296", 32, "0");
    test("-4294967296", 33, "4294967296");
    test("-4294967297", 31, "2147483647");
    test("-4294967297", 32, "4294967295");
    test("-4294967297", 33, "4294967295");
}

#[test]
fn test_rem_power_of_2() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.rem_power_of_2_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap().rem_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap().rem_power_of_2_ref(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };

    test("0", 0, "0");
    test("2", 1, "0");
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
    test("2147483647", 30, "1073741823");
    test("2147483647", 31, "2147483647");
    test("2147483647", 32, "2147483647");
    test("2147483648", 30, "0");
    test("2147483648", 31, "0");
    test("2147483648", 32, "2147483648");
    test("2147483649", 30, "1");
    test("2147483649", 31, "1");
    test("2147483649", 32, "2147483649");
    test("4294967295", 31, "2147483647");
    test("4294967295", 32, "4294967295");
    test("4294967295", 33, "4294967295");
    test("4294967296", 31, "0");
    test("4294967296", 32, "0");
    test("4294967296", 33, "4294967296");
    test("4294967297", 31, "1");
    test("4294967297", 32, "1");
    test("4294967297", 33, "4294967297");

    test("-2", 1, "0");
    test("-260", 8, "-4");
    test("-1611", 4, "-11");
    test("-123", 100, "-123");
    test("-1000000000000", 0, "0");
    test("-1000000000000", 12, "0");
    test("-1000000000001", 12, "-1");
    test("-999999999999", 12, "-4095");
    test("-1000000000000", 15, "-4096");
    test("-1000000000000", 100, "-1000000000000");
    test("-1000000000000000000000000", 40, "-1020608380928");
    test("-1000000000000000000000000", 64, "-2003764205206896640");
    test("-2147483647", 30, "-1073741823");
    test("-2147483647", 31, "-2147483647");
    test("-2147483647", 32, "-2147483647");
    test("-2147483648", 30, "0");
    test("-2147483648", 31, "0");
    test("-2147483648", 32, "-2147483648");
    test("-2147483649", 30, "-1");
    test("-2147483649", 31, "-1");
    test("-2147483649", 32, "-2147483649");
    test("-4294967295", 31, "-2147483647");
    test("-4294967295", 32, "-4294967295");
    test("-4294967295", 33, "-4294967295");
    test("-4294967296", 31, "0");
    test("-4294967296", 32, "0");
    test("-4294967296", 33, "-4294967296");
    test("-4294967297", 31, "-1");
    test("-4294967297", 32, "-1");
    test("-4294967297", 33, "-4294967297");
}

#[test]
fn test_ceiling_mod_power_of_2() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.ceiling_mod_power_of_2_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap().ceiling_mod_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap().ceiling_mod_power_of_2_ref(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };

    test("0", 0, "0");
    test("2", 1, "0");
    test("260", 8, "-252");
    test("1611", 4, "-5");
    test("123", 100, "-1267650600228229401496703205253");
    test("1000000000000", 0, "0");
    test("1000000000000", 12, "0");
    test("1000000000001", 12, "-4095");
    test("999999999999", 12, "-1");
    test("1000000000000", 15, "-28672");
    test("1000000000000", 100, "-1267650600228229400496703205376");
    test("1000000000000000000000000", 40, "-78903246848");
    test("1000000000000000000000000", 64, "-16442979868502654976");
    test("2147483647", 30, "-1");
    test("2147483647", 31, "-1");
    test("2147483647", 32, "-2147483649");
    test("2147483648", 30, "0");
    test("2147483648", 31, "0");
    test("2147483648", 32, "-2147483648");
    test("2147483649", 30, "-1073741823");
    test("2147483649", 31, "-2147483647");
    test("2147483649", 32, "-2147483647");
    test("4294967295", 31, "-1");
    test("4294967295", 32, "-1");
    test("4294967295", 33, "-4294967297");
    test("4294967296", 31, "0");
    test("4294967296", 32, "0");
    test("4294967296", 33, "-4294967296");
    test("4294967297", 31, "-2147483647");
    test("4294967297", 32, "-4294967295");
    test("4294967297", 33, "-4294967295");

    test("-2", 1, "0");
    test("-260", 8, "-4");
    test("-1611", 4, "-11");
    test("-123", 100, "-123");
    test("-1000000000000", 0, "0");
    test("-1000000000000", 12, "0");
    test("-1000000000001", 12, "-1");
    test("-999999999999", 12, "-4095");
    test("-1000000000000", 15, "-4096");
    test("-1000000000000", 100, "-1000000000000");
    test("-1000000000000000000000000", 40, "-1020608380928");
    test("-1000000000000000000000000", 64, "-2003764205206896640");
    test("-2147483647", 30, "-1073741823");
    test("-2147483647", 31, "-2147483647");
    test("-2147483647", 32, "-2147483647");
    test("-2147483648", 30, "0");
    test("-2147483648", 31, "0");
    test("-2147483648", 32, "-2147483648");
    test("-2147483649", 30, "-1");
    test("-2147483649", 31, "-1");
    test("-2147483649", 32, "-2147483649");
    test("-4294967295", 31, "-2147483647");
    test("-4294967295", 32, "-4294967295");
    test("-4294967295", 33, "-4294967295");
    test("-4294967296", 31, "0");
    test("-4294967296", 32, "0");
    test("-4294967296", 33, "-4294967296");
    test("-4294967297", 31, "-1");
    test("-4294967297", 32, "-1");
    test("-4294967297", 33, "-4294967297");
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
    let integer_and_u32 = |mut n: Integer, u: u32| {
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
    let integer_and_u32_divisible = |n: Integer, u: u32| {
        assert_eq!(n.mod_power_of_2(u), 0);
    };

    // If n is not divisible by 2^u, n.mod_power_of_2(u) != 0
    // If n is not divisible by 2^u, n.mod_power_of_2(u) - n.ceiling_mod_power_of_2(u) == 1 << u
    let integer_and_u32_non_divisible = |n: Integer, u: u32| {
        assert_ne!(n.mod_power_of_2_ref(u), 0);
        assert_eq!(
            n.mod_power_of_2_ref(u).into_integer() - n.ceiling_mod_power_of_2(u),
            Natural::ONE << u
        );
    };

    // n.mod_power_of_2(u).mod_power_of_2(v) == n.mod_power_of_2(min(u, v))
    let integer_and_two_u32s = |n: Integer, u: u32, v: u32| {
        assert_eq!(
            n.mod_power_of_2_ref(u).mod_power_of_2(v),
            n.mod_power_of_2(min(u, v))
        );
    };

    // n.mod_power_of_2(0) == 0
    let one_integer = |n: Integer| {
        assert_eq!(n.mod_power_of_2_ref(0), 0);
    };

    // 0.mod_power_of_2(n) == 0
    let one_u32 = |u: u32| {
        assert_eq!(Integer::ZERO.mod_power_of_2(u), 0);
    };

    for (n, u) in pairs_of_integer_and_small_u32(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32_var_1(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        integer_and_u32_divisible(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32_var_1(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        integer_and_u32_divisible(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32_var_2(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        integer_and_u32_non_divisible(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32_var_2(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        integer_and_u32_non_divisible(n, u);
    }

    for (n, u, v) in
        triples_of_integer_small_u32_and_small_u32(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        integer_and_two_u32s(n, u, v);
    }

    for (n, u, v) in
        triples_of_integer_small_u32_and_small_u32(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        integer_and_two_u32s(n, u, v);
    }

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in small_u32s(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u32(n);
    }

    for n in small_u32s(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32(n);
    }
}

#[test]
fn rem_power_of_2_properties() {
    // n.rem_power_of_2_assign(u); n is valid.
    // n.rem_power_of_2(u) is valid.
    // n.rem_power_of_2_ref(u) is valid.
    // n.rem_power_of_2_assign(u), n.rem_power_of_2(u), and n.rem_power_of_2_ref(u) give the same
    //      result.
    // (n.shr_round(u, Down) << u) + n.rem_power_of_2(u) == n
    // n.rem_power_of_2(u).lt_abs(1 << u)
    // (n.rem_power_of_2(u) == 0) == n.divisible_by_power_of_2(u)
    // n.rem_power_of_2(u).rem_power_of_2(u) == n.rem_power_of_2(u)
    // n.rem_power_of_2(u).abs() == n.abs().mod_power_of_2(u)
    let integer_and_u32 = |mut n: Integer, u: u32| {
        let old_n = n.clone();
        n.rem_power_of_2_assign(u);
        assert!(n.is_valid());

        let n2 = old_n.clone();
        let result = n2.rem_power_of_2_ref(u);
        assert_eq!(result, n);
        assert!(result.is_valid());
        let result = n2.rem_power_of_2(u);
        assert!(result.is_valid());
        assert_eq!(result, n);

        assert_eq!(
            (((&old_n).shr_round(u, RoundingMode::Down) << u) + &n),
            old_n
        );
        assert!(n.lt_abs(&(Natural::ONE << u)));
        assert_eq!(n == 0, old_n.divisible_by_power_of_2(u));
        assert_eq!(n.rem_power_of_2_ref(u), n);
        assert_eq!(n.abs_ref(), old_n.abs().mod_power_of_2(u));
    };

    // If n is divisible by 2^u, n.rem_power_of_2(u) == 0
    let integer_and_u32_divisible = |n: Integer, u: u32| {
        assert_eq!(n.rem_power_of_2(u), 0);
    };

    // If n is not divisible by 2^u, n.rem_power_of_2(u) != 0
    // If n is not divisible by 2^u, n.rem_power_of_2(u).sign() == n.sign()
    let integer_and_u32_non_divisible = |n: Integer, u: u32| {
        assert_ne!(n.rem_power_of_2_ref(u), 0);
        assert_eq!(n.rem_power_of_2_ref(u).sign(), n.sign());
    };

    // n.rem_power_of_2(u).rem_power_of_2(v) == n.rem_power_of_2(min(u, v))
    let integer_and_two_u32s = |n: Integer, u: u32, v: u32| {
        assert_eq!(
            n.rem_power_of_2_ref(u).rem_power_of_2(v),
            n.rem_power_of_2(min(u, v))
        );
    };

    // n.rem_power_of_2(0) == 0
    let one_integer = |n: Integer| {
        assert_eq!(n.rem_power_of_2_ref(0), 0);
    };

    // 0.rem_power_of_2(n) == 0
    let one_u32 = |u: u32| {
        assert_eq!(Integer::ZERO.rem_power_of_2(u), 0);
    };

    for (n, u) in pairs_of_integer_and_small_u32(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32_var_1(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        integer_and_u32_divisible(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32_var_1(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        integer_and_u32_divisible(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32_var_2(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        integer_and_u32_non_divisible(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32_var_2(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        integer_and_u32_non_divisible(n, u);
    }

    for (n, u, v) in
        triples_of_integer_small_u32_and_small_u32(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        integer_and_two_u32s(n, u, v);
    }

    for (n, u, v) in
        triples_of_integer_small_u32_and_small_u32(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        integer_and_two_u32s(n, u, v);
    }

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in small_u32s(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u32(n);
    }

    for n in small_u32s(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32(n);
    }
}

#[test]
fn ceiling_mod_power_of_2_properties() {
    // n.ceiling_mod_power_of_2_assign(u); n is valid.
    // n.ceiling_mod_power_of_2(u) is valid.
    // n.ceiling_mod_power_of_2_ref(u) is valid.
    // n.ceiling_mod_power_of_2_assign(u), n.ceiling_mod_power_of_2(u), and
    //      n.ceiling_mod_power_of_2_ref(u) give the same result.
    // (n.shr_round(u, Ceiling) << u) + n.ceiling_mod_power_of_2(u) == n
    // 0 < -(n.ceiling_mod_power_of_2(u)) < 1 << u
    // (n.ceiling_mod_power_of_2(u) == 0) == n.divisible_by_power_of_2(u)
    // -(n.ceiling_mod_power_of_2(u)) == (-n).mod_power_of_2(u)
    let integer_and_u32 = |mut n: Integer, u: u32| {
        let old_n = n.clone();
        n.ceiling_mod_power_of_2_assign(u);
        assert!(n.is_valid());

        let n2 = old_n.clone();
        let result = n2.ceiling_mod_power_of_2_ref(u);
        assert_eq!(result, n);
        assert!(result.is_valid());
        let result = n2.ceiling_mod_power_of_2(u);
        assert!(result.is_valid());
        assert_eq!(result, n);

        assert_eq!(
            (((&old_n).shr_round(u, RoundingMode::Ceiling) << u) + &n),
            old_n
        );
        assert!(n <= 0);
        assert!(-&n <= Natural::ONE << u);
        assert_eq!(n == 0, old_n.divisible_by_power_of_2(u));
        assert_eq!(-n, (-old_n).mod_power_of_2(u));
    };

    // If n is divisible by 2^u, n.ceiling_mod_power_of_2(u) == 0
    let integer_and_u32_divisible = |n: Integer, u: u32| {
        assert_eq!(n.ceiling_mod_power_of_2(u), 0);
    };

    // If n is not divisible by 2^u, n.ceiling_mod_power_of_2(u) != 0
    let integer_and_u32_non_divisible = |n: Integer, u: u32| {
        assert_ne!(n.ceiling_mod_power_of_2_ref(u), 0);
    };

    // n.ceiling_mod_power_of_2(0) == 0
    let one_integer = |n: Integer| {
        assert_eq!(n.ceiling_mod_power_of_2_ref(0), 0);
    };

    // 0.ceiling_mod_power_of_2(n) == 0
    let one_u32 = |u: u32| {
        assert_eq!(Integer::ZERO.ceiling_mod_power_of_2(u), 0);
    };

    for (n, u) in pairs_of_integer_and_small_u32(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32_var_1(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        integer_and_u32_divisible(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32_var_1(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        integer_and_u32_divisible(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32_var_2(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        integer_and_u32_non_divisible(n, u);
    }

    for (n, u) in pairs_of_integer_and_small_u32_var_2(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        integer_and_u32_non_divisible(n, u);
    }

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in small_u32s(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u32(n);
    }

    for n in small_u32s(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32(n);
    }
}
