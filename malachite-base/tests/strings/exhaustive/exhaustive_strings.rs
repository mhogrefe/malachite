use malachite_base::strings::exhaustive::exhaustive_strings;

#[test]
fn test_exhaustive_strings() {
    let ss = exhaustive_strings().take(20).collect::<Vec<_>>();
    assert_eq!(
        ss.iter().map(String::as_str).collect::<Vec<_>>().as_slice(),
        &[
            "", "a", "b", "aaa", "c", "aa", "d", "aaaa", "e", "ab", "f", "aab", "g", "ba", "h",
            "aaaaa", "i", "bb", "j", "aba"
        ],
    );
}
