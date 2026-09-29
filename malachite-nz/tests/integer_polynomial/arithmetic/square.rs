// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::mul::classical::mul_to_out_classical;
use malachite_nz::integer_polynomial::arithmetic::square::classical::square_to_out_classical;
use malachite_nz::integer_polynomial::arithmetic::square::square_to_out;
use malachite_nz::integer_polynomial::arithmetic::square::tiny::{
    square_to_out_tiny_1, square_to_out_tiny_2,
};
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_pair_gen, integer_vec_gen_var_1,
    integer_vec_gen_var_2, integer_vec_gen_var_3,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::integers_mul_naive;
use malachite_nz::test_util::integer_polynomial::arithmetic::square::*;

fn coefficients(p: &str) -> Vec<Integer> {
    IntegerPolynomial::from_str(p)
        .unwrap()
        .into_coefficients_asc()
}

fn parse(xs: &[&str]) -> Vec<Integer> {
    xs.iter().map(|x| Integer::from_str(x).unwrap()).collect()
}

#[test]
fn test_square_to_out_classical() {
    let test = |xs: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_classical(&mut result, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &xs), result);
    };
    test(&["3"], &["9"]);
    test(&["-5", "7"], &["25", "-70", "49"]);
    test(&["1", "2", "3"], &["1", "4", "10", "12", "9"]);
    test(
        &["1", "-1", "1", "-1"],
        &["1", "-2", "3", "-4", "3", "-2", "1"],
    );
    test(
        &["1180591620717411303424", "-3", "36893488147419103233"],
        &[
            "1393796574908163946345982392040522594123776",
            "-7083549724304467820544",
            "87112285931760246648985082743967484739593",
            "-221360928884514619398",
            "1361129467683753853927285406021911052289",
        ],
    );
    test(
        &["100000000000000000000", "-10000000000000000000000000", "7", "0", "3"],
        &[
            "10000000000000000000000000000000000000000",
            "-2000000000000000000000000000000000000000000000",
            "100000000000000000000000000001400000000000000000000",
            "-140000000000000000000000000",
            "600000000000000000049",
            "-60000000000000000000000000",
            "42",
            "0",
            "9",
        ],
    );
    test(
        &["9223372036854775808", "-9223372036854775807", "17", "-1"],
        &[
            "85070591730234615865843651857942052864",
            "-170141183460469231713240559642174554112",
            "85070591730234616160991557037294878721",
            "-332041393326771929054",
            "18446744073709551903",
            "-34",
            "1",
        ],
    );
}

#[test]
fn test_square_to_out_tiny_1() {
    let test = |xs: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_tiny_1(&mut result, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &xs), result);
    };
    test(&["1", "2", "3"], &["1", "4", "10", "12", "9"]);
    test(&["0", "1", "2"], &["0", "0", "1", "4", "4"]);
    test(
        &["-7", "0", "11", "-13"],
        &["49", "0", "-154", "182", "121", "-286", "169"],
    );
    test(
        &["1", "1", "1", "1", "1", "1", "1", "1"],
        &["1", "2", "3", "4", "5", "6", "7", "8", "7", "6", "5", "4", "3", "2", "1"],
    );
}

#[test]
fn test_square_to_out_tiny_2() {
    let test = |xs: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_tiny_2(&mut result, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &xs), result);
    };
    test(
        &["2305843009213693952", "-2305843009213693951", "7"],
        &[
            "5316911983139663491615228241121378304",
            "-10633823966279326978618770463815368704",
            "5316911983139663519285344351685705729",
            "-32281802128991715314",
            "49",
        ],
    );
    test(
        &["0", "1099511627776", "0", "5"],
        &["0", "0", "1208925819614629174706176", "0", "10995116277760", "0", "25"],
    );
    test(
        &["-1099511627776", "5", "1125899906842624", "1"],
        &[
            "1208925819614629174706176",
            "-10995116277760",
            "-2475880078570760549798248423",
            "11256800045170688",
            "1267650600228229401496703205386",
            "2251799813685248",
            "1",
        ],
    );
}

