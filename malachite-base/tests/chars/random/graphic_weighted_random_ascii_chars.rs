// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::chars::random::graphic_weighted_random_ascii_chars;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

fn graphic_weighted_random_ascii_chars_helper(
    p_numerator: u64,
    p_denominator: u64,
    expected_values: &str,
    expected_common_values: &[(char, usize)],
    expected_median: (char, Option<char>),
) {
    let xs = graphic_weighted_random_ascii_chars(EXAMPLE_SEED, p_numerator, p_denominator);
    let values = xs.clone().take(200).collect::<String>();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_str(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_graphic_weighted_random_ascii_chars() {
    // p = 1/2
    graphic_weighted_random_ascii_chars_helper(
        1,
        2,
        "\u{1b}x\u{8}1\r4\u{2}N\u{11}\u{11}(\u{13}bcXr$g)\t7/E\u{11}+fY\u{10}Po\u{1}\u{17}\u{17}\
        \u{13}o\u{1}.\u{0}\u{b}\u{3}$\u{6}\nV2R.\u{f}\u{5}\u{19}$\u{1f}V=\u{1c}\u{6}\u{15}\u{6}\
        \u{11}\r\u{19}6\u{2}\u{19}=\u{12}\u{18}Dq\u{6}S<\u{6}\u{1d}C\u{b}M\u{8}\u{15}\u{16}\u{f}W_\
        \u{0}\u{12}%\u{18}\u{10}\u{10}OX?\u{1f}\u{12}b\u{c}}\u{10}rJa\u{e}D\u{1e}`o635\u{2}Q:w\u{3}\
        \u{1}3m\u{5}Y\u{1f}=\u{7}8\tn\r\u{1}\nq\u{13}\u{3}\t\u{f}fR:/\u{f}2\u{2}\u{1c}6\u{13}\u{13}\
        z\u{15}r\u{f}\u{4}<\u{1d}<a\u{4}A\nZ\u{18}\u{7}<\u{19}$2\u{12}/2)jCz!\u{5}\u{14}r\u{15}\
        \u{16}{%\u{1f}\u{5}(r\u{14}\u{12}\u{11}\u{14}\u{4}N\u{c}?X\u{5}\u{1d}<\u{b} \u{0}!\u{10}\
        \u{1e}",
        &[
            ('\u{13}', 15346),
            ('\r', 15339),
            ('\u{19}', 15334),
            ('\u{1}', 15259),
            ('\u{1b}', 15259),
            ('\u{1d}', 15225),
            ('\n', 15209),
            ('\u{6}', 15206),
            ('\u{1e}', 15194),
            ('\u{12}', 15193),
        ],
        ('\"', None),
    );
    // p = 1/51
    graphic_weighted_random_ascii_chars_helper(
        1,
        51,
        "\u{1b}\u{8}x\r\u{2}\u{11}\u{11}\u{13}\t\u{11}\u{10}\u{1}\u{17}\u{17}\u{13}\u{1}\u{0}\u{b}\
        \u{3}\u{6}\n\u{f}\u{5}\u{19}\u{1f}\u{1c}\u{6}\u{15}\u{6}\u{11}\r\u{19}\u{2}\u{19}\u{12}\
        \u{18}\u{6}\u{6}1\u{1d}\u{b}\u{8}\u{15}\u{16}\u{f}\u{0}\u{12}\u{18}\u{10}\u{10}\u{1f}\u{12}\
        \u{c}\u{10}\u{e}\u{1e}\u{2}\u{3}\u{1}\u{5}\u{1f}\u{7}\t\r\u{1}\n\u{13}\u{3}\t\u{f}\u{f}\
        \u{2}\u{1c}\u{13}\u{13}\u{15}\u{f}\u{4}\u{1d}\u{4}\n\u{18}\u{7}\u{19}\u{12}\u{5}\u{14}\
        \u{15}\u{16}\u{1f}\u{5}\u{14}\u{12}\u{11}\u{14}\u{4}\u{c}\u{5}\u{1d}\u{b}\u{0}\u{10}\u{1e}\
        \u{1e}\u{10}\u{19}\u{4}\u{15}4\u{1f}\u{12}\u{1b}\u{e}\u{f}\u{7}\u{1e}\u{10}\u{1d}\u{c}\
        \u{13}\u{11}\u{f}\u{18}\u{1e}\u{15}\u{19}\u{b}\u{3}\u{15}N\u{16}\u{14}\u{6}\u{b}\u{1d}\u{c}\
        \u{1f}\u{17}\u{13}\n\u{1f}\u{7f}\u{1a}\u{c}\u{5}\u{7}\u{1f}\u{1a}\u{1d}\u{1d}\u{c}\u{1f}\
        \u{17}\t\u{1f}\u{14}\u{b}\u{5}\u{13}\u{17}\u{0}\u{b}\u{10}\u{10}\u{6}\u{19}\u{1d}\u{18}\
        \u{0}\u{11}\u{7}\t\u{2}\u{13}\u{11}\u{1f}\u{1b}\u{19}\u{13}\u{6}\u{4}\u{11}\u{1d}\u{15}\
        \u{8}\u{8}\u{2}\u{18}\u{f}\u{b}\u{f}\u{1a}\t\u{11}\u{1}\u{11}\u{3}\n\u{16}\u{e}",
        &[
            ('\u{13}', 30211),
            ('\u{19}', 30173),
            ('\r', 30045),
            ('\u{1}', 29977),
            ('\u{3}', 29971),
            ('\u{4}', 29936),
            ('\u{f}', 29920),
            ('\u{12}', 29882),
            ('\u{18}', 29864),
            ('\u{14}', 29793),
        ],
        ('\u{10}', None),
    );
    // p = 50/51
    graphic_weighted_random_ascii_chars_helper(
        50,
        51,
        "x14N(bcXr$g)7/E+fYPoo.$V2R.$V=6=DqS<CMW_%OX?b}rJaD`o635Q:w3mY=8nq\u{1b}fR:/26zr<<aAZ<$2/2)\
        jCz!r{%(rN?X< !:z*KWArQ1-#AJCW-/}v,m`xKpzt5?u\u{8}IJN\'wYe9f\".RmqMxz7l7qOjs`$-%s5LO`L{G5y\
        \r8%7/J ZD!4!Gq.kQb>pr\u{2}+UFFc;8P:",
        &[
            ('d', 10548),
            ('C', 10506),
            ('7', 10501),
            ('R', 10486),
            ('D', 10484),
            ('q', 10476),
            ('1', 10468),
            ('6', 10463),
            ('\'', 10452),
            ('$', 10448),
        ],
        ('N', None),
    );
}
#[test]
#[should_panic]
fn graphic_weighted_random_ascii_chars_fail_1() {
    graphic_weighted_random_ascii_chars(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn graphic_weighted_random_ascii_chars_fail_2() {
    graphic_weighted_random_ascii_chars(EXAMPLE_SEED, 2, 1);
}
