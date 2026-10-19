// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{BalancedMod, CoprimeWith, Pow};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::factorization::traits::Primes;
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::unsigned_vec_natural_matrix_pair_gen_var_1;
use malachite_nz::test_util::natural_matrix::arithmetic::multi_crt::natural_matrix_multi_crt_naive;

#[test]
fn test_multi_crt() {
    let test = |moduli: &[Limb], residues: &[&str], out: Option<&str>| {
        let residues: Vec<UnsignedMatrix<Limb>> = residues
            .iter()
            .map(|s| UnsignedMatrix::from_str(s).unwrap())
            .collect();
        let result = NaturalMatrix::multi_crt(moduli, &residues);
        assert!(result.as_ref().is_none_or(NaturalMatrix::is_valid));
        assert_eq!(result.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!(natural_matrix_multi_crt_naive(moduli, &residues), result);
    };
    test(
        &[3, 5, 7],
        &["[2, 0; 1, 0]", "[3, 1; 0, 0]", "[2, 6; 2, 0]"],
        Some("[23, 6; 100, 0]"),
    );
    test(&[3, 5], &["[]_{0,3}", "[]_{0,3}"], Some("[]_{0,3}"));
    test(&[3, 5], &["[]_{2,0}", "[]_{2,0}"], Some("[]_{2,0}"));
    test(&[5], &["[0, 1; 2, 3; 4, 0]"], Some("[0, 1; 2, 3; 4, 0]"));
    test(&[1], &["[0, 0]"], Some("[0, 0]"));
    test(&[0], &["[5]"], None);
    test(&[0, 3], &["[1]", "[1]"], None);
    test(&[2, 1], &["[1]", "[0]"], None);
    test(&[4, 6], &["[1]", "[1]"], None);
    test(&[4, 6], &["[]_{0,0}", "[]_{0,0}"], None);
    test(
        &[4294967291, 4294967279],
        &["[1342177280, 3; 4294967290, 0]", "[268435473, 3; 4294967278, 0]"],
        Some("[1152921504606846976, 3; 18446743979220271188, 0]"),
    );
}

#[test]
fn test_multi_crt_many_primes() {
    // Enough primes that the precomputed combination splits into several chunks.
    let primes: Vec<Limb> = Limb::primes().take(2000).collect();
    let m: Natural = primes.iter().copied().map(Natural::from).product();
    let half = &m >> 1u32;
    let a = NaturalMatrix::from_owned_elements(
        3,
        2,
        vec![
            Natural::from(3u32).pow(500),
            Natural::ZERO,
            Natural::ONE,
            &m - Natural::ONE,
            half.clone(),
            half + Natural::ONE,
        ],
    );
    let residues = a.multi_mod(&primes);
    assert_eq!(NaturalMatrix::multi_crt(&primes, &residues), Some(a));
}

#[test]
#[should_panic]
fn multi_crt_fail_1() {
    // No moduli.
    let _ = NaturalMatrix::multi_crt(&[], &[]);
}

#[test]
#[should_panic]
fn multi_crt_fail_2() {
    // Two moduli but one residue matrix.
    let _ = NaturalMatrix::multi_crt(&[3, 5], &[UnsignedMatrix::from_str("[1]").unwrap()]);
}

#[test]
#[should_panic]
fn multi_crt_fail_3() {
    // Residue matrices of different shapes.
    let _ = NaturalMatrix::multi_crt(
        &[3, 5],
        &[UnsignedMatrix::from_str("[1, 2]").unwrap(), UnsignedMatrix::from_str("[1; 2]").unwrap()],
    );
}

#[test]
#[should_panic]
fn multi_crt_fail_4() {
    // Residue matrices with no elements, of different shapes.
    let _ = NaturalMatrix::multi_crt(
        &[3, 5],
        &[
            UnsignedMatrix::from_str("[]_{0,2}").unwrap(),
            UnsignedMatrix::from_str("[]_{2,0}").unwrap(),
        ],
    );
}

#[test]
#[should_panic]
fn multi_crt_fail_5() {
    // A residue that is not reduced.
    let _ = NaturalMatrix::multi_crt(
        &[3, 5],
        &[UnsignedMatrix::from_str("[3]").unwrap(), UnsignedMatrix::from_str("[1]").unwrap()],
    );
}

#[test]
#[should_panic]
fn multi_crt_fail_6() {
    // With one modulus, a residue that is not reduced.
    let _ = NaturalMatrix::multi_crt(&[5], &[UnsignedMatrix::from_str("[5]").unwrap()]);
}

#[test]
fn multi_crt_properties() {
    unsigned_vec_natural_matrix_pair_gen_var_1().test_properties(|(ms, a)| {
        let residues = a.multi_mod(&ms);
        let b = NaturalMatrix::multi_crt(&ms, &residues).unwrap();
        assert!(b.is_valid());
        assert_eq!(
            natural_matrix_multi_crt_naive(&ms, &residues),
            Some(b.clone())
        );
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        let m: Natural = ms.iter().copied().map(Natural::from).product();
        assert_eq!(b, &a % &m);
        // Combination commutes with transposition.
        let transposed: Vec<UnsignedMatrix<Limb>> =
            residues.iter().map(Matrix::transpose).collect();
        assert_eq!(
            NaturalMatrix::multi_crt(&ms, &transposed),
            Some(b.transpose())
        );

        // Balancing the result gives the balanced combination.
        assert_eq!(
            Some(IntegerMatrix::from(b).balanced_mod(Integer::from(&m))),
            IntegerMatrix::multi_balanced_crt(&ms, &residues)
        );
    });

    unsigned_vec_gen::<Limb>().test_properties(|ms| {
        if ms.is_empty() {
            return;
        }
        // Zeros are reduced modulo any nonzero modulus, so the result exists exactly when the
        // moduli are usable, whatever the shape.
        let usable = if let [m] = *ms.as_slice() {
            m != 0
        } else {
            ms.iter()
                .enumerate()
                .all(|(i, &m)| m >= 2 && ms[..i].iter().all(|&prev| m.coprime_with(prev)))
        };
        for (height, width) in [(0, 0), (0, 2), (2, 0), (2, 3)] {
            let residues = vec![UnsignedMatrix::<Limb>::zero(height, width); ms.len()];
            let result = NaturalMatrix::multi_crt(&ms, &residues);
            assert_eq!(result.is_some(), usable);
            if let Some(b) = &result {
                assert_eq!(*b, NaturalMatrix::zero(height, width));
            }
            assert_eq!(natural_matrix_multi_crt_naive(&ms, &residues), result);
        }
    });
}
