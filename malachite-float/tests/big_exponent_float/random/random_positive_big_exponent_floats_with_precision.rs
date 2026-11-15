// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use malachite_base::random::EXAMPLE_SEED;
use malachite_float::big_exponent_float::random::*;
use malachite_float::test_util::big_exponent_float::random_big_exponent_floats_helper_helper;

fn random_positive_big_exponent_floats_with_precision_helper(
    mean_exponent_bits_numerator: u64,
    mean_exponent_bits_denominator: u64,
    prec: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
) {
    let xs = random_positive_big_exponent_floats_with_precision(
        EXAMPLE_SEED,
        mean_exponent_bits_numerator,
        mean_exponent_bits_denominator,
        prec,
    );
    for x in xs.clone().take(10000) {
        assert_eq!(x.get_prec(), Some(prec));
        assert!(x.is_sign_positive());
    }
    random_big_exponent_floats_helper_helper(
        xs,
        expected_values,
        expected_values_hex,
        expected_common_values,
        expected_common_values_hex,
    );
}

#[test]
fn test_random_positive_big_exponent_floats_with_precision() {
    // mean exponent bits = 1, precision = 1
    random_positive_big_exponent_floats_with_precision_helper(
        1,
        1,
        1,
        &[
            "1.0", "2.0", "8.0", "1.0", "1.0", "0.00012", "4.0", "1.0", "1.0", "1.0", "1.0", "1.0",
            "1.0", "2.0", "1.3e33", "4.8e-7", "32.0", "1.3e2", "0.50", "0.50",
        ],
        &[
            "0x1.0",
            "0x2.0",
            "0x8.0",
            "0x1.0",
            "0x1.0",
            "0x0.0008",
            "0x4.0",
            "0x1.0",
            "0x1.0",
            "0x1.0",
            "0x1.0",
            "0x1.0",
            "0x1.0",
            "0x2.0",
            "0x4.0E+27",
            "0x8.0E-6",
            "0x2.0E+1",
            "0x8.0E+1",
            "0x0.8",
            "0x0.8",
        ],
        &[
            ("1.0", 333732),
            ("0.50", 166936),
            ("2.0", 166566),
            ("0.12", 41707),
            ("8.0", 41569),
            ("0.25", 41510),
            ("4.0", 41127),
            ("1.3e2", 10447),
            ("0.031", 10440),
            ("16.0", 10421),
        ],
        &[
            ("0x1.0", 333732),
            ("0x0.8", 166936),
            ("0x2.0", 166566),
            ("0x0.2", 41707),
            ("0x8.0", 41569),
            ("0x0.4", 41510),
            ("0x4.0", 41127),
            ("0x8.0E+1", 10447),
            ("0x0.08", 10440),
            ("0x1.0E+1", 10421),
        ],
    );
    // mean exponent bits = 4, precision = 10
    random_positive_big_exponent_floats_with_precision_helper(
        4,
        1,
        10,
        &[
            "0.95996",
            "0.24780",
            "7.1744e-19",
            "2.1158e-8",
            "1.5566",
            "0.37842",
            "1.6251e33",
            "12.328",
            "0.00022101",
            "234.75",
            "5.8661e22",
            "7.3777e14",
            "0.89355",
            "11.453",
            "3.8867",
            "3.7305",
            "0.42529",
            "1.6441e13",
            "0.000033200",
            "3.9387e7",
        ],
        &[
            "0x0.f5c",
            "0x0.3f7",
            "0xd.3cE-16",
            "0x5.aeE-7",
            "0x1.8e8",
            "0x0.60e",
            "0x5.02E+27",
            "0xc.54",
            "0x0.000e7c",
            "0xea.c",
            "0xc.6cE+18",
            "0x2.9fE+12",
            "0x0.e4c",
            "0xb.74",
            "0x3.e3",
            "0x3.bb",
            "0x0.6ce",
            "0xe.f4E+10",
            "0x0.00022d",
            "0x2.59E+6",
        ],
        &[
            ("1.3223", 494),
            ("1.4277", 491),
            ("1.5293", 484),
            ("1.7910", 482),
            ("1.7480", 481),
            ("1.4004", 477),
            ("1.6934", 477),
            ("1.7285", 475),
            ("1.0605", 474),
            ("1.5020", 474),
        ],
        &[
            ("0x1.528", 494),
            ("0x1.6d8", 491),
            ("0x1.878", 484),
            ("0x1.ca8", 482),
            ("0x1.bf8", 481),
            ("0x1.668", 477),
            ("0x1.b18", 477),
            ("0x1.ba8", 475),
            ("0x1.0f8", 474),
            ("0x1.808", 474),
        ],
    );
    // mean exponent bits = 16, precision = 100
    random_positive_big_exponent_floats_with_precision_helper(
        16,
        1,
        100,
        &[
            "0.010021338353651594775649036779221",
            "2.4356558241305258917641530048479e-15",
            "5.1184420028984340154349065345453e-96",
            "3.9938985895611157046264927380646e2854",
            "4.7149913124074668583667743968864e120",
            "1.9506435204221623739290716399661e534",
            "6.7317464550853232295970075944985e-2946322415",
            "8.6356676326493692273914741333623e-24573365",
            "0.72417035602789059376967750346040",
            "9.1872672994677707839364464709396e-9",
            "8.7716092238220116405257794863409e-5525520",
            "1.2685519626775662166760224806901e9258",
            "1.7056543316107062159526263671018e-121968900440936019756729",
            "2.5864284528264938753154096796168",
            "5.9819400426500573226328908564200e862566",
            "0.00058904964772754477423765016277019",
            "3.5969128995739901706391554569206e78269862",
            "5.3661108996149779179377289630275e125137762",
            "1.1681682749836000028221894048512e7585026925221349",
            "2.8590034516836357681363071039229e-169",
        ],
        &[
            "0x0.0290c2287db7afb6a873dc7b75c",
            "0xa.f81eb6d7c73be5607e7c934fE-13",
            "0xa.eed43906b1290314433e911dE-80",
            "0x6.df0cb78aef3fb3e2f3e28b408E+2370",
            "0x1.d36fbcb56dd4f1f9e198a0f3eE+100",
            "0x7.5424304f4508a8b44be65b8d8E+443",
            "0xd.9af5ccdc5e1e57fad3242793E-2446867802",
            "0xb.8efbc5ba1f3caf4b471d67e3E-20407738",
            "0x0.b9633a7bdf59f7ffb12fb7167",
            "0x2.77581d9620aeb93c0344cf8b4E-7",
            "0x8.088185366f51d7dbc9c73cadE-4588845",
            "0x6.be5230d917d7be141b4990108E+7688",
            "0x2.b3ec01d502f43e0add6ebd01cE-101292979269316240387212",
            "0x2.96202cd2556b64f0377d2010c",
            "0x1.c0bb37961d18237d3e8233116E+716346",
            "0x0.00269a9cf90387e19cca2a7669a4",
            "0xa.9c77a2f311f698b4f7f92fddE+65001713",
            "0x3.58e9be942f633cf45c44dd594E+103924662",
            "0xf.50d9baff3ed9313e23ee29bfE+6299228510842475",
            "0x1.1437d8def0930e1b53f4991caE-140",
        ],
        &[
            ("8.0014594527193886107216602004030", 1),
            ("8.0015138848309419258808135829718", 1),
            ("8.0016331316476314499700282708346", 1),
            ("8.0017233158552534901915804036990", 1),
            ("8.0021907096186895258992545383857", 1),
            ("8.0022943083926182815013671266535", 1),
            ("8.0025971170191789703927629724968", 1),
            ("8.0027253254273477805504586386303", 1),
            ("8.0028998657553851923482204139549", 1),
            ("8.0033567929740177410977339774571", 1),
        ],
        &[
            ("0x8.005fa58db32836780c8bbeb1", 1),
            ("0x8.006336c5d6bbfe75ad65e451", 1),
            ("0x8.006b076704427f656c383553", 1),
            ("0x8.0070f0713d2e9493a77153b5", 1),
            ("0x8.008f92022ad471c3759a0775", 1),
            ("0x8.00965c1b83634b1b76e13b15", 1),
            ("0x8.00aa3464a9483fbfff311009", 1),
            ("0x8.00b29b5f94d7acd60a3c00d9", 1),
            ("0x8.00be0bac9509136d36e3ffe3", 1),
            ("0x8.00dbfda40af8232a788405a5", 1),
        ],
    );
}

#[test]
#[should_panic]
fn random_positive_big_exponent_floats_with_precision_fail_1() {
    random_positive_big_exponent_floats_with_precision(EXAMPLE_SEED, 0, 1, 0);
}

#[test]
#[should_panic]
fn random_positive_big_exponent_floats_with_precision_fail_2() {
    random_positive_big_exponent_floats_with_precision(EXAMPLE_SEED, 1, 0, 10);
}
