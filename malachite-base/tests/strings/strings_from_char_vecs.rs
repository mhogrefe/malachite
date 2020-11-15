use std::iter::repeat;

use malachite_base::strings::strings_from_char_vecs;

fn strings_from_char_vecs_helper<I: Iterator<Item = Vec<char>>>(css: I, out: &[&str]) {
    let css = strings_from_char_vecs(css).take(20).collect::<Vec<_>>();
    assert_eq!(
        css.iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .as_slice(),
        out
    );
}

#[test]
fn test_strings_from_char_vecs() {
    strings_from_char_vecs_helper([].iter().cloned(), &[]);
    strings_from_char_vecs_helper([vec!['a']].iter().cloned(), &["a"]);
    strings_from_char_vecs_helper(
        [vec!['a', 'b'], vec!['c', 'd']].iter().cloned(),
        &["ab", "cd"],
    );
    strings_from_char_vecs_helper(
        repeat(vec!['c', 'a', 't']),
        &[
            "cat", "cat", "cat", "cat", "cat", "cat", "cat", "cat", "cat", "cat", "cat", "cat",
            "cat", "cat", "cat", "cat", "cat", "cat", "cat", "cat",
        ],
    );
}
