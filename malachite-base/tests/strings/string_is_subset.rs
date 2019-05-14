use malachite_base::strings::string_is_subset;

#[test]
fn test_string_is_subset() {
    let test = |s, t, out| {
        assert_eq!(string_is_subset(s, t), out);
    };
    test("", "Hello, world!", true);
    test("o, well", "Hello, world!", true);
    test("MMM", "Mississippi", true);
    test("Hello, World!", "Hello, world!", false);
    test("j", "Mississippi", false);
    test(
        "abcdefghijklmnopqrstuvwxyz",
        "A quick brown fox jumps over the lazy dog",
        true,
    );
}
