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
use malachite_base::polynomial::{EqTruncated, MulTruncated, MulTruncatedAssign, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::classical::*;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::mul_truncated_to_out;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::tiny::{
    mul_truncated_to_out_tiny_1, mul_truncated_to_out_tiny_2,
};
use malachite_nz::test_util::generators::{
    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1,
    integer_vec_integer_vec_unsigned_triple_gen_var_1,
    integer_vec_integer_vec_unsigned_triple_gen_var_2,
    integer_vec_integer_vec_unsigned_triple_gen_var_3,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::integers_mul_naive;
use malachite_nz::test_util::integer_polynomial::arithmetic::mul_truncated::*;

fn coefficients(p: &str) -> Vec<Integer> {
    IntegerPolynomial::from_str(p)
        .unwrap()
        .into_coefficients_asc()
}

fn parse(xs: &[&str]) -> Vec<Integer> {
    xs.iter().map(|x| Integer::from_str(x).unwrap()).collect()
}

#[test]
fn test_mul_truncated_to_out_classical() {
    let test = |xs: &[&str], ys: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out_classical(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], result);
    };
    // - n == 1, so xs.len() == 1 after truncation
    test(&["1", "2", "3"], &["4", "5", "6"], 1, &["4"]);
    // - xs.len() == 1
    test(&["3"], &["4", "5", "6"], 2, &["12", "15"]);
    // - ys.len() == 1
    test(&["4", "5", "6"], &["3"], 2, &["12", "15"]);
    // - xs.len() != 1 && ys.len() != 1
    test(&["1", "2", "3"], &["4", "5", "6"], 3, &["4", "13", "28"]);
    test(
        &["1180591620717411303424", "-3", "36893488147419103233"],
        &["-18446744073709551616", "5"],
        3,
        &[
            "-21778071482940061661655974875633165533184",
            "5958298335808185171968",
            "-680564733841876926945195958937245974543",
        ],
    );
    test(
        &["100000000000000000000", "-10000000000000000000000000", "7", "0", "3"],
        &["-1", "1267650600228229401496703205376", "11"],
        5,
        &[
            "-100000000000000000000",
            "126765060022822940149670330537600000000000000000000",
            "-12676506002282294014967032053759998900000000000000000007",
            "8873444201597605810476922437632",
            "74",
        ],
    );
}

#[test]
fn test_mul_truncated_to_out_tiny_1() {
    let test = |xs: &[&str], ys: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out_tiny_1(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], result);
    };
    // - x != 0
    test(
        &["1", "2", "3"],
        &["4", "5", "6"],
        4,
        &["4", "13", "28", "27"],
    );
    // - x == 0
    test(&["0", "1", "2"], &["3", "4"], 3, &["0", "3", "10"]);
    test(
        &["1", "1", "1", "1", "1", "1", "1", "1"],
        &["-1", "-1", "-1", "-1", "-1", "-1", "-1", "-1"],
        10,
        &["-1", "-2", "-3", "-4", "-5", "-6", "-7", "-8", "-7", "-6"],
    );
}

#[test]
fn test_mul_truncated_to_out_tiny_2() {
    let test = |xs: &[&str], ys: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out_tiny_2(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], result);
    };
    // - x != 0
    // - y != 0
    test(
        &["2305843009213693952", "-2305843009213693951"],
        &["2305843009213693951", "1152921504606846976"],
        2,
        &["5316911983139663489309385231907684352", "-2658455991569831741195928102133301249"],
    );
    // - x == 0
    // - y == 0
    test(
        &["0", "1099511627776", "5"],
        &["3", "0", "1048576"],
        4,
        &["0", "3298534883328", "15", "1152921504606846976"],
    );
    test(
        &["-1099511627776", "5", "1125899906842624"],
        &["3", "-35184372088832", "1048576", "1"],
        5,
        &[
            "-3298534883328",
            "38685626227668133590597647",
            "-1149719726746763264",
            "-39614081257132169896278360064",
            "1180591620717411303429",
        ],
    );
}

#[test]
fn test_mul_truncated_to_out() {
    let test = |xs: &[&str], ys: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out(&mut result, &xs, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], result);
    };
    // Long inputs, written as polynomials.
    let test_poly = |p: &str, q: &str, n: usize, out: &str| {
        let xs = coefficients(p);
        let ys = coefficients(q);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out(&mut result, &xs, &ys);
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], result);
        assert_eq!(
            IntegerPolynomial::from_coefficients_asc(result).to_string(),
            out
        );
    };
    // - xs.len() < ys.len()
    test(&["3", "4"], &["1", "2", "3"], 3, &["3", "10", "17"]);
    // - ys.len() == 1
    test(&["1", "2", "3"], &["-6"], 2, &["-6", "-12"]);
    // - xs and ys are the same slice
    test_square(&["1", "2", "3"], 4, &["1", "4", "10", "12"]);
    // - len2 < 50
    // - rbits <= SMALL_FMPZ_BITCOUNT_MAX
    test(
        &["1", "2", "3"],
        &["4", "5", "6"],
        4,
        &["4", "13", "28", "27"],
    );
    // - rbits > SMALL_FMPZ_BITCOUNT_MAX && rbits < 2 * Limb::WIDTH
    test(
        &["2305843009213693952", "-2305843009213693951"],
        &["2305843009213693951", "1152921504606846976"],
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
        3,
        &[
            "-21778071482940061661655974875633165533184",
            "5958298335808185171968",
            "-680564733841876926945195958937245974543",
        ],
    );
    // - len2 >= 50 && 4 * len2 >= 3 * n && n < 150 + bits1 + bits2
    test_poly("x^59+1", "-x^59+2*x^30+1", 60, "2*x^30+1");
    // - len2 >= 50 && !(4 * len2 >= 3 * n && n < 150 + bits1 + bits2)
    test_poly("x^99+1", "-x^99+1", 199, "-x^198+1");
}

