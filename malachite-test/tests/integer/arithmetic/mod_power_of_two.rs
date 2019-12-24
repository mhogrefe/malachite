use std::cmp::min;
use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    Abs, CeilingModPowerOfTwo, CeilingModPowerOfTwoAssign, DivisibleByPowerOfTwo, ModPowerOfTwo,
    ModPowerOfTwoAssign, RemPowerOfTwo, RemPowerOfTwoAssign, ShrRound, Sign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_signed_and_small_unsigned, unsigneds};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_small_unsigned, pairs_of_integer_and_small_unsigned_var_1,
    pairs_of_integer_and_small_unsigned_var_2, triples_of_integer_integer_and_small_unsigned,
    triples_of_integer_small_unsigned_and_small_unsigned,
};
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;

#[test]
fn test_mod_power_of_two() {
    let test = |u, v: u64, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.mod_power_of_two_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap().mod_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Integer::from_str(u).unwrap()).mod_power_of_two(v);
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
    let test = |u, v: u64, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.rem_power_of_two_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap().rem_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Integer::from_str(u).unwrap()).rem_power_of_two(v);
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
    let test = |u, v: u64, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.ceiling_mod_power_of_two_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap().ceiling_mod_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Integer::from_str(u).unwrap()).ceiling_mod_power_of_two(v);
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

        let result_alt = (&n).mod_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);
        let result_alt = n.clone().mod_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!((n >> u << u) + &result, *n);
        assert!(result < (Natural::ONE << u));
        assert_eq!(result == 0 as Limb, n.divisible_by_power_of_two(u));
        assert_eq!((&result).mod_power_of_two(u), result);
        assert_eq!(n & ((Integer::ONE << u) - Integer::ONE), result);
    });

    test_properties(
        triples_of_integer_integer_and_small_unsigned,
        |&(ref x, ref y, u)| {
            let xm = Integer::from(x.mod_power_of_two(u));
            let ym = Integer::from(y.mod_power_of_two(u));
            assert_eq!((x + y).mod_power_of_two(u), (&xm + &ym).mod_power_of_two(u));
            assert_eq!((x - y).mod_power_of_two(u), (&xm - &ym).mod_power_of_two(u));
            assert_eq!((x * y).mod_power_of_two(u), (xm * ym).mod_power_of_two(u));
        },
    );

    test_properties(pairs_of_integer_and_small_unsigned_var_1, |&(ref n, u)| {
        assert_eq!(n.mod_power_of_two(u), 0 as Limb);
    });

    test_properties(pairs_of_integer_and_small_unsigned_var_2, |&(ref n, u)| {
        assert_ne!(n.mod_power_of_two(u), 0 as Limb);
        assert_eq!(
            Integer::from(n.mod_power_of_two(u)) - n.ceiling_mod_power_of_two(u),
            Natural::ONE << u
        );
    });

    test_properties(
        triples_of_integer_small_unsigned_and_small_unsigned,
        |&(ref n, u, v)| {
            assert_eq!(
                n.mod_power_of_two(u).mod_power_of_two(v),
                n.mod_power_of_two(min(u, v))
            );
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.mod_power_of_two(0), 0 as Limb);
    });

    test_properties(unsigneds, |&u| {
        assert_eq!(Integer::ZERO.mod_power_of_two(u), 0 as Limb);
    });
}

#[test]
fn rem_power_of_two_properties() {
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n.rem_power_of_two_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let result_alt = (&n).rem_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);
        let result_alt = n.clone().rem_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(((n.shr_round(u, RoundingMode::Down) << u) + &result), *n);
        assert!(result.lt_abs(&(Natural::ONE << u)));
        assert_eq!(result == 0 as Limb, n.divisible_by_power_of_two(u));
        assert_eq!((&result).rem_power_of_two(u), result);
        assert_eq!(n.abs().mod_power_of_two(u), result.abs());
    });

    test_properties(pairs_of_integer_and_small_unsigned_var_1, |&(ref n, u)| {
        assert_eq!(n.rem_power_of_two(u), 0 as Limb);
    });

    test_properties(pairs_of_integer_and_small_unsigned_var_2, |&(ref n, u)| {
        assert_ne!(n.rem_power_of_two(u), 0 as Limb);
        assert_eq!(n.rem_power_of_two(u).sign(), n.sign());
    });

    test_properties(
        triples_of_integer_small_unsigned_and_small_unsigned,
        |&(ref n, u, v)| {
            assert_eq!(
                n.rem_power_of_two(u).rem_power_of_two(v),
                n.rem_power_of_two(min(u, v))
            );
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.rem_power_of_two(0), 0 as Limb);
    });

    test_properties(unsigneds, |&u| {
        assert_eq!(Integer::ZERO.rem_power_of_two(u), 0 as Limb);
    });
}

#[test]
fn ceiling_mod_power_of_two_properties() {
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n.ceiling_mod_power_of_two_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let result_alt = (&n).ceiling_mod_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);
        let result_alt = n.clone().ceiling_mod_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(((n.shr_round(u, RoundingMode::Ceiling) << u) + &result), *n);
        assert!(result <= 0 as Limb);
        assert!(-&result <= Natural::ONE << u);
        assert_eq!(result == 0 as Limb, n.divisible_by_power_of_two(u));
        assert_eq!((-n).mod_power_of_two(u), -result);
    });

    test_properties(
        triples_of_integer_integer_and_small_unsigned,
        |&(ref x, ref y, u)| {
            let xm = Integer::from(x.mod_power_of_two(u));
            let ym = Integer::from(y.mod_power_of_two(u));
            assert_eq!(
                (x + y).ceiling_mod_power_of_two(u),
                Integer::from(&xm + &ym).ceiling_mod_power_of_two(u)
            );
            assert_eq!(
                (x - y).ceiling_mod_power_of_two(u),
                Integer::from(&xm - &ym).ceiling_mod_power_of_two(u)
            );
            assert_eq!(
                (x * y).ceiling_mod_power_of_two(u),
                Integer::from(xm * ym).ceiling_mod_power_of_two(u)
            );
        },
    );

    test_properties(pairs_of_integer_and_small_unsigned_var_1, |&(ref n, u)| {
        assert_eq!(n.ceiling_mod_power_of_two(u), 0 as Limb);
    });

    test_properties(pairs_of_integer_and_small_unsigned_var_2, |&(ref n, u)| {
        assert_ne!(n.ceiling_mod_power_of_two(u), 0 as Limb);
    });

    test_properties(integers, |n| {
        assert_eq!(n.ceiling_mod_power_of_two(0), 0 as Limb);
    });

    test_properties(unsigneds, |&u| {
        assert_eq!(Integer::ZERO.ceiling_mod_power_of_two(u), 0 as Limb);
    });

    test_properties(
        pairs_of_signed_and_small_unsigned::<SignedLimb, u64>,
        |&(i, pow)| {
            assert_eq!(
                i.divisible_by_power_of_two(pow),
                Integer::from(i).divisible_by_power_of_two(pow)
            );
        },
    );

    test_properties(
        pairs_of_natural_and_small_unsigned::<u64>,
        |&(ref n, pow)| {
            assert_eq!(
                n.divisible_by_power_of_two(pow),
                Integer::from(n).divisible_by_power_of_two(pow)
            );
        },
    );
}
