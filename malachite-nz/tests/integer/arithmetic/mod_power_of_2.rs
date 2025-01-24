// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    Abs, CeilingModPowerOf2, CeilingModPowerOf2Assign, DivisibleByPowerOf2, ModPowerOf2,
    ModPowerOf2Assign, NegModPowerOf2, PowerOf2, RemPowerOf2, RemPowerOf2Assign, ShrRound, Sign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_1, signed_unsigned_pair_gen_var_10,
    signed_unsigned_pair_gen_var_11, unsigned_gen,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_gen, integer_integer_unsigned_triple_gen_var_1, integer_unsigned_pair_gen_var_2,
    integer_unsigned_pair_gen_var_4, integer_unsigned_pair_gen_var_5,
    integer_unsigned_unsigned_triple_gen_var_3, natural_unsigned_pair_gen_var_4,
};
use std::cmp::min;
use std::str::FromStr;

#[test]
fn test_mod_power_of_2() {
    let test = |s, v: u64, out| {
        let u = Integer::from_str(s).unwrap();

        let mut n = u.clone();
        n.mod_power_of_2_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_power_of_2(v);
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
    let test = |s, v: u64, out| {
        let u = Integer::from_str(s).unwrap();

        let mut n = u.clone();
        n.rem_power_of_2_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().rem_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).rem_power_of_2(v);
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
    let test = |s, v: u64, out| {
        let u = Integer::from_str(s).unwrap();

        let mut n = u.clone();
        n.ceiling_mod_power_of_2_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().ceiling_mod_power_of_2(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).ceiling_mod_power_of_2(v);
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
    integer_unsigned_pair_gen_var_2().test_properties(|(n, u)| {
        let mut mut_n = n.clone();
        mut_n.mod_power_of_2_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let result_alt = (&n).mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);
        let result_alt = n.clone().mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!((&n >> u << u) + &result, n);
        assert!(result < Natural::power_of_2(u));
        assert_eq!(result == 0, n.divisible_by_power_of_2(u));
        assert_eq!((&result).mod_power_of_2(u), result);
        assert_eq!(n & Integer::low_mask(u), result);
    });

    integer_integer_unsigned_triple_gen_var_1().test_properties(|(ref x, ref y, u)| {
        let xm = Integer::from(x.mod_power_of_2(u));
        let ym = Integer::from(y.mod_power_of_2(u));
        assert_eq!((x + y).mod_power_of_2(u), (&xm + &ym).mod_power_of_2(u));
        assert_eq!((x - y).mod_power_of_2(u), (&xm - &ym).mod_power_of_2(u));
        assert_eq!((x * y).mod_power_of_2(u), (xm * ym).mod_power_of_2(u));
    });

    integer_unsigned_pair_gen_var_4().test_properties(|(n, u)| {
        assert_eq!(n.mod_power_of_2(u), 0);
    });

    integer_unsigned_pair_gen_var_5().test_properties(|(n, u)| {
        assert_ne!((&n).mod_power_of_2(u), 0);
        assert_eq!(
            Integer::from((&n).mod_power_of_2(u)) - n.ceiling_mod_power_of_2(u),
            Natural::power_of_2(u)
        );
    });

    integer_unsigned_unsigned_triple_gen_var_3().test_properties(|(n, u, v)| {
        assert_eq!(
            (&n).mod_power_of_2(u).mod_power_of_2(v),
            n.mod_power_of_2(min(u, v))
        );
    });

    signed_unsigned_pair_gen_var_10::<SignedLimb>().test_properties(|(i, pow)| {
        assert_eq!(i.mod_power_of_2(pow), Integer::from(i).mod_power_of_2(pow));
    });

    natural_unsigned_pair_gen_var_4().test_properties(|(n, pow)| {
        assert_eq!(
            (&n).mod_power_of_2(pow),
            Integer::from(n).mod_power_of_2(pow)
        );
    });

    integer_gen().test_properties(|n| {
        assert_eq!(n.mod_power_of_2(0), 0);
    });

    unsigned_gen().test_properties(|u| {
        assert_eq!(Integer::ZERO.mod_power_of_2(u), 0);
    });
}

#[test]
fn rem_power_of_2_properties() {
    integer_unsigned_pair_gen_var_2().test_properties(|(n, u)| {
        let mut mut_n = n.clone();
        mut_n.rem_power_of_2_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let result_alt = (&n).rem_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);
        let result_alt = n.clone().rem_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert!(result.le_abs(&n));
        assert_eq!((((&n).shr_round(u, Down).0 << u) + &result), n);
        assert!(result.lt_abs(&Natural::power_of_2(u)));
        assert_eq!(result == 0, (&n).divisible_by_power_of_2(u));
        assert_eq!((&result).rem_power_of_2(u), result);
        assert_eq!(n.abs().mod_power_of_2(u), result.abs());
    });

    integer_unsigned_pair_gen_var_4().test_properties(|(n, u)| {
        assert_eq!(n.rem_power_of_2(u), 0);
    });

    integer_unsigned_pair_gen_var_5().test_properties(|(n, u)| {
        assert_ne!((&n).rem_power_of_2(u), 0);
        assert_eq!((&n).rem_power_of_2(u).sign(), n.sign());
    });

    integer_unsigned_unsigned_triple_gen_var_3().test_properties(|(n, u, v)| {
        assert_eq!(
            (&n).rem_power_of_2(u).rem_power_of_2(v),
            n.rem_power_of_2(min(u, v))
        );
    });

    signed_unsigned_pair_gen_var_1::<SignedLimb, u64>().test_properties(|(i, pow)| {
        assert_eq!(i.rem_power_of_2(pow), Integer::from(i).rem_power_of_2(pow));
    });

    natural_unsigned_pair_gen_var_4().test_properties(|(n, pow)| {
        assert_eq!(
            (&n).rem_power_of_2(pow),
            Integer::from(n).rem_power_of_2(pow)
        );
    });

    integer_gen().test_properties(|n| {
        assert_eq!(n.rem_power_of_2(0), 0);
    });

    unsigned_gen().test_properties(|u| {
        assert_eq!(Integer::ZERO.rem_power_of_2(u), 0);
    });
}

