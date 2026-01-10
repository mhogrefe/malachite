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
        "\u{90141}𗄡\u{3a6f5}𭼭\u{d9ae0}礜\u{e9c8d}깬\u{3b5ce}\u{d5d73}ꅉ\u{8ea12}杼쭗𫆬╳𲐕𡺽⢏\u{3b038}𲣍𮛊瀊\
        \u{74682}𰥶𲛂ՠ\u{c9ca4}𥐷𨫉\u{c76d1}\u{55d29}\u{f10a9}\u{38719}𥧻\u{f48f9}𤜖\u{1943f}\u{38880}\
        \u{5d11b}𠡩\u{e2590}\u{c30b6}𰭝🈱𰠲厨\u{fc1b9}\u{ad565}\u{70791}𠀓\u{ed09c}𩰦⋘\u{1b138}\u{3596f}\
        \u{6457d}\u{798a6}\u{b3ee3}\u{f9067}\u{77896}𑇫\u{eeda4}\u{f41c9}𐤚\u{37247}\u{da9a4}⎖𡙄\
        \u{a2ae4}摕𮨱\u{aa868}\u{77563}𖭨\u{feca2}𰊗\u{707ed}\u{4f05d}\u{6a423}\u{cae98}蚆𓈧\u{1c058}\
        \u{103a5d}𥍎\u{6185a}\u{7e601}\u{55835}𓪞𤪨𠷯\u{c80f7}\u{d7686}뛕\u{15f71}䂇\u{8c53b}𡴬Ở쀖\
        \u{3cec3}𝓩\u{648a3}𪔭𪣂𓑫憒쇝\u{6b44b}𭋋㆐쬾\u{5a137}\u{8a66b}𣌺𲙃\u{11e7c}𪪯\u{a0d66}𐓍\u{d0f11}ჷ\
        \u{6d431}𡙫\u{70e7a}\u{c27cf}\u{a1819}𭕍\u{b3f9e}\u{44b14}\u{d5c59}\u{87e6d}𜵐𦀢죍呶\u{9f484}𫪬\
        \u{1000c5}\u{56709}䍪\u{bcc29}\u{8fb24}𧠨\u{53577}姖\u{f3a83}\u{a6dad}𭩝\u{e42cf}𦠄𰒁\u{f61a3}𫵥\
        \u{b56a5}𐌞\u{c789e}\u{e83ec}䆞\u{45ddc}𪠊𒐆\u{db711}𲪃𳇴뎒𡟨𘋯𓃸𥙨\u{ce6cc}\u{5a49d}㨟\u{92ece}\
        \u{c7883}ֆ뇫\u{bbf05}\u{85be0}𩬳𦜟\u{edae}\u{b7331}\u{fd6e7}\u{12bbe}\u{ae977}𘇶\u{c77c5}𝓈譀\
        \u{426f7}\u{9fda3}𢷏\u{ce98c}𘁉\u{6b0b9}𝆵\u{cc4fa}\u{49c49}",
        &[
            ('𗎫', 13),
            ('𧯐', 13),
            ('𪬏', 13),
            ('𲍾', 13),
            ('Ẅ', 12),
            ('罚', 12),
            ('뛱', 12),
            ('ﬢ', 12),
            ('𘀧', 12),
            ('𛰸', 12),
        ],
        ('𱎧', None),
    );
    // p = 1/51
    graphic_weighted_random_chars_helper(
        1,
        51,
        "\u{90141}\u{3a6f5}𗄡\u{d9ae0}\u{e9c8d}\u{3b5ce}\u{d5d73}\u{8ea12}\u{3b038}\u{74682}\
        \u{c9ca4}\u{c76d1}\u{55d29}\u{f10a9}\u{38719}\u{f48f9}\u{1943f}\u{38880}\u{5d11b}\u{e2590}\
        \u{c30b6}\u{fc1b9}\u{ad565}\u{70791}\u{ed09c}\u{1b138}\u{3596f}\u{6457d}\u{798a6}\u{b3ee3}\
        \u{f9067}\u{77896}\u{eeda4}\u{f41c9}\u{37247}\u{da9a4}\u{a2ae4}\u{aa868}𭼭\u{77563}\
        \u{feca2}\u{707ed}\u{4f05d}\u{6a423}\u{cae98}\u{1c058}\u{103a5d}\u{6185a}\u{7e601}\
        \u{55835}\u{c80f7}\u{d7686}\u{15f71}\u{8c53b}\u{3cec3}\u{648a3}\u{6b44b}\u{5a137}\u{8a66b}\
        \u{11e7c}\u{a0d66}\u{d0f11}\u{6d431}\u{70e7a}\u{c27cf}\u{a1819}\u{b3f9e}\u{44b14}\u{d5c59}\
        \u{87e6d}\u{9f484}\u{1000c5}\u{56709}\u{bcc29}\u{8fb24}\u{53577}\u{f3a83}\u{a6dad}\
        \u{e42cf}\u{f61a3}\u{b56a5}\u{c789e}\u{e83ec}\u{45ddc}\u{db711}\u{ce6cc}\u{5a49d}\u{92ece}\
        \u{c7883}\u{bbf05}\u{85be0}\u{edae}\u{b7331}\u{fd6e7}\u{12bbe}\u{ae977}\u{c77c5}\u{426f7}\
        \u{9fda3}\u{ce98c}\u{6b0b9}\u{cc4fa}\u{49c49}\u{fefd4}\u{49ed9}\u{78198}\u{4473f}\u{366ef}\
        礜\u{bd970}\u{ea269}\u{9212c}\u{71474}\u{3d023}\u{5a63f}\u{55acd}\u{fa199}\u{bb388}\
        \u{101b0}\u{9d8ed}\u{eb14a}\u{fa5db}\u{e338b}\u{f9cd0}\u{2f1db}\u{8e5ec}\u{37c8e}\u{771fa}\
        \u{b74cb}깬\u{aab2a}\u{e024b}\u{10d4e8}\u{92de4}\u{fa12f}\u{42b42}\u{1a2f2}\u{afd94}\
        \u{8318f}\u{73fd1}\u{44439}\u{72279}\u{103ded}\u{7ad52}\u{1098eb}\u{f3200}\u{97125}\
        \u{8165f}\u{c26b9}\u{ad664}\u{b0861}\u{cd9f9}\u{d068f}\u{f9180}\u{c6a23}\u{ba7a0}\u{896a0}\
        \u{a2ae2}\u{109104}\u{d8e4d}\u{53bb5}\u{10be12}\u{cf22d}\u{d361f}\u{a126c}\u{c6710}\
        \u{68b09}\u{bd7b6}\u{7894a}\u{c4004}\u{51bea}\u{506d1}\u{b9e4e}\u{a300c}\u{fbea6}\u{f0f37}\
        \u{565c6}\u{1184c}\u{1c479}\u{359ab}\u{38f38}\u{198e3}\u{ad730}\u{7df6f}\u{540ee}\u{14e95}\
        \u{3e57d}\u{fded1}\u{4613f}\u{bdc98}\u{567d6}\u{7f434}\u{b47aa}\u{8adc0}\u{f6f1d}\u{e2068}\
        \u{b8913}\u{3401b}\u{e9f10}\u{debe0}",
        &[
            ('\u{7cb51}', 9),
            ('\u{9f2b9}', 9),
            ('\u{39d1d}', 8),
            ('\u{3f6cd}', 8),
            ('\u{654c8}', 8),
            ('\u{66ab5}', 8),
            ('\u{6cbc3}', 8),
            ('\u{7cf6b}', 8),
            ('\u{9d925}', 8),
            ('\u{a99f8}', 8),
        ],
        ('\u{99011}', None),
    );
    // p = 50/51
    graphic_weighted_random_chars_helper(
        50,
        51,
        "𗄡𭼭礜깬ꅉ杼쭗𫆬╳𲐕𡺽⢏𲣍𮛊瀊𰥶𲛂ՠ𥐷𨫉𥧻𤜖𠡩𰭝🈱𰠲厨𠀓𩰦⋘𑇫𐤚⎖𡙄摕𮨱𖭨𰊗蚆𓈧𥍎𓪞𤪨𠷯뛕䂇𡴬Ở쀖𝓩𪔭𪣂𓑫憒쇝𭋋㆐쬾𣌺𲙃𪪯𐓍ჷ𡙫𭕍\u{90141}𜵐𦀢죍呶𫪬䍪𧠨姖𭩝𦠄𰒁𫵥𐌞䆞𪠊\
        𒐆𲪃𳇴뎒𡟨𘋯𓃸𥙨㨟ֆ뇫𩬳𦜟𘇶𝓈譀𢷏𘁉𝆵鎋彔𮂔𥌴𠵀𘯴𥷀𱠪𫃗𬁵𑄷蠃𰱩𨨜鐔𛇛ㅐ耂𤁤ꔺ𥚍𦖇歠𠝼퉫𤰃𤋡笃羼𠼆\u{3a6f5}𗼌𡝿𓳸𢏼𰮬𫽩敃置𥟓𢚩𢇠𣗡𗎫ᕵ붏삇𓡫𦬓闑贛𧴈𒾭𬱞𤉀𨯝𣘉𑨬𫸊𮂺𢽋掟𞡙\
        ✄㕵굜𖪢𥘀㾒\u{d9ae0}𖭤𱹜褱𓖫𭗺𦢓𐽺튕䯷𘳏𩶇𞹔𗟦釽쥐達헷𦩫𳈂𫛘\u{e9c8d}𗮶𠃅𘱜裳𦃿Ǐ𪥈𲜳䖇",
        &[
            ('𗔓', 20),
            ('𲍾', 20),
            ('ﱠ', 19),
            ('𓕋', 19),
            ('𬏄', 19),
            ('𳅸', 19),
            ('䦯', 18),
            ('幋', 18),
            ('跠', 18),
            ('𗢹', 18),
        ],
        ('🂄', Some('🂅')),
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
