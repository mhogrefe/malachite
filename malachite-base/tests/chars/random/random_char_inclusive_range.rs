// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::chars::random::random_char_inclusive_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

fn random_char_inclusive_range_helper(
    a: char,
    b: char,
    expected_values: &str,
    expected_common_values: &[(char, usize)],
    expected_median: (char, Option<char>),
) {
    let xs = random_char_inclusive_range(EXAMPLE_SEED, a, b);
    let values = xs.clone().take(200).collect::<String>();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_str(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_char_inclusive_range() {
    random_char_inclusive_range_helper(
        'a',
        'a',
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaa",
        &[('a', 1000000)],
        ('a', None),
    );
    random_char_inclusive_range_helper(
        'a',
        'c',
        "babcbbabacbabccabacccbaccbbbaacaccbabbaccacabcbbccbbbacabbbcabbcbbbcbcbbcabbbcabcbaacabbbc\
        bccccccbacabaacbcababbaabcacabbbabaaacbccccbccabbaaacabaabccacabccccabbcacccaaccaccaccbbbca\
        abcbaabcccbccbbbabc",
        &[('b', 333784), ('c', 333516), ('a', 332700)],
        ('b', None),
    );
    random_char_inclusive_range_helper(
        'a',
        'z',
        "rlewrsgkdlbeouylrelopxqkoonftexoshqulgvonioatekqesxybqjsrbsajhpzzvzpcxcuyfkcsrexwwcjymyxhj\
        pxkbyhasvvkwucaopwwpxyncdkvllxbdavqlxlsaxglzyhrulnqvqdipeklnnbgdhejggrhwgjjmmpndcjsvlqypdqq\
        odasmavfvcmxcvoopcr",
        &[
            ('x', 38904),
            ('q', 38795),
            ('s', 38687),
            ('w', 38660),
            ('d', 38655),
            ('f', 38647),
            ('a', 38592),
            ('n', 38569),
            ('j', 38531),
            ('z', 38530),
        ],
        ('n', None),
    );
    random_char_inclusive_range_helper(
        '!',
        '9',
        "2,%723\'+$,\"%/59,2%,/081+//.&4%8/3(15,\'6/.)/!4%+1%389\"1*32\"3!*(060#8#59&+#32%877#*9-98\
        (*08+\"9(!366+75#!/077089.#$+6,,8\"$!61,8,3!8\',9(25,.161$)0%+,..\"\'$(%*\'\'2(7\'**--0.$#*\
        36,190$11/$!3-!6&6#-8#6//0#2#5\"!",
        &[
            ('8', 40444),
            ('1', 40300),
            ('7', 40244),
            ('3', 40201),
            ('&', 40174),
            ('$', 40171),
            ('.', 40117),
            ('*', 40111),
            ('!', 40107),
            ('#', 40076),
        ],
        ('-', None),
    );
    random_char_inclusive_range_helper(
        '\u{D7FF}',
        '\u{E000}',
        "\u{e000}\u{d7ff}\u{d7ff}\u{d7ff}\u{e000}\u{e000}\u{e000}\u{d7ff}\u{e000}\u{e000}\u{e000}\
        \u{e000}\u{d7ff}\u{e000}\u{e000}\u{e000}\u{e000}\u{d7ff}\u{e000}\u{d7ff}\u{d7ff}\u{d7ff}\
        \u{e000}\u{d7ff}\u{d7ff}\u{d7ff}\u{e000}\u{e000}\u{d7ff}\u{e000}\u{e000}\u{d7ff}\u{d7ff}\
        \u{d7ff}\u{e000}\u{d7ff}\u{d7ff}\u{e000}\u{e000}\u{e000}\u{d7ff}\u{e000}\u{d7ff}\u{d7ff}\
        \u{e000}\u{d7ff}\u{e000}\u{e000}\u{d7ff}\u{d7ff}\u{d7ff}\u{e000}\u{d7ff}\u{e000}\u{d7ff}\
        \u{e000}\u{e000}\u{d7ff}\u{d7ff}\u{d7ff}\u{d7ff}\u{e000}\u{d7ff}\u{e000}\u{e000}\u{e000}\
        \u{e000}\u{d7ff}\u{e000}\u{d7ff}\u{e000}\u{d7ff}\u{d7ff}\u{d7ff}\u{d7ff}\u{d7ff}\u{d7ff}\
        \u{e000}\u{d7ff}\u{d7ff}\u{d7ff}\u{e000}\u{e000}\u{e000}\u{d7ff}\u{e000}\u{e000}\u{d7ff}\
        \u{e000}\u{e000}\u{d7ff}\u{d7ff}\u{e000}\u{d7ff}\u{e000}\u{d7ff}\u{d7ff}\u{d7ff}\u{e000}\
        \u{e000}\u{e000}\u{e000}\u{d7ff}\u{e000}\u{d7ff}\u{e000}\u{d7ff}\u{d7ff}\u{d7ff}\u{e000}\
        \u{d7ff}\u{d7ff}\u{e000}\u{d7ff}\u{d7ff}\u{e000}\u{e000}\u{d7ff}\u{e000}\u{d7ff}\u{d7ff}\
        \u{e000}\u{e000}\u{e000}\u{d7ff}\u{e000}\u{e000}\u{e000}\u{e000}\u{d7ff}\u{e000}\u{e000}\
        \u{e000}\u{d7ff}\u{e000}\u{d7ff}\u{d7ff}\u{d7ff}\u{d7ff}\u{e000}\u{d7ff}\u{d7ff}\u{e000}\
        \u{e000}\u{e000}\u{d7ff}\u{e000}\u{d7ff}\u{e000}\u{d7ff}\u{d7ff}\u{e000}\u{e000}\u{e000}\
        \u{d7ff}\u{d7ff}\u{e000}\u{e000}\u{e000}\u{d7ff}\u{e000}\u{d7ff}\u{e000}\u{e000}\u{e000}\
        \u{e000}\u{d7ff}\u{e000}\u{e000}\u{d7ff}\u{e000}\u{d7ff}\u{e000}\u{d7ff}\u{d7ff}\u{e000}\
        \u{e000}\u{d7ff}\u{d7ff}\u{e000}\u{e000}\u{d7ff}\u{e000}\u{e000}\u{e000}\u{d7ff}\u{d7ff}\
        \u{e000}\u{d7ff}\u{d7ff}\u{e000}\u{e000}\u{e000}\u{d7ff}\u{e000}\u{d7ff}\u{e000}\u{e000}\
        \u{e000}\u{d7ff}",
        &[('\u{e000}', 500473), ('\u{d7ff}', 499527)],
        ('\u{e000}', None),
    );
}

#[test]
#[should_panic]
fn random_char_inclusive_range_fail() {
    random_char_inclusive_range(EXAMPLE_SEED, 'b', 'a');
}
