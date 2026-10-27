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
    IsPowerOf2, ModCharacteristicPolynomial, ModDeterminant, ModNeg,
    ModPowerOf2CharacteristicPolynomial, ModSimilarityTransform, ModTrace, Parity,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::IsPrime;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_pair_gen_var_11, unsigned_matrix_unsigned_pair_gen_var_12,
    unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_1,
};
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_characteristic_polynomial::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_matrix::arithmetic::mod_characteristic_polynomial::*;
use malachite_base::unsigned_matrix::arithmetic::mod_determinant::charpoly_berkowitz;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use std::panic::catch_unwind;

// Checks Danilevsky's method, which must succeed for a prime modulus, against Berkowitz's
// algorithm, and returns the polynomial.
fn verify_prime(a: &UnsignedMatrix<u64>, m: u64) -> Vec<u64> {
    let n = usize::exact_from(a.height());
    let mut cp = vec![0; n + 1];
    charpoly_berkowitz(&mut cp, a.elements_ref(), n, m);
    assert_eq!(charpoly_danilevsky(a.elements_ref(), n, m).unwrap(), cp);
    assert_eq!(mod_charpoly(a.elements_ref(), n, m), cp);
    cp
}

#[test]
fn test_charpoly_danilevsky() {
    let test = |s, m, out: &[u64]| {
        let a = UnsignedMatrix::<u64>::from_str(s).unwrap();
        assert_eq!(verify_prime(&a, m), out);
    };
    // - dimension 0
    test("[]_{0,0}", 251, &[1]);
    // - dimension 1
    test("[5]", 251, &[246, 1]);
    // - a dense matrix
    test(
        "[3, 1, 4, 1; 5, 9, 2, 6; 5, 3, 5, 8; 9, 7, 9, 3]",
        251,
        &[98, 229, 235, 231, 1],
    );
    // - a diagonal matrix
    test(
        "[2, 0, 0, 0; 0, 3, 0, 0; 0, 0, 5, 0; 0, 0, 0, 7]",
        251,
        &[210, 4, 101, 234, 1],
    );
    // - a zero pivot with a nonzero element to its left
    test(
        "[1, 2, 3, 4; 5, 6, 7, 8; 9, 10, 11, 12; 13, 14, 0, 15]",
        251,
        &[0, 205, 82, 218, 1],
    );
    // - a block triangular matrix
    test(
        "[1, 2, 3, 4; 5, 6, 7, 8; 0, 0, 9, 10; 0, 0, 11, 12]",
        251,
        &[8, 98, 141, 223, 1],
    );
    // - modulo 2
    test("[1, 1, 0; 0, 1, 1; 1, 0, 1]", 2, &[0, 1, 1, 1]);
}

#[test]
fn test_mod_charpoly() {
    // - dimension 9, with a prime modulus: Danilevsky's method succeeds
    let a = UnsignedMatrix::<u64>::from_str(
        "[1, 6, 4, 2, 0, 5, 3, 1, 6; 4, 2, 0, 5, 3, 1, 6, 4, 2; 0, 5, 3, 1, 6, 4, 2, 0, 5; \
        3, 1, 6, 4, 2, 0, 5, 3, 1; 6, 4, 2, 0, 5, 3, 1, 6, 4; 2, 0, 5, 3, 1, 6, 4, 2, 0; \
        5, 3, 1, 6, 4, 2, 0, 5, 3; 1, 6, 4, 2, 0, 5, 3, 1, 6; 4, 2, 0, 5, 3, 1, 6, 4, 2]",
    )
    .unwrap();
    assert_eq!(
        verify_prime(&a, 251),
        &[0, 0, 164, 106, 60, 161, 236, 173, 227, 1]
    );
    // - a pivot that is not invertible: Danilevsky's method fails
    let a = UnsignedMatrix::<u64>::from_str("[1, 2; 2, 3]").unwrap();
    assert_eq!(charpoly_danilevsky(a.elements_ref(), 2, 6), None);
    assert_eq!(mod_charpoly(a.elements_ref(), 2, 6), &[5, 2, 1]);
    // - dimension 9, with a pivot that is not invertible: Danilevsky's method fails
    let a = UnsignedMatrix::<u64>::from_str(
        "[1, 0, 4, 2, 0, 5, 3, 1, 0; 4, 2, 0, 5, 3, 1, 0, 4, 2; 0, 5, 3, 1, 0, 4, 2, 0, 5; \
        3, 1, 0, 4, 2, 0, 5, 3, 1; 0, 4, 2, 0, 5, 3, 1, 0, 4; 2, 0, 5, 3, 1, 0, 4, 2, 0; \
        5, 3, 1, 0, 4, 2, 0, 5, 3; 1, 0, 4, 2, 0, 5, 3, 1, 0; 4, 2, 0, 5, 3, 1, 0, 2, 2]",
    )
    .unwrap();
    assert_eq!(charpoly_danilevsky(a.elements_ref(), 9, 6), None);
    assert_eq!(
        mod_charpoly(a.elements_ref(), 9, 6),
        &[0, 0, 4, 0, 1, 1, 0, 0, 0, 1]
    );
}

