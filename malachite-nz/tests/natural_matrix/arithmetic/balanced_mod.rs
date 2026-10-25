// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::*;
use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{BalancedMod, DivisibleBy, Mod, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::comparison::traits::OrdDouble;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_gen, natural_matrix_natural_pair_gen_var_1,
};

#[test]
fn test_balanced_mod() {
    let test = |s, m, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        // All four combinations of value and reference.
        let b = (&a).balanced_mod(&m);
        assert_eq!(b.to_string(), out);
        assert_eq!((&a).balanced_mod(m.clone()), b);
        assert_eq!(a.clone().balanced_mod(&m), b);
        assert_eq!(a.balanced_mod(m), b);
    };
    test("[]_{0,0}", "1", "[]_{0,0}");
    test("[]_{3,0}", "1", "[]_{3,0}");
    test("[]_{0,4}", "1", "[]_{0,4}");
    test("[5, 4; 1, 9]", "3", "[-1, 1; 1, 0]");
    test("[4; 3]", "4", "[0; -1]");
    test("[]_{0,0}", "7", "[]_{0,0}");
    // Modulo 1 every element is zero, but the shape is unchanged.
    test("[1, 2, 3]", "1", "[0, 0, 0]");
    // Each element goes to the representative closest to zero, which may be negative; half an even
    // modulus stays positive.
    test("[1, 27, 23, 5, 10]", "10", "[1, -3, 3, 5, 0]");
    // For an odd modulus no element is exactly half of it.
    test("[5, 6]", "11", "[5, -5]");
    // Residues of a modulus of more than one limb.
    test(
        "[18446744073709551615, 9223372036854775808]",
        "18446744073709551616",
        "[-1, 9223372036854775808]",
    );
}

#[test]
#[should_panic]
fn balanced_mod_fail() {
    let _ = NaturalMatrix::from_str("[1]")
        .unwrap()
        .balanced_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_val_ref_fail() {
    let _ = NaturalMatrix::from_str("[1]")
        .unwrap()
        .balanced_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_ref_val_fail() {
    let _ = (&NaturalMatrix::from_str("[1]").unwrap()).balanced_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_ref_ref_fail() {
    let _ = (&NaturalMatrix::from_str("[1]").unwrap()).balanced_mod(&Natural::ZERO);
}

// The empty matrix has no elements, so a zero divisor is never reached by the elementwise loop;
// only the explicit check makes this panic.
#[test]
#[should_panic]
fn balanced_mod_empty_matrix_fail() {
    let _ = (&NaturalMatrix::from_str("[]_{0,0}").unwrap()).balanced_mod(&Natural::ZERO);
}

#[test]
fn balanced_mod_properties() {
    natural_matrix_natural_pair_gen_var_1().test_properties(|(a, m)| {
        let b = (&a).balanced_mod(&m);
        // The forms agree.
        assert_eq!((&a).balanced_mod(m.clone()), b);
        assert_eq!(a.clone().balanced_mod(&m), b);
        assert_eq!(a.clone().balanced_mod(m.clone()), b);

        // The shape is unchanged.
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));

        // Element by element, this is the `Natural` operation: each result is congruent to its
        // element and lies in (-m/2, m/2].
        let m_i = Integer::from(&m);
        for (x, r) in a.elements_ref().iter().zip(b.elements_ref()) {
            assert_eq!(*r, x.balanced_mod(&m));
            assert!((r - Integer::from(x)).divisible_by(&m_i));
            // m >= 2|r|, and at equality the remainder is the positive one.
            match m.cmp_double(&r.unsigned_abs()) {
                Less => panic!("remainder too large"),
                Equal => assert!(*r > 0u32),
                Greater => {}
            }
        }

        // Reducing the result into [0, m) gives the ordinary remainder.
        assert_eq!((&b).mod_op(&m), &a % &m);
    });

    natural_matrix_gen().test_properties(|a| {
        // Modulo 1 everything vanishes, but the shape is unchanged.
        assert_eq!(
            (&a).balanced_mod(Natural::ONE),
            IntegerMatrix::from_owned_elements(
                a.height(),
                a.width(),
                vec![Integer::ZERO; a.elements_ref().len()]
            )
        );
        // Modulo 2 every odd element becomes 1 and every even one 0, which is `%`.
        assert_eq!(
            (&a).balanced_mod(Natural::TWO),
            IntegerMatrix::from(&a % Natural::TWO)
        );
    });
}
