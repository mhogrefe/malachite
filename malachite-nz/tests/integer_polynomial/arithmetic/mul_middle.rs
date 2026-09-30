// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Mod, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::classical::*;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::fft::mul_middle_to_out_fft;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::kronecker::*;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::mul_middle_to_out;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::schonhage_strassen::{
    mul_middle_to_out_schonhage_strassen, vec_get_fft, vec_set_fft,
};
use malachite_nz::integer_polynomial::arithmetic::mul_middle::tiny::{
    mul_middle_to_out_tiny_1, mul_middle_to_out_tiny_2,
};
use malachite_nz::integer_polynomial::arithmetic::vec::max_bits::vec_max_bits;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::normmod_2expp1::*;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1,
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_2,
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_3,
    integer_vec_unsigned_unsigned_triple_gen_var_1, integer_vec_unsigned_unsigned_triple_gen_var_2,
    integer_vec_unsigned_unsigned_triple_gen_var_3, large_type_gen_var_57, large_type_gen_var_58,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::{
    generated_coefficients, integers_mul_naive,
};
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

fn coefficients(p: &str) -> Vec<Integer> {
    IntegerPolynomial::from_str(p)
        .unwrap()
        .into_coefficients_asc()
}

fn parse(xs: &[&str]) -> Vec<Integer> {
    xs.iter().map(|x| Integer::from_str(x).unwrap()).collect()
}

#[test]
fn test_mul_middle_to_out_classical() {
    let test = |xs: &[&str], ys: &[&str], nlo: usize, nhi: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_classical(&mut result, &xs, &ys, nlo, nhi);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], nlo: usize, nhi: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_classical(&mut result, &xs, &xs, nlo, nhi);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[nlo..nhi], result);
    };
    // - xs.len() == 1
    test(&["3"], &["4", "5", "6"], 0, 2, &["12", "15"]);
    // - ys.len() == 1
    test(&["4", "5", "6"], &["3"], 1, 3, &["15", "18"]);
    // - xs and ys are the same slice
    test_square(&["1", "2", "3"], 0, 5, &["1", "4", "10", "12", "9"]);
    test_square(&["1", "2", "3"], 1, 3, &["4", "10"]);
    // - xs and ys are different slices
    test(
        &["1", "2", "3"],
        &["4", "5", "6"],
        1,
        4,
        &["13", "28", "27"],
    );
    test(
        &["1180591620717411303424", "-3", "36893488147419103233"],
        &["-18446744073709551616", "5"],
        1,
        3,
        &["5958298335808185171968", "-680564733841876926945195958937245974543"],
    );
    test_square(
        &["1180591620717411303424", "-3", "36893488147419103233"],
        2,
        5,
        &[
            "87112285931760246648985082743967484739593",
            "-221360928884514619398",
            "1361129467683753853927285406021911052289",
        ],
    );
}

#[test]
fn test_mul_middle_to_out_tiny_1() {
    let test = |xs: &[&str], ys: &[&str], nlo: usize, nhi: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_tiny_1(&mut result, &xs, &ys, nlo, nhi);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], nlo: usize, nhi: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_tiny_1(&mut result, &xs, &xs, nlo, nhi);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[nlo..nhi], result);
    };
    // - xs and ys are the same slice
    // - i even
    test_square(&["1", "2", "3"], 0, 5, &["1", "4", "10", "12", "9"]);
    // - xs and ys are different slices
    test(
        &["1", "2", "3"],
        &["4", "5", "6"],
        1,
        4,
        &["13", "28", "27"],
    );
    test(
        &["1", "1", "1", "1", "1", "1", "1", "1"],
        &["-1", "-1", "-1", "-1", "-1", "-1", "-1", "-1"],
        3,
        12,
        &["-4", "-5", "-6", "-7", "-8", "-7", "-6", "-5", "-4"],
    );
}

#[test]
fn test_mul_middle_to_out_tiny_2() {
    let test = |xs: &[&str], ys: &[&str], nlo: usize, nhi: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_tiny_2(&mut result, &xs, &ys, nlo, nhi);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], nlo: usize, nhi: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_tiny_2(&mut result, &xs, &xs, nlo, nhi);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[nlo..nhi], result);
    };
    // - xs and ys are the same slice
    // - i even
    #[cfg(not(feature = "32_bit_limbs"))]
    test_square(
        &["2305843009213693952", "-2305843009213693951", "7"],
        0,
        5,
        &[
            "5316911983139663491615228241121378304",
            "-10633823966279326978618770463815368704",
            "5316911983139663519285344351685705729",
            "-32281802128991715314",
            "49",
        ],
    );
    #[cfg(feature = "32_bit_limbs")]
    test_square(
        &["536870912", "-536870911", "0"],
        0,
        5,
        &["288230376151711744", "-576460751229681664", "288230375077969921", "0", "0"],
    );
    // - xs and ys are different slices
    #[cfg(not(feature = "32_bit_limbs"))]
    test(
        &["2305843009213693952", "-2305843009213693951"],
        &["2305843009213693951", "1152921504606846976"],
        1,
        3,
        &["-2658455991569831741195928102133301249", "-2658455991569831744654692615953842176"],
    );
    #[cfg(feature = "32_bit_limbs")]
    test(
        &["536870912", "-536870911"],
        &["536870911", "268435456"],
        1,
        3,
        &["-144115187002114049", "-144115187807420416"],
    );
    #[cfg(not(feature = "32_bit_limbs"))]
    test(
        &["-1099511627776", "5", "1125899906842624"],
        &["3", "-35184372088832", "1048576", "1"],
        2,
        5,
        &["-1149719726746763264", "-39614081257132169896278360064", "1180591620717411303429"],
    );
    #[cfg(feature = "32_bit_limbs")]
    test(
        &["-256", "0", "262144"],
        &["0", "-8192", "0", "0"],
        2,
        5,
        &["0", "-2147483648", "0"],
    );
}

