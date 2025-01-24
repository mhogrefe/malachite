// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::chars::constants::NUMBER_OF_CHARS;
use malachite_base::chars::exhaustive::chars_increasing;
use malachite_base::num::conversion::traits::ExactFrom;

#[test]
fn test_chars_increasing() {
    assert_eq!(
        chars_increasing().take(200).collect::<String>(),
        "\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t\n\u{b}\u{c}\r\u{e}\u{f}\u{10}\u{11}\u{12}\
        \u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&\'()*\
        +,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}\
        \u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\
        \u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\
        \u{9e}\u{9f}\u{a0}¡¢£¤¥¦§¨©ª«¬\u{ad}®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇ"
    );
    assert_eq!(
        chars_increasing().rev().take(200).collect::<String>(),
        "\u{10ffff}\u{10fffe}\u{10fffd}\u{10fffc}\u{10fffb}\u{10fffa}\u{10fff9}\u{10fff8}\u{10fff7}\
        \u{10fff6}\u{10fff5}\u{10fff4}\u{10fff3}\u{10fff2}\u{10fff1}\u{10fff0}\u{10ffef}\u{10ffee}\
        \u{10ffed}\u{10ffec}\u{10ffeb}\u{10ffea}\u{10ffe9}\u{10ffe8}\u{10ffe7}\u{10ffe6}\u{10ffe5}\
        \u{10ffe4}\u{10ffe3}\u{10ffe2}\u{10ffe1}\u{10ffe0}\u{10ffdf}\u{10ffde}\u{10ffdd}\u{10ffdc}\
        \u{10ffdb}\u{10ffda}\u{10ffd9}\u{10ffd8}\u{10ffd7}\u{10ffd6}\u{10ffd5}\u{10ffd4}\u{10ffd3}\
        \u{10ffd2}\u{10ffd1}\u{10ffd0}\u{10ffcf}\u{10ffce}\u{10ffcd}\u{10ffcc}\u{10ffcb}\u{10ffca}\
        \u{10ffc9}\u{10ffc8}\u{10ffc7}\u{10ffc6}\u{10ffc5}\u{10ffc4}\u{10ffc3}\u{10ffc2}\u{10ffc1}\
        \u{10ffc0}\u{10ffbf}\u{10ffbe}\u{10ffbd}\u{10ffbc}\u{10ffbb}\u{10ffba}\u{10ffb9}\u{10ffb8}\
        \u{10ffb7}\u{10ffb6}\u{10ffb5}\u{10ffb4}\u{10ffb3}\u{10ffb2}\u{10ffb1}\u{10ffb0}\u{10ffaf}\
        \u{10ffae}\u{10ffad}\u{10ffac}\u{10ffab}\u{10ffaa}\u{10ffa9}\u{10ffa8}\u{10ffa7}\u{10ffa6}\
        \u{10ffa5}\u{10ffa4}\u{10ffa3}\u{10ffa2}\u{10ffa1}\u{10ffa0}\u{10ff9f}\u{10ff9e}\u{10ff9d}\
        \u{10ff9c}\u{10ff9b}\u{10ff9a}\u{10ff99}\u{10ff98}\u{10ff97}\u{10ff96}\u{10ff95}\u{10ff94}\
        \u{10ff93}\u{10ff92}\u{10ff91}\u{10ff90}\u{10ff8f}\u{10ff8e}\u{10ff8d}\u{10ff8c}\u{10ff8b}\
        \u{10ff8a}\u{10ff89}\u{10ff88}\u{10ff87}\u{10ff86}\u{10ff85}\u{10ff84}\u{10ff83}\u{10ff82}\
        \u{10ff81}\u{10ff80}\u{10ff7f}\u{10ff7e}\u{10ff7d}\u{10ff7c}\u{10ff7b}\u{10ff7a}\u{10ff79}\
        \u{10ff78}\u{10ff77}\u{10ff76}\u{10ff75}\u{10ff74}\u{10ff73}\u{10ff72}\u{10ff71}\u{10ff70}\
        \u{10ff6f}\u{10ff6e}\u{10ff6d}\u{10ff6c}\u{10ff6b}\u{10ff6a}\u{10ff69}\u{10ff68}\u{10ff67}\
        \u{10ff66}\u{10ff65}\u{10ff64}\u{10ff63}\u{10ff62}\u{10ff61}\u{10ff60}\u{10ff5f}\u{10ff5e}\
        \u{10ff5d}\u{10ff5c}\u{10ff5b}\u{10ff5a}\u{10ff59}\u{10ff58}\u{10ff57}\u{10ff56}\u{10ff55}\
        \u{10ff54}\u{10ff53}\u{10ff52}\u{10ff51}\u{10ff50}\u{10ff4f}\u{10ff4e}\u{10ff4d}\u{10ff4c}\
        \u{10ff4b}\u{10ff4a}\u{10ff49}\u{10ff48}\u{10ff47}\u{10ff46}\u{10ff45}\u{10ff44}\u{10ff43}\
        \u{10ff42}\u{10ff41}\u{10ff40}\u{10ff3f}\u{10ff3e}\u{10ff3d}\u{10ff3c}\u{10ff3b}\u{10ff3a}\
        \u{10ff39}\u{10ff38}"
    );
    assert_eq!(
        chars_increasing().count(),
        usize::exact_from(NUMBER_OF_CHARS)
    );
}
