use malachite_base::strings::string_sort;

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
