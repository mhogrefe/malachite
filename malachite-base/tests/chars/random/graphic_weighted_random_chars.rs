// Copyright © 2025 Mikhail Hogrefe
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
        "\u{8eea5}𗇚\u{39459}𮃿\u{d8844}礡\u{e89f1}깶\u{3a332}\u{d4ad7}ꅎ\u{8d776}极쭡𫍬╷𢁽⢓𮢜\u{39d9c}瀏𰭈ՠ\
        \u{733e6}𥗷𨲉𥮻\u{c8a08}𤣖𠨩\u{c6435}\u{54a8d}\u{efe0d}\u{3747d}𰴯\u{f365d}💋\u{192f5}\u{375e4}\
        \u{5be7f}𰨄\u{e12f4}\u{c1e1a}厭𠇓𩷦⋜\u{faf1d}\u{ac2c9}\u{6f4f5}𑊈\u{ebe00}𐦅⎚\u{1aebd}\u{346d3}\
        \u{632e1}\u{7860a}\u{b2c47}\u{f7dcb}\u{765fa}𡠄\u{edb08}\u{f2f2d}摚\u{35fab}\u{d9708}𮰒𖹥\
        \u{a1848}𰑩蚋\u{a95cc}\u{762c7}𓊩\u{fda06}𥔎\u{6f551}\u{4ddc1}\u{69187}\u{c9bfc}𓬠𤱨\u{1bf0e}\
        \u{1027c1}𠾯\u{605be}\u{7d365}\u{54599}뛟䂌𡻬\u{c6e5b}\u{d63ea}ỡ\u{15eef}쀠\u{8b29f}𝙪𪜍𪪂\
        \u{3bc27}𓓭\u{63607}憗쇧𭒝㆕쭈\u{6a1af}𣓺𪱯𐓴\u{58e9b}\u{893cf}ჺ𡠫\u{11dfa}𭜟\u{9faca}𜺝\u{cfc75}𦇢\
        \u{6c195}죗\u{6fbde}\u{c1533}\u{a057d}呻\u{b2d02}\u{43878}\u{d49bd}\u{86bd1}𫱲䍯𧧨姛\u{9e1e8}𭰯\
        \u{fee29}\u{5546d}𦧄\u{bb98d}\u{8e888}𰙓\u{522db}𫼫\u{f27e7}\u{a5b11}𐍊\u{e3033}䆣𪧊\u{f4f07}𒒔\
        \u{b4409}뎜\u{c6602}\u{e7150}𡦨\u{44b40}𘎨𓅺\u{da475}𥠨㨤ֆ뇵𩳳𦣟𘊯\u{cd430}\u{59201}𝙉\u{91c32}\
        \u{c65e7}譅𢾏\u{bac69}\u{84944}𘄂𝑥\u{eda4}\u{b6095}\u{fc44b}\u{12b3c}\u{ad6db}鎐\u{c6529}彙𮉦\
        \u{4145b}\u{9eb07}𥓴\u{cd6f0}𠼀\u{69e1d}𘲵\u{cb25e}\u{489ad}",
        &[
            ('שּׁ', 13), ('𗑤', 13), ('𧶐', 13), ('𪳏', 13), ('ẇ', 12), ('罟', 12), ('뛻', 12), ('𓇸', 12),
            ('𘃠', 12), ('𜳵', 12)
        ],
        ('𰏇', None)
    );
    // p = 1/51
    graphic_weighted_random_chars_helper(
        1,
        51,
        "\u{8eea5}\u{39459}𗇚\u{d8844}\u{e89f1}\u{3a332}\u{d4ad7}\u{8d776}\u{39d9c}\u{733e6}\
        \u{c8a08}\u{c6435}\u{54a8d}\u{efe0d}\u{3747d}\u{f365d}\u{192f5}\u{375e4}\u{5be7f}\u{e12f4}\
        \u{c1e1a}\u{faf1d}\u{ac2c9}\u{6f4f5}\u{ebe00}\u{1aebd}\u{346d3}\u{632e1}\u{7860a}\u{b2c47}\
        \u{f7dcb}\u{765fa}\u{edb08}\u{f2f2d}\u{35fab}\u{d9708}\u{a1848}\u{a95cc}𮃿\u{762c7}\
        \u{fda06}\u{6f551}\u{4ddc1}\u{69187}\u{c9bfc}\u{1bf0e}\u{1027c1}\u{605be}\u{7d365}\
        \u{54599}\u{c6e5b}\u{d63ea}\u{15eef}\u{8b29f}\u{3bc27}\u{63607}\u{6a1af}\u{58e9b}\u{893cf}\
        \u{11dfa}\u{9faca}\u{cfc75}\u{6c195}\u{6fbde}\u{c1533}\u{a057d}\u{b2d02}\u{43878}\u{d49bd}\
        \u{86bd1}\u{9e1e8}\u{fee29}\u{5546d}\u{bb98d}\u{8e888}\u{522db}\u{f27e7}\u{a5b11}\u{e3033}\
        \u{f4f07}\u{b4409}\u{c6602}\u{e7150}\u{44b40}\u{da475}\u{cd430}\u{59201}\u{91c32}\u{c65e7}\
        \u{bac69}\u{84944}\u{eda4}\u{b6095}\u{fc44b}\u{12b3c}\u{ad6db}\u{c6529}\u{4145b}\u{9eb07}\
        \u{cd6f0}\u{69e1d}\u{cb25e}\u{489ad}\u{fdd38}\u{48c3d}\u{76efc}\u{434a3}\u{35453}礡\
        \u{bc6d4}\u{e8fcd}\u{90e90}\u{701d8}\u{3bd87}\u{593a3}\u{54831}\u{f8efd}\u{ba0ec}\u{1007c}\
        \u{9c651}\u{e9eae}\u{f933f}\u{e20ef}\u{f8a34}\u{2f009}\u{8d350}\u{369f2}\u{75f5e}\u{b622f}\
        깶\u{a988e}\u{defaf}\u{10c24c}\u{91b48}\u{f8e93}\u{418a6}\u{1a1a8}\u{aeaf8}\u{81ef3}\
        \u{72d35}\u{4319d}\u{70fdd}\u{102b51}\u{79ab6}\u{10864f}\u{f1f64}\u{95e89}\u{803c3}\
        \u{c141d}\u{ac3c8}\u{af5c5}\u{cc75d}\u{cf3f3}\u{f7ee4}\u{c5787}\u{b9504}\u{88404}\u{a1846}\
        \u{107e68}\u{d7bb1}\u{52919}\u{10ab76}\u{cdf91}\u{d2383}\u{10f29a}\u{9ffd0}\u{c5474}\
        \u{6786d}\u{bc51a}\u{776ae}\u{c2d68}\u{5094e}\u{4f435}\u{b8bb2}\u{a1d70}\u{fac0a}\u{efc9b}\
        \u{5532a}\u{117d2}\u{1c32f}\u{3470f}\u{37c9c}\u{19799}\u{ac494}\u{7ccd3}\u{52e52}\u{14e13}\
        \u{3d2e1}\u{fcc35}\u{44ea3}\u{bc9fc}\u{5553a}\u{7e198}\u{b350e}\u{89b24}\u{f5c81}\u{e0dcc}\
        \u{b7677}\u{32d7f}\u{e8c74}",
        &[
            ('\u{7b8b5}', 9),
            ('\u{9e01d}', 9),
            ('\u{38a81}', 8),
            ('\u{3e431}', 8),
            ('\u{65819}', 8),
            ('\u{6b927}', 8),
            ('\u{7bccf}', 8),
            ('\u{9c689}', 8),
            ('\u{a875c}', 8),
            ('\u{c7470}', 8),
        ],
        ('\u{986ca}', None),
    );
    // p = 50/51
    graphic_weighted_random_chars_helper(
        50,
        51,
        "𗇚𮃿礡깶ꅎ极쭡𫍬╷𢁽⢓𮢜瀏𰭈ՠ𥗷𨲉𥮻𤣖𠨩𰴯💋𰨄厭𠇓𩷦⋜𑊈𐦅⎚𡠄摚𮰒𖹥𰑩蚋𓊩𥔎𓬠𤱨𠾯뛟䂌𡻬ỡ쀠𝙪𪜍𪪂𓓭憗쇧𭒝㆕쭈𣓺𪱯𐓴ჺ𡠫𭜟𜺝𦇢죗呻\u{8eea5}𫱲䍯𧧨姛𭰯𦧄𰙓𫼫𐍊䆣𪧊𒒔뎜𡦨𘎨\
        𓅺𥠨㨤ֆ뇵𩳳𦣟𘊯𝙉譅𢾏𘄂𝑥鎐彙𮉦𥓴𠼀𘲵𥾀𱧼𫊗𬈻𑆔蠈𰸻𨯜鐙𛰩ㅕ耇𤈤ꔿ𥡍𦝇步𠤼퉵𤷃𤒡笈翁𡃆𗿅𡤿𓵺𢖼𰵾𬄯\u{39459}效罳𥦓𢡩𢎠𣞡𗑤ᕸ붙삑𓣭𦳓闖贠𧻈𓀼𬸤𤐀𨶝𣟉𑪠𫿐𮊌𣄋掤𞺇✈㕺굦𖭃𥟀㾗\
        𖹡𲀮褶𓘭𭟌𦩓\u{d8844}𑀋튟䯼𛂣𩽇🅲𗢟鈂쥚遙혁𦰫𫢠𗱯𠊅𛀰裸𦊿Ǐ𪬈\u{e89f1}䖌೭꺏𰭟🧩䔬팚𐦠羯",
        &[
            ('𗗌', 20), ('𡔕', 20), ('ﱺ', 19), ('𓗍', 19), ('𡃉', 19), ('𥡆', 19), ('𬖊', 19), ('䞹', 18),
            ('䦴', 18), ('幐', 18)
        ],
        ('𝙤', None)
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
