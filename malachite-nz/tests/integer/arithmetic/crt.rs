// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{BalancedCrt, CoprimeWith, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_natural_natural_natural_quadruple_gen_var_1;
use malachite_nz::test_util::integer::arithmetic::crt::{
    balanced_crt_simple, balanced_to_canonical,
};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_balanced_crt() {
    let test = |s, t, u, v, out| {
        let r1 = Integer::from_str(s).unwrap();
        let m1 = Natural::from_str(t).unwrap();
        let r2 = Natural::from_str(u).unwrap();
        let m2 = Natural::from_str(v).unwrap();

        let result = r1.clone().balanced_crt(m1.clone(), r2.clone(), m2.clone());
        assert_eq!(result.to_debug_string(), out);
        assert_eq!(
            r1.clone()
                .balanced_crt(m1.clone(), r2.clone(), &m2)
                .to_debug_string(),
            out
        );
        assert_eq!(
            r1.clone()
                .balanced_crt(m1.clone(), &r2, m2.clone())
                .to_debug_string(),
            out
        );
        assert_eq!(
            r1.clone()
                .balanced_crt(m1.clone(), &r2, &m2)
                .to_debug_string(),
            out
        );
        assert_eq!(
            r1.clone()
                .balanced_crt(&m1, r2.clone(), m2.clone())
                .to_debug_string(),
            out
        );
        assert_eq!(
            r1.clone()
                .balanced_crt(&m1, r2.clone(), &m2)
                .to_debug_string(),
            out
        );
        assert_eq!(
            r1.clone()
                .balanced_crt(&m1, &r2, m2.clone())
                .to_debug_string(),
            out
        );
        assert_eq!(
            r1.clone().balanced_crt(&m1, &r2, &m2).to_debug_string(),
            out
        );
        assert_eq!(
            (&r1)
                .balanced_crt(m1.clone(), r2.clone(), m2.clone())
                .to_debug_string(),
            out
        );
        assert_eq!(
            (&r1)
                .balanced_crt(m1.clone(), r2.clone(), &m2)
                .to_debug_string(),
            out
        );
        assert_eq!(
            (&r1)
                .balanced_crt(m1.clone(), &r2, m2.clone())
                .to_debug_string(),
            out
        );
        assert_eq!(
            (&r1).balanced_crt(m1.clone(), &r2, &m2).to_debug_string(),
            out
        );
        assert_eq!(
            (&r1)
                .balanced_crt(&m1, r2.clone(), m2.clone())
                .to_debug_string(),
            out
        );
        assert_eq!(
            (&r1).balanced_crt(&m1, r2.clone(), &m2).to_debug_string(),
            out
        );
        assert_eq!(
            (&r1).balanced_crt(&m1, &r2, m2.clone()).to_debug_string(),
            out
        );
        assert_eq!((&r1).balanced_crt(&m1, &r2, &m2).to_debug_string(), out);

        assert_eq!(balanced_crt_simple(r1, m1, r2, m2).to_debug_string(), out);
    };
    test("0", "1", "0", "1", "Some(0)");
    // - a negative first residue
    test("-1", "3", "3", "5", "Some(-7)");
    // - the same congruence class through a nonnegative representative
    test("2", "3", "3", "5", "Some(-7)");
    // - a canonical solution of exactly half the modulus product stays positive
    test("1", "2", "0", "7", "Some(7)");
    // - the moduli are not coprime
    test("0", "4", "1", "2", "None");
    test("1", "4", "3", "6", "None");
    // - the boundary representative r1 == -m1, with a multi-limb m1
    test(
        "-98765432123456789012345678990",
        "98765432123456789012345678990",
        "3",
        "7",
        "Some(-296296296370370367037037036970)",
    );
    // - a negative multi-limb first residue and multi-limb moduli
    test(
        "-12345678987654321012345678900",
        "12345678987654321012345678901",
        "123456789",
        "98765432123456789012345678990",
        "Some(-66938945431947197749668719354033469115717882907817218311)",
    );
}

#[test]
fn balanced_crt_fail() {
    assert_panic!(Integer::from(2).balanced_crt(Natural::from(2u8), Natural::ZERO, Natural::ONE));
    assert_panic!(Integer::from(-3).balanced_crt(Natural::from(2u8), Natural::ZERO, Natural::ONE));
    assert_panic!(Integer::ZERO.balanced_crt(Natural::ONE, Natural::from(3u8), Natural::from(3u8)));
    assert_panic!(Integer::ZERO.balanced_crt(Natural::ZERO, Natural::ZERO, Natural::ONE));
    assert_panic!((&Integer::from(2)).balanced_crt(
        &Natural::from(2u8),
        &Natural::ZERO,
        &Natural::ONE
    ));
}

#[test]
fn balanced_crt_properties() {
    integer_natural_natural_natural_quadruple_gen_var_1().test_properties(|(r1, m1, r2, m2)| {
        let result = (&r1).balanced_crt(&m1, &r2, &m2);

        assert_eq!(
            r1.clone().balanced_crt(m1.clone(), r2.clone(), m2.clone()),
            result
        );
        assert_eq!(r1.clone().balanced_crt(m1.clone(), r2.clone(), &m2), result);
        assert_eq!(r1.clone().balanced_crt(m1.clone(), &r2, m2.clone()), result);
        assert_eq!(r1.clone().balanced_crt(m1.clone(), &r2, &m2), result);
        assert_eq!(r1.clone().balanced_crt(&m1, r2.clone(), m2.clone()), result);
        assert_eq!(r1.clone().balanced_crt(&m1, r2.clone(), &m2), result);
        assert_eq!(r1.clone().balanced_crt(&m1, &r2, m2.clone()), result);
        assert_eq!(r1.clone().balanced_crt(&m1, &r2, &m2), result);
        assert_eq!(
            (&r1).balanced_crt(m1.clone(), r2.clone(), m2.clone()),
            result
        );
        assert_eq!((&r1).balanced_crt(m1.clone(), r2.clone(), &m2), result);
        assert_eq!((&r1).balanced_crt(m1.clone(), &r2, m2.clone()), result);
        assert_eq!((&r1).balanced_crt(m1.clone(), &r2, &m2), result);
        assert_eq!((&r1).balanced_crt(&m1, r2.clone(), m2.clone()), result);
        assert_eq!((&r1).balanced_crt(&m1, r2.clone(), &m2), result);
        assert_eq!((&r1).balanced_crt(&m1, &r2, m2.clone()), result);
        assert_eq!(
            balanced_crt_simple(r1.clone(), m1.clone(), r2.clone(), m2.clone()),
            result
        );

        assert_eq!(result.is_some(), (&m1).coprime_with(&m2));
        if let Some(x) = result {
            let m = &m1 * &m2;
            // The representative is the one of smallest absolute value, positive on ties.
            let doubled = x.unsigned_abs_ref() << 1u32;
            if x >= 0u32 {
                assert!(doubled <= m);
            } else {
                assert!(doubled < m);
            }
            // It satisfies both congruences.
            let canonical = balanced_to_canonical(&x, &m);
            let r1n = if r1 < 0u32 {
                &m1 - r1.unsigned_abs_ref()
            } else {
                r1.unsigned_abs()
            };
            assert_eq!(&canonical % &m1, r1n);
            assert_eq!(&canonical % &m2, r2);
            // A balanced result is a valid first residue for further chaining.
            assert_eq!(
                (&x).balanced_crt(&m, &Natural::ZERO, &Natural::ONE),
                Some(x.clone())
            );
        }
    });
}