#[test]
fn mod_charpoly_properties() {
    unsigned_matrix_unsigned_pair_gen_var_12().test_properties(|(a, m)| {
        verify_prime(&a, m);
    });

    unsigned_matrix_unsigned_pair_gen_var_11().test_properties(|(a, m)| {
        // For any modulus, Danilevsky's method, when it succeeds, agrees with Berkowitz's
        // algorithm.
        let n = usize::exact_from(a.height());
        let mut cp = vec![0; n + 1];
        charpoly_berkowitz(&mut cp, a.elements_ref(), n, m);
        if m != 1
            && let Some(p) = charpoly_danilevsky(a.elements_ref(), n, m)
        {
            assert_eq!(p, cp);
        }
        assert_eq!(mod_charpoly(a.elements_ref(), n, m), cp);
    });
}

// Checks the characteristic polynomial modulo `m` of `a` by value and by reference, against
// Berkowitz's algorithm, the reduction to Hessenberg form, Danilevsky's method when it succeeds,
// and that of the transpose, and, for small matrices, against the Cayley-Hamilton theorem, and
// returns it.
fn verify<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) -> UnsignedPolynomial<T> {
    let p = a.mod_characteristic_polynomial(m);
    assert!(p.is_valid());
    assert_eq!(a.clone().mod_characteristic_polynomial(m), p);
    assert_eq!(
        unsigned_matrix_mod_characteristic_polynomial_berkowitz(a, m),
        p
    );
    assert_eq!(
        unsigned_matrix_mod_characteristic_polynomial_hessenberg(a, m),
        p
    );
    if m != T::ONE
        && let Some(q) = unsigned_matrix_mod_characteristic_polynomial_danilevsky(a, m)
    {
        assert_eq!(q, p);
    }
    assert_eq!(a.transpose().mod_characteristic_polynomial(m), p);
    if a.height() <= 8 {
        assert!(unsigned_polynomial_mod_evaluate_at_matrix(&p, a, m).is_zero());
    }
    p
}

