use malachite_base::chars::random::graphic_weighted_random_chars;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map_debug;
use malachite_base_test_util::stats::median;

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
        "\u{8c6d6}𘌮\u{36c8a}𰠁\u{d6075}礣\u{e6222}깼\u{37b63}\u{d2308}ꅐ\u{8afa7}枃쭧𬡵╲𣕽⢎𰾞\u{375cd}瀑ՠ𦫷\
        \u{70c17}𪆉𧂻𥷖\u{c6239}𡼩𠹃\u{c3c66}\u{522be}\u{ed63e}\u{34cae}厯\u{f0e8e}𡛓\u{16084}\u{34e15}\
        \u{596b0}𫌆\u{deb25}\u{bf64b}⋔𑌪𐧃⎒\u{f874e}\u{a9afa}\u{6cd26}𢴄\u{e9631}摜𱌅\u{19d68}\u{31f04}\
        \u{60b12}\u{75e3b}\u{b0478}\u{f55fc}\u{73e2b}𘂊\u{eb339}\u{f075e}蚍\u{337dc}\u{d6f39}𓏸𦨎\
        \u{9f079}𗆊𦅨\u{a6dfd}\u{73af8}𢒯\u{fb237}뛥\u{6cd82}\u{10ff94}\u{4b5f2}\u{669b8}䂎𣏬\u{c742d}\
        \u{1aa67}ộ\u{ffff2}\u{5ddef}\u{7ab96}쀦𠎈𫰖\u{51dca}\u{c468c}𫾋\u{d3c1b}𔗣\u{14dfe}憙쇭𮦞\u{88ad0}\
        ㆔\u{39458}쭎𤧺𬅸𐓺ჶ\u{60e38}𢴫周🦥\u{679e0}\u{566cc}𧛢죝\u{86c00}命\u{11c3b}𭆁\u{9d2fb}䍱\u{cd4a6}𨻨\
        \u{699c6}\u{6d40f}\u{bed64}姝\u{9ddae}\u{b0533}\u{410a9}\u{d21ee}𰌱𧻄𭐺𐍕\u{84402}䆥\u{9ba19}\
        \u{fc65a}𫻓\u{52c9e}\u{b91be}𓀼\u{8c0b9}뎢\u{4fb0c}\u{f0018}𢺨\u{a3342}𝗷𓋉\u{e0864}𦴨\u{f2738}\
        㨦\u{b1c3a}\u{c3e33}ֆ\u{e4981}뇻𫈓\u{42371}𧷟𝓲𠍧譇𤒏𝈀𠆚\u{d7ca6}\u{cac61}鎒\u{56a32}\u{8f463}彛𰥨\
        \u{c3e18}\u{b849a}𦧴𢐀\u{82175}\u{ed9e}\u{b38c6}\u{f9c7c}\u{129ed}🉇\u{aaf0c}𧒀𬞠\u{c3d5a}\
        \u{3ec8c}𭝊\u{9c338}𑈬\u{caf21}蠊\u{6764e}\u{c8a8f}",
        &[
            ('𣵸', 14), ('𬇘', 14), ('גּ', 13), ('𘖸', 13), ('𧄅', 13), ('𩊐', 13), ('𫂦', 13), ('༉', 12),
            ('ṿ', 12), ('㏦', 12)
        ],
        ('𮋖', Some('𮋗'))
    );
    // p = 1/51
    graphic_weighted_random_chars_helper(
        1,
        51,
        "\u{8c6d6}\u{36c8a}𘌮\u{d6075}\u{e6222}\u{37b63}\u{d2308}\u{8afa7}\u{375cd}\u{70c17}\
        \u{c6239}\u{c3c66}\u{522be}\u{ed63e}\u{34cae}\u{f0e8e}\u{16084}\u{34e15}\u{596b0}\u{deb25}\
        \u{bf64b}\u{f874e}\u{a9afa}\u{6cd26}\u{e9631}\u{19d68}\u{31f04}\u{60b12}\u{75e3b}\u{b0478}\
        \u{f55fc}\u{73e2b}\u{eb339}\u{f075e}\u{337dc}\u{d6f39}\u{9f079}\u{a6dfd}𰠁\u{73af8}\
        \u{fb237}\u{6cd82}\u{10ff94}\u{4b5f2}\u{669b8}\u{c742d}\u{1aa67}\u{ffff2}\u{5ddef}\
        \u{7ab96}\u{51dca}\u{c468c}\u{d3c1b}\u{14dfe}\u{88ad0}\u{39458}\u{60e38}\u{679e0}\u{566cc}\
        \u{86c00}\u{11c3b}\u{9d2fb}\u{cd4a6}\u{699c6}\u{6d40f}\u{bed64}\u{9ddae}\u{b0533}\u{410a9}\
        \u{d21ee}\u{84402}\u{9ba19}\u{fc65a}\u{52c9e}\u{b91be}\u{8c0b9}\u{4fb0c}\u{f0018}\u{a3342}\
        \u{e0864}\u{f2738}\u{b1c3a}\u{c3e33}\u{e4981}\u{42371}\u{d7ca6}\u{cac61}\u{56a32}\u{8f463}\
        \u{c3e18}\u{b849a}\u{82175}\u{ed9e}\u{b38c6}\u{f9c7c}\u{129ed}\u{aaf0c}\u{c3d5a}\u{3ec8c}\
        \u{9c338}\u{caf21}\u{6764e}\u{c8a8f}\u{461de}\u{fb569}\u{4646e}\u{7472d}\u{40cd4}礣\
        \u{32c84}\u{b9f05}\u{e67fe}\u{8e6c1}\u{6da09}\u{395b8}\u{56bd4}\u{52062}\u{f672e}\u{b791d}\
        \u{10076}\u{99e82}\u{e76df}\u{f6b70}\u{df920}\u{f6265}\u{1e033}\u{8ab81}\u{34223}\u{7378f}\
        깼\u{b3a60}\u{a70bf}\u{dc7e0}\u{109a7d}\u{8f379}\u{f66c4}\u{3f0d7}\u{19053}\u{ac329}\
        \u{7f724}\u{70566}\u{409ce}\u{6e80e}\u{100382}\u{772e7}\u{105e80}\u{ef795}\u{936ba}\
        \u{7dbf4}\u{bec4e}\u{a9bf9}\u{acdf6}\u{c9f8e}\u{ccc24}\u{f5715}\u{c2fb8}\u{b6d35}\u{85c35}\
        \u{9f077}\u{105699}\u{d53e2}\u{5014a}\u{1083a7}\u{cb7c2}\u{cfbb4}\u{10cacb}\u{9d801}\
        \u{c2ca5}\u{6509e}\u{b9d4b}\u{74edf}\u{c0599}\u{4e17f}\u{4cc66}\u{b63e3}\u{9f5a1}\u{f843b}\
        \u{ed4cc}\u{52b5b}\u{116cf}\u{1ae88}\u{31f40}\u{354cd}\u{16528}\u{a9cc5}\u{7a504}\u{50683}\
        \u{13adb}\u{3ab12}\u{fa466}\u{426d4}\u{ba22d}\u{52d6b}\u{7b9c9}\u{b0d3f}\u{87355}\u{f34b2}\
        \u{de5fd}\u{b4ea8}\u{2f047}",
        &[
            ('\u{790e6}', 9),
            ('\u{9b84e}', 9),
            ('\u{362b2}', 8),
            ('\u{3bc62}', 8),
            ('\u{6304a}', 8),
            ('\u{69158}', 8),
            ('\u{79500}', 8),
            ('\u{99eba}', 8),
            ('\u{a5f8d}', 8),
            ('\u{c4ca1}', 8),
        ],
        ('\u{972fa}', Some('\u{972fb}')),
    );
    // p = 50/51
    graphic_weighted_random_chars_helper(
        50,
        51,
        "𘌮𰠁礣깼ꅐ枃쭧𬡵╲𣕽⢎𰾞瀑ՠ𦫷𪆉𧂻𥷖𡼩𠹃厯𡛓𫌆⋔𑌪𐧃⎒𢴄摜𱌅𘂊蚍𓏸𦨎𗆊𦅨𢒯뛥䂎𣏬ộ쀦𠎈𫰖𫾋𔗣憙쇭𮦞㆔쭎𤧺𬅸𐓺ჶ𢴫周🦥𧛢죝命𭆁䍱𨻨姝\u{8c6d6}𰌱𧻄𭐺𐍕䆥𫻓𓀼뎢𢺨𝗷𓋉𦴨㨦ֆ뇻\
        𫈓𧷟𝓲𠍧譇𤒏𝈀𠆚鎒彛𰥨𦧴𢐀🉇𧒀𬞠𭝊𑈬蠊𪃜鐛😏ㅔ耉𥜤ꕁ𦵍𧱇歧𡸼퉻𦋃𥦡笊翃𢗆𝁾𢸿𗏤𣪼𭘾敊罵𦺓𣵩𣢠𤲡𘖸ᕴ\u{36c8a}붟삗𖺈𨇓闘财𩏈𓆋𮌳𥤀𪊝𤳉𑴍𭓟𰦎𤘋掦𠪟✃㕼구𗿯𦳀㾙𘂆褸𖣜𣚣𧽓𑂪튥䯾\
        🏑𫑧𠰢𘧻鈄쥠\u{d6075}遛혇𨄫𬶡𛃟𡞅🍞裺𧞿Ǐ𬀑䖎೨꺕𡌜䔮팠𐧠羱𡋿\u{e6222}𭄳퍻𨠐茉𫝩黀㿻ڴ뵬",
        &[
            ('𘜠', 23), ('𤍽', 22), ('ﲀ', 20), ('𢨕', 20), ('幒', 19), ('跧', 19), ('𖡼', 19), ('𢗉', 19),
            ('𤐈', 19), ('𤪸', 19)
        ],
        ('𝙠', None)
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