#[test]
fn ceiling_mod_power_of_2_properties() {
    integer_unsigned_pair_gen_var_2().test_properties(|(n, u)| {
        let mut mut_n = n.clone();
        mut_n.ceiling_mod_power_of_2_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let result_alt = (&n).ceiling_mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);
        let result_alt = n.clone().ceiling_mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!((((&n).shr_round(u, Ceiling).0 << u) + &result), n);
        assert!(result <= 0);
        assert!(-&result <= Natural::power_of_2(u));
        assert_eq!(result == 0, (&n).divisible_by_power_of_2(u));
        assert_eq!((-n).mod_power_of_2(u), -result);
    });

    integer_integer_unsigned_triple_gen_var_1().test_properties(|(ref x, ref y, u)| {
        let xm = Integer::from(x.mod_power_of_2(u));
        let ym = Integer::from(y.mod_power_of_2(u));
        assert_eq!(
            (x + y).ceiling_mod_power_of_2(u),
            (&xm + &ym).ceiling_mod_power_of_2(u)
        );
        assert_eq!(
            (x - y).ceiling_mod_power_of_2(u),
            (&xm - &ym).ceiling_mod_power_of_2(u)
        );
        assert_eq!(
            (x * y).ceiling_mod_power_of_2(u),
            (xm * ym).ceiling_mod_power_of_2(u)
        );
    });

    integer_unsigned_pair_gen_var_4().test_properties(|(n, u)| {
        assert_eq!(n.ceiling_mod_power_of_2(u), 0);
    });

    integer_unsigned_pair_gen_var_5().test_properties(|(n, u)| {
        assert_ne!(n.ceiling_mod_power_of_2(u), 0);
    });

    signed_unsigned_pair_gen_var_11::<Limb, SignedLimb>().test_properties(|(i, pow)| {
        assert_eq!(
            i.ceiling_mod_power_of_2(pow),
            Integer::from(i).ceiling_mod_power_of_2(pow)
        );
    });

    natural_unsigned_pair_gen_var_4().test_properties(|(n, pow)| {
        assert_eq!(
            -(&n).neg_mod_power_of_2(pow),
            Integer::from(n).ceiling_mod_power_of_2(pow)
        );
    });

    integer_gen().test_properties(|n| {
        assert_eq!(n.ceiling_mod_power_of_2(0), 0);
    });

    unsigned_gen().test_properties(|u| {
        assert_eq!(Integer::ZERO.ceiling_mod_power_of_2(u), 0);
    });
}
