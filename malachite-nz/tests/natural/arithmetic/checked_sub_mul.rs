// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedSub, CheckedSubMul};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::unsigned_triple_gen_var_19;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen, natural_pair_gen, natural_triple_gen};
use std::str::FromStr;

#[test]
fn test_checked_sub_mul() {
    let test = |r, s, t, out| {
        let u = Natural::from_str(r).unwrap();
        let v = Natural::from_str(s).unwrap();
        let w = Natural::from_str(t).unwrap();

        let on = u.clone().checked_sub_mul(v.clone(), w.clone());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = u.clone().checked_sub_mul(v.clone(), &w);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = u.clone().checked_sub_mul(&v, w.clone());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = u.clone().checked_sub_mul(&v, &w);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = (&u).checked_sub_mul(&v, &w);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("0", "0", "0", "Some(0)");
    test("0", "0", "123", "Some(0)");
    test("123", "0", "5", "Some(123)");
    test("123", "5", "1", "Some(118)");
    test("123", "5", "100", "None");
    test("10", "3", "4", "None");
    test("15", "3", "4", "Some(3)");
    test("1000000000000", "0", "123", "Some(1000000000000)");
    test("1000000000000", "1", "123", "Some(999999999877)");
    test("1000000000000", "123", "1", "Some(999999999877)");
    test("1000000000000", "123", "100", "Some(999999987700)");
    test("1000000000000", "100", "123", "Some(999999987700)");
    test("1000000000000", "65536", "65536", "Some(995705032704)");
    test("1000000000000", "1000000000000", "0", "Some(1000000000000)");
    test("1000000000000", "1000000000000", "1", "Some(0)");
    test("1000000000000", "1000000000000", "100", "None");
    test("0", "1000000000000", "100", "None");
    test("4294967296", "1", "1", "Some(4294967295)");
    test("3902609153", "88817093856604", "1", "None");
}

#[test]
fn checked_sub_properties() {
    natural_triple_gen().test_properties(|(a, b, c)| {
        let result = (&a).checked_sub_mul(&b, &c);
        assert!(result.as_ref().map_or(true, Natural::is_valid));

        let result_alt = a.clone().checked_sub_mul(&b, &c);
        assert!(result_alt.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(result_alt, result);

        let result_alt = a.clone().checked_sub_mul(&b, c.clone());
        assert!(result_alt.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(result_alt, result);

        let result_alt = a.clone().checked_sub_mul(b.clone(), &c);
        assert!(result_alt.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(result_alt, result);

        let result_alt = a.clone().checked_sub_mul(b.clone(), c.clone());
        assert!(result_alt.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(result_alt, result);

        assert_eq!(a.checked_sub(b * c), result);
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).checked_sub_mul(&n, &Natural::ONE), Some(Natural::ZERO));
    });

    natural_pair_gen().test_properties(|(a, b)| {
        assert_eq!((&a).checked_sub_mul(&Natural::ZERO, &b).as_ref(), Some(&a));
        assert_eq!((&a).checked_sub_mul(&b, &Natural::ZERO).as_ref(), Some(&a));
        assert_eq!(
            (&a).checked_sub_mul(&Natural::ONE, &b),
            (&a).checked_sub(&b)
        );
        assert_eq!((&a).checked_sub_mul(&b, &Natural::ONE), a.checked_sub(b));
    });

    unsigned_triple_gen_var_19::<Limb>().test_properties(|(x, y, z)| {
        assert_eq!(
            x.checked_sub_mul(y, z).map(Natural::from),
            Natural::from(x).checked_sub_mul(Natural::from(y), Natural::from(z))
        );
    });
}
