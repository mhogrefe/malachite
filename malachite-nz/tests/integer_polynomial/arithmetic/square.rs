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
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::mul::classical::mul_to_out_classical;
use malachite_nz::integer_polynomial::arithmetic::square::classical::square_to_out_classical;
use malachite_nz::integer_polynomial::arithmetic::square::karatsuba::square_to_out_karatsuba;
use malachite_nz::integer_polynomial::arithmetic::square::kronecker::square_to_out_kronecker;
use malachite_nz::integer_polynomial::arithmetic::square::schonhage_strassen::*;
use malachite_nz::integer_polynomial::arithmetic::square::square_to_out;
use malachite_nz::integer_polynomial::arithmetic::square::tiny::{
    square_to_out_tiny_1, square_to_out_tiny_2,
};
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_pair_gen, integer_vec_gen_var_1,
    integer_vec_gen_var_2, integer_vec_gen_var_3,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::{
    generated_coefficients, integers_mul_naive,
};
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
    #[cfg(not(feature = "32_bit_limbs"))]
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
    #[cfg(feature = "32_bit_limbs")]
    test(
        &["536870912", "-536870911", "0"],
        &["288230376151711744", "-576460751229681664", "288230375077969921", "0", "0"],
    );
    #[cfg(not(feature = "32_bit_limbs"))]
    test(
        &["0", "1099511627776", "0", "5"],
        &["0", "0", "1208925819614629174706176", "0", "10995116277760", "0", "25"],
    );
    #[cfg(feature = "32_bit_limbs")]
    test(
        &["0", "256", "0", "0"],
        &["0", "0", "65536", "0", "0", "0", "0"],
    );
    #[cfg(not(feature = "32_bit_limbs"))]
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
    #[cfg(feature = "32_bit_limbs")]
    test(
        &["-256", "0", "262144", "0"],
        &["65536", "0", "-134217728", "0", "68719476736", "0", "0"],
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
    // Generated coefficients of `bits` bits.
    let test_generated = |len: usize, bits: u64| {
        let xs = generated_coefficients(len, bits);
        let mut result = vec![Integer::ZERO; (len << 1) - 1];
        square_to_out(&mut result, &xs);
        assert_eq!(integers_mul_naive(&xs, &xs), result);
    };
    // - karatsuba_preferred(len, bits, bits), with len <= 4
    test_generated(4, 6000);
    // - karatsuba_preferred(len, bits, bits), with 1500 <= 2 * bits <= 10000
    test_generated(8, 1000);
    // - !karatsuba_preferred(len, bits, bits)
    test_generated(8, 100);
    // - fft_preferred(len, bits, bits, 80, 160), and the FFT computes the square
    test_generated(100, 20);
    // - fft_preferred(len, bits, bits, 80, 160), but the FFT declines
    test_generated(170, 250);
    // - schonhage_strassen_preferred(len, len, bits, bits, 4097), with 16 <= len <= 100
    test_generated(20, 450);
    // - schonhage_strassen_preferred(len, len, bits, bits, 4097), with len > 100
    test_generated(150, 600);
    // - schonhage_strassen_preferred(len, len, bits, bits, 4097), with 8 <= len < 16
    test_generated(10, 550);
    // - !schonhage_strassen_preferred(len, len, bits, bits, 4097), with 8 <= len < 16 and 2 * bits
    //   < 1000
    test_generated(10, 450);
    // - !schonhage_strassen_preferred(len, len, bits, bits, 4097), with 2 * len > 4097
    test_generated(2049, 600);
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

#[test]
fn test_square_to_out_karatsuba() {
    let test = |xs: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_karatsuba(&mut result, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &xs), result);
    };
    // - len == 1
    test(&["3"], &["9"]);
    // - len != 1
    // - length == 1
    // - length != 1
    test(&["18", "20"], &["324", "720", "400"]);
    test(&["-16", "-19", "-6"], &["256", "608", "553", "228", "36"]);
    test(
        &["-14", "10", "9", "4"],
        &["196", "-280", "-152", "68", "161", "72", "16"],
    );
    test(
        &["-4", "7", "11", "-12", "11"],
        &["16", "-56", "-39", "250", "-135", "-110", "386", "-264", "121"],
    );
    test(
        &["-9", "-20", "-1", "-11", "18", "-5", "0", "0"],
        &[
            "81", "360", "418", "238", "117", "-608", "285", "-386", "434", "-180", "25", "0", "0",
            "0", "0",
        ],
    );
    test(
        &["9", "3", "18", "-15", "12", "-8", "5", "-10", "-5"],
        &[
            "81", "54", "333", "-162", "450", "-612", "699", "-798", "414", "-732", "304", "-170",
            "65", "-20", "50", "100", "25",
        ],
    );
    test(
        &[
            "6", "-16", "-18", "10", "15", "14", "0", "-10", "7", "-14", "-16", "-4", "19", "-15",
            "-7", "-14",
        ],
        &[
            "36", "-192", "40", "696", "184", "-672", "-888", "-324", "909", "388", "0", "808",
            "582", "-1188", "-1140", "268", "1187", "570", "-858", "-604", "542", "-474", "10",
            "328", "1097", "-66", "71", "-322", "469", "196", "196",
        ],
    );
    test(
        &[
            "6", "11", "8", "-9", "-6", "-12", "6", "9", "19", "-5", "14", "-13", "-2", "-2", "-3",
            "16", "-3",
        ],
        &[
            "36", "132", "217", "68", "-206", "-420", "-207", "156", "774", "538", "272", "-522",
            "-404", "-816", "383", "232", "1079", "222", "47", "-760", "-170", "-210", "271",
            "580", "-302", "564", "-484", "26", "-43", "-84", "274", "-96", "9",
        ],
    );
    test(
        &[
            "258310355284319173799020452054",
            "257122348541910768367567340522",
            "-784958425684266716367207977583",
            "-348292516565995701349735129537",
            "-810613512178953392276046349563",
            "-78511595705566051977176049842",
        ],
        &[
            "66724239647111198515500735834416036254592545484578512818916",
            "132834730406799033316097861767697402990017136724349984664376",
            "-339413877524137726321283258023180133880172803927991613378480",
            "-583595835233463118928586260725076730951280986582926982474648",
            "18272421716249642790821340754935930929471994913162981420457",
            "89375874578704985817421055627147998507207269406470310003434",
            "1353529318061468950182850923063224018068802820198225824931779",
            "687917917364417692551055303261861681444296417199314830068434",
            "711784268622905461871975788913747701600996862500949667057277",
            "127285120683325863451419478557924483626528662432552485838092",
            "6164070660234257734985444240448931172471260268814868224964",
        ],
    );
}

