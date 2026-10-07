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
        "\u{9013f}𗄣\u{3a6f3}𭼯\u{d9ade}礟\u{e9c8b}깯\u{3b5cc}\u{d5d71}ꅌ\u{8ea10}板쭚𫆮╵𲐗𡺿⢑\u{3b036}𲣏𮛌瀍\
        \u{74680}𰥸𲛄ՠ\u{c9ca2}𥐹𨫋\u{c76cf}\u{55d27}\u{f10a7}\u{38717}𥧽\u{f48f7}𤜘\u{1943d}\u{3887e}\
        \u{5d119}𠡫\u{e258e}\u{c30b4}𰭟🈳𰠴厫\u{fc1b7}\u{ad563}\u{7078f}𠀕\u{ed09a}𩰨⋚\u{1b136}\u{3596d}\
        \u{6457b}\u{798a4}\u{b3ee1}\u{f9065}\u{77894}𑇭\u{eeda2}\u{f41c7}𐤟\u{37245}\u{da9a2}⎘𡙆\
        \u{a2ae2}摘𮨳\u{aa866}\u{77561}𖭪\u{feca0}𰊙\u{707eb}\u{4f05b}\u{6a421}\u{cae96}蚉𓈩\u{1c056}\
        \u{103a5b}𥍐\u{61858}\u{7e5ff}\u{55833}𓪠𤪪𠷱\u{c80f5}\u{d7684}뛘\u{15f6f}䂊\u{8c539}𡴮Ỡ쀙\
        \u{3cec1}𝓫\u{648a1}𪔯𪣄𓑭憕쇠\u{6b449}𭋍㆓쭁\u{5a135}\u{8a669}𣌼𲙅\u{11e7a}𪪱\u{a0d64}𐓏\u{d0f0f}ჷ\
        \u{6d42f}𡙭\u{70e78}\u{c27cd}\u{a1817}𭕏\u{b3f9c}\u{44b12}\u{d5c57}\u{87e6b}𜵒𦀤죐呹\u{9f482}𫪮\
        \u{1000c3}\u{56707}䍭\u{bcc27}\u{8fb22}𧠪\u{53575}姙\u{f3a81}\u{a6dab}𭩟\u{e42cd}𦠆𰒃\u{f61a1}𫵧\
        \u{b56a3}𐌠\u{c789c}\u{e83ea}䆡\u{45dda}𪠌𒐈\u{db70f}𲪅𳇶뎕𡟪𘋱𓃺𥙪\u{ce6ca}\u{5a49b}㨢\u{92ecc}\
        \u{c7881}ֆ뇮\u{bbf03}\u{85bde}𩬵𦜡\u{edab}\u{b732f}\u{fd6e5}\u{12bbc}\u{ae975}𘇸\u{c77c3}𝓊譃\
        \u{426f5}\u{9fda1}𢷑\u{ce98a}𘁋\u{6b0b7}𝆷\u{cc4f8}\u{49c47}",
        &[
            ('𗎭', 13),
            ('𧯒', 13),
            ('𪬑', 13),
            ('𲎀', 13),
            ('Ẇ', 12),
            ('罝', 12),
            ('뛴', 12),
            ('ﬥ', 12),
            ('𘀩', 12),
            ('𛰺', 12),
        ],
        ('𱎣', None),
    );
    // p = 1/51
    graphic_weighted_random_chars_helper(
        1,
        51,
        "\u{9013f}\u{3a6f3}𗄣\u{d9ade}\u{e9c8b}\u{3b5cc}\u{d5d71}\u{8ea10}\u{3b036}\u{74680}\
        \u{c9ca2}\u{c76cf}\u{55d27}\u{f10a7}\u{38717}\u{f48f7}\u{1943d}\u{3887e}\u{5d119}\u{e258e}\
        \u{c30b4}\u{fc1b7}\u{ad563}\u{7078f}\u{ed09a}\u{1b136}\u{3596d}\u{6457b}\u{798a4}\u{b3ee1}\
        \u{f9065}\u{77894}\u{eeda2}\u{f41c7}\u{37245}\u{da9a2}\u{a2ae2}\u{aa866}𭼯\u{77561}\
        \u{feca0}\u{707eb}\u{4f05b}\u{6a421}\u{cae96}\u{1c056}\u{103a5b}\u{61858}\u{7e5ff}\
        \u{55833}\u{c80f5}\u{d7684}\u{15f6f}\u{8c539}\u{3cec1}\u{648a1}\u{6b449}\u{5a135}\u{8a669}\
        \u{11e7a}\u{a0d64}\u{d0f0f}\u{6d42f}\u{70e78}\u{c27cd}\u{a1817}\u{b3f9c}\u{44b12}\u{d5c57}\
        \u{87e6b}\u{9f482}\u{1000c3}\u{56707}\u{bcc27}\u{8fb22}\u{53575}\u{f3a81}\u{a6dab}\
        \u{e42cd}\u{f61a1}\u{b56a3}\u{c789c}\u{e83ea}\u{45dda}\u{db70f}\u{ce6ca}\u{5a49b}\u{92ecc}\
        \u{c7881}\u{bbf03}\u{85bde}\u{edab}\u{b732f}\u{fd6e5}\u{12bbc}\u{ae975}\u{c77c3}\u{426f5}\
        \u{9fda1}\u{ce98a}\u{6b0b7}\u{cc4f8}\u{49c47}\u{fefd2}\u{49ed7}\u{78196}\u{4473d}\u{366ed}\
        礟\u{bd96e}\u{ea267}\u{9212a}\u{71472}\u{3d021}\u{5a63d}\u{55acb}\u{fa197}\u{bb386}\
        \u{101ae}\u{9d8eb}\u{eb148}\u{fa5d9}\u{e3389}\u{f9cce}\u{2f1d9}\u{8e5ea}\u{37c8c}\u{771f8}\
        \u{b74c9}깯\u{aab28}\u{e0249}\u{10d4e6}\u{92de2}\u{fa12d}\u{42b40}\u{1a2f0}\u{afd92}\
        \u{8318d}\u{73fcf}\u{44437}\u{72277}\u{103deb}\u{7ad50}\u{1098e9}\u{f31fe}\u{97123}\
        \u{8165d}\u{c26b7}\u{ad662}\u{b085f}\u{cd9f7}\u{d068d}\u{f917e}\u{c6a21}\u{ba79e}\u{8969e}\
        \u{a2ae0}\u{109102}\u{d8e4b}\u{53bb3}\u{10be10}\u{cf22b}\u{d361d}\u{a126a}\u{c670e}\
        \u{68b07}\u{bd7b4}\u{78948}\u{c4002}\u{51be8}\u{506cf}\u{b9e4c}\u{a300a}\u{fbea4}\u{f0f35}\
        \u{565c4}\u{1184a}\u{1c477}\u{359a9}\u{38f36}\u{198e1}\u{ad72e}\u{7df6d}\u{540ec}\u{14e93}\
        \u{3e57b}\u{fdecf}\u{4613d}\u{bdc96}\u{567d4}\u{7f432}\u{b47a8}\u{8adbe}\u{f6f1b}\u{e2066}\
        \u{b8911}\u{34019}\u{e9f0e}\u{debde}",
        &[
            ('\u{7cb4f}', 9),
            ('\u{9f2b7}', 9),
            ('\u{39d1b}', 8),
            ('\u{3f6cb}', 8),
            ('\u{654c6}', 8),
            ('\u{66ab3}', 8),
            ('\u{6cbc1}', 8),
            ('\u{7cf69}', 8),
            ('\u{9d923}', 8),
            ('\u{a99f6}', 8),
        ],
        ('\u{99010}', None),
    );
    // p = 50/51
    graphic_weighted_random_chars_helper(
        50,
        51,
        "𗄣𭼯礟깯ꅌ板쭚𫆮╵𲐗𡺿⢑𲣏𮛌瀍𰥸𲛄ՠ𥐹𨫋𥧽𤜘𠡫𰭟🈳𰠴厫𠀕𩰨⋚𑇭𐤟⎘𡙆摘𮨳𖭪𰊙蚉𓈩𥍐𓪠𤪪𠷱뛘䂊𡴮Ỡ쀙𝓫𪔯𪣄𓑭憕쇠𭋍㆓쭁𣌼𲙅𪪱𐓏ჷ𡙭𭕏\u{9013f}𜵒𦀤죐呹𫪮䍭𧠪姙𭩟𦠆𰒃𫵧𐌠䆡𪠌\
        𒐈𲪅𳇶뎕𡟪𘋱𓃺𥙪㨢ֆ뇮𩬵𦜡𘇸𝓊譃𢷑𘁋𝆷鎎彗𮂖𥌶𠵂𘯶𥷂𱠬𫃙𬁷𑄹蠆𰱫𨨞鐗𛇝ㅒ者𤁦ꔽ𥚏𦖉正𠝾퉮𤰅𤋣笆羿𠼈\u{3a6f3}𗼎𡞁𓳺𢏾𰮮𫽫敆罱𥟕𢚫𢇢𣗣𗎭ᕷ붒삊𓡭𦬕闔贞𧴊𒾯𬱠𤉂𨯟𣘋𑨮𫸌𮂼𢽍探𞡛\
        ✆㕸굟𖪤𥘂㾕\u{d9ade}𖭦𱹞褴𓖭𭗼𦢕𐽼튘䯺𘳑𩶉𞹙𗟨鈀쥓遗헺𦩭𳈄𫛚\u{e9c8b}𗮸𠃇𘱞裶𦄁Ǐ𪥊𲜵䖊",
        &[
            ('𗔕', 20),
            ('𲎀', 20),
            ('ﱣ', 19),
            ('𓕍', 19),
            ('𬏆', 19),
            ('𳅺', 19),
            ('䦲', 18),
            ('幎', 18),
            ('跣', 18),
            ('𗢻', 18),
        ],
        ('🂅', None),
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