#[test]
fn test_mul_middle_to_out() {
    let test = |xs: &[&str], ys: &[&str], nlo: usize, nhi: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out(&mut result, &xs, &ys, nlo, nhi);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], nlo: usize, nhi: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out(&mut result, &xs, &xs, nlo, nhi);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[nlo..nhi], result);
    };
    // Long inputs, written as polynomials.
    let test_poly = |p: &str, q: &str, nlo: usize, nhi: usize, out: &str| {
        let xs = coefficients(p);
        let ys = coefficients(q);
        let mut result = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out(&mut result, &xs, &ys, nlo, nhi);
        assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], result);
        assert_eq!(
            IntegerPolynomial::from_coefficients_asc(result).to_string(),
            out
        );
    };
    // - xs.len() > nlo2
    // - ys.len() > nlo2
    // - ys.len() == 1
    test(&["1", "2", "3"], &["4", "5"], 3, 4, &["15"]);
    // - xs.len() < ys.len()
    // - len2 < 50
    // - rbits <= SMALL_FMPZ_BITCOUNT_MAX
    test(
        &["1", "2"],
        &["3", "4", "5"],
        0,
        4,
        &["3", "10", "13", "10"],
    );
    // - xs and ys are the same slice
    test_square(&["1", "2", "3"], 1, 4, &["4", "10", "12"]);
    // - rbits > SMALL_FMPZ_BITCOUNT_MAX && rbits < 2 * Limb::WIDTH
    test(
        &["2305843009213693952", "-2305843009213693951"],
        &["2305843009213693951", "1152921504606846976"],
        0,
        2,
        &["5316911983139663489309385231907684352", "-2658455991569831741195928102133301249"],
    );
    // - rbits >= 2 * Limb::WIDTH
    test(
        &[
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
        ],
        &[
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
        ],
        0,
        15,
        &[
            "21267647932558653957237540927630737409",
            "42535295865117307914475081855261474818",
            "63802943797675961871712622782892212227",
            "85070591730234615828950163710522949636",
            "106338239662793269786187704638153687045",
            "127605887595351923743425245565784424454",
            "148873535527910577700662786493415161863",
            "170141183460469231657900327421045899272",
            "148873535527910577700662786493415161863",
            "127605887595351923743425245565784424454",
            "106338239662793269786187704638153687045",
            "85070591730234615828950163710522949636",
            "63802943797675961871712622782892212227",
            "42535295865117307914475081855261474818",
            "21267647932558653957237540927630737409",
        ],
    );
    // - bits1 > SMALL_FMPZ_BITCOUNT_MAX || bits2 > SMALL_FMPZ_BITCOUNT_MAX
    // - classical
    test(
        &["1180591620717411303424", "-3", "36893488147419103233"],
        &["-18446744073709551616", "5"],
        1,
        3,
        &["5958298335808185171968", "-680564733841876926945195958937245974543"],
    );
    // - len2 >= 50 && 4 * len2 >= 3 * len && len < 150 + bits1 + bits2
    test_poly("x^59+1", "-x^59+2*x^30+1", 0, 60, "2*x^30+1");
    // Generated coefficients of `bits1` and `bits2` bits.
    let test_generated =
        |len1: usize, len2: usize, bits1: u64, bits2: u64, nlo: usize, nhi: usize| {
            let xs = generated_coefficients(len1, bits1);
            let ys = generated_coefficients(len2, bits2);
            let mut result = vec![Integer::ZERO; nhi - nlo];
            mul_middle_to_out(&mut result, &xs, &ys, nlo, nhi);
            assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], result);
        };
    // - nhi <= 8
    test_generated(8, 7, 100, 90, 2, 8);
    // - nhi > 8 && !classical_preferred(len2, bits1, bits2) && len <= 3
    test_generated(12, 12, 100, 90, 9, 12);
    // - nlo == 0 && karatsuba_preferred(len2, bits1, bits2)
    test_generated(8, 7, 1000, 900, 0, 12);
    // - nlo != 0, so Karatsuba is not used
    test_generated(8, 7, 1000, 900, 2, 12);
    // - !karatsuba_preferred(len2, bits1, bits2)
    test_generated(8, 7, 100, 90, 0, 12);
    // - fft_preferred(len2, bits1, bits2, 100, 200), and the FFT computes the product
    test_generated(120, 110, 20, 20, 30, 150);
    // - fft_preferred(len2, bits1, bits2, 100, 200), but the FFT declines
    test_generated(120, 110, 250, 250, 30, 150);
}