#[test]
fn test_mod_characteristic_polynomial() {
    let test = |s, m: u64, out| {
        let a = UnsignedMatrix::<u64>::from_str(s).unwrap();
        assert_eq!(verify(&a, m).to_string(), out);
    };
    // - dimension 0
    test("[]_{0,0}", 7, "1");
    // - modulo 1: a power of 2
    test("[0, 0; 0, 0]", 1, "0");
    // - dimension 1
    test("[5]", 7, "x+2");
    // - a composite modulus
    test("[1, 2; 3, 4]", 6, "x^2+x+4");
    // - a pivot that is not invertible, in a small matrix
    test("[1, 2; 2, 3]", 6, "x^2+2*x+5");
    // - a power of 2: reduction to Hessenberg form with pivots of smallest 2-adic valuation
    test(
        "[3932043701783186428, 1758098567137225126, 5443272873301836839, 3201967302902885659, 5304\
        85473279918343; 6912573887799929392, 5253914635986282347, 5636913162775944996, 21028359949\
        24914836, 3737328277751973534; 6159952501495858074, 7432809879688336381, 89520435333577410\
        2, 4938050692447711575, 3709754706851070772; 2994817779964610783, 2801529470595732247, 106\
        9128636030162772, 9043611109639802373, 8954466175042803203; 9180195807345982188, 453465918\
        878356648, 9034373415132696507, 8667683034820712703, 4379985732740320228]",
        9223372036854775808,
        "x^5+4165356577078961946*x^4+948376595490363075*x^3+1264238589692297337*x^2+31352076517767\
        82646*x+4274439530610672043",
    );
    // - a small prime modulus, below the cutoff: Berkowitz's algorithm
    test(
        "[2, 24, 96, 149, 72, 114, 45, 11, 33; 31, 145, 221, 157, 12, 236, 157, 227, 52; 201, 49, \
        242, 210, 221, 76, 176, 47, 141; 116, 142, 177, 131, 134, 181, 220, 128, 102; 58, 134, 49, \
        183, 157, 118, 78, 183, 209; 119, 83, 34, 95, 31, 133, 200, 88, 190; 49, 241, 74, 176, 97, \
        148, 197, 42, 10; 218, 171, 30, 187, 108, 53, 36, 62, 97; 225, 78, 52, 108, 227, 182, 78, \
        138, 150]",
        251,
        "x^9+36*x^8+194*x^7+24*x^6+163*x^5+144*x^4+87*x^3+52*x^2+77*x+172",
    );
    // - a prime modulus at the cutoff: Danilevsky's method
    test(
        "[521297214, 1974227911, 1672521278, 578637987, 1578637450, 2017022205, 1776446282, 811718\
        30; 70813317, 1014193755, 1320083939, 1777837447, 1922839007, 588594248, 555842470, 115195\
        9455; 434130482, 933878974, 1747777552, 1507768220, 781433940, 452184540, 861748305, 71685\
        015; 1755862455, 952595924, 824965367, 1518829853, 426155490, 515394027, 460387331, 691496\
        762; 742195352, 637862795, 1498661852, 2060273405, 790822467, 241195976, 1655833385, 46386\
        5953; 141652002, 2081146334, 534793893, 1900128068, 1342016289, 1489496849, 1029773957, 19\
        07496908; 1073418346, 970511628, 2008807502, 1400397457, 595614513, 1413576136, 389918334, \
        897172908; 72467763, 146088972, 1036107974, 330732396, 638114629, 1735580832, 1017434983, \
        1786496636]",
        2147483647,
        "x^8+1478585575*x^7+2001345919*x^6+226750141*x^5+1950891177*x^4+1449303108*x^3+1220109245*\
        x^2+1821857827*x+2110424771",
    );
    // - Danilevsky's method fails below dimension 32: Berkowitz's algorithm instead
    test(
        "[0, 0, 2, 3, 2, 2, 2, 1, 4; 2, 3, 1, 1, 4, 1, 5, 4, 2; 0, 5, 4, 5, 1, 0, 2, 4, 3; 2, 5, 3\
        , 4, 2, 2, 3, 3, 5; 1, 4, 1, 3, 0, 4, 1, 5, 1; 0, 3, 5, 2, 2, 0, 4, 0, 2; 1, 5, 3, 3, 1, 0\
        , 2, 2, 0; 3, 1, 4, 3, 4, 1, 0, 0, 3; 4, 2, 2, 2, 4, 4, 2, 4, 4]",
        2147483646,
        "x^9+2147483629*x^8+2147483534*x^7+2147483625*x^6+2147482551*x^5+2147483089*x^4+8328*x^3+2\
        147478311*x^2+58706*x+2147466878",
    );
    // - Danilevsky's method fails from dimension 32 on: reduction to Hessenberg form instead
    test(
        "[4, 3, 1, 3, 1, 0, 5, 3, 0, 5, 3, 3, 3, 1, 5, 5, 4, 2, 3, 1, 4, 2, 0, 3, 0, 1, 4, 2, 5, 1\
        , 1, 5; 3, 3, 1, 2, 2, 2, 3, 2, 2, 4, 3, 5, 3, 2, 0, 1, 5, 1, 2, 1, 0, 0, 5, 2, 2, 5, 3, 3\
        , 1, 0, 4, 0; 2, 4, 3, 3, 2, 1, 5, 3, 3, 1, 0, 2, 4, 2, 2, 2, 4, 0, 2, 0, 5, 5, 4, 5, 1, 4\
        , 4, 4, 4, 1, 2, 1; 4, 2, 3, 4, 3, 4, 2, 4, 5, 2, 5, 0, 5, 4, 1, 1, 5, 1, 3, 4, 5, 5, 5, 1\
        , 3, 3, 1, 1, 3, 4, 1, 5; 4, 2, 3, 3, 1, 3, 2, 0, 3, 2, 5, 1, 5, 1, 3, 2, 1, 1, 5, 0, 5, 3\
        , 4, 3, 2, 1, 0, 5, 4, 2, 1, 0; 0, 0, 0, 2, 4, 1, 1, 0, 2, 3, 3, 2, 4, 0, 2, 3, 0, 0, 0, 3\
        , 1, 4, 2, 1, 5, 0, 2, 0, 4, 1, 3, 2; 5, 5, 0, 2, 1, 1, 4, 3, 2, 4, 3, 3, 5, 5, 1, 2, 5, 0\
        , 5, 4, 1, 1, 4, 0, 1, 5, 5, 4, 5, 5, 0, 2; 5, 1, 3, 1, 2, 3, 1, 2, 3, 5, 2, 1, 2, 5, 2, 5\
        , 2, 3, 2, 5, 5, 3, 1, 3, 3, 4, 4, 5, 0, 0, 3, 4; 2, 4, 4, 3, 3, 0, 4, 4, 5, 4, 5, 3, 3, 4\
        , 4, 4, 0, 0, 1, 4, 5, 0, 3, 2, 3, 2, 0, 3, 1, 1, 3, 4; 1, 3, 1, 5, 3, 1, 1, 4, 5, 5, 4, 4\
        , 2, 2, 0, 2, 2, 5, 5, 0, 0, 3, 3, 0, 3, 5, 4, 0, 3, 0, 4, 3; 5, 5, 3, 2, 1, 0, 2, 2, 1, 4\
        , 2, 2, 2, 1, 0, 1, 2, 4, 0, 0, 5, 1, 3, 0, 2, 4, 3, 1, 2, 0, 5, 0; 4, 3, 4, 0, 0, 0, 4, 1\
        , 1, 2, 0, 3, 4, 0, 0, 0, 0, 1, 2, 3, 2, 2, 0, 2, 4, 0, 1, 3, 4, 3, 1, 5; 3, 5, 1, 4, 4, 5\
        , 0, 0, 5, 4, 2, 4, 4, 1, 4, 1, 1, 0, 3, 1, 4, 4, 4, 0, 5, 4, 3, 5, 4, 2, 0, 2; 3, 2, 4, 2\
        , 5, 4, 4, 3, 4, 2, 0, 3, 4, 0, 1, 2, 0, 1, 1, 0, 3, 1, 5, 1, 5, 2, 4, 1, 5, 2, 0, 2; 5, 2\
        , 3, 3, 2, 4, 5, 2, 1, 2, 1, 0, 5, 5, 5, 3, 4, 2, 5, 5, 2, 4, 1, 5, 1, 4, 3, 5, 0, 5, 0, 4\
        ; 2, 2, 5, 5, 4, 2, 4, 2, 2, 2, 4, 3, 4, 4, 0, 5, 4, 4, 1, 3, 4, 2, 0, 4, 5, 2, 2, 2, 3, 0\
        , 5, 3; 0, 5, 5, 0, 0, 4, 4, 5, 2, 1, 3, 3, 5, 3, 3, 4, 2, 5, 5, 1, 0, 4, 5, 0, 1, 4, 0, 1\
        , 3, 0, 3, 4; 4, 5, 3, 5, 2, 4, 1, 5, 0, 4, 5, 1, 3, 2, 3, 5, 0, 4, 2, 2, 1, 0, 5, 2, 4, 5\
        , 0, 2, 3, 4, 4, 5; 1, 5, 4, 2, 4, 1, 5, 2, 5, 0, 5, 5, 3, 5, 1, 3, 4, 5, 1, 4, 4, 4, 5, 4\
        , 2, 1, 3, 0, 3, 0, 5, 3; 4, 2, 2, 1, 5, 2, 4, 3, 0, 1, 2, 0, 1, 5, 3, 0, 0, 2, 3, 0, 5, 2\
        , 2, 3, 1, 2, 0, 1, 5, 1, 0, 3; 3, 1, 5, 2, 5, 2, 2, 3, 1, 2, 3, 2, 1, 2, 4, 3, 1, 1, 2, 5\
        , 2, 4, 4, 2, 5, 2, 4, 5, 2, 4, 0, 2; 2, 1, 0, 4, 1, 1, 2, 4, 3, 0, 3, 5, 0, 0, 5, 1, 0, 2\
        , 3, 0, 5, 3, 3, 1, 3, 3, 4, 2, 2, 4, 3, 2; 1, 1, 5, 2, 4, 1, 1, 5, 4, 1, 4, 3, 0, 3, 4, 3\
        , 2, 4, 5, 4, 0, 2, 5, 4, 0, 3, 3, 5, 0, 2, 5, 4; 2, 5, 3, 2, 1, 1, 2, 3, 0, 5, 5, 4, 4, 2\
        , 0, 5, 3, 5, 0, 4, 4, 5, 2, 4, 1, 1, 1, 3, 1, 3, 1, 3; 1, 1, 0, 0, 3, 4, 5, 1, 1, 0, 4, 3\
        , 3, 1, 3, 1, 5, 2, 4, 4, 5, 2, 0, 4, 2, 2, 1, 0, 3, 5, 4, 5; 0, 4, 3, 5, 4, 5, 3, 3, 2, 3\
        , 5, 4, 5, 1, 0, 4, 1, 2, 4, 4, 1, 0, 3, 3, 0, 5, 1, 1, 3, 4, 0, 5; 5, 5, 1, 1, 3, 0, 3, 0\
        , 5, 2, 5, 4, 0, 4, 0, 2, 4, 0, 3, 1, 4, 3, 5, 0, 4, 1, 5, 3, 4, 5, 1, 5; 4, 3, 4, 5, 2, 3\
        , 5, 4, 0, 0, 0, 3, 4, 2, 2, 2, 0, 3, 0, 4, 2, 0, 1, 5, 4, 2, 4, 3, 0, 0, 2, 3; 2, 2, 1, 2\
        , 2, 2, 4, 0, 3, 5, 5, 0, 4, 2, 3, 3, 3, 5, 5, 5, 3, 0, 5, 1, 1, 4, 5, 4, 0, 1, 4, 1; 5, 3\
        , 2, 0, 5, 3, 3, 0, 1, 2, 3, 3, 4, 2, 2, 1, 3, 1, 1, 0, 2, 5, 2, 3, 4, 0, 5, 2, 0, 2, 0, 3\
        ; 4, 3, 4, 1, 2, 5, 4, 1, 5, 5, 0, 0, 5, 1, 4, 0, 0, 2, 3, 3, 5, 0, 1, 1, 3, 0, 3, 5, 3, 5\
        , 0, 5; 4, 4, 4, 4, 2, 2, 4, 2, 2, 4, 4, 2, 2, 4, 2, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 4, 2\
        , 2, 2, 4, 4]",
        6,
        "x^32+3*x^31+4*x^29+3*x^28+3*x^26+4*x^25+2*x^24+5*x^23+5*x^22+2*x^21+2*x^18+x^17+2*x^16+5*\
        x^14+3*x^12+5*x^10+4*x^9+4*x^8+3*x^6+4*x^5+x^4+2*x^3+2*x^2+4*x+2",
    );
    // - the reduction to Hessenberg form, with a dense matrix
    test(
        "[0, 3, 0, 3, 1, 5, 0, 1, 0; 4, 1, 5, 1, 1, 5, 0, 5, 0; 1, 1, 4, 5, 3, 4, 3, 1, 4; 0, 0, 0\
        , 3, 2, 1, 1, 2, 3; 2, 4, 1, 1, 5, 1, 3, 4, 5; 4, 0, 0, 1, 2, 1, 1, 1, 3; 0, 0, 5, 4, 3, 5\
        , 1, 1, 3; 5, 2, 1, 2, 5, 1, 3, 0, 2; 2, 2, 2, 4, 3, 3, 5, 5, 2]",
        6,
        "x^9+x^8+3*x^7+5*x^6+4*x^5+x^4+4*x^3+5*x^2+4*x+4",
    );
    // - the reduction to Hessenberg form, with a zero subdiagonal pivot and a nonzero element below
    //   it: a row and column swap
    test(
        "[0, 3, 0, 3, 1, 5, 0, 1, 0; 0, 1, 5, 1, 1, 5, 0, 5, 0; 0, 1, 4, 5, 3, 4, 3, 1, 4; 0, 0, 0\
        , 3, 2, 1, 1, 2, 3; 0, 4, 1, 1, 5, 1, 3, 4, 5; 3, 0, 0, 1, 2, 1, 1, 1, 3; 0, 0, 5, 4, 3, 5\
        , 1, 1, 3; 0, 2, 1, 2, 5, 1, 3, 0, 2; 0, 2, 2, 4, 3, 3, 5, 5, 2]",
        6,
        "x^9+x^8+3*x^7+5*x^6+x^4+3*x^2+2*x",
    );
    // - the reduction to Hessenberg form, with a column that is already 0 below the subdiagonal
    test(
        "[0, 3, 0, 3, 1, 5, 0, 1, 0; 0, 1, 5, 1, 1, 5, 0, 5, 0; 0, 1, 4, 5, 3, 4, 3, 1, 4; 0, 0, 0\
        , 3, 2, 1, 1, 2, 3; 0, 4, 1, 1, 5, 1, 3, 4, 5; 0, 0, 0, 1, 2, 1, 1, 1, 3; 0, 0, 5, 4, 3, 5\
        , 1, 1, 3; 0, 2, 1, 2, 5, 1, 3, 0, 2; 0, 2, 2, 4, 3, 3, 5, 5, 2]",
        6,
        "x^9+x^8+5*x^6+3*x^5+4*x^4+2*x",
    );
    // - the reduction to Hessenberg form, with a pivot that is not invertible becoming invertible
    test(
        "[0, 3, 0, 3, 1, 5, 0, 1, 0; 2, 1, 5, 1, 1, 5, 0, 5, 0; 3, 1, 4, 5, 3, 4, 3, 1, 4; 0, 0, 0\
        , 3, 2, 1, 1, 2, 3; 0, 4, 1, 1, 5, 1, 3, 4, 5; 0, 0, 0, 1, 2, 1, 1, 1, 3; 0, 0, 5, 4, 3, 5\
        , 1, 1, 3; 0, 2, 1, 2, 5, 1, 3, 0, 2; 0, 2, 2, 4, 3, 3, 5, 5, 2]",
        6,
        "x^9+x^8+2*x^6+3*x^3+x^2+2*x+4",
    );
    // - the largest prime u64 modulus
    test(
        "[14350466822973449815, 3036521976424486597, 9198251665622388387, 7594477879063625992, 429\
        5769332028898661, 6105715379486961722, 3519319813179719357, 5581262911360218106, 815754658\
        9136864308, 7230704071601237894; 12169809761984031736, 16218088818781668543, 9313366450691\
        477473, 3681786729996621016, 9404048119888740716, 2778186421555261127, 1551617513271593359\
        4, 279701317729744294, 5208646954657031081, 17101464748005875502; 10730053401128153883, 63\
        12383512655799182, 644595796247751547, 3305617563223838929, 15442526834356050341, 14182016\
        77618150896, 12818019468156677649, 3003719290845084707, 13973333507208194443, 209362592813\
        2345739; 2396970162589498945, 3614802371231246916, 17545536868536217344, 32778200859423673\
        6, 8804226629501152399, 16755725429624710740, 12464609635681414991, 283958242944624232, 12\
        231025702630813709, 5706118288730067237; 4084811043971622145, 2473096357095988717, 2098179\
        811752029728, 10378875334640705013, 13940502521787334848, 16594327018731396909, 1176706651\
        7788313772, 8068778439533887925, 16846628012864974815, 3125734845879055272; 17033215767750\
        905382, 18116524198336587568, 9968422249903970150, 13745687629055699093, 11390682819087485\
        947, 12873243507164437852, 12544771147715152798, 12733534849223490222, 1249370709187270104\
        0, 8928043559638026967; 3478587955582920577, 6155671233534725809, 6022997133491548313, 126\
        75555821508619955, 2467305641045686010, 12767319111359572653, 8709434787628598205, 1702040\
        5340754919271, 7790713542835768623, 13786371895932704068; 2671626972265935910, 26401033988\
        25823533, 15714405092865192165, 15145922637249839442, 4501660804552683100, 776859605077107\
        0661, 14072831646577183739, 1529146036766279599, 12596422154119560520, 1282646006867297008\
        8; 17534437228609648226, 18395208454983194557, 14299466983098862374, 2160509027728152330, \
        15080737997561777451, 2959609748496860858, 5146558780538585412, 15044929499561473849, 9118\
        729708153099242, 7816513051085455565; 16337015385875086674, 15636350818156241703, 55003977\
        36493538203, 15444724244251486254, 18289643656167804134, 12606256734886132922, 40717955218\
        81180583, 9833771227313567538, 10335982655933138995, 5255378188907569026]",
        18446744073709551557,
        "x^10+9266352171543332372*x^9+5341527412665617331*x^8+6400252256120009440*x^7+170527446818\
        64725470*x^6+14095044865246809422*x^5+14615876619382639065*x^4+9441428280031478579*x^3+806\
        8031884193991486*x^2+12559046643061589008*x+5684566982825695733",
    );
    // - the largest u64 modulus, which is composite
    test(
        "[13717579906771376138, 2182228125071252173, 3134833379495757288, 17529194822794586733, 62\
        44255461219180001, 2733258421488911639, 14425439510390823391, 14783827572732567463, 813063\
        5686104327527, 9003381667211491618; 171298362172149839, 277338032532447648, 16662866266129\
        062172, 17544445322776769788, 13242491143199299525, 12680710224118600332, 7455288635681193\
        066, 5595560568515916418, 13444489189108922514, 14577355631700980933; 1524286156283362346, \
        2151066383461575879, 1276988818830435783, 13007685221730973689, 8903921798980054995, 13919\
        95679950522395, 9259556763471516229, 17289250279309027522, 14171352617312607514, 134889401\
        78649590348; 11797711249851302389, 3644816985373099571, 10048205205955782649, 821716412043\
        6026193, 17307258753573938186, 458083241615332059, 442284428317812360, 587384786819859752, \
        1360390586446865665, 5831883859402956943; 11793949254558298045, 13898793482135367491, 1489\
        2871884003030046, 1500320530680923145, 7663384997222879950, 10824310728662706530, 62207433\
        57923552670, 5019672255913276666, 3179648672190918367, 6380140655112468128; 18089850924730\
        32307, 18421731568019696965, 13335801972843737568, 17504397449233650067, 11267056175211911\
        91, 2087828415662925795, 12047579654876730335, 18010248027667976734, 7426297532894983277, \
        5778408907793568161; 7330949090079517057, 59517256811187923, 3736673102848889131, 27369148\
        51535941268, 2664092843980613652, 17953319687978213323, 12021763926361490404, 153413430181\
        54942318, 7831453044444730417, 9858171696034326476; 6753346137513505876, 85626775967674374\
        94, 14575599168028853723, 12535156769133991553, 2857356016121067071, 4889275769150417815, \
        12063852430880471149, 9376009158791224416, 9920689170271775594, 847330148511166439; 528512\
        2095428141078, 8118116679436616212, 16621383678077820500, 17957186773218805661, 1613285995\
        232077229, 7873120550039550975, 3464356854921991542, 7639739592512344015, 1104768906205188\
        9167, 13578190181617086731; 10818339501791616322, 11469368182140174094, 109084056112814296\
        91, 3369485051960761029, 3488451970909643101, 9114887864618242820, 1338072377495120760, 69\
        99091965607240065, 11526770512583742981, 16121874503498790457]",
        18446744073709551615,
        "x^10+10426099426388272124*x^9+10616699921291967713*x^8+1475277703247132685*x^7+3080068270\
        145503914*x^6+5442773734066456803*x^5+12520294175227584216*x^4+445500572850239682*x^3+2085\
        003885474120935*x^2+11279307012232806230*x+10401111694684826691",
    );
}

