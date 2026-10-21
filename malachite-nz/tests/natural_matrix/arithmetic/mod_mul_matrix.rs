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
    IsPowerOf2, ModIsReduced, ModMul, ModMulAssign, ModPowerOf2Mul,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_matrix_natural_triple_gen_var_2, natural_matrix_natural_pair_gen_var_2,
};
use malachite_nz::test_util::natural_matrix::arithmetic::mod_mul_matrix::*;

#[test]
fn test_mod_mul_matrix() {
    let test = |s, t, m, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let b = NaturalMatrix::from_str(t).unwrap();
        let m = Natural::from_str(m).unwrap();
        let c = a.clone().mod_mul(b.clone(), m.clone());
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(a.clone().mod_mul(b.clone(), &m), c);
        assert_eq!(a.clone().mod_mul(&b, m.clone()), c);
        assert_eq!(a.clone().mod_mul(&b, &m), c);
        assert_eq!((&a).mod_mul(b.clone(), m.clone()), c);
        assert_eq!((&a).mod_mul(b.clone(), &m), c);
        assert_eq!((&a).mod_mul(&b, m.clone()), c);
        assert_eq!((&a).mod_mul(&b, &m), c);
        let mut d = a.clone();
        d.mod_mul_assign(b.clone(), m.clone());
        assert!(d.is_valid());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_mul_assign(b.clone(), &m);
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_mul_assign(&b, m.clone());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_mul_assign(&b, &m);
        assert_eq!(d, c);
        assert_eq!(natural_matrix_mod_mul_matrix_naive(&a, &b, &m), c);
    };
    test("[]_{0,0}", "[]_{0,0}", "3", "[]_{0,0}");
    test("[]_{0,3}", "[1, 2; 0, 1; 2, 0]", "3", "[]_{0,2}");
    test("[]_{2,0}", "[]_{0,3}", "3", "[0, 0, 0; 0, 0, 0]");
    test("[1, 2, 3; 4, 5, 6]", "[]_{3,0}", "7", "[]_{2,0}");
    test("[0, 0; 0, 0]", "[0, 0; 0, 0]", "1", "[0, 0; 0, 0]");
    test("[3]", "[4]", "5", "[2]");
    test("[1; 2; 3]", "[4, 5]", "7", "[4, 5; 1, 3; 5, 1]");
    test("[1, 2; 3, 4]", "[5, 6; 7, 8]", "9", "[1, 4; 7, 5]");
    test(
        "[1, 2, 3; 4, 5, 6]",
        "[6, 5; 4, 3; 2, 1]",
        "11",
        "[9, 3; 1, 8]",
    );
    test(
        "[200, 250; 100, 7]",
        "[254, 253; 3, 128]",
        "255",
        "[40, 235; 176, 186]",
    );
    test("[1, 1; 1, 0]", "[1, 1; 1, 0]", "2", "[0, 1; 1, 1]");
    test(
        "[18446744073709551614, 9223372036854775808; 5, 18446744073709551613]",
        "[18446744073709551611, 7; 18446744073709551614, 1]",
        "18446744073709551615",
        "[9223372036854775811, 9223372036854775801; 18446744073709551597, 33]",
    );
    test(
        "[18446744073709551615, 9223372036854775808; 5, 18446744073709551613]",
        "[18446744073709551611, 7; 18446744073709551615, 1]",
        "18446744073709551616",
        "[9223372036854775813, 9223372036854775801; 18446744073709551594, 32]",
    );
    test(
        "[18446744073709551621, 3; 10000000000000000000, 36893488147419103232]",
        "[18446744073709551615, 10000000000000000007; 17, 36893488147419103233]",
        "36893488147419103245",
        "[27670116110564327496, 3446744073709551576; 35680464442257309514, 35450828945621779331]",
    );
    test(
        "[100000000000000000000000000000, 3; 7, 100000000000000000000000000001]",
        "[2, 100000000000000000000000000000; 100000000000000000000000000005, 9]",
        "1000000000000000000000000000000",
        "[500000000000000000000000000015, 27; 600000000000000000000000000019, 60000000000000000000\
        0000000009]",
    );
    test(
        "[170141183460469231731687303715884105728, 1; 3, 170141183460469231731687303715884105739]",
        "[340282366920938463463374607431768211455, 2; 170141183460469231731687303715884105728, 9]",
        "340282366920938463463374607431768211457",
        "[170141183460469231731687303715884105729, 8; 85070591730234615865843651857942052853, 1701\
        41183460469231731687303715884105829]",
    );
}