#[test]
fn mul_middle_to_out_classical_properties() {
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1().test_properties(
        |(xs, ys, nlo, nhi)| {
            let nlo = usize::exact_from(nlo);
            let nhi = usize::exact_from(nhi);
            let mut out = vec![Integer::ZERO; nhi - nlo];
            mul_middle_to_out_classical(&mut out, &xs, &ys, nlo, nhi);
            assert!(out.iter().all(Integer::is_valid));
            assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], out);
            // Multiplication is commutative.
            let mut out_alt = vec![Integer::ZERO; nhi - nlo];
            mul_middle_to_out_classical(&mut out_alt, &ys, &xs, nlo, nhi);
            assert_eq!(out_alt, out);
            mul_middle_to_out(&mut out_alt, &xs, &ys, nlo, nhi);
            assert_eq!(out_alt, out);
        },
    );

    // The square path.
    integer_vec_unsigned_unsigned_triple_gen_var_1().test_properties(|(xs, nlo, nhi)| {
        let nlo = usize::exact_from(nlo);
        let nhi = usize::exact_from(nhi);
        let mut out = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_classical(&mut out, &xs, &xs, nlo, nhi);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &xs)[nlo..nhi], out);
        let mut out_alt = vec![Integer::ZERO; nhi - nlo];
        // The general path agrees.
        let xs_alt = xs.clone();
        mul_middle_to_out_classical(&mut out_alt, &xs, &xs_alt, nlo, nhi);
        assert_eq!(out_alt, out);
        mul_middle_to_out(&mut out_alt, &xs, &xs, nlo, nhi);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn mul_middle_to_out_tiny_1_properties() {
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_2().test_properties(
        |(xs, ys, nlo, nhi)| {
            let nlo = usize::exact_from(nlo);
            let nhi = usize::exact_from(nhi);
            let mut out = vec![Integer::ZERO; nhi - nlo];
            mul_middle_to_out_tiny_1(&mut out, &xs, &ys, nlo, nhi);
            assert!(out.iter().all(Integer::is_valid));
            assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], out);
            // Multiplication is commutative.
            let mut out_alt = vec![Integer::ZERO; nhi - nlo];
            mul_middle_to_out_tiny_1(&mut out_alt, &ys, &xs, nlo, nhi);
            assert_eq!(out_alt, out);
            mul_middle_to_out_tiny_2(&mut out_alt, &xs, &ys, nlo, nhi);
            assert_eq!(out_alt, out);
            mul_middle_to_out_classical(&mut out_alt, &xs, &ys, nlo, nhi);
            assert_eq!(out_alt, out);
        },
    );

    // The square path.
    integer_vec_unsigned_unsigned_triple_gen_var_2().test_properties(|(xs, nlo, nhi)| {
        let nlo = usize::exact_from(nlo);
        let nhi = usize::exact_from(nhi);
        let mut out = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_tiny_1(&mut out, &xs, &xs, nlo, nhi);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &xs)[nlo..nhi], out);
        let mut out_alt = vec![Integer::ZERO; nhi - nlo];
        // The general path agrees.
        let xs_alt = xs.clone();
        mul_middle_to_out_tiny_1(&mut out_alt, &xs, &xs_alt, nlo, nhi);
        assert_eq!(out_alt, out);
        mul_middle_to_out_tiny_2(&mut out_alt, &xs, &xs, nlo, nhi);
        assert_eq!(out_alt, out);
        mul_middle_to_out_classical(&mut out_alt, &xs, &xs, nlo, nhi);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn mul_middle_to_out_tiny_2_properties() {
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_3().test_properties(
        |(xs, ys, nlo, nhi)| {
            let nlo = usize::exact_from(nlo);
            let nhi = usize::exact_from(nhi);
            let mut out = vec![Integer::ZERO; nhi - nlo];
            mul_middle_to_out_tiny_2(&mut out, &xs, &ys, nlo, nhi);
            assert!(out.iter().all(Integer::is_valid));
            assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], out);
            // Multiplication is commutative.
            let mut out_alt = vec![Integer::ZERO; nhi - nlo];
            mul_middle_to_out_tiny_2(&mut out_alt, &ys, &xs, nlo, nhi);
            assert_eq!(out_alt, out);
            mul_middle_to_out_classical(&mut out_alt, &xs, &ys, nlo, nhi);
            assert_eq!(out_alt, out);
        },
    );

    // The square path.
    integer_vec_unsigned_unsigned_triple_gen_var_3().test_properties(|(xs, nlo, nhi)| {
        let nlo = usize::exact_from(nlo);
        let nhi = usize::exact_from(nhi);
        let mut out = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_tiny_2(&mut out, &xs, &xs, nlo, nhi);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &xs)[nlo..nhi], out);
        let mut out_alt = vec![Integer::ZERO; nhi - nlo];
        // The general path agrees.
        let xs_alt = xs.clone();
        mul_middle_to_out_tiny_2(&mut out_alt, &xs, &xs_alt, nlo, nhi);
        assert_eq!(out_alt, out);
        mul_middle_to_out_classical(&mut out_alt, &xs, &xs, nlo, nhi);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn mul_middle_to_out_properties() {
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1().test_properties(
        |(xs, ys, nlo, nhi)| {
            let nlo = usize::exact_from(nlo);
            let nhi = usize::exact_from(nhi);
            let mut out = vec![Integer::ZERO; nhi - nlo];
            mul_middle_to_out(&mut out, &xs, &ys, nlo, nhi);
            assert!(out.iter().all(Integer::is_valid));
            assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], out);
            // Multiplication is commutative.
            let mut out_alt = vec![Integer::ZERO; nhi - nlo];
            mul_middle_to_out(&mut out_alt, &ys, &xs, nlo, nhi);
            assert_eq!(out_alt, out);
        },
    );

    // The square path.
    integer_vec_unsigned_unsigned_triple_gen_var_1().test_properties(|(xs, nlo, nhi)| {
        let nlo = usize::exact_from(nlo);
        let nhi = usize::exact_from(nhi);
        let mut out = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out(&mut out, &xs, &xs, nlo, nhi);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &xs)[nlo..nhi], out);
        let mut out_alt = vec![Integer::ZERO; nhi - nlo];
        // The general path agrees.
        let xs_alt = xs.clone();
        mul_middle_to_out(&mut out_alt, &xs, &xs_alt, nlo, nhi);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn test_mul_middle_to_out_kronecker() {
    let test = |xs: &[&str], ys: &[&str], nlo: usize, nhi: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_kronecker(&mut result, &xs, &ys, nlo, nhi);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], nlo: usize, nhi: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_kronecker(&mut result, &xs, &xs, nlo, nhi);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[nlo..nhi], result);
    };
    // - !sign, so the unsigned unpacking is used
    test(
        &["1", "2", "3"],
        &["4", "5", "6"],
        0,
        5,
        &["4", "13", "28", "27", "18"],
    );
    // - sign
    test(
        &["1", "-2", "3"],
        &["4", "5", "-6"],
        0,
        5,
        &["4", "-3", "-4", "27", "-18"],
    );
    test(
        &["3"],
        &[
            "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1",
            "1", "1", "1",
        ],
        0,
        20,
        &[
            "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3",
            "3", "3", "3",
        ],
    );
    test(
        &["1", "2", "3", "4", "5", "6"],
        &["3"],
        2,
        6,
        &["9", "12", "15", "18"],
    );
    // - the leading coefficients are zero, and are stripped
    // - nhi > full_len, so zero_high != 0
    test(
        &["1", "2", "0", "0"],
        &["3", "0"],
        0,
        5,
        &["3", "6", "0", "0", "0"],
    );
    // - xs.is_empty() after stripping
    test(&["0", "0"], &["5", "7"], 0, 3, &["0", "0", "0"]);
    // - nlo >= min(nhi, full_len) after stripping
    test(&["1", "0", "0"], &["1", "0", "0"], 2, 5, &["0", "0", "0"]);
    // - xs.len() > nlo2
    test(&["1", "2", "3", "4"], &["5", "6"], 4, 5, &["24"]);
    // - ys.len() > nlo2
    test(&["5", "6"], &["1", "2", "3", "4"], 4, 5, &["24"]);
    // - xs and ys are the same slice, so the packed input is squared
    test_square(&["1", "-2", "3"], 0, 5, &["1", "-4", "10", "-12", "9"]);
    test_square(
        &[
            "1177081956389695314697442642252",
            "780797157184699260681239349366",
            "-595154718217192200576044492408",
            "-182494249513761438690895755667",
            "-942052997583205342725389406559",
        ],
        2,
        7,
        &[
            "-791447559479592902668681632442021440344727617258884343443676",
            "-1359011600633143864688817143866501833720627245321956658510824",
            "-2148520014665647297360674516384939001172466325327284043737516",
            "-1253879977569340881831272259252236545363653423778406816830916",
            "1154638723750178658882415895927897734172734595161513964423033",
        ],
    );
    // - the leading coefficient of xs is negative
    test(&["3", "-1"], &["2", "2"], 0, 3, &["6", "4", "-2"]);
    test(
        &[
            "-1219809464491112424001080559307",
            "841051527322013215685593233302",
            "-699719491634243930371614136486",
            "-1124264126040579300201032760787",
            "-616245236613870071598110773797",
            "304428441122895878829422060984",
        ],
        &[
            "-39596108692041012711732684877",
            "-497407310794063395325018870327",
            "-975716784831680884582779126524",
            "-81585352299193465116417615994",
        ],
        1,
        8,
        &[
            "573439777722221257976973109959126635386749168227746557809535",
            "799549559400729501322001644873734886474392496727868148851336",
            "-328547432040767729704441807453204433842115173887652932576969",
            "1197728676429079352423188246209632307987320362400752103357794",
            "1448520943887611629521790899104048902739782186779076775254123",
            "541579373508920747537304524486530680068238355901212252097138",
            "-246759355051910205285956390962732536086211179583218036630398",
        ],
    );
    test(
        &["16", "-14", "5", "-14", "-2", "4", "-16", "-19", "-20", "-7", "-7", "-17", "10"],
        &["4", "5", "6", "-16", "16", "20", "-8", "-3", "1"],
        3,
        17,
        &[
            "-371", "432", "-62", "-160", "-160", "-629", "121", "8", "-489", "-561", "-248",
            "281", "-475", "-123",
        ],
    );
}