#[test]
fn mod_characteristic_polynomial_fail() {
    // - not square
    let a = UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap();
    assert_panic!(a.mod_characteristic_polynomial(5));
    let a = UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap();
    assert_panic!((&a).mod_characteristic_polynomial(5));
    // - not reduced
    let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 9]").unwrap();
    assert_panic!(a.mod_characteristic_polynomial(5));
    // - m = 0
    let a = UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap();
    assert_panic!(a.mod_characteristic_polynomial(0));
}

#[test]
fn mod_characteristic_polynomial_properties() {
    unsigned_matrix_unsigned_pair_gen_var_11().test_properties(|(a, m)| {
        let p = verify(&a, m);
        let n = a.height();
        if m == 1 {
            // Modulo 1, every polynomial is 0.
            assert_eq!(p, UnsignedPolynomial::ZERO);
            return;
        }
        // The polynomial is monic of degree n.
        assert_eq!(p.degree(), Some(n));
        assert_eq!(p.leading_coefficient(), 1);
        // The constant coefficient is (-1)^n times the determinant, and the next-to-leading one is
        // the negative of the trace.
        let det = (&a).mod_determinant(m);
        assert_eq!(
            p.coefficient(0),
            if n.even() { det } else { det.mod_neg(m) }
        );
        if n > 0 {
            assert_eq!(p.coefficient(n - 1), a.to_mod_trace(m).mod_neg(m));
        }
        // Modulo a power of 2, it agrees with the power-of-2 version.
        if m.is_power_of_2() {
            assert_eq!(
                (&a).mod_power_of_2_characteristic_polynomial(m.trailing_zeros().into()),
                p
            );
        }
        // For a prime modulus, Danilevsky's method succeeds.
        if m.is_prime() {
            assert!(unsigned_matrix_mod_characteristic_polynomial_danilevsky(&a, m).is_some());
        }
    });

    fn similarity_properties_helper<T: PrimitiveUnsigned>() {
        unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_1::<T>().test_properties(
            |(a, r, d, m)| {
                // Similar matrices have the same characteristic polynomial.
                assert_eq!(
                    (&a).mod_similarity_transform(r, d, m)
                        .mod_characteristic_polynomial(m),
                    verify(&a, m)
                );
            },
        );
    }
    similarity_properties_helper::<u8>();
    similarity_properties_helper::<u64>();
}
