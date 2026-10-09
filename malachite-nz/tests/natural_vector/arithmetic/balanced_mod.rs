// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::*;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{BalancedMod, DivisibleBy, Mod, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::comparison::traits::OrdDouble;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_vector_gen, natural_vector_natural_pair_gen_var_1,
};

#[test]
fn test_balanced_mod() {
    let test = |s, m, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        // All four combinations of value and reference.
        let w = (&v).balanced_mod(&m);
        assert_eq!(w.to_string(), out);
        assert_eq!((&v).balanced_mod(m.clone()), w);
        assert_eq!(v.clone().balanced_mod(&m), w);
        assert_eq!(v.balanced_mod(m), w);
    };
    test("()", "1", "()");
    test("()", "7", "()");
    // Modulo 1 every element is zero, but the dimension is unchanged.
    test("(1, 2, 3)", "1", "(0, 0, 0)");
    // Each element goes to the representative closest to zero, which may be negative; half an even
    // modulus stays positive.
    test("(1, 27, 23, 5, 10)", "10", "(1, -3, 3, 5, 0)");
    // For an odd modulus no element is exactly half of it.
    test("(5, 6)", "11", "(5, -5)");
    // Residues of a modulus of more than one limb.
    test(
        "(18446744073709551615, 9223372036854775808)",
        "18446744073709551616",
        "(-1, 9223372036854775808)",
    );
}

#[test]
#[should_panic]
fn balanced_mod_fail() {
    let _ = NaturalVector::from_str("(1)")
        .unwrap()
        .balanced_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_val_ref_fail() {
    let _ = NaturalVector::from_str("(1)")
        .unwrap()
        .balanced_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_ref_val_fail() {
    let _ = (&NaturalVector::from_str("(1)").unwrap()).balanced_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_ref_ref_fail() {
    let _ = (&NaturalVector::from_str("(1)").unwrap()).balanced_mod(&Natural::ZERO);
}

// The empty vector has no elements, so a zero divisor is never reached by the elementwise loop;
// only the explicit check makes this panic.
#[test]
#[should_panic]
fn balanced_mod_empty_vector_fail() {
    let _ = (&NaturalVector::from_str("()").unwrap()).balanced_mod(&Natural::ZERO);
}

#[test]
fn balanced_mod_properties() {
    natural_vector_natural_pair_gen_var_1().test_properties(|(v, m)| {
        let w = (&v).balanced_mod(&m);
        // The forms agree.
        assert_eq!((&v).balanced_mod(m.clone()), w);
        assert_eq!(v.clone().balanced_mod(&m), w);
        assert_eq!(v.clone().balanced_mod(m.clone()), w);

        // The dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());

        // Element by element, this is the `Natural` operation: each result is congruent to its
        // element and lies in (-m/2, m/2].
        let m_i = Integer::from(&m);
        for (x, r) in v.elements.iter().zip(&w.elements) {
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
        assert_eq!((&w).mod_op(&m), &v % &m);
    });

    natural_vector_gen().test_properties(|v| {
        // Modulo 1 everything vanishes, but the dimension is unchanged.
        assert_eq!(
            (&v).balanced_mod(Natural::ONE),
            IntegerVector::zero(v.dimension())
        );
        // Modulo 2 every odd element becomes 1 and every even one 0, which is `%`.
        assert_eq!(
            (&v).balanced_mod(Natural::TWO),
            IntegerVector::from(&v % Natural::TWO)
        );
    });
}
