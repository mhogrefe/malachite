// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{BalancedMod, CoprimeWith, ModIsReduced, Pow};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::factorization::traits::Primes;
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::unsigned_vec_natural_vector_pair_gen_var_1;
use malachite_nz::test_util::natural_vector::arithmetic::multi_crt::natural_vector_multi_crt_naive;

#[test]
fn test_multi_crt() {
    let test = |moduli: &[Limb], residues: &[&str], out: Option<&str>| {
        let residues: Vec<UnsignedVector<Limb>> = residues
            .iter()
            .map(|s| UnsignedVector::from_str(s).unwrap())
            .collect();
        let result = NaturalVector::multi_crt(moduli, &residues);
        assert_eq!(result.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!(natural_vector_multi_crt_naive(moduli, &residues), result);
    };
    // 23 is 2 mod 3, 3 mod 5, and 2 mod 7; 6 is 0, 1, and 6; and 100 is 1, 0, and 2.
    test(
        &[3, 5, 7],
        &["(2, 0, 1)", "(3, 1, 0)", "(2, 6, 2)"],
        Some("(23, 6, 100)"),
    );
    test(&[3, 5], &["()", "()"], Some("()"));
    // With one modulus the residues are the answer.
    test(&[5], &["(0, 1, 2, 3, 4)"], Some("(0, 1, 2, 3, 4)"));
    // A single modulus of 1 is usable, and every residue modulo it is 0.
    test(&[1], &["(0, 0)"], Some("(0, 0)"));
    // Unusable moduli: a single 0; a 0 or 1 among several; and a pair that is not coprime. They are
    // rejected even when there is no element to combine.
    test(&[0], &["(5)"], None);
    test(&[0, 3], &["(1)", "(1)"], None);
    test(&[2, 1], &["(1)", "(0)"], None);
    test(&[4, 6], &["(1)", "(1)"], None);
    test(&[4, 6], &["()", "()"], None);
    // Moduli near the word size, whose product does not fit in a word.
    test(
        &[4294967291, 4294967279],
        &["(1342177280, 3, 4294967290)", "(268435473, 3, 4294967278)"],
        Some("(1152921504606846976, 3, 18446743979220271188)"),
    );
}

#[test]
fn test_multi_crt_many_primes() {
    // Enough primes that the precomputed combination splits into several chunks. The FLINT oracle
    // checks the same row.
    let primes: Vec<Limb> = Limb::primes().take(2000).collect();
    let m: Natural = primes.iter().copied().map(Natural::from).product();
    let half = &m >> 1u32;
    let v = NaturalVector::from_owned_elements(vec![
        Natural::from(3u32).pow(500),
        Natural::ZERO,
        Natural::ONE,
        &m - Natural::ONE,
        half.clone(),
        half + Natural::ONE,
    ]);
    let residues: Vec<UnsignedVector<Limb>> = primes.iter().map(|&p| &v % p).collect();
    assert_eq!(NaturalVector::multi_crt(&primes, &residues), Some(v));
}

#[test]
#[should_panic]
fn multi_crt_fail_1() {
    // No moduli.
    let _ = NaturalVector::multi_crt(&[], &[]);
}

#[test]
#[should_panic]
fn multi_crt_fail_2() {
    // Two moduli but one residue vector.
    let _ = NaturalVector::multi_crt(&[3, 5], &[UnsignedVector::from_str("(1)").unwrap()]);
}

#[test]
#[should_panic]
fn multi_crt_fail_3() {
    // Residue vectors of different dimensions.
    let _ = NaturalVector::multi_crt(
        &[3, 5],
        &[UnsignedVector::from_str("(1, 2)").unwrap(), UnsignedVector::from_str("(1)").unwrap()],
    );
}

#[test]
#[should_panic]
fn multi_crt_fail_4() {
    // A residue that is not reduced.
    let _ = NaturalVector::multi_crt(
        &[3, 5],
        &[UnsignedVector::from_str("(3)").unwrap(), UnsignedVector::from_str("(1)").unwrap()],
    );
}

#[test]
#[should_panic]
fn multi_crt_fail_5() {
    // With one modulus, a residue that is not reduced.
    let _ = NaturalVector::multi_crt(&[5], &[UnsignedVector::from_str("(5)").unwrap()]);
}

#[test]
fn multi_crt_properties() {
    unsigned_vec_natural_vector_pair_gen_var_1().test_properties(|(ms, v)| {
        let residues: Vec<UnsignedVector<Limb>> = ms.iter().map(|&m| &v % m).collect();
        let w = NaturalVector::multi_crt(&ms, &residues).unwrap();
        assert_eq!(
            natural_vector_multi_crt_naive(&ms, &residues),
            Some(w.clone())
        );

        // The result is the original vector reduced modulo the product of the moduli.
        let m: Natural = ms.iter().copied().map(Natural::from).product();
        assert!(w.mod_is_reduced(&m));
        assert_eq!(w, &v % &m);

        // Balancing the result gives the balanced combination.
        assert_eq!(
            Some(IntegerVector::from(w).balanced_mod(Integer::from(&m))),
            IntegerVector::multi_balanced_crt(&ms, &residues)
        );
    });

    unsigned_vec_gen::<Limb>().test_properties(|ms| {
        if ms.is_empty() {
            return;
        }
        // Zeros are reduced modulo any nonzero modulus, so the result exists exactly when the
        // moduli are usable, whatever the dimension.
        let usable = if let [m] = *ms.as_slice() {
            m != 0
        } else {
            ms.iter()
                .enumerate()
                .all(|(i, &m)| m >= 2 && ms[..i].iter().all(|&prev| m.coprime_with(prev)))
        };
        for dimension in [0, 2] {
            let residues = vec![UnsignedVector::<Limb>::zero(dimension); ms.len()];
            let result = NaturalVector::multi_crt(&ms, &residues);
            assert_eq!(result.is_some(), usable);
            if let Some(v) = &result {
                assert_eq!(*v, NaturalVector::zero(dimension));
            }
            assert_eq!(natural_vector_multi_crt_naive(&ms, &residues), result);
        }
    });
}
