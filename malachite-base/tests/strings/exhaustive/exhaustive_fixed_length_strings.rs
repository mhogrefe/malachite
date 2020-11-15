use malachite_base::strings::exhaustive::exhaustive_fixed_length_strings;

fn exhaustive_fixed_length_strings_helper(len: usize, out: &[&str]) {
    let css = exhaustive_fixed_length_strings(len)
        .take(20)
        .collect::<Vec<_>>();
    assert_eq!(
        css.iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .as_slice(),
        out
    );
}

#[test]
fn test_exhaustive_fixed_length_strings() {
    exhaustive_fixed_length_strings_helper(0, &[""]);
    exhaustive_fixed_length_strings_helper(
        1,
        &[
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t",
        ],
    );
    exhaustive_fixed_length_strings_helper(
        2,
        &[
            "aa", "ab", "ba", "bb", "ac", "ad", "bc", "bd", "ca", "cb", "da", "db", "cc", "cd",
            "dc", "dd", "ae", "af", "be", "bf",
        ],
    );
    exhaustive_fixed_length_strings_helper(
        3,
        &[
            "aaa", "aab", "aba", "abb", "baa", "bab", "bba", "bbb", "aac", "aad", "abc", "abd",
            "bac", "bad", "bbc", "bbd", "aca", "acb", "ada", "adb",
        ],
    );
    exhaustive_fixed_length_strings_helper(
        10,
        &[
            "aaaaaaaaaa",
            "aaaaaaaaab",
            "aaaaaaaaba",
            "aaaaaaaabb",
            "aaaaaaabaa",
            "aaaaaaabab",
            "aaaaaaabba",
            "aaaaaaabbb",
            "aaaaaabaaa",
            "aaaaaabaab",
            "aaaaaababa",
            "aaaaaababb",
            "aaaaaabbaa",
            "aaaaaabbab",
            "aaaaaabbba",
            "aaaaaabbbb",
            "aaaaabaaaa",
            "aaaaabaaab",
            "aaaaabaaba",
            "aaaaabaabb",
        ],
    );
}
