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
use malachite_base::num::arithmetic::traits::{
    BalancedMod, BalancedModAssign, DivisibleBy, Mod, UnsignedAbs,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
use malachite_base::num::comparison::traits::OrdDouble;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    integer_matrix_gen, integer_matrix_integer_pair_gen_var_1,
};

#[test]
fn test_balanced_mod() {
    let test = |s, m, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let m = Integer::from_str(m).unwrap();
        // All four combinations of value and reference, and in place with both.
        let b = (&a).balanced_mod(&m);
        assert_eq!(b.to_string(), out);
        assert_eq!((&a).balanced_mod(m.clone()), b);
        assert_eq!(a.clone().balanced_mod(&m), b);
        assert_eq!(a.clone().balanced_mod(m.clone()), b);
        let mut x = a.clone();
        x.balanced_mod_assign(&m);
        assert_eq!(x, b);
        let mut x = a;
        x.balanced_mod_assign(m);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", "1", "[]_{0,0}");
    test("[]_{3,0}", "1", "[]_{3,0}");
    test("[]_{0,4}", "1", "[]_{0,4}");
    test("[5, -4; -1, 9]", "3", "[-1, -1; -1, 0]");
    test("[-4; 3]", "-4", "[0; -1]");
    test("[]_{0,0}", "-7", "[]_{0,0}");
    // Modulo 1 or -1 every element is zero, but the shape is unchanged.
    test("[1, 27, -23]", "1", "[0, 0, 0]");
    test("[1, 27, -23]", "-1", "[0, 0, 0]");
    // Each element goes to the representative closest to zero.
    test("[1, 27, -23]", "10", "[1, -3, -3]");
    test("[1, 27, -23]", "-10", "[1, -3, -3]");
    // Half the modulus is positive, whichever side it comes from.
    test("[5, -5]", "10", "[5, 5]");
    test("[1, -1]", "2", "[1, 1]");
    // With an odd modulus there is no tie.
    test("[2, -2, 3, -3]", "5", "[2, -2, -2, 2]");
    // Multiples of the modulus become zero and stay.
    test("[-6, -3, -9]", "3", "[0, 0, 0]");
    // Elements and divisor of many limbs.
    test(
        "[1000000000000000000000000, 1]",
        "1234567890987",
        "[530068894399, 1]",
    );
}

#[test]
#[should_panic]
fn balanced_mod_fail() {
    let _ = IntegerMatrix::from_str("[1]")
        .unwrap()
        .balanced_mod(Integer::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_ref_ref_fail() {
    let _ = (&IntegerMatrix::from_str("[1]").unwrap()).balanced_mod(&Integer::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_assign_fail() {
    let mut a = IntegerMatrix::from_str("[1]").unwrap();
    a.balanced_mod_assign(Integer::ZERO);
}

// The empty matrix has no elements, so a zero divisor is never reached by the elementwise loop;
// only the explicit check makes these panic rather than quietly returning the empty matrix.
#[test]
#[should_panic]
fn balanced_mod_empty_matrix_fail() {
    let _ = IntegerMatrix::from_str("[]_{0,0}")
        .unwrap()
        .balanced_mod(Integer::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_ref_empty_matrix_fail() {
    let _ = (&IntegerMatrix::from_str("[]_{0,0}").unwrap()).balanced_mod(&Integer::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_assign_empty_matrix_fail() {
    let mut a = IntegerMatrix::from_str("[]_{0,0}").unwrap();
    a.balanced_mod_assign(&Integer::ZERO);
}

#[test]
fn balanced_mod_properties() {
    integer_matrix_integer_pair_gen_var_1().test_properties(|(a, m)| {
        let b = (&a).balanced_mod(&m);
        // The forms agree.
        assert_eq!((&a).balanced_mod(m.clone()), b);
        assert_eq!(a.clone().balanced_mod(&m), b);
        assert_eq!(a.clone().balanced_mod(m.clone()), b);
        let mut x = a.clone();
        x.balanced_mod_assign(&m);
        assert_eq!(x, b);
        let mut x = a.clone();
        x.balanced_mod_assign(m.clone());
        assert_eq!(x, b);

        // The sign of the modulus makes no difference.
        assert_eq!((&a).balanced_mod(-&m), b);

        // Reducing again changes nothing, and the shape is unchanged.
        assert_eq!((&b).balanced_mod(&m), b);
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));

        // Element by element, this is the `Integer` operation: each result is congruent to its
        // element and lies in (-|m|/2, |m|/2].
        let abs_m = (&m).unsigned_abs();
        for (x, r) in a.elements.iter().zip(&b.elements) {
            assert_eq!(*r, x.balanced_mod(&m));
            assert!((r - x).divisible_by(&m));
            // |m| >= 2|r|, and at equality the remainder is the positive one.
            match abs_m.cmp_double(&r.unsigned_abs()) {
                Less => panic!("remainder too large"),
                Equal => assert!(*r > 0u32),
                Greater => {}
            }
        }

        // Reducing the result into [0, |m|) gives the same as reducing the original.
        assert_eq!((&b).mod_op(&abs_m), (&a).mod_op(&abs_m));

        // On a matrix with no negative elements and a positive modulus, this is the `NaturalMatrix`
        // operation.
        if let Ok(n) = NaturalMatrix::try_from(&a)
            && m > 0u32
        {
            assert_eq!(n.balanced_mod(&abs_m), b);
        }
    });

    integer_matrix_gen().test_properties(|a| {
        // Modulo 1 or -1 everything vanishes, but the shape is unchanged.
        let zero = IntegerMatrix::from_owned_elements(
            a.height(),
            a.width(),
            vec![Integer::ZERO; a.elements.len()],
        );
        assert_eq!((&a).balanced_mod(Integer::ONE), zero);
        assert_eq!((&a).balanced_mod(Integer::NEGATIVE_ONE), zero);
        // Modulo 2 every odd element becomes 1 and every even one 0, which is `mod_op`.
        assert_eq!(
            (&a).balanced_mod(Integer::TWO),
            IntegerMatrix::from((&a).mod_op(Natural::TWO))
        );
    });
}
