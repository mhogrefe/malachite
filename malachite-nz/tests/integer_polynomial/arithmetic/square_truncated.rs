// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Square;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{
    MulTruncated, Polynomial, SquareTruncated, SquareTruncatedAssign,
};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::classical::*;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::square_truncated_to_out;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::tiny::{
    square_truncated_to_out_tiny_1, square_truncated_to_out_tiny_2,
};
use malachite_nz::test_util::generators::{
    integer_polynomial_unsigned_pair_gen_var_3, integer_vec_unsigned_pair_gen_var_1,
    integer_vec_unsigned_pair_gen_var_2, integer_vec_unsigned_pair_gen_var_3,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::integers_mul_naive;
use malachite_nz::test_util::integer_polynomial::arithmetic::square_truncated::*;

fn coefficients(p: &str) -> Vec<Integer> {
    IntegerPolynomial::from_str(p)
        .unwrap()
        .into_coefficients_asc()
}

fn parse(xs: &[&str]) -> Vec<Integer> {
    xs.iter().map(|x| Integer::from_str(x).unwrap()).collect()
}

#[test]
fn test_square_truncated_to_out_classical() {
    let test = |xs: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; n];
        square_truncated_to_out_classical(&mut result, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], result);
    };
    // - i even in square_coefficient
    test(&["1", "2", "3"], 1, &["1"]);
    // - i odd in square_coefficient
    test(&["1", "2", "3"], 4, &["1", "4", "10", "12"]);
    test(&["1", "2", "3"], 5, &["1", "4", "10", "12", "9"]);
    test(&["1", "2", "3", "4"], 5, &["1", "4", "10", "20", "25"]);
    test(
        &["1180591620717411303424", "-3", "36893488147419103233"],
        3,
        &[
            "1393796574908163946345982392040522594123776",
            "-7083549724304467820544",
            "87112285931760246648985082743967484739593",
        ],
    );
    test(
        &["100000000000000000000", "-10000000000000000000000000", "7", "0", "3"],
        6,
        &[
            "10000000000000000000000000000000000000000",
            "-2000000000000000000000000000000000000000000000",
            "100000000000000000000000000001400000000000000000000",
            "-140000000000000000000000000",
            "600000000000000000049",
            "-60000000000000000000000000",
        ],
    );
}

#[test]
fn test_square_truncated_to_out_tiny_1() {
    let test = |xs: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; n];
        square_truncated_to_out_tiny_1(&mut result, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], result);
    };
    // - x == 0
    test(&["0", "1", "2"], 3, &["0", "0", "1"]);
    // - x != 0
    // - 2 * i < n
    // - 2 * i >= n
    test(&["1", "2", "3"], 3, &["1", "4", "10"]);
    test(
        &["1", "1", "1", "1", "1", "1", "1", "1"],
        11,
        &["1", "2", "3", "4", "5", "6", "7", "8", "7", "6", "5"],
    );
}

#[test]
fn test_square_truncated_to_out_tiny_2() {
    let test = |xs: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; n];
        square_truncated_to_out_tiny_2(&mut result, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], result);
    };
    // - x == 0
    // - y == 0
    test(
        &["0", "1099511627776", "0", "5"],
        5,
        &["0", "0", "1208925819614629174706176", "0", "10995116277760"],
    );
    // - x != 0
    // - y != 0
    // - 2 * i < n
    // - 2 * i >= n
    test(
        &["2305843009213693952", "-2305843009213693951", "7"],
        3,
        &[
            "5316911983139663491615228241121378304",
            "-10633823966279326978618770463815368704",
            "5316911983139663519285344351685705729",
        ],
    );
}

