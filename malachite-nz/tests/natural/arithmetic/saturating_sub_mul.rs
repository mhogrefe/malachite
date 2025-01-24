// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    SaturatingSub, SaturatingSubMul, SaturatingSubMulAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::unsigned_triple_gen_var_19;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen, natural_pair_gen, natural_triple_gen};
use std::str::FromStr;

#[test]
fn test_saturating_sub_mul() {
    let test = |r, s, t, out: &str| {
        let u = Natural::from_str(r).unwrap();
        let v = Natural::from_str(s).unwrap();
        let w = Natural::from_str(t).unwrap();

        let mut n = u.clone();
        n.saturating_sub_mul_assign(v.clone(), w.clone());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = u.clone();
        n.saturating_sub_mul_assign(v.clone(), &w);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = u.clone();
        n.saturating_sub_mul_assign(&v, w.clone());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = u.clone();
        n.saturating_sub_mul_assign(&v, &w);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = u.clone().saturating_sub_mul(v.clone(), w.clone());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = u.clone().saturating_sub_mul(v.clone(), &w);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = u.clone().saturating_sub_mul(&v, w.clone());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = u.clone().saturating_sub_mul(&v, &w);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = (&u).saturating_sub_mul(&v, &w);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0", "0");
    test("0", "0", "123", "0");
    test("123", "0", "5", "123");
    test("123", "5", "1", "118");
    test("123", "5", "100", "0");
    test("10", "3", "4", "0");
    test("15", "3", "4", "3");
    test("1000000000000", "0", "123", "1000000000000");
    test("1000000000000", "1", "123", "999999999877");
    test("1000000000000", "123", "1", "999999999877");
    test("1000000000000", "123", "100", "999999987700");
    test("1000000000000", "100", "123", "999999987700");
    test("1000000000000", "65536", "65536", "995705032704");
    test("1000000000000", "1000000000000", "0", "1000000000000");
    test("1000000000000", "1000000000000", "1", "0");
    test("1000000000000", "1000000000000", "100", "0");
    test("0", "1000000000000", "100", "0");
    test("4294967296", "1", "1", "4294967295");
    test("3902609153", "88817093856604", "1", "0");
}

#[allow(clippy::useless_conversion)]
#[test]
fn saturating_sub_mul_properties() {
    natural_triple_gen().test_properties(|(a, b, c)| {
        let mut mut_a = a.clone();
        mut_a.saturating_sub_mul_assign(&b, &c);
        assert!(mut_a.is_valid());
        let result = mut_a;

        let mut mut_a = a.clone();
        mut_a.saturating_sub_mul_assign(&b, c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.saturating_sub_mul_assign(b.clone(), &c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.saturating_sub_mul_assign(b.clone(), c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let result_alt = (&a).saturating_sub_mul(&b, &c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().saturating_sub_mul(&b, &c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().saturating_sub_mul(&b, c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().saturating_sub_mul(b.clone(), &c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().saturating_sub_mul(b.clone(), c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!((&a).saturating_sub(b * c), result);
        assert!(result <= a);
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).saturating_sub_mul(&n, &Natural::ONE), 0);
    });

    natural_pair_gen().test_properties(|(a, b)| {
        assert_eq!(Natural::ZERO.saturating_sub_mul(&a, &b), 0);
        assert_eq!((&a).saturating_sub_mul(&Natural::ZERO, &b), a);
        assert_eq!((&a).saturating_sub_mul(&b, &Natural::ZERO), a);
        assert_eq!((&a * &b).saturating_sub_mul(&a, &b), 0);
        assert_eq!(
            (&a).saturating_sub_mul(&Natural::ONE, &b),
            (&a).saturating_sub(&b)
        );
        assert_eq!(
            (&a).saturating_sub_mul(&b, &Natural::ONE),
            a.saturating_sub(b)
        );
    });

    unsigned_triple_gen_var_19::<Limb>().test_properties(|(x, y, z)| {
        assert_eq!(
            Limb::from(x).saturating_sub_mul(Limb::from(y), Limb::from(z)),
            Natural::from(x).saturating_sub_mul(Natural::from(y), Natural::from(z))
        );
    });
}
