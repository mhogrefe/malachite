use common::test_properties;
use malachite_base::num::{Abs, One, PartialOrdAbs, ShrRound, Zero};
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_small_unsigned, pairs_of_integer_and_small_unsigned_var_1,
    pairs_of_integer_and_small_unsigned_var_2,
    triples_of_integer_small_unsigned_and_small_unsigned,
};
use std::cmp::min;
use std::str::FromStr;

#[test]
fn test_mod_power_of_two() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.mod_power_of_two_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap().mod_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap().mod_power_of_two_ref(v);
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
fn test_rem_power_of_two() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.rem_power_of_two_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap().rem_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap().rem_power_of_two_ref(v);
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
fn test_ceiling_mod_power_of_two() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.ceiling_mod_power_of_two_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap().ceiling_mod_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u)
            .unwrap()
            .ceiling_mod_power_of_two_ref(v);
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
fn mod_power_of_two_properties() {
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n.mod_power_of_two_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let result_alt = n.mod_power_of_two_ref(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);
        let result_alt = n.clone().mod_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!((n >> u << u) + &result, *n);
        assert!(result < (Natural::ONE << u));
        assert_eq!(result == 0, n.divisible_by_power_of_two(u));
        assert_eq!(result.mod_power_of_two_ref(u), result);
    });

    test_properties(pairs_of_integer_and_small_unsigned_var_1, |&(ref n, u)| {
        assert_eq!(n.mod_power_of_two_ref(u), 0);
    });

    test_properties(pairs_of_integer_and_small_unsigned_var_2, |&(ref n, u)| {
        assert_ne!(n.mod_power_of_two_ref(u), 0);
        assert_eq!(
            Integer::from(n.mod_power_of_two_ref(u)) - n.ceiling_mod_power_of_two_ref(u),
            Natural::ONE << u
        );
    });

    test_properties(
        triples_of_integer_small_unsigned_and_small_unsigned,
        |&(ref n, u, v)| {
            assert_eq!(
                n.mod_power_of_two_ref(u).mod_power_of_two(v),
                n.mod_power_of_two_ref(min(u, v))
            );
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.mod_power_of_two_ref(0), 0);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(Integer::ZERO.mod_power_of_two(u), 0);
    });
}

#[test]
fn rem_power_of_two_properties() {
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n.rem_power_of_two_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let result_alt = n.rem_power_of_two_ref(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);
        let result_alt = n.clone().rem_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(((n.shr_round(u, RoundingMode::Down) << u) + &result), *n);
        assert!(result.lt_abs(&(Natural::ONE << u)));
        assert_eq!(result == 0, n.divisible_by_power_of_two(u));
        assert_eq!(result.rem_power_of_two_ref(u), result);
        assert_eq!(n.abs().mod_power_of_two(u), result.abs());
    });

    test_properties(pairs_of_integer_and_small_unsigned_var_1, |&(ref n, u)| {
        assert_eq!(n.rem_power_of_two_ref(u), 0);
    });

    test_properties(pairs_of_integer_and_small_unsigned_var_2, |&(ref n, u)| {
        assert_ne!(n.rem_power_of_two_ref(u), 0);
        assert_eq!(n.rem_power_of_two_ref(u).sign(), n.sign());
    });

    test_properties(
        triples_of_integer_small_unsigned_and_small_unsigned,
        |&(ref n, u, v)| {
            assert_eq!(
                n.rem_power_of_two_ref(u).rem_power_of_two(v),
                n.rem_power_of_two_ref(min(u, v))
            );
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.rem_power_of_two_ref(0), 0);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(Integer::ZERO.rem_power_of_two(u), 0);
    });
}

#[test]
fn ceiling_mod_power_of_two_properties() {
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n.ceiling_mod_power_of_two_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let result_alt = n.ceiling_mod_power_of_two_ref(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);
        let result_alt = n.clone().ceiling_mod_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(((n.shr_round(u, RoundingMode::Ceiling) << u) + &result), *n);
        assert!(result <= 0);
        assert!(-&result <= Natural::ONE << u);
        assert_eq!(result == 0, n.divisible_by_power_of_two(u));
        assert_eq!((-n).mod_power_of_two(u), -result);
    });

    test_properties(pairs_of_integer_and_small_unsigned_var_1, |&(ref n, u)| {
        assert_eq!(n.ceiling_mod_power_of_two_ref(u), 0);
    });

    test_properties(pairs_of_integer_and_small_unsigned_var_2, |&(ref n, u)| {
        assert_ne!(n.ceiling_mod_power_of_two_ref(u), 0);
    });

    test_properties(integers, |n| {
        assert_eq!(n.ceiling_mod_power_of_two_ref(0), 0);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(Integer::ZERO.ceiling_mod_power_of_two(u), 0);
    });
}