#[test]
fn test_mul_middle_to_out_schonhage_strassen() {
    let test = |xs: &[&str], ys: &[&str], nlo: usize, nhi: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_schonhage_strassen(&mut result, &xs, &ys, nlo, nhi);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], nlo: usize, nhi: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_schonhage_strassen(&mut result, &xs, &xs, nlo, nhi);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[nlo..nhi], result);
    };
    // - nhi > 2
    // - xs.len() >= ys.len()
    // - !square
    // - limbs <= FFT_MULMOD_2EXPP1_CUTOFF
    // - !square
    // - res_bits != 0
    test(
        &["1", "2", "3"],
        &["4", "5", "6"],
        0,
        5,
        &["4", "13", "28", "27", "18"],
    );
    test(
        &["1", "-2", "3"],
        &["4", "5", "-6"],
        0,
        5,
        &["4", "-3", "-4", "27", "-18"],
    );
    // - xs.len() < ys.len()
    test(
        &["3"],
        &[
            "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1",
            "1", "1", "1",
        ],
        0,
        20,
        &[
            "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3",
            "3", "3", "3",
        ],
    );
    test(
        &["1", "2", "3", "4", "5", "6"],
        &["3"],
        2,
        6,
        &["9", "12", "15", "18"],
    );
    test(
        &["1", "2", "0", "0"],
        &["3", "0"],
        0,
        5,
        &["3", "6", "0", "0", "0"],
    );
    test(&["0", "0"], &["5", "7"], 0, 3, &["0", "0", "0"]);
    test(&["1", "0", "0"], &["1", "0", "0"], 2, 5, &["0", "0", "0"]);
    // - nhi <= 2
    test(&["1", "2", "3", "4"], &["5", "6"], 4, 5, &["24"]);
    test(&["5", "6"], &["1", "2", "3", "4"], 4, 5, &["24"]);
    // - square
    // - square
    test_square(&["1", "-2", "3"], 0, 5, &["1", "-4", "10", "-12", "9"]);
    test_square(
        &[
            "1177081956389695314697442642252",
            "780797157184699260681239349366",
            "-595154718217192200576044492408",
            "-182494249513761438690895755667",
            "-942052997583205342725389406559",
        ],
        2,
        7,
        &[
            "-791447559479592902668681632442021440344727617258884343443676",
            "-1359011600633143864688817143866501833720627245321956658510824",
            "-2148520014665647297360674516384939001172466325327284043737516",
            "-1253879977569340881831272259252236545363653423778406816830916",
            "1154638723750178658882415895927897734172734595161513964423033",
        ],
    );
    test(&["3", "-1"], &["2", "2"], 0, 3, &["6", "4", "-2"]);
    test(
        &[
            "-1219809464491112424001080559307",
            "841051527322013215685593233302",
            "-699719491634243930371614136486",
            "-1124264126040579300201032760787",
            "-616245236613870071598110773797",
            "304428441122895878829422060984",
        ],
        &[
            "-39596108692041012711732684877",
            "-497407310794063395325018870327",
            "-975716784831680884582779126524",
            "-81585352299193465116417615994",
        ],
        1,
        8,
        &[
            "573439777722221257976973109959126635386749168227746557809535",
            "799549559400729501322001644873734886474392496727868148851336",
            "-328547432040767729704441807453204433842115173887652932576969",
            "1197728676429079352423188246209632307987320362400752103357794",
            "1448520943887611629521790899104048902739782186779076775254123",
            "541579373508920747537304524486530680068238355901212252097138",
            "-246759355051910205285956390962732536086211179583218036630398",
        ],
    );
    test(
        &["16", "-14", "5", "-14", "-2", "4", "-16", "-19", "-20", "-7", "-7", "-17", "10"],
        &["4", "5", "6", "-16", "16", "20", "-8", "-3", "1"],
        3,
        17,
        &[
            "-371", "432", "-62", "-160", "-160", "-629", "121", "8", "-489", "-561", "-248",
            "281", "-475", "-123",
        ],
    );
    // - res_bits == 0
    test(&["0", "0", "0"], &["0"], 0, 3, &["0", "0", "0"]);
}

