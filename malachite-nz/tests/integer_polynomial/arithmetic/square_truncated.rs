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
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::classical::*;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::karatsuba::*;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::kronecker::*;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::square_truncated_to_out;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::tiny::{
    square_truncated_to_out_tiny_1, square_truncated_to_out_tiny_2,
};
use malachite_nz::test_util::generators::{
    integer_polynomial_unsigned_pair_gen_var_3, integer_vec_unsigned_pair_gen_var_1,
    integer_vec_unsigned_pair_gen_var_2, integer_vec_unsigned_pair_gen_var_3,
    integer_vec_unsigned_pair_gen_var_4,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::{
    generated_coefficients, integers_mul_naive,
};
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
    // Generated coefficients of `bits` bits.
    let test_generated = |len: usize, bits: u64, n: usize| {
        let xs = generated_coefficients(len, bits);
        let mut result = vec![Integer::ZERO; n];
        square_truncated_to_out(&mut result, &xs);
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], result);
    };
    // - karatsuba_preferred(len, bits, bits), with len <= 4
    test_generated(4, 6000, 5);
    // - karatsuba_preferred(len, bits, bits), with 1500 <= 2 * bits <= 10000
    test_generated(8, 1000, 12);
    // - !karatsuba_preferred(len, bits, bits)
    test_generated(8, 100, 12);
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
    // A constant, which the forms taking it by value square in place.
    test("-3", 1, "9");
    test("-3", 0, "0");
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

#[test]
fn test_square_truncated_to_out_karatsuba_n() {
    let test = |xs: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; n];
        square_truncated_to_out_karatsuba_n(&mut result, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], result);
    };
    // - n == 1
    test(&["3", "5"], 1, &["9"]);
    // - n != 1
    // - len <= 6
    test(&["-14", "-20"], 2, &["196", "560"]);
    test(
        &["10", "-6", "8", "3", "-18", "-2"],
        6,
        &["100", "-120", "196", "-36", "-332", "224"],
    );
    // - len > 6
    // - len odd
    test(
        &["-6", "-13", "-17", "-8", "18", "17", "-8"],
        7,
        &["36", "156", "373", "538", "281", "-400", "-894"],
    );
    // - len even
    test(
        &["-16", "3", "12", "-9", "8", "18", "-4", "-20"],
        8,
        &["256", "-96", "-375", "360", "-166", "-744", "509", "904"],
    );
    test(
        &["-14", "20", "18", "19", "2", "-7", "-18", "3", "1", "-11", "-18", "-7", "-4"],
        13,
        &[
            "196", "-560", "-104", "188", "1028", "960", "657", "-980", "-818", "-256", "191",
            "-618", "-948",
        ],
    );
    test(
        &["-18", "18", "-7", "-20", "0", "6", "3", "-9", "19", "-1", "-16", "-7", "-18", "11"],
        14,
        &[
            "324", "-648", "576", "468", "-671", "64", "508", "348", "-1290", "726", "670",
            "-1034", "561", "-132",
        ],
    );
    test(
        &[
            "15", "10", "-16", "6", "-14", "5", "15", "-11", "20", "14", "-15", "-10", "5", "-3",
            "6", "-2", "-1", "6", "-17", "-1", "16", "2", "6", "6", "-19", "3", "-8", "5", "5",
            "-7",
        ],
        30,
        &[
            "225", "300", "-380", "-140", "-44", "-322", "1034", "-358", "156", "1212", "-1337",
            "-350", "153", "-372", "1121", "326", "-896", "1020", "-614", "-1478", "1475", "158",
            "352", "566", "-1909", "160", "481", "-1466", "2310", "348",
        ],
    );
    test(
        &["-20", "7", "-10", "7", "-13", "-15", "5", "16", "3", "9", "-10", "-12"],
        7,
        &["400", "-280", "449", "-420", "718", "278", "-101"],
    );
    test(
        &[
            "-803571385319908293476825688302",
            "1158489609398241869667104374322",
            "172058664669926539054859180195",
            "199746785534255823636509623691",
            "-989600128385667396352958126354",
            "-1254759567326656926785421617389",
            "-434387137500919054853967014843",
            "-525787220506740812351819127426",
            "-920658421284976414749164766325",
        ],
        9,
        &[
            "645726971304956526026659883740087709773848513996676059643204",
            "-1861858200605729339453220940263703244289752942478725409162488",
            "1065575336033478155656687237970399216554822481337218449801904",
            "77634748124171408334625790245661639121337047312990496380216",
            "2082842027351526768414219603086762905582106933065360598184845",
            "-207568834485474253205381365352480458732987568315108723997530",
            "-2509769469351589090312824948156237898552339946351717689891923",
            "-988574240660635201581575713106649714887486273778821145508526",
            "589951354634756464532715627544991833949042942692494323488904",
        ],
    );
}

