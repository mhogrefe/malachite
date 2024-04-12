// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivExact, DivisibleBy, Gcd, Lcm, LcmAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_pair_gen_var_34;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen, natural_pair_gen, natural_triple_gen};
use num::BigUint;
use num::Integer as rug_integer;
use std::str::FromStr;

#[test]
fn test_lcm() {
    let test = |s, t, out| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        let mut n = u.clone();
        n.lcm_assign(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.lcm_assign(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().lcm(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).lcm(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().lcm(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).lcm(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigUint::from_str(s)
            .unwrap()
            .lcm(&BigUint::from_str(t).unwrap());
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(s)
            .unwrap()
            .lcm(&rug::Integer::from_str(t).unwrap());
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "6", "0");
    test("6", "0", "0");
    test("1", "6", "6");
    test("6", "1", "6");
    test("8", "12", "24");
    test("54", "24", "216");
    test("42", "56", "168");
    test("48", "18", "144");
    test("3", "5", "15");
    test("12", "60", "60");
    test("12", "90", "180");
    test(
        "12345678987654321",
        "98765432123456789",
        "1219326320073159566072245112635269",
    );
    test(
        "12345678987654321",
        "98765432123456827",
        "32954765407382703654271530905391",
    );
}

#[test]
fn lcm_properties() {
    natural_pair_gen().test_properties(|(x, y)| {
        let lcm_val_val = x.clone().lcm(y.clone());
        let lcm_val_ref = x.clone().lcm(&y);
        let lcm_ref_val = (&x).lcm(y.clone());
        let lcm = (&x).lcm(&y);
        assert!(lcm_val_val.is_valid());
        assert!(lcm_val_ref.is_valid());
        assert!(lcm_ref_val.is_valid());
        assert!(lcm.is_valid());
        assert_eq!(lcm_val_val, lcm);
        assert_eq!(lcm_val_ref, lcm);
        assert_eq!(lcm_ref_val, lcm);

        let mut mut_x = x.clone();
        mut_x.lcm_assign(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, lcm);

        let mut mut_x = x.clone();
        mut_x.lcm_assign(&y);
        assert_eq!(mut_x, lcm);
        assert!(mut_x.is_valid());

        assert_eq!(
            Natural::from(&(BigUint::from(&x).lcm(&BigUint::from(&y)))),
            lcm
        );
        assert_eq!(
            Natural::exact_from(&(rug::Integer::from(&x).lcm(&rug::Integer::from(&y)))),
            lcm
        );

        assert_eq!((&y).lcm(&x), lcm);
        assert!((&lcm).divisible_by(&x));
        assert!((&lcm).divisible_by(&y));
        let gcd = (&x).gcd(&y);
        if x != 0 {
            assert_eq!((&lcm).div_exact(&x) * &gcd, y);
        }
        if y != 0 {
            assert_eq!((&lcm).div_exact(&y) * &gcd, x);
        }
        if gcd != 0 {
            assert_eq!(x.div_exact(gcd) * y, lcm);
        }
    });

    natural_gen().test_properties(|x| {
        assert_eq!((&x).lcm(&x), x);
        assert_eq!((&x).lcm(Natural::ONE), x);
        assert_eq!(x.lcm(Natural::ZERO), 0);
    });

    natural_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!((&x).lcm(&y).lcm(&z), x.lcm(y.lcm(z)));
    });

    unsigned_pair_gen_var_34::<Limb>().test_properties(|(x, y)| {
        assert_eq!(Natural::from(x).lcm(Natural::from(y)), x.lcm(y));
    });
}
