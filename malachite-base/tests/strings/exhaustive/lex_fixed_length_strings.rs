use malachite_base::strings::exhaustive::lex_fixed_length_strings;

fn lex_fixed_length_strings_helper(len: usize, out: &[&str]) {
    let css = lex_fixed_length_strings(len).take(20).collect::<Vec<_>>();
    assert_eq!(
        css.iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .as_slice(),
        out
    );
}

#[test]
fn test_lex_fixed_length_strings() {
    lex_fixed_length_strings_helper(0, &[""]);
    lex_fixed_length_strings_helper(
        1,
        &[
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t",
        ],
    );
    lex_fixed_length_strings_helper(
        2,
        &[
            "aa", "ab", "ac", "ad", "ae", "af", "ag", "ah", "ai", "aj", "ak", "al", "am", "an",
            "ao", "ap", "aq", "ar", "as", "at",
        ],
    );
    lex_fixed_length_strings_helper(
        3,
        &[
            "aaa", "aab", "aac", "aad", "aae", "aaf", "aag", "aah", "aai", "aaj", "aak", "aal",
            "aam", "aan", "aao", "aap", "aaq", "aar", "aas", "aat",
        ],
    );
    lex_fixed_length_strings_helper(
        10,
        &[
            "aaaaaaaaaa",
            "aaaaaaaaab",
            "aaaaaaaaac",
            "aaaaaaaaad",
            "aaaaaaaaae",
            "aaaaaaaaaf",
            "aaaaaaaaag",
            "aaaaaaaaah",
            "aaaaaaaaai",
            "aaaaaaaaaj",
            "aaaaaaaaak",
            "aaaaaaaaal",
            "aaaaaaaaam",
            "aaaaaaaaan",
            "aaaaaaaaao",
            "aaaaaaaaap",
            "aaaaaaaaaq",
            "aaaaaaaaar",
            "aaaaaaaaas",
            "aaaaaaaaat",
        ],
    );
}