#[test]
fn mul_middle_to_out_kronecker_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_len_n", 32);
    let test = |(xs, ys, nlo, nhi): (Vec<Integer>, Vec<Integer>, u64, u64)| {
        let nlo = usize::exact_from(nlo);
        let nhi = usize::exact_from(nhi);
        let mut out = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_kronecker(&mut out, &xs, &ys, nlo, nhi);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], out);
        let mut out_alt = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_kronecker(&mut out_alt, &ys, &xs, nlo, nhi);
        assert_eq!(out_alt, out);
        mul_middle_to_out_classical(&mut out_alt, &xs, &ys, nlo, nhi);
        assert_eq!(out_alt, out);
    };
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1().test_properties(test);
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1()
        .test_properties_with_config(&config, test);

    // The square path.
    let test = |(xs, nlo, nhi): (Vec<Integer>, u64, u64)| {
        let nlo = usize::exact_from(nlo);
        let nhi = usize::exact_from(nhi);
        let mut out = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_kronecker(&mut out, &xs, &xs, nlo, nhi);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &xs)[nlo..nhi], out);
    };
    integer_vec_unsigned_unsigned_triple_gen_var_1().test_properties(test);
    integer_vec_unsigned_unsigned_triple_gen_var_1().test_properties_with_config(&config, test);
}

