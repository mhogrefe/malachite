use malachite_base::strings::string_unique;

#[test]
fn test_string_unique() {
    let test = |s, out| {
        assert_eq!(string_unique(s), out);
    };
    test("", "");
    test("x", "x");
    test("xxxxxxxxx", "x");
    test("Hello, world!", "Helo, wrd!");
    test("Mississippi", "Misp");
    test(
        "A quick brown fox jumps over the lazy dog",
        "A quickbrownfxjmpsvethlazydg",
    );
}
