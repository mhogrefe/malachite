// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::linear_algebra::traits::ModDeterminant;
use malachite_base::num::modular::traits::{ModCross, ModDot, ModGeneralizedCross, ModNeg};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::natural_vector::modular::mod_generalized_cross::*;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_10;
use malachite_nz::test_util::natural_vector::modular::mod_generalized_cross::*;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<NaturalVector> {
    xs.iter()
        .map(|s| NaturalVector::from_str(s).unwrap())
        .collect()
}

#[test]
fn test_mod_generalized_cross() {
    let test = |xs: &[&str], m: u32, out: &str| {
        let xs = vectors(xs);
        let m = Natural::from(m);
        let w = NaturalVector::mod_generalized_cross(&xs, &m);
        assert_eq!(w.to_string(), out);
        assert_eq!(NaturalVector::mod_generalized_cross(&xs, m.clone()), w);
        assert_eq!(mod_generalized_cross_by_determinants(&xs, &m), w);
    };
    test(&[], 5, "(1)");
    test(&[], 1, "(0)");
    test(&["(3, 4)"], 7, "(3, 3)");
    test(&["(1, 2, 3)", "(4, 5, 6)"], 7, "(4, 6, 4)");
    test(
        &["(1, 2, 3, 4)", "(0, 1, 6, 2)", "(2, 0, 1, 1)"],
        7,
        "(5, 3, 6, 5)",
    );
    test(&["(2, 0, 1)", "(0, 2, 3)"], 4, "(2, 2, 0)");
}

#[test]
fn mod_generalized_cross_fail() {
    assert_panic!(NaturalVector::mod_generalized_cross(
        &vectors(&["(1, 2, 3)"]),
        Natural::from(7u32)
    ));
    assert_panic!(NaturalVector::mod_generalized_cross(
        &vectors(&["(1, 9)"]),
        Natural::from(7u32)
    ));
    assert_panic!(NaturalVector::mod_generalized_cross(
        &vectors(&["(1, 2)"]),
        Natural::ZERO
    ));
}

#[test]
fn mod_generalized_cross_properties() {
    natural_matrix_natural_pair_gen_var_10().test_properties(|(a, m)| {
        let xs: Vec<NaturalVector> = a
            .rows()
            .map(|row| NaturalVector::from_elements(row.as_slice()))
            .collect();
        let n = xs.len() + 1;
        let w = NaturalVector::mod_generalized_cross(&xs, &m);
        assert_eq!(w.elements.len(), n);
        assert_eq!(mod_generalized_cross_by_determinants(&xs, &m), w);
        assert_eq!(
            NaturalVector::from_owned_elements(mod_generalized_cross_by_integers(&xs, &m)),
            w
        );
        // Moduli that fit in a u64 give the same answer as for UnsignedVectors.
        if let Ok(m64) = u64::try_from(&m) {
            let ys: Vec<UnsignedVector<u64>> = xs
                .iter()
                .map(|x| {
                    UnsignedVector::from_owned_elements(
                        x.elements
                            .iter()
                            .map(|e| u64::try_from(e).unwrap())
                            .collect(),
                    )
                })
                .collect();
            assert_eq!(
                NaturalVector::from(&UnsignedVector::mod_generalized_cross(&ys, m64)),
                w
            );
        }
        // w is perpendicular to every vector modulo the modulus.
        for x in &xs {
            assert_eq!(x.mod_dot(&w, &m), 0u32);
        }
        // The determinant of the vectors followed by w is w . w.
        let mut rows = xs.clone();
        rows.push(w.clone());
        assert_eq!(
            NaturalMatrix::from_rows(&rows).mod_determinant(&m),
            (&w).mod_dot(&w, &m)
        );
        match n {
            2 => assert_eq!(
                w.elements,
                [xs[0].elements[1].clone().mod_neg(&m), xs[0].elements[0].clone()]
            ),
            3 => assert_eq!((&xs[0]).mod_cross(&xs[1], &m), w),
            _ => {}
        }
    });
}