// Coefficients large enough that the residues have more than `FFT_MULMOD_2EXPP1_CUTOFF` limbs. The
// inputs are too large to write out, so the result is checked against the naive product.
#[test]
fn test_mul_middle_to_out_schonhage_strassen_large() {
    let test = |bits: u64, len1: usize, len2: usize, nlo: usize, nhi: usize| {
        let xs: Vec<Integer> = (0..len1)
            .map(|i| Integer::power_of_2(bits) - Integer::from(i))
            .collect();
        let ys: Vec<Integer> = (0..len2)
            .map(|i| -Integer::power_of_2(bits - 1) + Integer::from(i * i))
            .collect();
        let mut out = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_schonhage_strassen(&mut out, &xs, &ys, nlo, nhi);
        assert_eq!(out, &integers_mul_naive(&xs, &ys)[nlo..nhi]);
    };
    // - nhi > 2
    // - xs.len() >= ys.len()
    // - !square
    // - limbs > FFT_MULMOD_2EXPP1_CUTOFF
    // - !square
    // - res_bits != 0
    test(5000, 8, 8, 0, 15);
    test(5000, 10, 9, 3, 12);
}

#[test]
fn mul_middle_to_out_schonhage_strassen_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_len_n", 32);
    let mut wide_config = GenConfig::new();
    wide_config.insert("mean_len_n", 20);
    wide_config.insert("mean_bits_n", 1000);
    let test = |(xs, ys, nlo, nhi): (Vec<Integer>, Vec<Integer>, u64, u64)| {
        let nlo = usize::exact_from(nlo);
        let nhi = usize::exact_from(nhi);
        let mut out = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_schonhage_strassen(&mut out, &xs, &ys, nlo, nhi);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], out);
        let mut out_alt = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_schonhage_strassen(&mut out_alt, &ys, &xs, nlo, nhi);
        assert_eq!(out_alt, out);
        mul_middle_to_out_classical(&mut out_alt, &xs, &ys, nlo, nhi);
        assert_eq!(out_alt, out);
    };
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1().test_properties(test);
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1()
        .test_properties_with_config(&config, test);
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1()
        .test_properties_with_config(&wide_config, test);

    // The square path.
    let test = |(xs, nlo, nhi): (Vec<Integer>, u64, u64)| {
        let nlo = usize::exact_from(nlo);
        let nhi = usize::exact_from(nhi);
        let mut out = vec![Integer::ZERO; nhi - nlo];
        mul_middle_to_out_schonhage_strassen(&mut out, &xs, &xs, nlo, nhi);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &xs)[nlo..nhi], out);
    };
    integer_vec_unsigned_unsigned_triple_gen_var_1().test_properties(test);
    integer_vec_unsigned_unsigned_triple_gen_var_1().test_properties_with_config(&config, test);
    integer_vec_unsigned_unsigned_triple_gen_var_1()
        .test_properties_with_config(&wide_config, test);
}

