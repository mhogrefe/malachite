use malachite_base::strings::exhaustive::shortlex_strings;

#[test]
fn test_shortlex_strings() {
    let ss = shortlex_strings().take(20).collect::<Vec<_>>();
    assert_eq!(
        ss.iter().map(String::as_str).collect::<Vec<_>>().as_slice(),
        &[
            "", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
            "q", "r", "s",
        ]
    );
}