#[test]
fn square_to_out_karatsuba_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_len_n", 32);
    let test = |xs: Vec<Integer>| {
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_karatsuba(&mut out, &xs);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(integers_mul_naive(&xs, &xs), out);
        let mut out_alt = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_classical(&mut out_alt, &xs);
        assert_eq!(out_alt, out);
    };
    integer_vec_gen_var_1().test_properties(test);
    integer_vec_gen_var_1().test_properties_with_config(&config, test);
}

#[test]
fn test_square_to_out_kronecker() {
    let test = |xs: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_kronecker(&mut result, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &xs), result);
    };
    test(&["3"], &["9"]);
    test(&["1", "2", "3"], &["1", "4", "10", "12", "9"]);
    test(&["1", "-2", "3"], &["1", "-4", "10", "-12", "9"]);
    test(
        &[
            "127045069494630545379112202255",
            "8639614695295244635568505446",
            "531191510928739164466910347515",
            "784741010512824463010586856737",
            "1123093540348611836825705550910",
        ],
        &[
            "16140449682895504779053258210494836768807819848636027085025",
            "2195240898741231317393919544763125977619598147104041961460",
            "135045167783882097511084974182705446645536863956980164951566",
            "208573532399448805453187042240657064087616639707806566817250",
            "581091134980731786257942541995202471076514713515326317858729",
            "853101717034879681778904311398516399835993788194997908428830",
            "1808973962804860882952008390185728112267022980998475011264469",
            "1762675119507190493336990192826453550789672153672859659961340",
            "1261339100372779003934498233113316595678042798323586601828100",
        ],
    );
}

#[test]
fn test_square_to_out_schonhage_strassen() {
    let test = |xs: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_schonhage_strassen(&mut result, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &xs), result);
    };
    test(&["3"], &["9"]);
    test(&["1", "2", "3"], &["1", "4", "10", "12", "9"]);
    test(&["1", "-2", "3"], &["1", "-4", "10", "-12", "9"]);
    test(
        &[
            "127045069494630545379112202255",
            "8639614695295244635568505446",
            "531191510928739164466910347515",
            "784741010512824463010586856737",
            "1123093540348611836825705550910",
        ],
        &[
            "16140449682895504779053258210494836768807819848636027085025",
            "2195240898741231317393919544763125977619598147104041961460",
            "135045167783882097511084974182705446645536863956980164951566",
            "208573532399448805453187042240657064087616639707806566817250",
            "581091134980731786257942541995202471076514713515326317858729",
            "853101717034879681778904311398516399835993788194997908428830",
            "1808973962804860882952008390185728112267022980998475011264469",
            "1762675119507190493336990192826453550789672153672859659961340",
            "1261339100372779003934498233113316595678042798323586601828100",
        ],
    );
}

#[test]
fn square_to_out_kronecker_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_len_n", 32);
    let test = |xs: Vec<Integer>| {
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_kronecker(&mut out, &xs);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(integers_mul_naive(&xs, &xs), out);
    };
    integer_vec_gen_var_1().test_properties(test);
    integer_vec_gen_var_1().test_properties_with_config(&config, test);
}

#[test]
fn square_to_out_schonhage_strassen_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_len_n", 32);
    let mut wide_config = GenConfig::new();
    wide_config.insert("mean_len_n", 20);
    wide_config.insert("mean_bits_n", 1000);
    let test = |xs: Vec<Integer>| {
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        square_to_out_schonhage_strassen(&mut out, &xs);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(integers_mul_naive(&xs, &xs), out);
    };
    integer_vec_gen_var_1().test_properties(test);
    integer_vec_gen_var_1().test_properties_with_config(&config, test);
    integer_vec_gen_var_1().test_properties_with_config(&wide_config, test);
}