#[test]
fn test_mul_truncated() {
    let test = |s, t, len, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = IntegerPolynomial::from_str(t).unwrap();
        let r = (&p).mul_truncated(&q, len);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mul_truncated(q.clone(), len), r);
        assert_eq!(p.clone().mul_truncated(&q, len), r);
        assert_eq!(p.clone().mul_truncated(q.clone(), len), r);
        let mut s = p.clone();
        s.mul_truncated_assign(&q, len);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mul_truncated_assign(q.clone(), len);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mul_truncated_naive(&p, &q, len), r);
    };
    test("0", "x+1", 3, "0");
    test("x+1", "x-1", 0, "0");
    test("x+1", "x-1", 1, "-1");
    test("x+1", "x-1", 2, "-1");
    test("x+1", "x-1", 3, "x^2-1");
    test("x+1", "x-1", 100, "x^2-1");
    test("2*x^2+3", "-x+4", 2, "-3*x+12");
    test("x^3+x^2+x+1", "x-1", 3, "-1");
    test(
        "18446744073709551616*x+1",
        "18446744073709551616*x-1",
        2,
        "-1",
    );
}

#[test]
fn mul_truncated_to_out_classical_properties() {
    integer_vec_integer_vec_unsigned_triple_gen_var_1().test_properties(|(xs, ys, n)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        mul_truncated_to_out_classical(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], out);
        // Multiplication is commutative.
        let mut out_alt = vec![Integer::ZERO; n];
        mul_truncated_to_out_classical(&mut out_alt, &ys, &xs);
        assert_eq!(out_alt, out);
        // The dispatcher agrees.
        mul_truncated_to_out(&mut out_alt, &xs, &ys);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn mul_truncated_to_out_tiny_1_properties() {
    integer_vec_integer_vec_unsigned_triple_gen_var_2().test_properties(|(xs, ys, n)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        mul_truncated_to_out_tiny_1(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], out);
        let mut out_alt = vec![Integer::ZERO; n];
        mul_truncated_to_out_tiny_2(&mut out_alt, &xs, &ys);
        assert_eq!(out_alt, out);
        mul_truncated_to_out_classical(&mut out_alt, &xs, &ys);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn mul_truncated_to_out_tiny_2_properties() {
    integer_vec_integer_vec_unsigned_triple_gen_var_3().test_properties(|(xs, ys, n)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        mul_truncated_to_out_tiny_2(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], out);
        let mut out_alt = vec![Integer::ZERO; n];
        mul_truncated_to_out_classical(&mut out_alt, &xs, &ys);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn mul_truncated_to_out_properties() {
    integer_vec_integer_vec_unsigned_triple_gen_var_1().test_properties(|(xs, ys, n)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        mul_truncated_to_out(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], out);
        // The square path.
        if n < xs.len() << 1 {
            mul_truncated_to_out(&mut out, &xs, &xs);
            assert_eq!(&integers_mul_naive(&xs, &xs)[..n], out);
        }
    });
}

#[test]
fn mul_truncated_properties() {
    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, len)| {
            let r = (&p).mul_truncated(&q, len);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!((&p).mul_truncated(q.clone(), len), r);
            assert_eq!(p.clone().mul_truncated(&q, len), r);
            assert_eq!(p.clone().mul_truncated(q.clone(), len), r);
            let mut s = p.clone();
            s.mul_truncated_assign(&q, len);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mul_truncated_assign(q.clone(), len);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // This is the truncation of the whole product, and of the product of the truncations.
            assert_eq!(mul_truncated_naive(&p, &q, len), r);
            assert_eq!((p.truncate(len) * q.truncate(len)).truncate(len), r);
            assert!(r.eq_truncated(&(&p * &q), len));
            assert!(r.len() <= len);
            // Multiplication is commutative.
            assert_eq!((&q).mul_truncated(&p, len), r);
            // Negating a factor negates the product.
            assert_eq!((-&p).mul_truncated(&q, len), -&r);
            // The square path.
            assert_eq!((&p).mul_truncated(&p, len), (&p * &p).truncate(len));
        },
    );
}