#[test]
fn test_square_truncated_to_out() {
    let test = |xs: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; n];
        square_truncated_to_out(&mut result, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], result);
    };
    // Long inputs, written as polynomials.
    let test_poly = |p: &str, n: usize, out: &str| {
        let xs = coefficients(p);
        let mut result = vec![Integer::ZERO; n];
        square_truncated_to_out(&mut result, &xs);
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], result);
        assert_eq!(
            IntegerPolynomial::from_coefficients_asc(result).to_string(),
            out
        );
    };
    // - xs.len() == 1
    test(&["1", "2", "3"], 1, &["1"]);
    // - len < 50 + 2 * bits
    // - rbits <= SMALL_FMPZ_BITCOUNT_MAX
    test(&["1", "2", "3"], 4, &["1", "4", "10", "12"]);
    // - rbits > SMALL_FMPZ_BITCOUNT_MAX && rbits < 2 * Limb::WIDTH
    test(
        &["2305843009213693952", "-2305843009213693951", "7"],
        3,
        &[
            "5316911983139663491615228241121378304",
            "-10633823966279326978618770463815368704",
            "5316911983139663519285344351685705729",
        ],
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
    // - bits > SMALL_FMPZ_BITCOUNT_MAX
    // - classical
    test(
        &["1180591620717411303424", "-3", "36893488147419103233"],
        3,
        &[
            "1393796574908163946345982392040522594123776",
            "-7083549724304467820544",
            "87112285931760246648985082743967484739593",
        ],
    );
    // - len >= 50 + 2 * bits && 4 * len >= 3 * n && n < 140 + 6 * bits
    test_poly("x^59+1", 60, "2*x^59+1");
    // - len >= 50 + 2 * bits && !(4 * len >= 3 * n && n < 140 + 6 * bits)
    test_poly("-x^99+1", 199, "x^198-2*x^99+1");
}

#[test]
fn test_square_truncated() {
    let test = |s, len, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let r = (&p).square_truncated(len);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().square_truncated(len), r);
        let mut s = p.clone();
        s.square_truncated_assign(len);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(square_truncated_naive(&p, len), r);
    };
    test("0", 3, "0");
    test("x+1", 0, "0");
    test("x+1", 1, "1");
    test("x+1", 2, "2*x+1");
    test("x+1", 100, "x^2+2*x+1");
    test("x^2+x-1", 3, "-x^2-2*x+1");
    test("x^3-x", 4, "x^2");
    test("x^3-x", 5, "-2*x^4+x^2");
}

#[test]
fn square_truncated_to_out_classical_properties() {
    integer_vec_unsigned_pair_gen_var_1().test_properties(|(xs, n)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        square_truncated_to_out_classical(&mut out, &xs);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], out);
        let mut out_alt = vec![Integer::ZERO; n];
        square_truncated_to_out(&mut out_alt, &xs);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn square_truncated_to_out_tiny_1_properties() {
    integer_vec_unsigned_pair_gen_var_2().test_properties(|(xs, n)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        square_truncated_to_out_tiny_1(&mut out, &xs);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], out);
        let mut out_alt = vec![Integer::ZERO; n];
        square_truncated_to_out_tiny_2(&mut out_alt, &xs);
        assert_eq!(out_alt, out);
        square_truncated_to_out_classical(&mut out_alt, &xs);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn square_truncated_to_out_tiny_2_properties() {
    integer_vec_unsigned_pair_gen_var_3().test_properties(|(xs, n)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        square_truncated_to_out_tiny_2(&mut out, &xs);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], out);
        let mut out_alt = vec![Integer::ZERO; n];
        square_truncated_to_out_classical(&mut out_alt, &xs);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn square_truncated_to_out_properties() {
    integer_vec_unsigned_pair_gen_var_1().test_properties(|(xs, n)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        square_truncated_to_out(&mut out, &xs);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], out);
        let mut out_alt = vec![Integer::ZERO; n];
        square_truncated_to_out_classical(&mut out_alt, &xs);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn square_truncated_properties() {
    integer_polynomial_unsigned_pair_gen_var_3::<u64>().test_properties(|(p, len)| {
        let r = (&p).square_truncated(len);
        assert!(r.is_valid());
        // The forms agree.
        assert_eq!(p.clone().square_truncated(len), r);
        let mut s = p.clone();
        s.square_truncated_assign(len);
        assert!(s.is_valid());
        assert_eq!(s, r);

        assert_eq!(square_truncated_naive(&p, len), r);
        assert_eq!((&p).square().truncate(len), r);
        assert_eq!((&p).mul_truncated(&p, len), r);
        assert_eq!(p.truncate(len).square().truncate(len), r);
        assert_eq!((-&p).square_truncated(len), r);
        assert!(r.len() <= len);
    });
}
