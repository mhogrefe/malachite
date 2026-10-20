// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{BalancedMod, Mod};
use malachite_base::num::factorization::traits::Primes;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::unsigned_vec_integer_matrix_pair_gen_var_1;
use malachite_nz::test_util::integer_matrix::arithmetic::multi_mod::integer_matrix_multi_mod_naive;

#[test]
fn test_multi_mod() {
    let test = |s, moduli: &[Limb], out: &[&str]| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let residues = a.multi_mod(moduli);
        assert!(residues.iter().all(UnsignedMatrix::is_valid));
        assert_eq!(
            residues.iter().map(ToString::to_string).collect::<Vec<_>>(),
            out
        );
        assert_eq!(integer_matrix_multi_mod_naive(&a, moduli), residues);
    };
    test(
        "[-23, 6; -100, 0]",
        &[3, 5, 7],
        &["[1, 0; 2, 0]", "[2, 1; 0, 0]", "[5, 6; 5, 0]"],
    );
    test("[]_{0,2}", &[3, 5], &["[]_{0,2}", "[]_{0,2}"]);
    test("[]_{3,0}", &[2], &["[]_{3,0}"]);
    test("[-23, 6; -100, 0]", &[], &[]);
    test("[0, -1]", &[1], &["[0, 0]"]);
    test(
        "[-23, 6; -100, 0]",
        &[4, 6, 4],
        &["[1, 2; 0, 0]", "[1, 0; 2, 0]", "[1, 2; 0, 0]"],
    );
    test(
        "[-18446744073709551616; 340282366920938463463374607431768211455]",
        &[4294967291, 4294967279, 2],
        &["[4294967266; 624]", "[4294966990; 83520]", "[0; 1]"],
    );
    #[cfg(not(feature = "32_bit_limbs"))]
    test(
        "[-18446744073709551616; 340282366920938463463374607431768211455]",
        &[18446744073709551557, 18446744073709551615],
        &["[18446744073709551498; 3480]", "[18446744073709551614; 0]"],
    );
}

#[test]
fn test_multi_mod_many_moduli() {
    // Enough moduli that the reduction goes through a comb.
    let primes: Vec<Limb> = Limb::primes().take(300).collect();
    let a =
        IntegerMatrix::from_str("[-18446744073709551616; 340282366920938463463374607431768211455]")
            .unwrap();
    assert_eq!(
        a.multi_mod(&primes),
        integer_matrix_multi_mod_naive(&a, &primes)
    );
    let moduli: Vec<Limb> = (1..=300).collect();
    assert_eq!(
        a.multi_mod(&moduli),
        integer_matrix_multi_mod_naive(&a, &moduli)
    );
}

#[test]
#[should_panic]
fn multi_mod_fail_1() {
    let _ = IntegerMatrix::from_str("[1, 2]")
        .unwrap()
        .multi_mod(&[3, 0]);
}

#[test]
#[should_panic]
fn multi_mod_fail_2() {
    let _ = IntegerMatrix::from_str("[]_{0,0}").unwrap().multi_mod(&[0]);
}

#[test]
fn multi_mod_properties() {
    unsigned_vec_integer_matrix_pair_gen_var_1().test_properties(|(ms, a)| {
        let residues = a.multi_mod(&ms);
        assert!(residues.iter().all(UnsignedMatrix::is_valid));
        assert_eq!(integer_matrix_multi_mod_naive(&a, &ms), residues);
        assert_eq!(residues.len(), ms.len());
        for (rs, &m) in residues.iter().zip(&ms) {
            assert_eq!((rs.height(), rs.width()), (a.height(), a.width()));
            assert!(rs.elements.iter().all(|&r| r < m));
        }
        // The residues are those of the elements, as a vector, in the same order.
        let v_residues = IntegerVector::from_elements(&a.elements).multi_mod(&ms);
        for (rs, vrs) in residues.iter().zip(&v_residues) {
            assert_eq!(rs.elements, vrs.elements);
        }
        // Reduction commutes with transposition.
        assert_eq!(
            a.transpose().multi_mod(&ms),
            residues.iter().map(Matrix::transpose).collect::<Vec<_>>()
        );
        // Combining the residues again gives the matrix reduced modulo the product of the moduli.
        let m: Natural = ms.iter().copied().map(Natural::from).product();
        assert_eq!(
            NaturalMatrix::multi_crt(&ms, &residues),
            Some((&a).mod_op(&m))
        );
        assert_eq!(
            IntegerMatrix::multi_balanced_crt(&ms, &residues),
            Some((&a).balanced_mod(Integer::from(&m)))
        );
    });
}