#[test]
fn test_mul_middle_to_out_fft() {
    let test = |xs: &[&str], ys: &[&str], nlo: usize, nhi: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; nhi - nlo];
        assert!(mul_middle_to_out_fft(&mut result, &xs, &ys, nlo, nhi));
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], result);
    };
    // - np == 1, and the coefficients are nonnegative
    test(
        &["1", "2", "3"],
        &["4", "5", "6"],
        0,
        5,
        &["4", "13", "28", "27", "18"],
    );
    // - np == 1, and some coefficients are negative
    test(
        &["1", "-2", "3"],
        &["4", "5", "-6"],
        0,
        5,
        &["4", "-3", "-4", "27", "-18"],
    );
    test(&["7"], &["3", "-2"], 1, 2, &["-14"]);
    // Generated coefficients of `bits` bits. If `fits`, the product is computed, and the entries of
    // the result past the product's length are zero; otherwise the function declines.
    let test_generated =
        |len1: usize, len2: usize, bits: u64, nlo: usize, nhi: usize, square: bool, fits: bool| {
            let xs = generated_coefficients(len1, bits);
            let ys = if square {
                xs.clone()
            } else {
                generated_coefficients(len2, bits)
            };
            let ys: &[Integer] = if square { &xs } else { &ys };
            let full = integers_mul_naive(&xs, ys);
            let mut result = vec![Integer::ZERO; nhi - nlo];
            assert_eq!(mul_middle_to_out_fft(&mut result, &xs, ys, nlo, nhi), fits);
            if fits {
                for (i, r) in (nlo..nhi).zip(&result) {
                    assert_eq!(*r, full.get(i).cloned().unwrap_or_default());
                }
            }
        };
    // - np == 2
    test_generated(20, 15, 30, 0, 34, false, true);
    // - np == 3
    // - the coefficients have fewer bits than the prime, and some are negative
    test_generated(300, 200, 40, 0, 499, false, true);
    // - the coefficients have more bits than the prime, but at most 62
    test_generated(40, 30, 55, 10, 60, false, true);
    // - the coefficients have more than 62 bits
    // - np == 4
    test_generated(30, 30, 80, 0, 59, false, true);
    // - np == 5
    test_generated(30, 30, 100, 3, 50, false, true);
    // - np == 6
    test_generated(30, 20, 130, 0, 49, false, true);
    // - np == 7
    test_generated(30, 20, 160, 0, 49, false, true);
    // - np == 8
    test_generated(30, 20, 185, 0, 49, false, true);
    // - np > MPN_CTX_NCRTS, so the function declines
    test_generated(30, 20, 250, 0, 49, false, false);
    // - xs and ys are the same slice
    test_generated(300, 300, 20, 100, 400, true, true);
    // - a power of two between zh and zn allows wraparound
    test_generated(256, 256, 20, 255, 256, false, true);
    // - zl >= zh
    test_generated(5, 5, 20, 3, 3, false, true);
    // - zh > zn, but zl < zn
    test_generated(5, 5, 20, 7, 12, false, true);
    // - zl >= zn
    test_generated(5, 5, 20, 9, 12, false, true);
}

