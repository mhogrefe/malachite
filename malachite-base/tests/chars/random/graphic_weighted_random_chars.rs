// Copyright © 2024 Mikhail Hogrefe
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
        "\u{8daa8}𘋈\u{3805c}𰒏\u{d7447}礝\u{e75f4}깶\u{38f35}\u{d36da}ꅊ\u{8c379}杽쭡𬝱╱𣑺⢍𰰬\u{3899f}瀋𱽊ՠ\
        \u{71fe9}𦧴𪂆𦾸\u{c760b}𥳓𡸦\u{c5038}\u{53690}\u{eea10}\u{36080}𲄱\u{f2260}𠵀\u{160ea}\u{361e7}\
        \u{5aa82}𱸆\u{dfef7}\u{c0a1d}厩𡗐𫈃⋓\u{f9b20}\u{aaecc}\u{6e0f8}𑌡\u{eaa03}𐦽⎑\u{19dce}\u{332d6}\
        \u{61ee4}\u{7720d}\u{b184a}\u{f69ce}\u{751fd}𢰁\u{ec70b}\u{f1b30}摖\u{34bae}\u{d830b}𰾓𘀤\
        \u{a044b}𱡫蚇\u{a81cf}\u{74eca}𓎙\u{fc609}𦤋\u{6e154}\u{4c9c4}\u{67d8a}\u{c87ff}𗄤𦁥\u{1aacd}\
        \u{1013c4}𢎬\u{5f1c1}\u{7bf68}\u{5319c}뛟䂈𣋩\u{c5a5e}\u{d4fed}Ộ\u{14e64}쀠\u{89ea2}𠊅𫬒𫺇\
        \u{3a82a}𔕽\u{6220a}憓쇧𮢚ㆎ쭈\u{68db2}𤣷𬁴𐓴\u{57a9e}\u{87fd2}ჵ𢰨\u{11c91}𮬜\u{9e6cd}🡗\u{ce878}𧗟\
        \u{6ad98}죗\u{6e7e1}\u{c0136}\u{9f180}呷\u{b1905}\u{4247b}\u{d35c0}\u{857d4}𭁽䍫𨷥姗\u{9cdeb}𠣞\
        \u{fda2c}\u{54070}𧷁\u{ba590}\u{8d48b}𱩕\u{50ede}𭌶\u{f13ea}\u{a4714}𐍊\u{e1c36}䆟𫷏\u{f3b0a}𒿐\
        \u{b300c}뎜\u{c5205}\u{e5d53}𢶥\u{43743}𝕻𓉪\u{d9078}𦰥㨠ֆ뇵𫄐𧳜𝑫\u{cc033}\u{57e04}𠉤\u{90835}\
        \u{c51ea}譁𤎌\u{b986c}\u{83547}𝅘𝅥𠂗\u{eda4}\u{b4c98}\u{fb04e}\u{12a4c}\u{ac2de}鎌\u{c512c}录𰗶\
        \u{4005e}\u{9d70a}𦣱\u{cc2f3}𢋽\u{68a20}🄘\u{c9e61}\u{475b0}",
        &[
            ('שּׁ', 13), ('𘕒', 13), ('𣱵', 13), ('𧀂', 13), ('𩆍', 13), ('𪾣', 13), ('𬃔', 13),
            ('༈', 12), ('Ṿ', 12), ('㏠', 12)
        ],
        ('𰆁', None)
    );
    // p = 1/51
    graphic_weighted_random_chars_helper(
        1,
        51,
        "\u{8daa8}\u{3805c}𘋈\u{d7447}\u{e75f4}\u{38f35}\u{d36da}\u{8c379}\u{3899f}\u{71fe9}\
        \u{c760b}\u{c5038}\u{53690}\u{eea10}\u{36080}\u{f2260}\u{160ea}\u{361e7}\u{5aa82}\u{dfef7}\
        \u{c0a1d}\u{f9b20}\u{aaecc}\u{6e0f8}\u{eaa03}\u{19dce}\u{332d6}\u{61ee4}\u{7720d}\u{b184a}\
        \u{f69ce}\u{751fd}\u{ec70b}\u{f1b30}\u{34bae}\u{d830b}\u{a044b}\u{a81cf}𰒏\u{74eca}\
        \u{fc609}\u{6e154}\u{4c9c4}\u{67d8a}\u{c87ff}\u{1aacd}\u{1013c4}\u{5f1c1}\u{7bf68}\
        \u{5319c}\u{c5a5e}\u{d4fed}\u{14e64}\u{89ea2}\u{3a82a}\u{6220a}\u{68db2}\u{57a9e}\u{87fd2}\
        \u{11c91}\u{9e6cd}\u{ce878}\u{6ad98}\u{6e7e1}\u{c0136}\u{9f180}\u{b1905}\u{4247b}\u{d35c0}\
        \u{857d4}\u{9cdeb}\u{fda2c}\u{54070}\u{ba590}\u{8d48b}\u{50ede}\u{f13ea}\u{a4714}\u{e1c36}\
        \u{f3b0a}\u{b300c}\u{c5205}\u{e5d53}\u{43743}\u{d9078}\u{cc033}\u{57e04}\u{90835}\u{c51ea}\
        \u{b986c}\u{83547}\u{eda4}\u{b4c98}\u{fb04e}\u{12a4c}\u{ac2de}\u{c512c}\u{4005e}\u{9d70a}\
        \u{cc2f3}\u{68a20}\u{c9e61}\u{475b0}\u{fc93b}\u{47840}\u{75aff}\u{420a6}\u{34056}礝\
        \u{bb2d7}\u{e7bd0}\u{8fa93}\u{6eddb}\u{3a98a}\u{57fa6}\u{53434}\u{f7b00}\u{b8cef}\u{1007c}\
        \u{9b254}\u{e8ab1}\u{f7f42}\u{e0cf2}\u{f7637}\u{1e0f3}\u{8bf53}\u{355f5}\u{74b61}\u{b4e32}\
        깶\u{a8491}\u{ddbb2}\u{10ae4f}\u{9074b}\u{f7a96}\u{404a9}\u{190b9}\u{ad6fb}\u{80af6}\
        \u{71938}\u{41da0}\u{6fbe0}\u{101754}\u{786b9}\u{107252}\u{f0b67}\u{94a8c}\u{7efc6}\
        \u{c0020}\u{aafcb}\u{ae1c8}\u{cb360}\u{cdff6}\u{f6ae7}\u{c438a}\u{b8107}\u{87007}\u{a0449}\
        \u{106a6b}\u{d67b4}\u{5151c}\u{109779}\u{ccb94}\u{d0f86}\u{10de9d}\u{9ebd3}\u{c4077}\
        \u{66470}\u{bb11d}\u{762b1}\u{c196b}\u{4f551}\u{4e038}\u{b77b5}\u{a0973}\u{f980d}\u{ee89e}\
        \u{53f2d}\u{116d7}\u{1aeee}\u{33312}\u{3689f}\u{1658e}\u{ab097}\u{7b8d6}\u{51a55}\u{13b41}\
        \u{3bee4}\u{fb838}\u{43aa6}\u{bb5ff}\u{5413d}\u{7cd9b}\u{b2111}\u{88727}\u{f4884}\u{df9cf}\
        \u{b627a}\u{2f3b9}\u{e7877}",
        &[
            ('\u{7a4b8}', 9),
            ('\u{9cc20}', 9),
            ('\u{37684}', 8),
            ('\u{3d034}', 8),
            ('\u{6441c}', 8),
            ('\u{6a52a}', 8),
            ('\u{7a8d2}', 8),
            ('\u{9b28c}', 8),
            ('\u{a735f}', 8),
            ('\u{c6073}', 8),
        ],
        ('\u{97cbe}', Some('\u{97cbf}')),
    );
    // p = 50/51
    graphic_weighted_random_chars_helper(
        50,
        51,
        "𘋈𰒏礝깶ꅊ杽쭡𬝱╱𣑺⢍𰰬瀋𱽊ՠ𦧴𪂆𦾸𥳓𡸦𲄱𠵀𱸆厩𡗐𫈃⋓𑌡𐦽⎑𢰁摖𰾓𘀤𱡫蚇𓎙𦤋𗄤𦁥𢎬뛟䂈𣋩Ộ쀠𠊅𫬒𫺇𔕽憓쇧𮢚ㆎ쭈𤣷𬁴𐓴ჵ𢰨𮬜🡗𧗟죗呷\u{8daa8}𭁽䍫𨷥姗𠣞𧷁𱩕𭌶𐍊䆟𫷏𒿐뎜𢶥𝕻\
        𓉪𦰥㨠ֆ뇵𫄐𧳜𝑫𠉤譁𤎌𝅘𝅥𠂗鎌录𰗶𦣱𢋽🄘𧍽𬚜𭙆𑈦蠄𲈽𩿙鐕🔩ㅏ考𥘡ꔻ𦱊𧭄歡𡴹퉵𦇀𥢞笄羽𢓃𝀖𢴼𗍾𣦹𲆀𭔺敄\u{3805c}罯𦶐𣱦𣞝𤮞𘕒ᕳ붙삑𖭭𨃐闒贜𩋅𓄬𮈯𥟽𪆚𤯆𑲌𭏛𰘜𤔈掠𠦜✂㕶굦𗾉𦮽㾓𘀠\
        褲𖡶𮯉𧹐𑂤튟\u{d7447}䯸🈸𫍤𠬟𘦕釾쥚違혁𨀨𬲝𛁹𡚂🆀裴𧚼Ǐ𫼍䖈೨\u{e75f4}꺏𱽡𡈙䔨팚𐧚羫𡇼𭀯",
        &[
            ('𘚺', 23), ('𢤒', 20), ('𤉺', 20), ('ﱺ', 19), ('𖠖', 19), ('𢓆', 19), ('𤌅', 19), ('𤦵', 19),
            ('𦱃', 19), ('𭦕', 19)
        ],
        ('🍄', None)
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
