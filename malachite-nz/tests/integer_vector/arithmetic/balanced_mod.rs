// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::*;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    BalancedMod, BalancedModAssign, DivisibleBy, Mod, UnsignedAbs,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
use malachite_base::num::comparison::traits::OrdDouble;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    integer_vector_gen, integer_vector_integer_pair_gen_var_1,
};

#[test]
fn test_balanced_mod() {
    let test = |s, m, out| {
        let v = IntegerVector::from_str(s).unwrap();
        let m = Integer::from_str(m).unwrap();
        // All four combinations of value and reference, and in place with both.
        let w = (&v).balanced_mod(&m);
        assert_eq!(w.to_string(), out);
        assert_eq!((&v).balanced_mod(m.clone()), w);
        assert_eq!(v.clone().balanced_mod(&m), w);
        assert_eq!(v.clone().balanced_mod(m.clone()), w);
        let mut x = v.clone();
        x.balanced_mod_assign(&m);
        assert_eq!(x, w);
        let mut x = v;
        x.balanced_mod_assign(m);
        assert_eq!(x, w);
    };
    test("()", "1", "()");
    test("()", "-7", "()");
    // Modulo 1 or -1 every element is zero, but the dimension is unchanged.
    test("(1, 27, -23)", "1", "(0, 0, 0)");
    test("(1, 27, -23)", "-1", "(0, 0, 0)");
    // Each element goes to the representative closest to zero.
    test("(1, 27, -23)", "10", "(1, -3, -3)");
    test("(1, 27, -23)", "-10", "(1, -3, -3)");
    // Half the modulus is positive, whichever side it comes from.
    test("(5, -5)", "10", "(5, 5)");
    test("(1, -1)", "2", "(1, 1)");
    // With an odd modulus there is no tie.
    test("(2, -2, 3, -3)", "5", "(2, -2, -2, 2)");
    // Multiples of the modulus become zero and stay.
    test("(-6, -3, -9)", "3", "(0, 0, 0)");
    // Elements and divisor of many limbs.
    test(
        "(1000000000000000000000000, 1)",
        "1234567890987",
        "(530068894399, 1)",
    );
}

#[test]
#[should_panic]
fn balanced_mod_fail() {
    let _ = IntegerVector::from_str("(1)")
        .unwrap()
        .balanced_mod(Integer::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_ref_ref_fail() {
    let _ = (&IntegerVector::from_str("(1)").unwrap()).balanced_mod(&Integer::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_assign_fail() {
    let mut v = IntegerVector::from_str("(1)").unwrap();
    v.balanced_mod_assign(Integer::ZERO);
}

// The empty vector has no elements, so a zero divisor is never reached by the elementwise loop;
// only the explicit check makes these panic rather than quietly returning the empty vector.
#[test]
#[should_panic]
fn balanced_mod_empty_vector_fail() {
    let _ = IntegerVector::from_str("()")
        .unwrap()
        .balanced_mod(Integer::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_ref_empty_vector_fail() {
    let _ = (&IntegerVector::from_str("()").unwrap()).balanced_mod(&Integer::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_assign_empty_vector_fail() {
    let mut v = IntegerVector::from_str("()").unwrap();
    v.balanced_mod_assign(&Integer::ZERO);
}

#[test]
fn balanced_mod_properties() {
    integer_vector_integer_pair_gen_var_1().test_properties(|(v, m)| {
        let w = (&v).balanced_mod(&m);
        // The forms agree.
        assert_eq!((&v).balanced_mod(m.clone()), w);
        assert_eq!(v.clone().balanced_mod(&m), w);
        assert_eq!(v.clone().balanced_mod(m.clone()), w);
        let mut x = v.clone();
        x.balanced_mod_assign(&m);
        assert_eq!(x, w);
        let mut x = v.clone();
        x.balanced_mod_assign(m.clone());
        assert_eq!(x, w);

        // The sign of the modulus makes no difference.
        assert_eq!((&v).balanced_mod(-&m), w);

        // Reducing again changes nothing, and the dimension is unchanged.
        assert_eq!((&w).balanced_mod(&m), w);
        assert_eq!(w.dimension(), v.dimension());

        // Element by element, this is the `Integer` operation: each result is congruent to its
        // element and lies in (-|m|/2, |m|/2].
        let abs_m = (&m).unsigned_abs();
        for (x, r) in v.elements.iter().zip(&w.elements) {
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
        assert_eq!((&w).mod_op(&abs_m), (&v).mod_op(&abs_m));

        // On a vector with no negative elements and a positive modulus, this is the `NaturalVector`
        // operation.
        if let Ok(n) = NaturalVector::try_from(&v)
            && m > 0u32
        {
            assert_eq!(n.balanced_mod(&abs_m), w);
        }
    });

    integer_vector_gen().test_properties(|v| {
        // Modulo 1 or -1 everything vanishes, but the dimension is unchanged.
        let zero = IntegerVector::zero(v.dimension());
        assert_eq!((&v).balanced_mod(Integer::ONE), zero);
        assert_eq!((&v).balanced_mod(Integer::NEGATIVE_ONE), zero);
        // Modulo 2 every odd element becomes 1 and every even one 0, which is `mod_op`.
        assert_eq!(
            (&v).balanced_mod(Integer::TWO),
            IntegerVector::from((&v).mod_op(Natural::TWO))
        );
    });
}
