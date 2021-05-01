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
        "\u{8c401}𘓸\u{369b5}𰫖\u{d5da0}祝\u{e5f4d}껆\u{3788e}\u{d2033}ꆍ\u{8acd2}枽쮱𬭊▟𣡌⢻𱉳\u{372f8}\
        灋ՠ𦷆\u{70942}𪑘𧎊𦂥\u{c5f64}𢇸𡄒\u{c3991}\u{51fe9}\u{ed369}\u{349d9}叩\u{f0bb9}𡦢\u{15f13}\
        \u{34b40}\u{593db}𫗗\u{de850}\u{bf376}⌁𑗇𐫱⎿\u{f8479}\u{a9825}\u{6ca51}𢿓\u{e935c}撖𘉔\
        \u{19b9e}\u{31c2f}\u{6083d}\u{75b66}\u{b01a3}\u{f5327}\u{73b56}蛇\u{eb064}\u{f0489}𔔺\
        \u{33507}\u{d6c64}𦳝𗍔\u{9eda4}𦐷𢝾\u{a6b28}\u{73823}뜯\u{faf62}䃈\u{6caad}\u{10fcbf}\
        \u{4b31d}\u{666e3}𣚻ἅ\u{c7158}\u{1a89d}쁰\u{ffd1d}\u{5db1a}\u{7a8c1}𠙗𫻫𬉠\u{51af5}\u{c43b7}\
        𖤍\u{d3946}懓\u{14c8d}숷𢌱㇎\u{887fb}쮘\u{39183}𤳉𬑍𐕯ᄛ𢿺\u{60b63}𰃶𠃦𧦱\u{6770b}\u{563f7}줧咷\
        \u{8692b}𭑖\u{11afe}䎫\u{9d026}𩆷\u{cd1d1}娗\u{696f1}\u{6d13a}\u{bea8f}𰘆\u{9dad9}\u{b025e}\
        \u{40dd4}\u{d1f19}𨆓𭜏𐎾䇟\u{8412d}𬆨\u{9b744}\u{fc385}𓆭\u{529c9}\u{b8ee9}돬\u{8bde4}𣅷\
        \u{4f837}\u{efd43}𝡌\u{a306d}𔐋𦿷\u{e058f}㩠\u{f2463}ֆ\u{b1965}\u{c3b5e}뉅\u{e46ac}𫓤𨂮\
        \u{4209c}𝝑𠘶讁𤝞𝖢𠑩鏌\u{d79d1}\u{ca98c}徕\u{5675d}\u{8f18e}𰰽𦳃\u{c3b43}\u{b81c5}𢛏🖠\u{81ea0}\
        \u{ed54}\u{b35f1}\u{f99a7}\u{128df}𧝏\u{aac37}𬩵𭨟\u{c3a85}\u{3e9b7}𑑑\u{9c063}衄\u{cac4c}𪎫\
        \u{67379}\u{c87ba}",
        &[('𤁇', 14), ('𬒭', 14), ('ﮁ', 13), ('𘞂', 13), ('𧏔', 13), ('𩕟', 13), ('𫍷', 13),
          ('༰', 12), ('ẫ', 12), ('㐠', 12)],
        ('𮊳', None)
    );
    // p = 1/51
    graphic_weighted_random_chars_helper(
        1,
        51,
        "\u{8c401}\u{369b5}𘓸\u{d5da0}\u{e5f4d}\u{3788e}\u{d2033}\u{8acd2}\u{372f8}\u{70942}\
        \u{c5f64}\u{c3991}\u{51fe9}\u{ed369}\u{349d9}\u{f0bb9}\u{15f13}\u{34b40}\u{593db}\u{de850}\
        \u{bf376}\u{f8479}\u{a9825}\u{6ca51}\u{e935c}\u{19b9e}\u{31c2f}\u{6083d}\u{75b66}\u{b01a3}\
        \u{f5327}\u{73b56}\u{eb064}\u{f0489}\u{33507}\u{d6c64}\u{9eda4}\u{a6b28}𰫖\u{73823}\u{faf62}\
        \u{6caad}\u{10fcbf}\u{4b31d}\u{666e3}\u{c7158}\u{1a89d}\u{ffd1d}\u{5db1a}\u{7a8c1}\u{51af5}\
        \u{c43b7}\u{d3946}\u{14c8d}\u{887fb}\u{39183}\u{60b63}\u{6770b}\u{563f7}\u{8692b}\u{11afe}\
        \u{9d026}\u{cd1d1}\u{696f1}\u{6d13a}\u{bea8f}\u{9dad9}\u{b025e}\u{40dd4}\u{d1f19}\u{8412d}\
        \u{9b744}\u{fc385}\u{529c9}\u{b8ee9}\u{8bde4}\u{4f837}\u{efd43}\u{a306d}\u{e058f}\u{f2463}\
        \u{b1965}\u{c3b5e}\u{e46ac}\u{4209c}\u{d79d1}\u{ca98c}\u{5675d}\u{8f18e}\u{c3b43}\u{b81c5}\
        \u{81ea0}\u{ed54}\u{b35f1}\u{f99a7}\u{128df}\u{aac37}\u{c3a85}\u{3e9b7}\u{9c063}\u{cac4c}\
        \u{67379}\u{c87ba}\u{45f09}\u{fb294}\u{46199}\u{74458}\u{409ff}祝\u{329af}\u{b9c30}\
        \u{e6529}\u{8e3ec}\u{6d734}\u{392e3}\u{568ff}\u{51d8d}\u{f6459}\u{b7648}\u{fe0f}\u{99bad}\
        \u{e740a}\u{f689b}\u{df64b}\u{f5f90}\u{1ddc3}\u{8a8ac}\u{33f4e}\u{734ba}껆\u{b378b}\
        \u{a6dea}\u{dc50b}\u{1097a8}\u{8f0a4}\u{f63ef}\u{3ee02}\u{18e89}\u{ac054}\u{7f44f}\u{70291}\
        \u{406f9}\u{6e539}\u{1000ad}\u{77012}\u{105bab}\u{ef4c0}\u{933e5}\u{7d91f}\u{be979}\
        \u{a9924}\u{acb21}\u{c9cb9}\u{cc94f}\u{f5440}\u{c2ce3}\u{b6a60}\u{85960}\u{9eda2}\u{1053c4}\
        \u{d510d}\u{4fe75}\u{1080d2}\u{cb4ed}\u{cf8df}\u{10c7f6}\u{9d52c}\u{c29d0}\u{64dc9}\
        \u{b9a76}\u{74c0a}\u{c02c4}\u{4deaa}\u{4c991}\u{b610e}\u{9f2cc}\u{f8166}\u{ed1f7}\u{52886}\
        \u{114fc}\u{1acbe}\u{31c6b}\u{351f8}\u{163b7}\u{a99f0}\u{7a22f}\u{503ae}\u{1396a}\u{3a83d}\
        \u{fa191}\u{423ff}\u{b9f58}\u{10fe12}\u{52a96}\u{7b6f4}\u{b0a6a}\u{87080}\u{f31dd}\u{de328}\
        \u{b4bd3}",
        &[('\u{78e11}', 9), ('\u{9b579}', 9), ('\u{35fdd}', 8), ('\u{3b98d}', 8), ('\u{62d75}', 8),
          ('\u{68e83}', 8), ('\u{7922b}', 8), ('\u{a5cb8}', 8), ('\u{c49cc}', 8), ('\u{c62dc}', 8)],
        ('\u{97180}', Some('\u{97182}'))
    );
    // p = 50/51
    graphic_weighted_random_chars_helper(
        50,
        51,
        "𘓸𰫖祝껆ꆍ枽쮱𬭊▟𣡌⢻𱉳灋ՠ𦷆𪑘𧎊𦂥𢇸𡄒叩𡦢𫗗⌁𑗇𐫱⎿𢿓撖𘉔蛇𔔺𦳝𗍔𦐷𢝾뜯䃈𣚻ἅ쁰𠙗𫻫𬉠𖤍懓숷𢌱㇎쮘𤳉𬑍𐕯ᄛ𢿺𰃶𠃦𧦱줧\
        咷𭑖䎫𩆷娗𰘆\u{8c401}𨆓𭜏𐎾䇟𬆨𓆭돬𣅷𝡌𔐋𦿷㩠ֆ뉅𫓤𨂮𝝑𠘶讁𤝞𝖢𠑩鏌徕𰰽𦳃𢛏🖠𧝏𬩵𭨟𑑑衄𪎫鑕🥘ㆎ聃𥧳ꕾ𧀜𧼖殡𢄋틅𦖒𥱰\
        筄翽𢢕𝑍𣄎𗖮𣶋𭤓斄羯𧅢𤀸𣭯𤽰𘞂ᖙ뷩\u{369b5}샡𗄡𨒢阒赜𩚗𓋼𮘈𥯏𪕬𤾘𒁙𭞴𰱣𤣚揠𠵮✰㖶궶𘆹𦾏㿓𘉐襲𖩔𰆣𨈢𑈁틯䰸🙻𫝃𠻱𘯅\
        鈾즪邕\u{d5da0}홑𨏺𭂄𛰄𡩔😈褴𧪎Ǐ𬋦䗈ട껟𡗫䕨퍪𐬕翫𡗎𭐈\u{e5f4d}폅𨫟荃𫩀黺䀵ڵ붶𥵂",
        &[('𘣲', 23), ('𤙌', 22), ('ﳋ', 20), ('𢳤', 20), ('𭵮', 20), ('埅', 19), ('庌', 19),
          ('踡', 19), ('𖧭', 19), ('𢢘', 19)],
        ('𝝀', None)
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