#[test]
fn test_square_truncated_to_out_karatsuba() {
    let test = |xs: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; n];
        square_truncated_to_out_karatsuba(&mut result, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], result);
    };
    // - xs.len() >= n in padded
    test(&["-10"], 1, &["100"]);
    // - xs.len() < n in padded
    test(
        &["20", "-6", "19"],
        5,
        &["400", "-240", "796", "-228", "361"],
    );
    test(
        &["5", "19", "-8", "10", "-9", "16", "-7"],
        13,
        &[
            "25", "190", "281", "-204", "354", "-342", "782", "-702", "513", "-428", "382", "-224",
            "49",
        ],
    );
    test(
        &["-18", "5", "13", "-10", "4", "2", "-13", "-11", "-5", "-8"],
        8,
        &["324", "-180", "-443", "490", "-75", "-292", "692", "238"],
    );
    test(
        &[
            "-18", "15", "-18", "0", "-13", "4", "18", "9", "15", "20", "-1", "6", "-1", "17",
            "-5", "7", "4", "3", "8", "12",
        ],
        25,
        &[
            "324", "-540", "873", "-540", "792", "-534", "-60", "72", "-749", "-698", "-356",
            "-1056", "258", "-934", "1533", "-188", "869", "108", "366", "212", "361", "142",
            "654", "-104", "1053",
        ],
    );
}

#[test]
fn square_truncated_to_out_karatsuba_n_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_len_n", 32);
    let test = |(xs, n): (Vec<Integer>, u64)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        square_truncated_to_out_karatsuba_n(&mut out, &xs);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs[..n], &xs[..n])[..n], out);
        let mut out_alt = vec![Integer::ZERO; n];
        square_truncated_to_out_classical(&mut out_alt, &xs);
        assert_eq!(out_alt, out);
    };
    integer_vec_unsigned_pair_gen_var_4().test_properties(test);
    integer_vec_unsigned_pair_gen_var_4().test_properties_with_config(&config, test);
}

#[test]
fn square_truncated_to_out_karatsuba_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_len_n", 32);
    let test = |(xs, n): (Vec<Integer>, u64)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        square_truncated_to_out_karatsuba(&mut out, &xs);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], out);
        let mut out_alt = vec![Integer::ZERO; n];
        square_truncated_to_out(&mut out_alt, &xs);
        assert_eq!(out_alt, out);
    };
    integer_vec_unsigned_pair_gen_var_1().test_properties(test);
    integer_vec_unsigned_pair_gen_var_1().test_properties_with_config(&config, test);
}

#[test]
fn test_square_truncated_to_out_kronecker() {
    let test = |xs: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; n];
        square_truncated_to_out_kronecker(&mut result, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], result);
    };
    test(&["3"], 1, &["9"]);
    test(&["1", "2", "3"], 3, &["1", "4", "10"]);
    test(&["1", "-2", "3"], 5, &["1", "-4", "10", "-12", "9"]);
    test(
        &[
            "-671799350756517727438919942920",
            "-1181875036209060863636840150344",
            "-322391619931018672584987082004",
            "4789784227305365630917132762",
            "-755123903681646289778639808352",
        ],
        6,
        &[
            "451314367676878735686206236640734168298267001972896058126400",
            "1587965764001165937622675587699309335709737860972305396728960",
            "1829993563132169656989716323623191475242468918414434601741696",
            "755617667090605856218372408828462576838210288399156229728672",
            "1107198000255700568886643303206473597583929381832238803251440",
            "1781835809419824154682981492090362467607750075603876755716080",
        ],
    );
}

#[test]
fn square_truncated_to_out_kronecker_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_len_n", 32);
    let test = |(xs, n): (Vec<Integer>, u64)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        square_truncated_to_out_kronecker(&mut out, &xs);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], out);
    };
    integer_vec_unsigned_pair_gen_var_1().test_properties(test);
    integer_vec_unsigned_pair_gen_var_1().test_properties_with_config(&config, test);
}
