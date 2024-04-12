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
        "\u{8d835}𘋍\u{37de9}𰜂\u{d71d4}礢\u{e7381}깻\u{38cc2}\u{d3467}ꅏ\u{8c106}枂쭦𬝶╱𣑿⢍𰺟\u{3872c}瀐𲆽ՠ\
        \u{71d76}𦧹𪂋𦾽\u{c7398}𥳘𡸫\u{c4dc5}\u{5341d}\u{ee79d}\u{35e0d}𲎤\u{f1fed}𠵅\u{160e5}\u{35f74}\
        \u{5a80f}𲁹\u{dfc84}\u{c07aa}厮𡗕𫈈⋓\u{f98ad}\u{aac59}\u{6de85}𑌦\u{ea790}𐧂⎑\u{19dc9}\u{33063}\
        \u{61c71}\u{76f9a}\u{b15d7}\u{f675b}\u{74f8a}𢰆\u{ec498}\u{f18bd}摛\u{3493b}\u{d8098}𱈆𘀩\
        \u{a01d8}𱫞蚌\u{a7f5c}\u{74c57}𓎞\u{fc396}𦤐\u{6dee1}\u{4c751}\u{67b17}\u{c858c}𗄩𦁪\u{1aac8}\
        \u{101151}𢎱\u{5ef4e}\u{7bcf5}\u{52f29}뛤䂍𣋮\u{c57eb}\u{d4d7a}Ộ\u{14e5f}쀥\u{89c2f}𠊊𫬗𫺌\
        \u{3a5b7}𔖂\u{61f97}憘쇬𮢟㆓쭍\u{68b3f}𤣼𬁹𐓹\u{5782b}\u{87d5f}ჵ𢰭\u{11c4f}𮬡\u{9e45a}🡢\u{ce605}𧗤\
        \u{6ab25}죜\u{6e56e}\u{bfec3}\u{9ef0d}呼\u{b1692}\u{42208}\u{d334d}\u{85561}𭂂䍰𨷪姜\u{9cb78}𰈲\
        \u{fd7b9}\u{53dfd}𧷆\u{ba31d}\u{8d218}𱳈\u{50c6b}𭌻\u{f1177}\u{a44a1}𐍔\u{e19c3}䆤𫷔\u{f3897}𒿕\
        \u{b2d99}뎡\u{c4f92}\u{e5ae0}𢶪\u{434d0}𝖀𓉯\u{d8e05}𦰪㨥ֆ뇺𫄕𧳡𝑰\u{cbdc0}\u{57b91}𠉩\u{905c2}\
        \u{c4f77}譆𤎑\u{b95f9}\u{832d4}𝅘𝅥𝅲𠂜\u{ed9f}\u{b4a25}\u{faddb}\u{12a47}\u{ac06b}鎑\u{c4eb9}彚𰡩\
        \u{3fdeb}\u{9d497}𦣶\u{cc080}𢌂\u{687ad}🄝\u{c9bee}\u{4733d}",
        &[
            ('בּ', 13), ('𘕗', 13), ('𣱺', 13), ('𧀇', 13), ('𩆒', 13), ('𪾨', 13), ('𬃙', 13), ('༈', 12),
            ('Ṿ', 12), ('㏥', 12)
        ],
        ('𰅏', None)
    );
    // p = 1/51
    graphic_weighted_random_chars_helper(
        1,
        51,
        "\u{8d835}\u{37de9}𘋍\u{d71d4}\u{e7381}\u{38cc2}\u{d3467}\u{8c106}\u{3872c}\u{71d76}\
        \u{c7398}\u{c4dc5}\u{5341d}\u{ee79d}\u{35e0d}\u{f1fed}\u{160e5}\u{35f74}\u{5a80f}\u{dfc84}\
        \u{c07aa}\u{f98ad}\u{aac59}\u{6de85}\u{ea790}\u{19dc9}\u{33063}\u{61c71}\u{76f9a}\u{b15d7}\
        \u{f675b}\u{74f8a}\u{ec498}\u{f18bd}\u{3493b}\u{d8098}\u{a01d8}\u{a7f5c}𰜂\u{74c57}\
        \u{fc396}\u{6dee1}\u{4c751}\u{67b17}\u{c858c}\u{1aac8}\u{101151}\u{5ef4e}\u{7bcf5}\
        \u{52f29}\u{c57eb}\u{d4d7a}\u{14e5f}\u{89c2f}\u{3a5b7}\u{61f97}\u{68b3f}\u{5782b}\u{87d5f}\
        \u{11c4f}\u{9e45a}\u{ce605}\u{6ab25}\u{6e56e}\u{bfec3}\u{9ef0d}\u{b1692}\u{42208}\u{d334d}\
        \u{85561}\u{9cb78}\u{fd7b9}\u{53dfd}\u{ba31d}\u{8d218}\u{50c6b}\u{f1177}\u{a44a1}\u{e19c3}\
        \u{f3897}\u{b2d99}\u{c4f92}\u{e5ae0}\u{434d0}\u{d8e05}\u{cbdc0}\u{57b91}\u{905c2}\u{c4f77}\
        \u{b95f9}\u{832d4}\u{ed9f}\u{b4a25}\u{faddb}\u{12a47}\u{ac06b}\u{c4eb9}\u{3fdeb}\u{9d497}\
        \u{cc080}\u{687ad}\u{c9bee}\u{4733d}\u{fc6c8}\u{475cd}\u{7588c}\u{41e33}\u{33de3}礢\
        \u{bb064}\u{e795d}\u{8f820}\u{6eb68}\u{3a717}\u{57d33}\u{531c1}\u{f788d}\u{b8a7c}\u{10077}\
        \u{9afe1}\u{e883e}\u{f7ccf}\u{e0a7f}\u{f73c4}\u{1e0ee}\u{8bce0}\u{35382}\u{748ee}\u{b4bbf}\
        깻\u{a821e}\u{dd93f}\u{10abdc}\u{904d8}\u{f7823}\u{40236}\u{190b4}\u{ad488}\u{80883}\
        \u{716c5}\u{41b2d}\u{6f96d}\u{1014e1}\u{78446}\u{106fdf}\u{f08f4}\u{94819}\u{7ed53}\
        \u{bfdad}\u{aad58}\u{adf55}\u{cb0ed}\u{cdd83}\u{f6874}\u{c4117}\u{b7e94}\u{86d94}\u{a01d6}\
        \u{1067f8}\u{d6541}\u{512a9}\u{109506}\u{cc921}\u{d0d13}\u{10dc2a}\u{9e960}\u{c3e04}\
        \u{661fd}\u{baeaa}\u{7603e}\u{c16f8}\u{4f2de}\u{4ddc5}\u{b7542}\u{a0700}\u{f959a}\u{ee62b}\
        \u{53cba}\u{116d2}\u{1aee9}\u{3309f}\u{3662c}\u{16589}\u{aae24}\u{7b663}\u{517e2}\u{13b3c}\
        \u{3bc71}\u{fb5c5}\u{43833}\u{bb38c}\u{53eca}\u{7cb28}\u{b1e9e}\u{884b4}\u{f4611}\u{df75c}\
        \u{b6007}\u{2f146}\u{e7604}",
        &[
            ('\u{7a245}', 9),
            ('\u{9c9ad}', 9),
            ('\u{37411}', 8),
            ('\u{3cdc1}', 8),
            ('\u{641a9}', 8),
            ('\u{6a2b7}', 8),
            ('\u{7a65f}', 8),
            ('\u{9b019}', 8),
            ('\u{a70ec}', 8),
            ('\u{c5e00}', 8),
        ],
        ('\u{97b8b}', Some('\u{97b8c}')),
    );
    // p = 50/51
    graphic_weighted_random_chars_helper(
        50,
        51,
        "𘋍𰜂礢깻ꅏ枂쭦𬝶╱𣑿⢍𰺟瀐𲆽ՠ𦧹𪂋𦾽𥳘𡸫𲎤𠵅𲁹厮𡗕𫈈⋓𑌦𐧂⎑𢰆摛𱈆𘀩𱫞蚌𓎞𦤐𗄩𦁪𢎱뛤䂍𣋮Ộ쀥𠊊𫬗𫺌𔖂憘쇬𮢟㆓쭍𤣼𬁹𐓹ჵ𢰭𮬡🡢𧗤죜呼\u{8d835}𭂂䍰𨷪姜𰈲𧷆𱳈𭌻𐍔䆤𫷔𒿕뎡𢶪𝖀\
        𓉯𦰪㨥ֆ뇺𫄕𧳡𝑰𠉩譆𤎑𝅘𝅥𝅲𠂜鎑彚𰡩𦣶𢌂🄝𧎂𬚡𭙋𑈫蠉𩿞鐚🔮ㅓ耈𥘦ꕀ𦱏𧭉武𡴾퉺𦇅𥢣笉翂𢓈𝀛𢵁𗎃𣦾𭔿敉罴𦶕\u{37de9}𣱫𣞢𤮣𘕗ᕳ붞삖𖭲𨃕闗贡𩋊𓄱𮈴𥠂𪆟𤯋𑲱𭏠𰢏𤔍接𠦡✂㕻굫𗾎𦯂㾘𘀥褷𖡻\
        𮯎𧹕𑂩튤䯽🉁\u{d71d4}𫍩𠬤𘦚鈃쥟遚혆𨀭𬲢𛁾𡚇🆅裹𧛁Ǐ𫼒䖍೨꺔𲇔\u{e7381}𡈞䔭팟𐧟羰𡈁𭀴퍺𨜒",
        &[
            ('𘚿', 23), ('𢤗', 20), ('𤉿', 20), ('ﱿ', 19), ('𖠛', 19), ('𢓋', 19), ('𤌊', 19), ('𤦺', 19),
            ('𦱈', 19), ('𭦚', 19)
        ], ('🄡', None)
    );
}

#[test]
#[should_panic]
fn graphic_weighted_random_chars_fail_1() {
    graphic_weighted_random_chars(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn graphic_weighted_random_chars_fail_2() {
    graphic_weighted_random_chars(EXAMPLE_SEED, 2, 1);
}
