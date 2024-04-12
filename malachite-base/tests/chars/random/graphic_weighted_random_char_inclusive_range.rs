// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::chars::random::graphic_weighted_random_char_inclusive_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

fn graphic_weighted_random_char_inclusive_range_helper(
    a: char,
    b: char,
    p_numerator: u64,
    p_denominator: u64,
    expected_values: &str,
    expected_common_values: &[(char, usize)],
    expected_median: (char, Option<char>),
) {
    let xs = graphic_weighted_random_char_inclusive_range(
        EXAMPLE_SEED,
        a,
        b,
        p_numerator,
        p_denominator,
    );
    let values = xs.clone().take(200).collect::<String>();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_str(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_graphic_weighted_random_char_inclusive_range() {
    // 'a', '\u{7f}', p = 1/2
    graphic_weighted_random_char_inclusive_range_helper(
        'a',
        '\u{7f}',
        1,
        2,
        "\u{7f}y\u{7f}g\u{7f}c\u{7f}k\u{7f}\u{7f}}\u{7f}ccjlybr\u{7f}tue\u{7f}}ye\u{7f}oz\u{7f}\
        \u{7f}\u{7f}\u{7f}d\u{7f}f\u{7f}\u{7f}\u{7f}f\u{7f}\u{7f}fmxr\u{7f}\u{7f}\u{7f}g\u{7f}hy\
        \u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}q\u{7f}\u{7f}t\u{7f}\u{7f}pk\u{7f}oh\u{7f}\u{7f}i\
        \u{7f}q\u{7f}\u{7f}\u{7f}\u{7f}nf\u{7f}\u{7f}l\u{7f}\u{7f}\u{7f}sqf\u{7f}\u{7f}t\u{7f}d\
        \u{7f}emm\u{7f}|\u{7f}{z|wu\u{7f}dsc\u{7f}\u{7f}f{\u{7f}t\u{7f}r\u{7f}t\u{7f}q\u{7f}\u{7f}\
        \u{7f}h\u{7f}\u{7f}\u{7f}\u{7f}klx~\u{7f}x\u{7f}\u{7f}c\u{7f}\u{7f}u\u{7f}l\u{7f}\u{7f}y\
        \u{7f}~d\u{7f}~\u{7f}f\u{7f}\u{7f}m\u{7f}xf\u{7f}lkvcu{e\u{7f}\u{7f}b\u{7f}\u{7f}yh\u{7f}\
        \u{7f}nz\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}e\u{7f}vu\u{7f}\u{7f}}\u{7f}y\u{7f}u\u{7f}\u{7f}",
        &[
            ('\u{7f}', 499332),
            ('a', 16868),
            ('t', 16831),
            ('j', 16829),
            ('r', 16822),
            ('i', 16816),
            ('f', 16806),
            ('{', 16780),
            ('s', 16763),
            ('d', 16749),
        ],
        ('~', None),
    );
    // 'a', '\u{7f}', p = 1/51
    graphic_weighted_random_char_inclusive_range_helper(
        'a',
        '\u{7f}',
        1,
        51,
        "\u{7f}\u{7f}y\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\
        \u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\
        \u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}g\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\
        \u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\
        \u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\
        \u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\
        \u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\
        \u{7f}\u{7f}c\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\
        \u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}k\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\
        \u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\
        \u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\
        \u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\
        \u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\
        \u{7f}\u{7f}",
        &[
            ('\u{7f}', 980198),
            ('h', 697),
            ('n', 691),
            ('i', 681),
            ('w', 680),
            ('r', 677),
            ('a', 675),
            ('|', 674),
            ('q', 673),
            ('e', 670),
        ],
        ('\u{7f}', None),
    );
    // 'a', '\u{7f}', p = 50/51
    graphic_weighted_random_char_inclusive_range_helper(
        'a',
        '\u{7f}',
        50,
        51,
        "ygck}ccjlybrtue}yeozdfffmxrghyqtpkohiqnflsqftdemm|{z|wudscf{trtqh\u{7f}klx~xculy~d~fmxflkv\
        cu{ebyhnzevu}yuzvtuz}{b~yyzin}zsjthazlcluvu}a\u{7f}yvhhbgevdxo}unwjqepanjmpckouxtwwbi{nla\
        \u{7f}cs{vphyrdaqhzndmqdqg\u{7f}{{mfwk|xf",
        &[
            ('a', 32996),
            ('f', 32964),
            ('q', 32960),
            ('{', 32942),
            ('s', 32886),
            ('t', 32809),
            ('d', 32768),
            ('n', 32767),
            ('p', 32766),
            ('i', 32765),
        ],
        ('p', None),
    );

    // '\u{100}', '\u{3ff}', p = 1/2
    graphic_weighted_random_char_inclusive_range_helper(
        '\u{100}',
        '\u{3ff}',
        1,
        2,
        "\u{31b}ǘ\u{324}ɂ\u{35a}Ŝ\u{367}Ȣ\u{337}\u{342}Ι\u{36e}Ƙƣʅΰǟ˳ˊ\u{315}ȇʰɥ\u{32c}ΈϞƃ\u{30b}ʌμ\
        \u{346}\u{36a}\u{328}\u{334}Ƣ\u{380}ϳ\u{32e}\u{359}\u{36b}Ϫ\u{333}\u{312}ǰɪŗŽ\u{302}\u{305}\
        \u{32e}ƽ\u{357}ɥϻ\u{349}\u{35f}\u{316}\u{33a}\u{383}\u{301}\u{31d}ƚ\u{360}\u{35d}ʧ\u{30c}\
        \u{30c}ƙϮ\u{34a}șϒ\u{367}\u{35f}Ϣ\u{308}ć\u{353}\u{333}\u{381}\u{30f}ɾɢ\u{367}\u{340}ǧ\
        \u{356}\u{30c}\u{351}ǁʴȶ\u{33e}\u{358}Ɛ\u{351}ď\u{366}ȭˬɂ\u{38b}ϰ\u{342}ȁʺċλǿ\u{30c}αăǰ\
        \u{31c}\u{327}ʹʃ\u{34a}Ű\u{36a}ǐ\u{33f}Ƭ\u{34c}ɖ\u{361}\u{335}\u{318}Ͼ\u{349}\u{35d}\u{330}\
        \u{35c}ƾƱǒȆ\u{320}Ɉ\u{348}\u{324}Ϫ\u{31d}\u{35c}ʶ\u{332}ť\u{33d}\u{300}ˍ\u{326}ďǑ\u{329}ɶ\
        \u{33d}Ɂ\u{31f}\u{363}ȭ\u{320}ƭğ\u{321}Ⱦ˹ĺǗʋǆ˺\u{31a}\u{35c}ʐ\u{304}\u{365}Ǎŋ\u{34f}\u{31f}\
        ɽɷ\u{31e}\u{32a}\u{31a}\u{307}\u{319}ϲ\u{310}Ÿȃ\u{369}\u{359}Ĩ\u{33e}Ş\u{38d}˭\u{31e}\
        \u{321}",
        &[
            ('\u{326}', 4271),
            ('\u{357}', 4238),
            ('\u{33b}', 4235),
            ('\u{36b}', 4235),
            ('\u{348}', 4228),
            ('\u{30b}', 4225),
            ('\u{304}', 4224),
            ('\u{30d}', 4224),
            ('\u{327}', 4221),
            ('\u{325}', 4217),
        ],
        ('\u{319}', None),
    );
    // '\u{100}', '\u{3ff}', p = 1/51
    graphic_weighted_random_char_inclusive_range_helper(
        '\u{100}',
        '\u{3ff}',
        1,
        51,
        "\u{31b}\u{324}ǘ\u{35a}\u{367}\u{337}\u{342}\u{36e}\u{315}\u{32c}\u{30b}\u{346}\u{36a}\
        \u{328}\u{334}\u{380}\u{32e}\u{359}\u{36b}\u{333}\u{312}\u{302}\u{305}\u{32e}\u{357}\u{349}\
        \u{35f}\u{316}\u{33a}\u{383}\u{301}\u{31d}\u{360}\u{35d}\u{30c}\u{30c}\u{34a}\u{367}ɂ\
        \u{35f}\u{308}\u{353}\u{333}\u{381}\u{30f}\u{367}\u{340}\u{356}\u{30c}\u{351}\u{33e}\u{358}\
        \u{351}\u{366}\u{38b}\u{342}\u{30c}\u{31c}\u{327}\u{34a}\u{36a}\u{33f}\u{34c}\u{361}\u{335}\
        \u{318}\u{349}\u{35d}\u{330}\u{35c}\u{320}\u{348}\u{324}\u{31d}\u{35c}\u{332}\u{33d}\u{300}\
        \u{326}\u{329}\u{33d}\u{31f}\u{363}\u{320}\u{321}\u{31a}\u{35c}\u{304}\u{365}\u{34f}\u{31f}\
        \u{31e}\u{32a}\u{31a}\u{307}\u{319}\u{310}\u{369}\u{359}\u{33e}\u{38d}\u{31e}\u{321}\u{339}\
        \u{330}\u{30f}\u{303}\u{345}Ŝ\u{32f}\u{35f}\u{33e}\u{30c}\u{313}\u{365}\u{356}\u{36c}\
        \u{36f}\u{304}\u{314}\u{369}\u{310}\u{339}\u{33f}\u{354}\u{347}\u{378}\u{366}\u{380}Ȣ\
        \u{326}\u{313}\u{383}\u{36c}\u{336}\u{357}\u{31e}\u{324}\u{317}\u{351}\u{323}\u{328}\u{330}\
        \u{307}\u{336}\u{326}\u{32a}\u{350}\u{342}\u{315}\u{36b}\u{32b}\u{382}\u{34b}\u{33d}\u{34a}\
        \u{314}\u{322}\u{311}\u{364}\u{34c}\u{322}\u{337}\u{30c}\u{362}\u{317}\u{300}\u{348}\u{367}\
        \u{35b}\u{353}\u{32e}\u{328}\u{346}\u{328}\u{324}\u{329}\u{379}\u{366}\u{328}\u{30f}\u{31e}\
        \u{368}\u{368}\u{38b}\u{318}\u{353}\u{35d}\u{31e}\u{360}\u{33c}\u{315}\u{33c}\u{31f}\u{323}\
        \u{38b}\u{379}\u{38d}\u{365}\u{340}",
        &[
            ('\u{30d}', 8469),
            ('\u{326}', 8280),
            ('\u{327}', 8268),
            ('\u{357}', 8250),
            ('\u{30c}', 8244),
            ('\u{325}', 8242),
            ('\u{30a}', 8232),
            ('\u{30b}', 8232),
            ('\u{35d}', 8222),
            ('\u{36b}', 8221),
        ],
        ('\u{33b}', None),
    );
    // '\u{100}', '\u{3ff}', p = 50/51
    graphic_weighted_random_char_inclusive_range_helper(
        '\u{100}',
        '\u{3ff}',
        50,
        51,
        "ǘɂŜȢΙƘƣʅΰǟ˳ˊȇʰɥΈϞƃʌμƢϳϪǰɪŗŽƽɥϻƚʧƙϮșϒϢćɾɢǧǁʴȶƐďȭˬɂϰȁʺċλǿαăǰʹʃŰǐƬɖϾ\u{31b}ƾƱǒȆɈϪʶťˍďǑɶɁȭƭğȾ˹\
        ĺǗʋǆ˺ʐǍŋɽɷϲŸȃĨŞ˭ƪřœɃˡϹƤǁĥɨʎʄˬıɐƺˀǍǷěέƼȞŞȩǀ΄ʱǱȭ\u{324}ʚǃϧśŅˊǉΰϿǟϵȟǘĹϋȎʜɴϻϝͺĪƕ˪ϥŪˍʸĻέ˙ĠĢĄšǝ˿Ί\
        \u{35a}ΗʩǊŰŦʝŋƶˈąŠɂŻǱƲɛΚʜťγ\u{367}ϽϹčǇīϥſʏȵ",
        &[
            ('Ώ', 1637),
            ('Ɣ', 1627),
            ('ɭ', 1618),
            ('Έ', 1616),
            ('Ɯ', 1613),
            ('Ȓ', 1610),
            ('Ǧ', 1609),
            ('˵', 1608),
            ('˷', 1607),
            ('ƒ', 1606),
        ],
        ('ɉ', None),
    );
}

#[test]
#[should_panic]
fn graphic_weighted_random_char_inclusive_range_fail_1() {
    graphic_weighted_random_char_inclusive_range(EXAMPLE_SEED, 'a', '\u{80}', 1, 0);
}

#[test]
#[should_panic]
fn graphic_weighted_random_char_inclusive_range_fail_2() {
    graphic_weighted_random_char_inclusive_range(EXAMPLE_SEED, 'a', '\u{80}', 2, 1);
}

#[test]
#[should_panic]
fn graphic_weighted_random_char_inclusive_range_fail_3() {
    graphic_weighted_random_char_inclusive_range(EXAMPLE_SEED, '\u{80}', 'a', 1, 1);
}

#[test]
#[should_panic]
fn graphic_weighted_random_char_inclusive_range_fail_4() {
    graphic_weighted_random_char_inclusive_range(EXAMPLE_SEED, 'a', 'z', 1, 1);
}
