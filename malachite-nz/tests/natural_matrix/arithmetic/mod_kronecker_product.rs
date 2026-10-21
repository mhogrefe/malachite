// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    IsPowerOf2, KroneckerProduct, ModIsReduced, ModKroneckerProduct, ModKroneckerProductAssign,
    ModPowerOf2KroneckerProduct,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_matrix_natural_triple_gen_var_3, natural_matrix_natural_pair_gen_var_2,
};
use malachite_nz::test_util::natural_matrix::arithmetic::mod_kronecker_product::*;

#[test]
fn test_mod_kronecker_product() {
    let test = |s, t, m, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let b = NaturalMatrix::from_str(t).unwrap();
        let m = Natural::from_str(m).unwrap();
        let c = (&a).mod_kronecker_product(&b, &m);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(a.clone().mod_kronecker_product(b.clone(), m.clone()), c);
        assert_eq!(a.clone().mod_kronecker_product(b.clone(), &m), c);
        assert_eq!(a.clone().mod_kronecker_product(&b, m.clone()), c);
        assert_eq!(a.clone().mod_kronecker_product(&b, &m), c);
        assert_eq!((&a).mod_kronecker_product(b.clone(), m.clone()), c);
        assert_eq!((&a).mod_kronecker_product(b.clone(), &m), c);
        assert_eq!((&a).mod_kronecker_product(&b, m.clone()), c);
        let mut d = a.clone();
        d.mod_kronecker_product_assign(b.clone(), m.clone());
        assert!(d.is_valid());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_kronecker_product_assign(b.clone(), &m);
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_kronecker_product_assign(&b, m.clone());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_kronecker_product_assign(&b, &m);
        assert_eq!(d, c);
        assert_eq!(natural_matrix_mod_kronecker_product_naive(&a, &b, &m), c);
    };
    test("[]_{0,0}", "[]_{0,0}", "3", "[]_{0,0}");
    test("[]_{2,0}", "[1, 2]", "3", "[]_{2,0}");
    test("[1, 2]", "[]_{0,3}", "3", "[]_{0,6}");
    test("[0, 0]", "[0; 0]", "1", "[0, 0; 0, 0]");
    test("[3]", "[1, 2; 3, 4]", "5", "[3, 1; 4, 2]");
    test(
        "[1, 2; 3, 4]",
        "[0, 5; 6, 7]",
        "9",
        "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]",
    );
    test("[254, 2]", "[3; 128]", "255", "[252, 6; 127, 1]");
    test(
        "[18446744073709551614, 2]",
        "[18446744073709551614; 3]",
        "18446744073709551615",
        "[1, 18446744073709551613; 18446744073709551612, 6]",
    );
    test(
        "[18446744073709551615, 2]",
        "[18446744073709551615; 3]",
        "18446744073709551616",
        "[1, 18446744073709551614; 18446744073709551613, 6]",
    );
    test(
        "[10000000000000000001, 3]",
        "[10000000000000000007; 5]",
        "100000000000000000000",
        "[80000000000000000007, 30000000000000000021; 50000000000000000005, 15]",
    );
}

#[test]
#[should_panic]
fn mod_kronecker_product_fail_1() {
    // The modulus is zero.
    let a = NaturalMatrix::from_str("[]_{0,0}").unwrap();
    let _ = (&a).mod_kronecker_product(&a, Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_kronecker_product_fail_2() {
    // The first matrix is not reduced.
    let a = NaturalMatrix::from_str("[7]").unwrap();
    let b = NaturalMatrix::from_str("[1]").unwrap();
    let _ = a.mod_kronecker_product(b, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_kronecker_product_fail_3() {
    // The second matrix is not reduced.
    let a = NaturalMatrix::from_str("[1]").unwrap();
    let b = NaturalMatrix::from_str("[7]").unwrap();
    let _ = a.mod_kronecker_product(b, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_kronecker_product_assign_fail() {
    let mut a = NaturalMatrix::from_str("[1, 2]").unwrap();
    a.mod_kronecker_product_assign(NaturalMatrix::from_str("[4]").unwrap(), Natural::from(3u32));
}

#[test]
fn mod_kronecker_product_properties() {
    natural_matrix_natural_matrix_natural_triple_gen_var_3().test_properties(|(a, b, m)| {
        let c = (&a).mod_kronecker_product(&b, &m);
        assert!(c.is_valid());
        assert_eq!(a.clone().mod_kronecker_product(b.clone(), m.clone()), c);
        assert_eq!(a.clone().mod_kronecker_product(b.clone(), &m), c);
        assert_eq!(a.clone().mod_kronecker_product(&b, m.clone()), c);
        assert_eq!(a.clone().mod_kronecker_product(&b, &m), c);
        assert_eq!((&a).mod_kronecker_product(b.clone(), m.clone()), c);
        assert_eq!((&a).mod_kronecker_product(b.clone(), &m), c);
        assert_eq!((&a).mod_kronecker_product(&b, m.clone()), c);
        let mut d = a.clone();
        d.mod_kronecker_product_assign(b.clone(), m.clone());
        assert!(d.is_valid());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_kronecker_product_assign(b.clone(), &m);
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_kronecker_product_assign(&b, m.clone());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_kronecker_product_assign(&b, &m);
        assert_eq!(d, c);
        assert_eq!(natural_matrix_mod_kronecker_product_naive(&a, &b, &m), c);
        assert!(c.mod_is_reduced(&m));
        assert_eq!(
            (c.height(), c.width()),
            (a.height() * b.height(), a.width() * b.width())
        );
        // The transpose of the product is the product of the transposes.
        assert_eq!(
            a.transpose().mod_kronecker_product(b.transpose(), &m),
            c.transpose()
        );
        // The product modulo m is the exact product, reduced.
        assert_eq!((&a).kronecker_product(&b) % &m, c);
        // When m is a power of 2, the product agrees with the product modulo a power of 2.
        if m.is_power_of_2() {
            assert_eq!(
                (&a).mod_power_of_2_kronecker_product(&b, m.significant_bits() - 1),
                c
            );
        }
    });

    natural_matrix_natural_pair_gen_var_2().test_properties(|(a, m)| {
        let (h, w) = (a.height(), a.width());
        // The 1 by 1 identity matrix, reduced, is an identity on either side, unless m is 1.
        let one = NaturalMatrix::identity(1) % &m;
        if m != 1u32 {
            assert_eq!(one.clone().mod_kronecker_product(&a, &m), a);
            assert_eq!((&a).mod_kronecker_product(&one, &m), a);
        }
        // A zero matrix gives a zero matrix.
        assert_eq!(
            (&a).mod_kronecker_product(NaturalMatrix::zero(2, 3), &m),
            NaturalMatrix::zero(h * 2, w * 3)
        );
    });

    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_6().test_properties(|(a, b, m)| {
        // The `u64` elements multiply as their `Natural` counterparts do.
        assert_eq!(
            NaturalMatrix::from(a.clone())
                .mod_kronecker_product(NaturalMatrix::from(b.clone()), Natural::from(m)),
            NaturalMatrix::from(a.mod_kronecker_product(b, m))
        );
    });
}
