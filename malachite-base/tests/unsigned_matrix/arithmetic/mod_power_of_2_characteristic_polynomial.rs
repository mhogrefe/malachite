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
    ModPowerOf2, ModPowerOf2CharacteristicPolynomial, ModPowerOf2Determinant, ModPowerOf2Neg,
    ModPowerOf2SimilarityTransform, ModPowerOf2Trace, Parity,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_pair_gen_var_4,
    unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_2,
};
use malachite_base::test_util::unsigned_matrix::arithmetic::{
    mod_power_of_2_characteristic_polynomial::*,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use std::panic::catch_unwind;

// Checks the characteristic polynomial modulo 2^pow of `a` by value and by reference, against
// Berkowitz's algorithm and that of the transpose, and, for small matrices, against the
// Cayley-Hamilton theorem, and returns it.
fn verify<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, pow: u64) -> UnsignedPolynomial<T> {
    let p = a.mod_power_of_2_characteristic_polynomial(pow);
    assert!(p.is_valid());
    assert_eq!(a.clone().mod_power_of_2_characteristic_polynomial(pow), p);
    assert_eq!(
        unsigned_matrix_mod_power_of_2_characteristic_polynomial_berkowitz(a, pow),
        p
    );
    assert_eq!(
        a.transpose().mod_power_of_2_characteristic_polynomial(pow),
        p
    );
    if a.height() <= 8 {
        assert!(unsigned_polynomial_mod_power_of_2_evaluate_at_matrix(&p, a, pow).is_zero());
    }
    p
}

#[test]
fn test_mod_power_of_2_characteristic_polynomial() {
    let test = |s, pow, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        assert_eq!(verify(&a, pow).to_string(), out);
    };
    // - dimension 0
    test("[]_{0,0}", 3, "1");
    // - modulo 1
    test("[0]", 0, "0");
    // - modulo 1, dimension 3
    test("[0, 0, 0; 0, 0, 0; 0, 0, 0]", 0, "0");
    // - dimension 1
    test("[5]", 3, "x+3");
    // - modulo 2
    test("[1, 1; 1, 0]", 1, "x^2+x+1");
    // - dimension 2
    test("[1, 2; 3, 4]", 3, "x^2+3*x+6");
    // - a pivot of smaller valuation lower down: a row and column swap
    test("[1, 2, 3; 4, 5, 6; 7, 0, 1]", 4, "x^3+9*x^2+14*x+8");
    // - an even pivot that divides the elements below it
    test("[0, 1, 1; 2, 0, 1; 4, 1, 0]", 3, "x^3+x+2");
    // - zeros below the subdiagonal entry: nothing to clear
    test("[1, 2, 3; 0, 4, 5; 0, 6, 7]", 3, "x^3+4*x^2+x+2");
    // - a zero column below the diagonal: block triangular
    test(
        "[1, 2, 3, 4; 0, 5, 6, 7; 0, 0, 1, 2; 0, 0, 3, 4]",
        4,
        "x^4+5*x^3+x^2+3*x+6",
    );
    // - a nilpotent matrix
    test("[0, 1, 0, 0; 0, 0, 1, 0; 0, 0, 0, 1; 0, 0, 0, 0]", 8, "x^4");
    // - modulo 2^8, the full width of a u8
    test(
        "[3, 1, 4, 1, 5; 9, 2, 6, 5, 3; 5, 8, 9, 7, 9; 3, 2, 3, 8, 4; 6, 2, 6, 4, 3]",
        8,
        "x^5+231*x^4+12*x^3+77*x^2+222*x+21",
    );
    // - all 255
    test(
        "[255, 255, 255, 255, 255, 255; 255, 255, 255, 255, 255, 255; 255, 255, 255, 255, 255, 255\
        ; 255, 255, 255, 255, 255, 255; 255, 255, 255, 255, 255, 255; 255, 255, 255, 255, 255, 255\
        ]",
        8,
        "x^6+6*x^5",
    );
    // - dimension 10
    test(
        "[165, 77, 202, 24, 37, 48, 187, 29, 109, 19; 44, 222, 214, 35, 123, 46, 217, 30, 63, 114; \
        31, 203, 25, 113, 23, 68, 148, 214, 73, 60; 157, 92, 52, 96, 190, 49, 32, 30, 105, 254; 21\
        8, 160, 238, 232, 185, 153, 127, 92, 124, 41; 153, 253, 175, 229, 147, 37, 60, 214, 84, 17\
        5; 77, 250, 215, 20, 39, 160, 174, 179, 254, 233; 35, 47, 138, 242, 33, 31, 158, 228, 145, \
        197; 177, 11, 236, 181, 86, 59, 252, 30, 111, 147; 66, 126, 203, 200, 254, 41, 85, 229, 20\
        5, 142]",
        8,
        "x^10+151*x^9+106*x^8+121*x^7+111*x^6+92*x^5+119*x^4+117*x^3+218*x^2+242*x+234",
    );
    // - even elements: the coefficients of x^(n-k) are divisible by 2^k
    test(
        "[70, 220, 142, 212, 182, 194; 118, 76, 42, 90, 76, 118; 118, 6, 248, 92, 134, 144; 2, 74, \
        214, 188, 162, 64; 26, 232, 200, 202, 204, 200; 52, 246, 204, 30, 96, 34]",
        8,
        "x^6+204*x^5+100*x^4+232*x^3+128*x^2+64*x+192",
    );

    let test = |s, pow, out| {
        let a = UnsignedMatrix::<u64>::from_str(s).unwrap();
        assert_eq!(verify(&a, pow).to_string(), out);
    };
    // - modulo 2^64, the full width of a u64
    test(
        "[10835337665455605072, 3434298343398869075, 5596765429999857094, 17796639924064796018, 10\
        98524073332020107; 16225109976529599755, 6019049990461352110, 15540037672059346467, 921918\
        7472956490868, 1567766003366382319; 5131199888064844944, 15499241312481373247, 46853760080\
        20493052, 548826014839375280, 14135516583701387087; 7129916304139001011, 10052296588488942\
        , 8669734981505188722, 13076881145020861865, 4976481267635116274; 5740804462466766099, 177\
        84067281842656897, 17023535457172910436, 202084392168729537, 14702451914164292652]",
        64,
        "x^5+6021135498006050097*x^4+9615826589058188524*x^3+7172908048171272634*x^2+1090868209597\
        0781954*x+497463742513606160",
    );
    // - modulo 2^37
    test(
        "[80981804368, 24880646227, 6213070278, 104780709234, 81875669899; 65241447691, 8359696196\
        6, 58372726307, 39918077044, 135989185263; 123071435920, 115670707263, 33104790268, 112910\
        566832, 71138154319; 112918568115, 11531546862, 34584462194, 128764661161, 105977927922; 1\
        29223300371, 103144281729, 59090334052, 64858500033, 130845920300]",
        37,
        "x^5+92461675825*x^4+94465279212*x^3+127671077306*x^2+76537694978*x+14447865360",
    );
}