#[test]
fn mul_middle_to_out_fft_properties() {
    // With long inputs, the reference is Kronecker substitution, which is much faster than the
    // naive product and is tested against it.
    let mut config = GenConfig::new();
    config.insert("mean_len_n", 300);
    config.insert("mean_bits_n", 16);
    let test = |(xs, ys, nlo, nhi): (Vec<Integer>, Vec<Integer>, u64, u64), naive: bool| {
        let nlo = usize::exact_from(nlo);
        let nhi = usize::exact_from(nhi);
        let mut out = vec![Integer::ZERO; nhi - nlo];
        if mul_middle_to_out_fft(&mut out, &xs, &ys, nlo, nhi) {
            assert!(out.iter().all(Integer::is_valid));
            if naive {
                assert_eq!(&integers_mul_naive(&xs, &ys)[nlo..nhi], out);
            } else {
                let mut out_alt = vec![Integer::ZERO; nhi - nlo];
                mul_middle_to_out_kronecker(&mut out_alt, &xs, &ys, nlo, nhi);
                assert_eq!(out_alt, out);
            }
        } else {
            // Only a product with coefficients too large for eight primes is declined.
            assert!(vec_max_bits(&xs).0 + vec_max_bits(&ys).0 > 300);
        }
    };
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1()
        .test_properties(|x| test(x, true));
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1()
        .test_properties_with_config(&config, |x| test(x, false));

    // The square path.
    let test = |(xs, nlo, nhi): (Vec<Integer>, u64, u64), naive: bool| {
        let nlo = usize::exact_from(nlo);
        let nhi = usize::exact_from(nhi);
        let mut out = vec![Integer::ZERO; nhi - nlo];
        if mul_middle_to_out_fft(&mut out, &xs, &xs, nlo, nhi) {
            if naive {
                assert_eq!(&integers_mul_naive(&xs, &xs)[nlo..nhi], out);
            } else {
                let mut out_alt = vec![Integer::ZERO; nhi - nlo];
                mul_middle_to_out_kronecker(&mut out_alt, &xs, &xs, nlo, nhi);
                assert_eq!(out_alt, out);
            }
        } else {
            assert!(vec_max_bits(&xs).0 << 1 > 300);
        }
    };
    integer_vec_unsigned_unsigned_triple_gen_var_1().test_properties(|x| test(x, true));
    integer_vec_unsigned_unsigned_triple_gen_var_1()
        .test_properties_with_config(&config, |x| test(x, false));
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_vec_get_fft() {
    let test = |xs: &[&str], limbs: usize, out: &[Limb]| {
        let xs = parse(xs);
        let mut coeffs_f = vec![vec![0; limbs + 1]; xs.len()];
        vec_get_fft(&mut coeffs_f, &xs, limbs);
        assert_eq!(coeffs_f.concat(), out);
    };
    // - *x >= 0u32
    // - *x < 0u32
    test(
        &["0", "1", "-1"],
        1,
        &[0, 0, 1, 0, 18446744073709551615, 18446744073709551615],
    );
    test(
        &["18446744073709551616", "-18446744073709551616"],
        2,
        &[0, 1, 0, 0, 18446744073709551615, 18446744073709551615],
    );
    test(
        &["5", "-5"],
        3,
        &[
            5,
            0,
            0,
            0,
            18446744073709551611,
            18446744073709551615,
            18446744073709551615,
            18446744073709551615,
        ],
    );
}

#[test]
fn vec_get_fft_properties() {
    large_type_gen_var_57().test_properties(|(xs, limbs)| {
        let mut coeffs_f = vec![vec![0; limbs + 1]; xs.len()];
        vec_get_fft(&mut coeffs_f, &xs, limbs);
        for (x, f) in xs.iter().zip(coeffs_f.iter()) {
            // Each residue is the coefficient in two's complement.
            let bits = (u64::exact_from(limbs) + 1) << Limb::LOG_WIDTH;
            let value = Integer::from(Natural::from_limbs_asc(f));
            let value = if f[limbs] >> (Limb::WIDTH - 1) != 0 {
                value - Integer::power_of_2(bits)
            } else {
                value
            };
            assert_eq!(&value, x);
        }
    });
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_vec_set_fft() {
    let test = |coeffs_f: &[&[Limb]], limbs: usize, sign: bool, out: &[&str]| {
        let coeffs_f: Vec<Vec<Limb>> = coeffs_f.iter().map(|x| x.to_vec()).collect();
        let mut xs = vec![Integer::ZERO; coeffs_f.len()];
        vec_set_fft(&mut xs, &coeffs_f, limbs, sign);
        assert_eq!(xs, parse(out));
    };
    // - !(sign && (f[limbs - 1] >= HALF_LIMB || f[limbs] != 0))
    test(
        &[&[5, 0], &[18446744073709551615, 0]],
        1,
        false,
        &["5", "18446744073709551615"],
    );
    // - sign && (f[limbs - 1] >= HALF_LIMB || f[limbs] != 0)
    test(
        &[&[5, 0], &[18446744073709551615, 0]],
        1,
        true,
        &["5", "-2"],
    );
    test(&[&[0, 1]], 1, true, &["-1"]);
    // A top limb of `HALF_LIMB` means a negative coefficient. (FLINT reads it as positive.)
    test(
        &[&[0, 9223372036854775808, 0]],
        2,
        true,
        &["-170141183460469231731687303715884105729"],
    );
    test(
        &[&[1, 9223372036854775808, 0]],
        2,
        true,
        &["-170141183460469231731687303715884105728"],
    );
    test(
        &[&[0, 9223372036854775809, 0]],
        2,
        true,
        &["-170141183460469231713240559642174554113"],
    );
}

#[test]
fn vec_set_fft_properties() {
    large_type_gen_var_58().test_properties(|(coeffs_f, limbs, sign)| {
        let mut xs = vec![Integer::ZERO; coeffs_f.len()];
        vec_set_fft(&mut xs, &coeffs_f, limbs, sign);
        let p = Integer::from(fermat_modulus(limbs));
        for (x, f) in xs.iter().zip(coeffs_f.iter()) {
            if sign {
                let v = residue_mod(f, limbs);
                assert_eq!(Natural::exact_from(x.mod_op(&p)), v);
                // Residues from $2^{N-1}$ up, where $N$ is `limbs * Limb::WIDTH`, are negative.
                assert_eq!(*x >= 0u32, v < Natural::power_of_2(fermat_bits(limbs) - 1));
            } else if f[limbs] == 0 {
                assert_eq!(Natural::exact_from(x), residue_mod(f, limbs));
            }
        }
    });
    // Reading back coefficients that were written with a limb to spare, and then normalized, gives
    // them back.
    large_type_gen_var_57().test_properties(|(xs, limbs)| {
        let limbs = limbs + 1;
        let mut coeffs_f = vec![vec![0; limbs + 1]; xs.len()];
        vec_get_fft(&mut coeffs_f, &xs, limbs);
        for f in &mut coeffs_f {
            limbs_norm_mod_2expp1(f, limbs);
        }
        let mut xs_alt = vec![Integer::ZERO; xs.len()];
        vec_set_fft(&mut xs_alt, &coeffs_f, limbs, true);
        assert_eq!(xs_alt, xs);
    });
}