#[test]
fn test_square_to_out() {
    let test = |xs: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out(&mut result, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &xs), result);
    };
    // Long inputs, written as polynomials.
    let test_poly = |p: &str, out: &str| {
        let xs = coefficients(p);
        let mut result = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out(&mut result, &xs);
        assert_eq!(integers_mul_naive(&xs, &xs), result);
        assert_eq!(
            IntegerPolynomial::from_coefficients_asc(result).to_string(),
            out
        );
    };
    // - xs.len() == 1
    test(&["3"], &["9"]);
    // - bits <= SMALL_FMPZ_BITCOUNT_MAX && len < 50 + 3 * bits
    // - rbits <= SMALL_FMPZ_BITCOUNT_MAX
    test(&["1", "2", "3"], &["1", "4", "10", "12", "9"]);
    // - rbits > SMALL_FMPZ_BITCOUNT_MAX && rbits < 2 * Limb::WIDTH
    test(
        &["2305843009213693952", "-2305843009213693951", "7"],
        &[
            "5316911983139663491615228241121378304",
            "-10633823966279326978618770463815368704",
            "5316911983139663519285344351685705729",
            "-32281802128991715314",
            "49",
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
        &[
            "1393796574908163946345982392040522594123776",
            "-7083549724304467820544",
            "87112285931760246648985082743967484739593",
            "-221360928884514619398",
            "1361129467683753853927285406021911052289",
        ],
    );
    // - bits <= SMALL_FMPZ_BITCOUNT_MAX && len >= 50 + 3 * bits
    test_poly("-x^53+1", "x^106-2*x^53+1");
}

#[test]
fn test_square() {
    let test = |s, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let r = (&p).square();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().square(), r);
        let mut s = p.clone();
        s.square_assign();
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(square_naive(&p), r);
    };
    test("0", "0");
    test("1", "1");
    test("-3", "9");
    test("x+1", "x^2+2*x+1");
    test("x-1", "x^2-2*x+1");
    test("2*x^2+3", "4*x^4+12*x^2+9");
    test("x^3-x", "x^6-2*x^4+x^2");
    test(
        "18446744073709551616*x-1",
        "340282366920938463463374607431768211456*x^2-36893488147419103232*x+1",
    );
}

#[test]
fn square_to_out_classical_properties() {
    integer_vec_gen_var_1().test_properties(|xs| {
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_classical(&mut out, &xs);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(integers_mul_naive(&xs, &xs), out);
        // This is multiplying by a copy.
        let mut out_alt = vec![Integer::ZERO; (xs.len() << 1) - 1];
        mul_to_out_classical(&mut out_alt, &xs, &xs.clone());
        assert_eq!(out_alt, out);
        square_to_out(&mut out_alt, &xs);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn square_to_out_tiny_1_properties() {
    integer_vec_gen_var_2().test_properties(|xs| {
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_tiny_1(&mut out, &xs);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(integers_mul_naive(&xs, &xs), out);
        // This is multiplying by a copy.
        let mut out_alt = vec![Integer::ZERO; (xs.len() << 1) - 1];
        mul_to_out_classical(&mut out_alt, &xs, &xs.clone());
        assert_eq!(out_alt, out);
        square_to_out_tiny_2(&mut out_alt, &xs);
        assert_eq!(out_alt, out);
        square_to_out_classical(&mut out_alt, &xs);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn square_to_out_tiny_2_properties() {
    integer_vec_gen_var_3().test_properties(|xs| {
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_tiny_2(&mut out, &xs);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(integers_mul_naive(&xs, &xs), out);
        // This is multiplying by a copy.
        let mut out_alt = vec![Integer::ZERO; (xs.len() << 1) - 1];
        mul_to_out_classical(&mut out_alt, &xs, &xs.clone());
        assert_eq!(out_alt, out);
        square_to_out_classical(&mut out_alt, &xs);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn square_to_out_properties() {
    integer_vec_gen_var_1().test_properties(|xs| {
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out(&mut out, &xs);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(integers_mul_naive(&xs, &xs), out);
        // This is multiplying by a copy.
        let mut out_alt = vec![Integer::ZERO; (xs.len() << 1) - 1];
        mul_to_out_classical(&mut out_alt, &xs, &xs.clone());
        assert_eq!(out_alt, out);
    });
}

#[test]
fn square_properties() {
    integer_polynomial_gen().test_properties(|p| {
        let r = (&p).square();
        assert!(r.is_valid());
        // The forms agree.
        assert_eq!(p.clone().square(), r);
        let mut s = p.clone();
        s.square_assign();
        assert!(s.is_valid());
        assert_eq!(s, r);

        assert_eq!(square_naive(&p), r);
        assert_eq!(&p * &p, r);
        assert_eq!(&p * p.clone(), r);
        assert_eq!((-&p).square(), r);
        // The leading coefficient is positive.
        assert!(r == 0u32 || r.leading_coefficient() > &0u32);
    });

    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        // (p + q)^2 = p^2 + 2pq + q^2
        let pq = &p * &q;
        assert_eq!(
            (&p + &q).square(),
            (&p).square() + &pq + &pq + (&q).square()
        );
    });
}