#[test]
#[should_panic]
fn mod_mul_matrix_fail_1() {
    // The modulus is zero.
    let a = NaturalMatrix::from_str("[]_{0,0}").unwrap();
    let _ = (&a).mod_mul(&a, Natural::from(0u32));
}

#[test]
#[should_panic]
fn mod_mul_matrix_fail_2() {
    // The shapes are incompatible.
    let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    let _ = (&a).mod_mul(&a, Natural::from(3u32));
}

#[test]
#[should_panic]
fn mod_mul_matrix_fail_3() {
    // The first matrix is not reduced.
    let a = NaturalMatrix::from_str("[7]").unwrap();
    let b = NaturalMatrix::from_str("[1]").unwrap();
    let _ = a.mod_mul(b, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_mul_matrix_fail_4() {
    // The second matrix is not reduced.
    let a = NaturalMatrix::from_str("[1]").unwrap();
    let b = NaturalMatrix::from_str("[7]").unwrap();
    let _ = a.mod_mul(b, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_mul_matrix_assign_fail_1() {
    // The shapes are incompatible.
    let mut a = NaturalMatrix::from_str("[1; 2]").unwrap();
    a.mod_mul_assign(
        NaturalMatrix::from_str("[1; 2]").unwrap(),
        Natural::from(3u32),
    );
}

#[test]
#[should_panic]
fn mod_mul_matrix_assign_fail_2() {
    // The second matrix is not reduced, and the modulus does not fit in a limb.
    let mut a = NaturalMatrix::from_str("[1]").unwrap();
    a.mod_mul_assign(
        NaturalMatrix::from_str("[1000000000000000000000000000000]").unwrap(),
        Natural::from_str("1000000000000000000000000000000").unwrap(),
    );
}

#[test]
fn mod_mul_matrix_properties() {
    natural_matrix_natural_matrix_natural_triple_gen_var_2().test_properties(|(a, b, m)| {
        let c = (&a).mod_mul(&b, &m);
        assert!(c.is_valid());
        assert_eq!((c.height(), c.width()), (a.height(), b.width()));
        assert!(c.mod_is_reduced(&m));
        assert_eq!(a.clone().mod_mul(b.clone(), m.clone()), c);
        assert_eq!(a.clone().mod_mul(b.clone(), &m), c);
        assert_eq!(a.clone().mod_mul(&b, m.clone()), c);
        assert_eq!(a.clone().mod_mul(&b, &m), c);
        assert_eq!((&a).mod_mul(b.clone(), m.clone()), c);
        assert_eq!((&a).mod_mul(b.clone(), &m), c);
        assert_eq!((&a).mod_mul(&b, m.clone()), c);
        let mut d = a.clone();
        d.mod_mul_assign(b.clone(), m.clone());
        assert!(d.is_valid());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_mul_assign(b.clone(), &m);
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_mul_assign(&b, m.clone());
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_mul_assign(&b, &m);
        assert_eq!(d, c);
        assert_eq!(natural_matrix_mod_mul_matrix_naive(&a, &b, &m), c);
        // The product modulo m is the exact product, reduced.
        assert_eq!(&a * &b % &m, c);
        // When m is a power of 2, the product agrees with the product modulo a power of 2.
        if m.is_power_of_2() {
            assert_eq!((&a).mod_power_of_2_mul(&b, m.significant_bits() - 1), c);
        }
        // The transpose of a product is the product of the transposes in the other order.
        assert_eq!(b.transpose().mod_mul(a.transpose(), &m), c.transpose());
    });

    natural_matrix_natural_pair_gen_var_2().test_properties(|(a, m)| {
        let (h, w) = (a.height(), a.width());
        // Multiplying by the identity matrix leaves a matrix unchanged, unless m is 1.
        if m != 1u32 {
            assert_eq!(NaturalMatrix::identity(h).mod_mul(&a, &m), a);
            assert_eq!((&a).mod_mul(NaturalMatrix::identity(w), &m), a);
        }
        // Zero matrices give zero products.
        assert_eq!(
            (&a).mod_mul(NaturalMatrix::zero(w, 3), &m),
            NaturalMatrix::zero(h, 3)
        );
    });

    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_3().test_properties(|(a, b, m)| {
        // The `u64` elements multiply as their `Natural` counterparts do.
        assert_eq!(
            NaturalMatrix::from(a.clone())
                .mod_mul(NaturalMatrix::from(b.clone()), Natural::from(m)),
            NaturalMatrix::from(a.mod_mul(b, m))
        );
    });
}
