// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    HermiteNormalForm, ModPowerOf2, ModPowerOf2Mul, ModPowerOf2Solve, PowerOf2,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_4;

// The nonzero rows of the Hermite normal form of `m`.
fn nonzero_hnf_rows(m: IntegerMatrix) -> Vec<Vec<Integer>> {
    let w = usize::try_from(m.width()).unwrap();
    if w == 0 {
        return Vec::new();
    }
    m.hermite_normal_form()
        .elements_ref()
        .chunks_exact(w)
        .filter(|row| row.iter().any(|x| *x != 0u32))
        .map(<[Integer]>::to_vec)
        .collect()
}

// Whether Ax ≡ b (mod 2^pow) has a solution: whether b is in the integer column lattice of [A |
// 2^pow·I].
fn solvable(a: &NaturalMatrix, b: &[Natural], pow: u64) -> bool {
    let h = usize::try_from(a.height()).unwrap();
    let w = usize::try_from(a.width()).unwrap();
    // The rows of the transpose of [A | 2^pow·I], and with b appended.
    let mut rows = Vec::with_capacity((w + h + 1) * h);
    for j in 0..w {
        rows.extend((0..h).map(|i| Integer::from(&a.elements_ref()[i * w + j])));
    }
    let m = Integer::from(Natural::power_of_2(pow));
    for j in 0..h {
        rows.extend((0..h).map(|i| if i == j { m.clone() } else { Integer::ZERO }));
    }
    let n = u64::try_from(w + h).unwrap();
    let lattice = IntegerMatrix::from_owned_elements(n, a.height(), rows.clone());
    rows.extend(b.iter().map(Integer::from));
    let extended = IntegerMatrix::from_owned_elements(n + 1, a.height(), rows);
    nonzero_hnf_rows(lattice) == nonzero_hnf_rows(extended)
}

fn verify(a: &NaturalMatrix, b: &NaturalVector, pow: u64) {
    let x = a.mod_power_of_2_solve(b, pow);
    assert_eq!(a.clone().mod_power_of_2_solve(b.clone(), pow), x);
    assert_eq!(x.is_some(), solvable(a, &b.elements, pow));
    let bm = NaturalMatrix::from_owned_elements(a.height(), 1, b.elements.clone());
    let xm = a.mod_power_of_2_solve(&bm, pow);
    assert_eq!(
        xm.as_ref().map(|x| x.to_elements()),
        x.as_ref().map(|x| x.elements.clone())
    );
    if let Some(xm) = xm {
        assert_eq!(xm.height(), a.width());
        assert_eq!(a.mod_power_of_2_mul(&xm, pow), bm);
    }
}

#[test]
fn mod_power_of_2_solve_properties() {
    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        let h = usize::try_from(a.height()).unwrap();
        let w = a.width();
        // A consistent right-hand side, A·(1, ..., 1), and the first unit vector.
        let ones = NaturalMatrix::from_owned_elements(
            w,
            1,
            vec![Natural::ONE; usize::try_from(w).unwrap()],
        )
        .mod_power_of_2(pow);
        let b = (&a).mod_power_of_2_mul(&ones, pow);
        verify(
            &a,
            &NaturalVector {
                elements: b.into_elements(),
            },
            pow,
        );
        if h != 0 {
            let mut e = vec![Natural::ZERO; h];
            e[0] = Natural::ONE.mod_power_of_2(pow);
            verify(&a, &NaturalVector { elements: e }, pow);
        }
    });

    // The word-sized solutions, including modulo 2^64, agree with those of the corresponding
    // NaturalMatrix.
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        let h = a.height();
        let b = UnsignedMatrix::<u64>::from_owned_elements(
            h,
            1,
            (0..h).map(|i| i.mod_power_of_2(pow)).collect(),
        );
        let n = NaturalMatrix::from(a.clone());
        assert_eq!(
            (&a).mod_power_of_2_solve(&b, pow).map(NaturalMatrix::from),
            n.mod_power_of_2_solve(NaturalMatrix::from(b), pow)
        );
    });
}