#[test]
fn mod_power_of_2_characteristic_polynomial_fail() {
    // - not square
    let a = UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap();
    assert_panic!(a.mod_power_of_2_characteristic_polynomial(3));
    // - not reduced
    let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 9]").unwrap();
    assert_panic!(a.mod_power_of_2_characteristic_polynomial(3));
    // - pow too large
    let a = UnsignedMatrix::<u8>::from_str("[1]").unwrap();
    assert_panic!(a.mod_power_of_2_characteristic_polynomial(9));
    let a = UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap();
    assert_panic!((&a).mod_power_of_2_characteristic_polynomial(3));
}

#[test]
fn mod_power_of_2_characteristic_polynomial_properties() {
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        if !a.is_square() {
            return;
        }
        let p = verify(&a, pow);
        let n = a.height();
        if pow == 0 {
            // Modulo 1, every polynomial is 0.
            assert_eq!(p, UnsignedPolynomial::ZERO);
            return;
        }
        // The polynomial is monic of degree n.
        assert_eq!(p.degree(), Some(n));
        assert_eq!(p.leading_coefficient(), 1);
        // The constant coefficient is (-1)^n times the determinant, and the next-to-leading one is
        // the negative of the trace.
        let det = (&a).mod_power_of_2_determinant(pow);
        assert_eq!(
            p.coefficient(0),
            if n.even() {
                det
            } else {
                det.mod_power_of_2_neg(pow)
            }
        );
        if n > 0 {
            assert_eq!(
                p.coefficient(n - 1),
                a.to_mod_power_of_2_trace(pow).mod_power_of_2_neg(pow)
            );
        }
        // Reducing the matrix modulo a smaller power of 2 reduces the coefficients.
        for smaller in [0, pow >> 1, pow - 1] {
            let q = (&a)
                .mod_power_of_2(smaller)
                .mod_power_of_2_characteristic_polynomial(smaller);
            assert_eq!(
                q,
                UnsignedPolynomial::from_coefficients_asc(
                    p.coefficients_asc()
                        .iter()
                        .map(|&c| c.mod_power_of_2(smaller))
                        .collect()
                )
            );
        }
    });

    fn similarity_properties_helper<T: PrimitiveUnsigned>() {
        unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_2::<T>().test_properties(
            |(a, r, d, pow)| {
                // Similar matrices have the same characteristic polynomial.
                assert_eq!(
                    (&a).mod_power_of_2_similarity_transform(r, d, pow)
                        .mod_power_of_2_characteristic_polynomial(pow),
                    verify(&a, pow)
                );
            },
        );
    }
    similarity_properties_helper::<u8>();
    similarity_properties_helper::<u64>();
}
