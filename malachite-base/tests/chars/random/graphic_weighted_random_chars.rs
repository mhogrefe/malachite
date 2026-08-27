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
        "\u{9013d}𗄥\u{3a6f1}𭼱\u{d9adc}礟\u{e9c89}깯\u{3b5ca}\u{d5d6f}ꅌ\u{8ea0e}板쭚𫆰╵𲐙𡻁⢑\u{3b034}𲣑𮛎瀍\
        \u{7467e}𰥺𲛆ՠ\u{c9ca0}𥐻𨫍\u{c76cd}\u{55d25}\u{f10a5}\u{38715}𥧿\u{f48f5}𤜚\u{1943b}\u{3887c}\
        \u{5d117}𠡭\u{e258c}\u{c30b2}𰭡🈵𰠶厫\u{fc1b5}\u{ad561}\u{7078d}𠀗\u{ed098}𩰪⋚\u{1b134}\u{3596b}\
        \u{64579}\u{798a2}\u{b3edf}\u{f9063}\u{77892}𑇯\u{eeda0}\u{f41c5}𐤡\u{37243}\u{da9a0}⎘𡙈\
        \u{a2ae0}摘𮨵\u{aa864}\u{7755f}𖭬\u{fec9e}𰊛\u{707e9}\u{4f059}\u{6a41f}\u{cae94}蚉𓈫\u{1c054}\
        \u{103a59}𥍒\u{61856}\u{7e5fd}\u{55831}𓪢𤪬𠷳\u{c80f3}\u{d7682}뛘\u{15f6d}䂊\u{8c537}𡴰Ỡ쀙\
        \u{3cebf}𝓭\u{6489f}𪔱𪣆𓑯憕쇠\u{6b447}𭋏㆓쭁\u{5a133}\u{8a667}𣌾𲙇\u{11e78}𪪳\u{a0d62}𐓑\u{d0f0d}ჷ\
        \u{6d42d}𡙯\u{70e76}\u{c27cb}\u{a1815}𭕑\u{b3f9a}\u{44b10}\u{d5c55}\u{87e69}𜵔𦀦죐呹\u{9f480}𫪰\
        \u{1000c1}\u{56705}䍭\u{bcc25}\u{8fb20}𧠬\u{53573}姙\u{f3a7f}\u{a6da9}𭩡\u{e42cb}𦠈𰒅\u{f619f}𫵩\
        \u{b56a1}𐌢\u{c789a}\u{e83e8}䆡\u{45dd8}𪠎𒐊\u{db70d}𲪇𳇸뎕𡟬𘋳𓃼𥙬\u{ce6c8}\u{5a499}㨢\u{92eca}\
        \u{c787f}ֆ뇮\u{bbf01}\u{85bdc}𩬷𦜣\u{edab}\u{b732d}\u{fd6e3}\u{12bba}\u{ae973}𘇺\u{c77c1}𝓌譃\
        \u{426f3}\u{9fd9f}𢷓\u{ce988}𘁍\u{6b0b5}𝆹\u{cc4f6}\u{49c45}",
        &[
            ('𗎯', 13),
            ('𧯔', 13),
            ('𪬓', 13),
            ('𲎂', 13),
            ('Ẇ', 12),
            ('罝', 12),
            ('뛴', 12),
            ('ﬥ', 12),
            ('𘀫', 12),
            ('𛰼', 12),
        ],
        ('𱎤', None),
    );
    // p = 1/51
    graphic_weighted_random_chars_helper(
        1,
        51,
        "\u{9013d}\u{3a6f1}𗄥\u{d9adc}\u{e9c89}\u{3b5ca}\u{d5d6f}\u{8ea0e}\u{3b034}\u{7467e}\
        \u{c9ca0}\u{c76cd}\u{55d25}\u{f10a5}\u{38715}\u{f48f5}\u{1943b}\u{3887c}\u{5d117}\u{e258c}\
        \u{c30b2}\u{fc1b5}\u{ad561}\u{7078d}\u{ed098}\u{1b134}\u{3596b}\u{64579}\u{798a2}\u{b3edf}\
        \u{f9063}\u{77892}\u{eeda0}\u{f41c5}\u{37243}\u{da9a0}\u{a2ae0}\u{aa864}𭼱\u{7755f}\
        \u{fec9e}\u{707e9}\u{4f059}\u{6a41f}\u{cae94}\u{1c054}\u{103a59}\u{61856}\u{7e5fd}\
        \u{55831}\u{c80f3}\u{d7682}\u{15f6d}\u{8c537}\u{3cebf}\u{6489f}\u{6b447}\u{5a133}\u{8a667}\
        \u{11e78}\u{a0d62}\u{d0f0d}\u{6d42d}\u{70e76}\u{c27cb}\u{a1815}\u{b3f9a}\u{44b10}\u{d5c55}\
        \u{87e69}\u{9f480}\u{1000c1}\u{56705}\u{bcc25}\u{8fb20}\u{53573}\u{f3a7f}\u{a6da9}\
        \u{e42cb}\u{f619f}\u{b56a1}\u{c789a}\u{e83e8}\u{45dd8}\u{db70d}\u{ce6c8}\u{5a499}\u{92eca}\
        \u{c787f}\u{bbf01}\u{85bdc}\u{edab}\u{b732d}\u{fd6e3}\u{12bba}\u{ae973}\u{c77c1}\u{426f3}\
        \u{9fd9f}\u{ce988}\u{6b0b5}\u{cc4f6}\u{49c45}\u{fefd0}\u{49ed5}\u{78194}\u{4473b}\u{366eb}\
        礟\u{bd96c}\u{ea265}\u{92128}\u{71470}\u{3d01f}\u{5a63b}\u{55ac9}\u{fa195}\u{bb384}\
        \u{101ac}\u{9d8e9}\u{eb146}\u{fa5d7}\u{e3387}\u{f9ccc}\u{2f1d7}\u{8e5e8}\u{37c8a}\u{771f6}\
        \u{b74c7}깯\u{aab26}\u{e0247}\u{10d4e4}\u{92de0}\u{fa12b}\u{42b3e}\u{1a2ee}\u{afd90}\
        \u{8318b}\u{73fcd}\u{44435}\u{72275}\u{103de9}\u{7ad4e}\u{1098e7}\u{f31fc}\u{97121}\
        \u{8165b}\u{c26b5}\u{ad660}\u{b085d}\u{cd9f5}\u{d068b}\u{f917c}\u{c6a1f}\u{ba79c}\u{8969c}\
        \u{a2ade}\u{109100}\u{d8e49}\u{53bb1}\u{10be0e}\u{cf229}\u{d361b}\u{a1268}\u{c670c}\
        \u{68b05}\u{bd7b2}\u{78946}\u{c4000}\u{51be6}\u{506cd}\u{b9e4a}\u{a3008}\u{fbea2}\u{f0f33}\
        \u{565c2}\u{11848}\u{1c475}\u{359a7}\u{38f34}\u{198df}\u{ad72c}\u{7df6b}\u{540ea}\u{14e91}\
        \u{3e579}\u{fdecd}\u{4613b}\u{bdc94}\u{567d2}\u{7f430}\u{b47a6}\u{8adbc}\u{f6f19}\u{e2064}\
        \u{b890f}\u{34017}\u{e9f0c}\u{debdc}",
        &[
            ('\u{7cb4d}', 9),
            ('\u{9f2b5}', 9),
            ('\u{39d19}', 8),
            ('\u{3f6c9}', 8),
            ('\u{654c4}', 8),
            ('\u{66ab1}', 8),
            ('\u{6cbbf}', 8),
            ('\u{7cf67}', 8),
            ('\u{9d921}', 8),
            ('\u{a99f4}', 8),
        ],
        ('\u{9900e}', None),
    );
    // p = 50/51
    graphic_weighted_random_chars_helper(
        50,
        51,
        "𗄥𭼱礟깯ꅌ板쭚𫆰╵𲐙𡻁⢑𲣑𮛎瀍𰥺𲛆ՠ𥐻𨫍𥧿𤜚𠡭𰭡🈵𰠶厫𠀗𩰪⋚𑇯𐤡⎘𡙈摘𮨵𖭬𰊛蚉𓈫𥍒𓪢𤪬𠷳뛘䂊𡴰Ỡ쀙𝓭𪔱𪣆𓑯憕쇠𭋏㆓쭁𣌾𲙇𪪳𐓑ჷ𡙯𭕑\u{9013d}𜵔𦀦죐呹𫪰䍭𧠬姙𭩡𦠈𰒅𫵩𐌢䆡𪠎\
        𒐊𲪇𳇸뎕𡟬𘋳𓃼𥙬㨢ֆ뇮𩬷𦜣𘇺𝓌譃𢷓𘁍𝆹鎎彗𮂘𥌸𠵄𘯸𥷄𱠮𫃛𬁹𑄻蠆𰱭𨨠鐗𛇟ㅒ者𤁨ꔽ𥚑𦖋正𠞀퉮𤰇𤋥笆羿𠼊\u{3a6f1}𗼐𡞃𓳼𢐀𰮰𫽭敆罱𥟗𢚭𢇤𣗥𗎯ᕷ붒삊𓡯𦬗闔贞𧴌𒾱𬱢𤉄𨯡𣘍𑨰𫸎𮂾𢽏探𞡝\
        ✆㕸굟𖪦𥘄㾕\u{d9adc}𖭨𱹠褴𓖯𭗾𦢗𐽾튘䯺𘳓𩶋𞹝𗟪鈀쥓遗헺𦩯𳈆𫛜\u{e9c89}𗮺𠃉𘱠裶𦄃Ǐ𪥌𲜷䖊",
        &[
            ('𗔗', 20),
            ('𲎂', 20),
            ('ﱣ', 19),
            ('𓕏', 19),
            ('𬏈', 19),
            ('𳅼', 19),
            ('䦲', 18),
            ('幎', 18),
            ('跣', 18),
            ('𗢽', 18),
        ],
        ('🂆', Some('🂇')),
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
