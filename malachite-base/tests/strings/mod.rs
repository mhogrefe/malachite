use malachite_base::strings::{string_is_subset, string_nub, string_sort};

#[test]
fn test_string_sort() {
    let test = |s, out| {
        assert_eq!(string_sort(s), out);
    };
    test("", "");
    test("x", "x");
    test("Hello, world!", " !,Hdellloorw");
    test("Mississippi", "Miiiippssss");
    test(
        "A quick brown fox jumps over the lazy dog",
        "        Aabcdeefghijklmnoooopqrrstuuvwxyz",
    );
}

#[test]
fn test_string_nub() {
    let test = |s, out| {
        assert_eq!(string_nub(s), out);
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
