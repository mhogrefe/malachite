// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{BalancedMod, Mod, Pow};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::factorization::traits::Primes;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::unsigned_vec_integer_vector_pair_gen_var_1;
use malachite_nz::test_util::integer_vector::arithmetic::multi_mod::integer_vector_multi_mod_naive;

#[test]
fn test_multi_mod() {
    let test = |s, moduli: &[Limb], out: &[&str]| {
        let v = IntegerVector::from_str(s).unwrap();
        let residues = v.multi_mod(moduli);
        assert_eq!(
            residues.iter().map(ToString::to_string).collect::<Vec<_>>(),
            out
        );
        assert_eq!(integer_vector_multi_mod_naive(&v, moduli), residues);
    };
    test(
        "(-23, 6, -100)",
        &[3, 5, 7],
        &["(1, 0, 2)", "(2, 1, 0)", "(5, 6, 5)"],
    );
    test("()", &[3, 5], &["()", "()"]);
    test("(-23, 6, -100)", &[], &[]);
    test("(0, -1)", &[1], &["(0, 0)"]);
    test(
        "(-23, 6, -100)",
        &[4, 6, 4],
        &["(1, 2, 0)", "(1, 0, 2)", "(1, 2, 0)"],
    );
    test(
        "(-18446744073709551616, 340282366920938463463374607431768211455)",
        &[4294967291, 4294967279, 2],
        &["(4294967266, 624)", "(4294966990, 83520)", "(0, 1)"],
    );
    test(
        "(-18446744073709551616, 340282366920938463463374607431768211455)",
        &[18446744073709551557, 18446744073709551615],
        &["(18446744073709551498, 3480)", "(18446744073709551614, 0)"],
    );
}

#[test]
fn test_multi_mod_many_moduli() {
    // Enough moduli that the reduction goes through a comb.
    let primes: Vec<Limb> = Limb::primes().take(300).collect();
    let m: Natural = primes.iter().copied().map(Natural::from).product();
    let v = IntegerVector::from_owned_elements(vec![
        -Integer::from(3u32).pow(500),
        Integer::ZERO,
        Integer::ONE,
        Integer::from(&m - Natural::ONE),
        Integer::from(&m),
        Integer::from(&m * &m + Natural::ONE),
        -Integer::from(&m) - Integer::ONE,
    ]);
    let residues = v.multi_mod(&primes);
    assert_eq!(integer_vector_multi_mod_naive(&v, &primes), residues);
    // Moduli that are not coprime cannot use a comb, but give the same residues as reducing modulo
    // each separately.
    let moduli: Vec<Limb> = (1..=300).collect();
    assert_eq!(
        v.multi_mod(&moduli),
        integer_vector_multi_mod_naive(&v, &moduli)
    );
}

#[test]
#[should_panic]
fn multi_mod_fail_1() {
    let _ = IntegerVector::from_str("(1, 2)")
        .unwrap()
        .multi_mod(&[3, 0]);
}

#[test]
#[should_panic]
fn multi_mod_fail_2() {
    let _ = IntegerVector::from_str("()").unwrap().multi_mod(&[0]);
}

#[test]
fn multi_mod_properties() {
    unsigned_vec_integer_vector_pair_gen_var_1().test_properties(|(ms, v)| {
        let residues = v.multi_mod(&ms);
        assert_eq!(integer_vector_multi_mod_naive(&v, &ms), residues);
        assert_eq!(residues.len(), ms.len());
        for (rs, &m) in residues.iter().zip(&ms) {
            assert_eq!(rs.elements.len(), v.elements.len());
            assert!(rs.elements.iter().all(|&r| r < m));
        }
        // Combining the residues again gives the vector reduced modulo the product of the moduli,
        // canonically or balanced.
        let m: Natural = ms.iter().copied().map(Natural::from).product();
        assert_eq!(
            NaturalVector::multi_crt(&ms, &residues),
            Some((&v).mod_op(&m))
        );
        assert_eq!(
            IntegerVector::multi_balanced_crt(&ms, &residues),
            Some((&v).balanced_mod(Integer::from(&m)))
        );

        // Negating the vector negates each residue modulo its modulus.
        let neg = (-&v).multi_mod(&ms);
        for ((rs, ns), &m) in residues.iter().zip(&neg).zip(&ms) {
            for (&r, &n) in rs.elements.iter().zip(&ns.elements) {
                assert_eq!(n, if r == 0 { 0 } else { m - r });
            }
        }
    });
}
