// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::chars::constants::NUMBER_OF_CHARS;
use malachite_base::chars::exhaustive::exhaustive_chars;
use malachite_base::num::conversion::traits::ExactFrom;

#[test]
fn test_exhaustive_chars() {
    assert_eq!(
        exhaustive_chars().take(200).collect::<String>(),
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 !\"#$%&\'()*+,-./:;<=>?@[\\\
        ]^_`{|}~¡¢£¤¥¦§¨©ª«¬®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóô\
        õö÷øùúûüýþÿĀāĂăĄąĆćĈĉĊ"
    );
    assert_eq!(
        exhaustive_chars().count(),
        usize::exact_from(NUMBER_OF_CHARS)
    );
    let mut chars = exhaustive_chars()
        .skip(usize::exact_from(NUMBER_OF_CHARS) - 200)
        .collect_vec();
    chars.reverse();
    assert_eq!(
        chars.iter().collect::<String>(),
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
        exhaustive_chars().skip(94).take(200).collect::<String>(),
        "~¡¢£¤¥¦§¨©ª«¬®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùú\
        ûüýþÿĀāĂăĄąĆćĈĉĊċČčĎďĐđĒēĔĕĖėĘęĚěĜĝĞğĠġĢģĤĥĦħĨĩĪīĬĭĮįİıĲĳĴĵĶķĸĹĺĻļĽľĿŀŁłŃńŅņŇňŉŊŋŌōŎŏŐőŒœŔŕ\
        ŖŗŘřŚśŜŝŞşŠšŢţŤťŦŧŨ"
    );

    assert_eq!(
        exhaustive_chars().skip(141698).take(200).collect::<String>(),
        "𮤐𮤑𮤒𮤓𮤔𮤕𮤖𮤗𮤘𮤙𮤚𮤛𮤜𮤝𮤞𮤟𮤠𮤡𮤢𮤣𮤤𮤥𮤦𮤧𮤨𮤩𮤪𮤫𮤬𮤭𮤮𮤯𮤰𮤱𮤲𮤳𮤴𮤵𮤶𮤷𮤸𮤹𮤺𮤻𮤼𮤽𮤾𮤿𮥀𮥁𮥂𮥃𮥄𮥅𮥆𮥇𮥈𮥉𮥊𮥋𮥌𮥍𮥎𮥏𮥐𮥑𮥒𮥓𮥔𮥕𮥖𮥗𮥘𮥙𮥚𮥛𮥜𮥝𮥞𮥟𮥠𮥡𮥢𮥣𮥤𮥥𮥦𮥧𮥨\
        𮥩𮥪𮥫𮥬𮥭𮥮𮥯𮥰𮥱𮥲𮥳𮥴𮥵𮥶𮥷𮥸𮥹𮥺𮥻𮥼𮥽𮥾𮥿𮦀𮦁𮦂𮦃𮦄𮦅𮦆𮦇𮦈𮦉𮦊𮦋𮦌𮦍𮦎𮦏𮦐𮦑𮦒𮦓𮦔𮦕𮦖𮦗𮦘𮦙𮦚𮦛𮦜𮦝𮦞𮦟𮦠𮦡𮦢𮦣𮦤𮦥𮦦𮦧𮦨𮦩𮦪𮦫𮦬𮦭𮦮𮦯𮦰𮦱𮦲𮦳𮦴𮦵𮦶𮦷𮦸𮦹𮦺𮦻𮦼𮦽𮦾𮦿𮧀𮧁𮧂\
        𮧃𮧄𮧅𮧆𮧇𮧈𮧉𮧊𮧋𮧌𮧍𮧎𮧏𮧐𮧑𮧒𮧓𮧔𮧕𮧖𮧗"
    );
}
