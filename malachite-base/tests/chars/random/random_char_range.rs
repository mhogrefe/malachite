// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::chars::random::random_char_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

fn random_char_range_helper(
    a: char,
    b: char,
    expected_values: &str,
    expected_common_values: &[(char, usize)],
    expected_median: (char, Option<char>),
) {
    let xs = random_char_range(EXAMPLE_SEED, a, b);
    let values = xs.clone().take(200).collect::<String>();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_str(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_char_range() {
    random_char_range_helper(
        'a',
        'b',
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
        aaaaaaaaaaaaaaaaaaa",
        &[('a', 1000000)],
        ('a', None),
    );
    random_char_range_helper(
        'a',
        'd',
        "babcbbabacbabccabacccbaccbbbaacaccbabbaccacabcbbccbbbacabbbcabbcbbbcbcbbcabbbcabcbaacabbbc\
        bccccccbacabaacbcababbaabcacabbbabaaacbccccbccabbaaacabaabccacabccccabbcacccaaccaccaccbbbca\
        abcbaabcccbccbbbabc",
        &[('b', 333784), ('c', 333516), ('a', 332700)],
        ('b', None),
    );
    random_char_range_helper(
        'a',
        'z',
        "rlewrsgkdlbeouylrelopxqkoonftexoshqulgvonioatekqesxybqjsrbsajhpvpcxcuyfkcsrexwwcjymyxhjpxk\
        byhasvvkwucaopwwpxyncdkvllxbdavqlxlsaxglyhrulnqvqdipeklnnbgdhejggrhwgjjmmpndcjsvlqypdqqodas\
        mavfvcmxcvoopcrcuba",
        &[
            ('x', 40444),
            ('q', 40300),
            ('w', 40244),
            ('s', 40201),
            ('f', 40174),
            ('d', 40171),
            ('n', 40117),
            ('j', 40111),
            ('a', 40107),
            ('c', 40076),
        ],
        ('m', None),
    );
    random_char_range_helper(
        '!',
        '9',
        "2,%723\'+$,\"%/5,2%,/081+//.&4%8/3(15,\'6/.)/!4%+1%38\"1*32\"3!*(060#8#5&+#32%877#*-8(*08+\
        \"(!366+75#!/07708.#$+6,,8\"$!61,8,3!8\',(25,.161$)0%+,..\"\'$(%*\'\'2(7\'**--0.$#*36,10$11\
        /$!3-!6&6#-8#6//0#2#5\"!#8/)%238/",
        &[
            ('8', 42093),
            ('1', 42031),
            ('$', 41908),
            ('3', 41887),
            ('&', 41836),
            ('7', 41830),
            ('!', 41753),
            ('.', 41753),
            ('*', 41699),
            ('+', 41673),
        ],
        ('-', None),
    );
    random_char_range_helper(
        '\u{D7FF}',
        '\u{E001}',
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
fn random_char_range_fail() {
    random_char_range(EXAMPLE_SEED, 'a', 'a');
}
