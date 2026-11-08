// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::linear_algebra::traits::ModPowerOf2Determinant;
use malachite_base::num::modular::traits::{
    ModGeneralizedCross, ModPowerOf2Cross, ModPowerOf2Dot, ModPowerOf2GeneralizedCross,
    ModPowerOf2Neg,
};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_18;
use malachite_base::test_util::unsigned_vector::modular::mod_power_of_2_generalized_cross::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<UnsignedVector<u64>> {
    xs.iter()
        .map(|s| UnsignedVector::<u64>::from_str(s).unwrap())
        .collect()
}

#[test]
fn test_mod_power_of_2_generalized_cross() {
    let test = |xs: &[&str], pow: u64, out: &str| {
        let xs = vectors(xs);
        let w = UnsignedVector::mod_power_of_2_generalized_cross(&xs, pow);
        assert_eq!(w.to_string(), out);
        assert_eq!(
            mod_power_of_2_generalized_cross_by_determinants(&xs, pow),
            w
        );
    };
    test(&[], 0, "(0)");
    test(&[], 3, "(1)");
    test(&["(3, 4)"], 3, "(4, 3)");
    test(&["(2, 0, 1)", "(0, 2, 3)"], 2, "(2, 2, 0)");
    test(&["(1, 2, 3)", "(4, 5, 6)"], 3, "(5, 6, 5)");
    test(&["(1, 2, 3)", "(2, 4, 6)"], 3, "(0, 0, 0)");
}

#[test]
fn mod_power_of_2_generalized_cross_fail() {
    assert_panic!(UnsignedVector::mod_power_of_2_generalized_cross(
        &vectors(&["(1, 2, 3)"]),
        3
    ));
    assert_panic!(UnsignedVector::mod_power_of_2_generalized_cross(
        &vectors(&["(1, 9)"]),
        3
    ));
}

#[test]
fn mod_power_of_2_generalized_cross_properties() {
    unsigned_matrix_unsigned_pair_gen_var_18().test_properties(|(a, pow)| {
        let xs: Vec<UnsignedVector<u64>> = a
            .rows()
            .map(|row| UnsignedVector::from_elements(row.as_slice()))
            .collect();
        let n = xs.len() + 1;
        let w = UnsignedVector::mod_power_of_2_generalized_cross(&xs, pow);
        assert_eq!(w.elements.len(), n);
        assert_eq!(
            mod_power_of_2_generalized_cross_by_determinants(&xs, pow),
            w
        );
        if pow < u64::WIDTH {
            assert_eq!(
                UnsignedVector::mod_generalized_cross(&xs, u64::power_of_2(pow)),
                w
            );
        }
        // w is perpendicular to every vector modulo the modulus.
        for x in &xs {
            assert_eq!(x.mod_power_of_2_dot(&w, pow), 0);
        }
        // The determinant of the vectors followed by w is w . w.
        let mut rows = xs.clone();
        rows.push(w.clone());
        assert_eq!(
            UnsignedMatrix::from_rows(&rows).mod_power_of_2_determinant(pow),
            (&w).mod_power_of_2_dot(&w, pow)
        );
        match n {
            2 => assert_eq!(
                w.elements,
                [xs[0].elements[1].mod_power_of_2_neg(pow), xs[0].elements[0]]
            ),
            3 => assert_eq!((&xs[0]).mod_power_of_2_cross(&xs[1], pow), w),
            _ => {}
        }
    });
}
