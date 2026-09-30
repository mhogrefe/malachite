// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::classical::*;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::mul_middle_to_out;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::tiny::{
    mul_middle_to_out_tiny_1, mul_middle_to_out_tiny_2,
};
use malachite_nz::test_util::generators::{
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1,
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_2,
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_3,
    integer_vec_unsigned_unsigned_triple_gen_var_1, integer_vec_unsigned_unsigned_triple_gen_var_2,
    integer_vec_unsigned_unsigned_triple_gen_var_3,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::{
    generated_coefficients, integers_mul_naive,
};

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
    // - xs and ys are different slices
    test(
        &["2305843009213693952", "-2305843009213693951"],
        &["2305843009213693951", "1152921504606846976"],
        1,
        3,
        &["-2658455991569831741195928102133301249", "-2658455991569831744654692615953842176"],
    );
    test(
        &["-1099511627776", "5", "1125899906842624"],
        &["3", "-35184372088832", "1048576", "1"],
        2,
        5,
        &["-1149719726746763264", "-39614081257132169896278360064", "1180591620717411303429"],
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
