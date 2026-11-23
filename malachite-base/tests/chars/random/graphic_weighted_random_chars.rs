// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::chars::random::graphic_weighted_random_chars;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

fn graphic_weighted_random_chars_helper(
    p_numerator: u64,
    p_denominator: u64,
    expected_values: &str,
    expected_common_values: &[(char, usize)],
    expected_median: (char, Option<char>),
) {
    let xs = graphic_weighted_random_chars(EXAMPLE_SEED, p_numerator, p_denominator);
    let values = xs.clone().take(200).collect::<String>();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_str(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_graphic_weighted_random_chars() {
    // p = 1/2
    graphic_weighted_random_chars_helper(
        1,
        2,
        "\u{933e9}𖽈\u{3ad5d}𭣅\u{dcd88}礑\u{ecf35}깝\u{3bc36}\u{d901b}ꄾ\u{91cba}𿆠東쭈𪭅╫𱶭𡡖\u{3b6a0}⢇𲉥𮁢\
        \u{7792a}濿𰌎𲁚\u{ccf4c}𾔜՟\u{ca979}\u{58fd1}\u{f4351}\u{38d81}𤷐\u{f7ba1}𨑢\u{1997d}\u{38ee8}\
        \u{603c3}𾧎\u{e5838}\u{c635e}𥎔𽭆𤂯𠈂\u{ff461}\u{b080d}\u{73a39}𰓵\u{f0344}𝪇𰇊\u{1b811}\u{35fd7}\
        \u{67825}\u{7cb4e}\u{b718b}\u{fc30f}\u{7ab3e}厝\u{f204c}\u{f7471}🔧\u{378af}\u{ddc4c}𩖿⋐\
        \u{a5d8c}𿋲𑆮\u{adb10}\u{7a80b}𐤅\u{101f4a}⎎\u{73a95}\u{52305}\u{6d6cb}\u{ce140}𠿝摊\u{1c59d}\
        \u{106d05}𮏉\u{64b02}\u{818a9}\u{58add}𖦿𮲫虻\u{cb39f}\u{da92e}𓂵\u{160e3}𤳧\u{8f7e3}𓤬𤑁𠞈\u{4016b}\
        뛆\u{67b4b}䁼𡛅ờ쀇𜵼\u{6e6f3}𩻆𪈻𓋏\u{5d3df}\u{8d913}𽷝憇\u{11eab}쇎\u{a400e}𬱡\u{d41b9}ㆄ\u{706d9}쬯\
        \u{74122}\u{c5a77}\u{a4ac1}𽊗\u{b7246}\u{47dbc}\u{d8f01}\u{8b115}𢳓𱿛𪐨𐒽\u{a272c}ჴ\u{10336d}\
        \u{599b1}𡀄\u{bfed1}\u{92dcc}𬻥\u{5681f}𙄦\u{f6d2b}\u{aa055}𥦻\u{e7577}좾呫\u{f944b}𫑃\u{b894d}䍟\
        \u{cab46}\u{eb694}𧇁\u{49084}始𭏵\u{de9b9}𦆝叟𫛼𐌎䆓𪆃𒍱\u{d1974}\u{5d745}𲐛\u{96176}\u{cab2b}𲮌뎃\
        \u{bf1ad}\u{88e88}𡆁𘅽\u{edbd}\u{ba5d9}\u{10098f}\u{12d30}\u{b1c1f}𒙰\u{caa6d}𥀁㨔\u{4599f}\
        \u{a304b}օ\u{d1c34}뇜\u{6e361}𩓌\u{cf7a2}\u{4cef1}",
        &[
            ('𿂢', 13),
            ('ẃ', 12),
            ('𗈹', 12),
            ('𠖓', 12),
            ('𡗒', 12),
            ('𧕩', 12),
            ('𫵜', 12),
            ('𬝡', 12),
            ('𭘰', 12),
            ('𱴖', 12),
        ],
        ('\u{36623}', None),
    );
    // p = 1/51
    graphic_weighted_random_chars_helper(
        1,
        51,
        "\u{933e9}\u{3ad5d}𖽈\u{dcd88}\u{ecf35}\u{3bc36}\u{d901b}\u{91cba}\u{3b6a0}\u{7792a}\
        \u{ccf4c}\u{ca979}\u{58fd1}\u{f4351}\u{38d81}\u{f7ba1}\u{1997d}\u{38ee8}\u{603c3}\u{e5838}\
        \u{c635e}\u{ff461}\u{b080d}\u{73a39}\u{f0344}\u{1b811}\u{35fd7}\u{67825}\u{7cb4e}\u{b718b}\
        \u{fc30f}\u{7ab3e}\u{f204c}\u{f7471}\u{378af}\u{ddc4c}\u{a5d8c}\u{adb10}𭣅\u{7a80b}\
        \u{101f4a}\u{73a95}\u{52305}\u{6d6cb}\u{ce140}\u{1c59d}\u{106d05}\u{64b02}\u{818a9}\
        \u{58add}\u{cb39f}\u{da92e}\u{160e3}\u{8f7e3}\u{4016b}\u{67b4b}\u{6e6f3}\u{5d3df}\u{8d913}\
        \u{11eab}\u{a400e}\u{d41b9}\u{706d9}\u{74122}\u{c5a77}\u{a4ac1}\u{b7246}\u{47dbc}\u{d8f01}\
        \u{8b115}\u{a272c}\u{10336d}\u{599b1}\u{bfed1}\u{92dcc}\u{5681f}\u{f6d2b}\u{aa055}\u{e7577}\
        \u{f944b}\u{b894d}\u{cab46}\u{eb694}\u{49084}\u{de9b9}\u{d1974}\u{5d745}\u{96176}\u{cab2b}\
        \u{bf1ad}\u{88e88}\u{edbd}\u{ba5d9}\u{10098f}\u{12d30}\u{b1c1f}\u{caa6d}\u{4599f}\u{a304b}\
        \u{d1c34}\u{6e361}\u{cf7a2}\u{4cef1}\u{10227c}\u{4d181}\u{7b440}\u{479e7}\u{36d57}礑\
        \u{c0c18}\u{ed511}\u{953d4}\u{7471c}\u{402cb}\u{5d8e7}\u{58d75}\u{fd441}\u{be630}\u{101c0}\
        \u{a0b95}\u{ee3f2}\u{fd883}\u{e6633}\u{fcf78}\u{2fa61}\u{91894}\u{382f6}\u{7a4a2}\u{ba773}깝\
        \u{addd2}\u{e34f3}\u{9608c}\u{fd3d7}\u{45dea}\u{1a830}\u{b303c}\u{86437}\u{77279}\u{476e1}\
        \u{75521}\u{107095}\u{7dffa}\u{10cb93}\u{f64a8}\u{9a3cd}\u{84907}\u{c5961}\u{b090c}\
        \u{b3b09}\u{d0ca1}\u{d3937}\u{fc428}\u{c9ccb}\u{bda48}\u{8c948}\u{a5d8a}\u{10c3ac}\u{dc0f5}\
        \u{56e5d}\u{10f0ba}\u{d24d5}\u{d68c7}\u{a4514}\u{c99b8}\u{6bdb1}\u{c0a5e}\u{7bbf2}\u{c72ac}\
        \u{54e92}\u{53979}\u{bd0f6}\u{a62b4}\u{ff14e}\u{f41df}\u{5986e}\u{11879}\u{1c9be}\u{36013}\
        \u{395a0}\u{19e21}\u{b09d8}\u{81217}\u{57396}\u{15007}\u{41825}\u{101179}\u{493e7}\u{c0f40}\
        \u{59a7e}\u{826dc}\u{b7a52}\u{8e068}\u{fa1c5}\u{e5310}\u{bbbbb}\u{34683}\u{ed1b8}\u{e1e88}\
        \u{469bb}",
        &[
            ('\u{7fdf9}', 9),
            ('\u{a2561}', 9),
            ('\u{3a385}', 8),
            ('\u{42975}', 8),
            ('\u{68770}', 8),
            ('\u{69d5d}', 8),
            ('\u{6fe6b}', 8),
            ('\u{80213}', 8),
            ('\u{a0bcd}', 8),
            ('\u{acca0}', 8),
        ],
        ('\u{9aa2b}', None),
    );
    // p = 50/51
    graphic_weighted_random_chars_helper(
        50,
        51,
        "𖽈𭣅礑깝ꄾ𿆠東쭈𪭅╫𱶭𡡖⢇𲉥𮁢濿𰌎𲁚𾔜՟𤷐𨑢𾧎𥎔𽭆𤂯𠈂𰓵𝪇𰇊厝🔧𩖿⋐𿋲𑆮𐤅⎎𠿝摊𮏉𖦿𮲫虻𓂵𤳧𓤬𤑁𠞈뛆䁼𡛅ờ쀇𜵼𩻆𪈻𓋏𽷝憇쇎𬱡ㆄ쬯𽊗\u{933e9}𢳓𱿛𪐨𐒽ჴ𡀄𬻥𙄦𥦻좾呫𫑃䍟𧇁始𭏵\
        𦆝叟𫛼𐌎䆓𪆃𒍱𲐛𲮌뎃𡆁𘅽𒙰𥀁㨔օ뇜𩓌𦂸𘂄𜵛謵𢝨𗻗𛰘鎀彉𭨬𤳍𠛙𘪂𥝙𱆽𪩰𫨍𑃳蟸𰘁𨎵鐉𘸫ㅄ翷𣧽ꔯ𥀦𥼠歕\u{3ad5d}𠄕퉜𤖜𣱺𿂪竸羱𠢟𗶚𡄘𓮆𡶕𰕄𫤁攸罣𥅬𢁂𡭹𢽺𗈹ᕴ부쁸𓛹𦒬闆贐𧚡𒔦𽛉𬗶𣯙𨕶\
        𢾢𑧑𫞡𭩒\u{dcd88}𢣤掔𝙵⛼㕪굍𖤤𤾙㾇𖦻𱟴褦𓐏𬾒𦈬𐼤튆䯬𘭝𩜠\u{ecf35}𝟻𗙴釲쥁遉헨𦐄𲮚𫁱",
        &[
            ('𱴖', 19),
            ('𓎯', 18),
            ('𥀟', 18),
            ('𪒈', 18),
            ('𫵜', 18),
            ('ﱑ', 17),
            ('𗎡', 17),
            ('𗝇', 17),
            ('𘱏', 17),
            ('𡮿', 17),
        ],
        ('𠥰', None),
    );
}

#[test]
#[should_panic]
fn graphic_weighted_random_chars_fail_1() {
    graphic_weighted_random_chars(EXAMPLE_SEED, 0, 0);
}

#[test]
#[should_panic]
fn graphic_weighted_random_chars_fail_2() {
    graphic_weighted_random_chars(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn graphic_weighted_random_chars_fail_3() {
    graphic_weighted_random_chars(EXAMPLE_SEED, 2, 1);
}
