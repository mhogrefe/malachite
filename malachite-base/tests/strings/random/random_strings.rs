// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::random::random_strings;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

fn random_strings_helper(
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
) {
    let ss = random_strings(EXAMPLE_SEED, mean_length_numerator, mean_length_denominator);
    let values = ss.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, ss.clone());
    let (median_lo, median_hi) = median(ss.take(1000000));
    assert_eq!(
        (
            values.iter().map(String::as_str).collect_vec().as_slice(),
            common_values
                .iter()
                .map(|(s, f)| (s.as_str(), *f))
                .collect_vec()
                .as_slice(),
            (median_lo.as_str(), median_hi.as_deref())
        ),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_strings() {
    random_strings_helper(
        4,
        1,
        &[
            "",
            "\u{81355}\u{a331d}\u{b707b}\u{1354b}\u{b16ac}𣙘\u{67377}\u{4aaa4}\u{a6d6e}\u{45616}\
            \u{7725f}\u{41e2d}\u{d6b59}\u{de165}",
            "\u{c2d29}\u{695af}\u{98fd7}\u{10ca51}",
            "\u{bec46}\u{c0bec}\u{cb677}\u{71318}",
            "\u{755e1}",
            "",
            "𫮜\u{a2f84}柂\u{f5560}\u{6737b}",
            "\u{8442e}\u{a6883}",
            "\u{49cf2}\u{32d2b}\u{1e6e5}\u{1084bd}",
            "",
            "\u{85172}\u{103bd2}\u{bffa3}\u{c792c}\u{4c421}\u{905ef}",
            "",
            "",
            "\u{c0f92}匥\u{105164}𦤃\u{8ebb2}\u{9e650}\u{399cd}\u{51328}\u{61706}\u{1096f6}\
            \u{d9e03}",
            "\u{ac5ce}\u{db625}\u{f1185}\u{f170e}\u{b7772}\u{dfa6f}\u{f65d0}\u{10c939}",
            "",
            "\u{3e110}𪐛\u{84380}",
            "",
            "\u{46189}\u{5832d}ꌍ𫁜\u{1083cd}",
            "\u{102636}\u{c0f80}\u{cacb1}\u{89486}\u{1ad3c}𖢸\u{b7896}\u{3dadc}\u{82b15}",
        ],
        &[
            ("", 199913),
            ("姑", 4),
            ("\u{36097}", 4),
            ("\u{3d772}", 4),
            ("\u{40698}", 4),
            ("\u{6e63c}", 4),
            ("\u{7d354}", 4),
            ("\u{8465e}", 4),
            ("\u{9090b}", 4),
            ("\u{929c5}", 4),
        ],
        (
            "\u{667cc}\u{850e8}\u{42572}𪩋\u{637c9}\u{ef6f1}",
            Some("\u{667cc}\u{a3e1a}\u{bead8}𢰏\u{d2d0b}"),
        ),
    );
    random_strings_helper(
        10,
        1,
        &[
            "\u{81355}\u{a331d}\u{b707b}\u{1354b}\u{b16ac}𣙘\u{67377}\u{4aaa4}\u{a6d6e}\u{45616}\
            \u{7725f}\u{41e2d}\u{d6b59}\u{de165}\u{c2d29}\u{695af}\u{98fd7}\u{10ca51}\u{bec46}\
            \u{c0bec}\u{cb677}\u{71318}\u{755e1}𫮜\u{a2f84}柂\u{f5560}\u{6737b}",
            "\u{8442e}\u{a6883}\u{49cf2}\u{32d2b}\u{1e6e5}\u{1084bd}\u{85172}\u{103bd2}\u{bffa3}\
            \u{c792c}\u{4c421}\u{905ef}\u{c0f92}匥\u{105164}𦤃\u{8ebb2}\u{9e650}",
            "\u{399cd}\u{51328}\u{61706}\u{1096f6}\u{d9e03}\u{ac5ce}\u{db625}\u{f1185}\u{f170e}\
            \u{b7772}\u{dfa6f}\u{f65d0}\u{10c939}\u{3e110}𪐛\u{84380}\u{46189}\u{5832d}ꌍ",
            "𫁜\u{1083cd}\u{102636}",
            "\u{c0f80}\u{cacb1}\u{89486}\u{1ad3c}𖢸\u{b7896}\u{3dadc}\u{82b15}\u{e01b7}\u{53091}\
            \u{eafa3}\u{e6a8c}",
            "\u{af3e0}",
            "\u{56c4c}\u{b06f5}\u{76039}\u{49cbf}\u{103396}\u{99695}\u{fb2c5}\u{1269b}\u{d63a8}\
            \u{7b2ad}\u{62abc}\u{426cf}\u{917bd}\u{3ef1e}\u{900a7}𤧏\u{1079f9}\u{41860}\u{9861d}\
            \u{cbfd1}\u{48859}\u{5826c}\u{b5d68}\u{9d637}\u{10039d}\u{8e8d5}𡩔\u{50c19}\u{b9d8e}",
            "\u{f97c0}\u{5e025}\u{71838}\u{bdfd5}\u{d2042}",
            "\u{bdb07}\u{54a51}",
            "𞠮\u{f718c}轜\u{511f8}\u{d45cc}\u{cdcb6}\u{51127}\u{62ef1}𗛜\u{5a889}\u{4f824}",
            "\u{e4fc5}𛂮\u{67155}\u{990bb}\u{54368}\u{1ec21}\u{155e9}\u{cda02}𥵯\u{ac392}\u{41517}\
            \u{6d7f1}\u{101067}\u{127d7}\u{76266}\u{e4c58}\u{ba16f}\u{50d46}\u{69b8a}\u{b8556}\
            \u{e2a57}\u{c71bb}\u{b2276}\u{db65b}\u{72eba}\u{9ce32}𨚕쬽\u{b14b4}\u{721f3}",
            "\u{e92d0}\u{79265}\u{3413c}\u{8a37b}\u{3b281}",
            "\u{1038bc}\u{105572}\u{60004}\u{2f067}",
            "\u{8d937}\u{a77ba}\u{5cdfa}\u{d281b}\u{4fad0}\u{926dd}탘\u{589d1}",
            "\u{4c96e}\u{31c01}\u{d7b70}\u{3b41a}\u{fa158}\u{37ba2}\u{f7772}\u{51268}\u{a5a58}\
            \u{6dac7}\u{13515}",
            "\u{94c1f}\u{72cc7}\u{4e95e}\u{ce473}",
            "\u{10125f}\u{e1343}\u{ff236}\u{883de}凴\u{6274f}\u{ada6c}\u{b0b8b}\u{7f144}",
            "𠛾\u{72db9}\u{a49b2}𓅹\u{7fe45}\u{fb3c5}\u{86a5e}\u{b91ae}\u{7ef4b}",
            "\u{4e11e}\u{48ae3}\u{8e62a}\u{6a125}\u{692a6}\u{d811f}\u{1a0d4}\u{70978}\u{1b985}\
            \u{d83b6}\u{82dff}\u{41a7f}\u{94b31}깥\u{87f4b}\u{aa0ce}\u{f6b18}⠋\u{ded9c}𬟺\u{10df0a}\
            \u{b9982}\u{adaaa}𠰮\u{10db78}",
            "\u{1c15b}",
        ],
        &[
            ("", 90709),
            ("瞁", 4),
            ("\u{7cb3d}", 4),
            ("\u{844e7}", 4),
            ("摄", 3),
            ("覨", 3),
            ("霮", 3),
            ("鬻", 3),
            ("뀳", 3),
            ("𥛋", 3),
        ],
        (
            "\u{7abb9}⼥\u{8df6b}\u{6f616}\u{7661d}\u{68c62}",
            Some(
                "\u{7abb9}\u{6e013}\u{10c6dc}\u{5be7a}\u{99103}\u{f9c20}\u{108e71}\u{12917}\
                \u{9f018}𥱗",
            ),
        ),
    );
    random_strings_helper(
        1,
        4,
        &[
            "",
            "",
            "\u{81355}",
            "\u{a331d}",
            "\u{b707b}\u{1354b}",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "\u{b16ac}𣙘",
            "\u{67377}",
            "",
            "\u{4aaa4}\u{a6d6e}",
            "",
            "",
            "",
            "",
        ],
        &[
            ("", 800023),
            ("\u{4a9ab}", 5),
            ("悧", 4),
            ("𦹝", 4),
            ("𱩖", 4),
            ("\u{1b9f0}", 4),
            ("\u{56b1b}", 4),
            ("\u{61e16}", 4),
            ("\u{8a520}", 4),
            ("\u{959f7}", 4),
        ],
        ("", None),
    );
}

#[test]
#[should_panic]
fn random_strings_fail_1() {
    random_strings(EXAMPLE_SEED, 0, 1);
}

#[test]
#[should_panic]
fn random_strings_fail_2() {
    random_strings(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_strings_fail_3() {
    random_strings(EXAMPLE_SEED, u64::MAX, u64::MAX - 1);
}
