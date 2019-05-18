use std::collections::HashSet;

/// Returns a `String` with the same characters and multiplicities, but in order.
///
/// Time: worst case O(n * log(n))
///
/// Additional memory: worst case O(n)
///
/// where n = `s.len()`
///
/// # Examples
/// ```
/// use malachite_base::strings::string_sort;
///
/// assert_eq!(string_sort("Hello, world!"), " !,Hdellloorw");
/// assert_eq!(string_sort("Mississippi"), "Miiiippssss");
/// ```
pub fn string_sort(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars.iter().collect()
}

/// Returns a `String` with the same characters, but each character appears no more than once. If a
/// previously-seen character is seen, it is not included in the output.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `s.len()`
///
/// # Examples
/// ```
/// use malachite_base::strings::string_nub;
///
/// assert_eq!(string_nub("Hello, world!"), "Helo, wrd!");
/// assert_eq!(string_nub("Mississippi"), "Misp");
/// ```
pub fn string_nub(s: &str) -> String {
    let mut chars = HashSet::new();
    let mut nub = String::new();
    for c in s.chars() {
        if chars.insert(c) {
            nub.push(c);
        }
    }
    nub
}

/// Returns whether all of the first `&str`'s characters are present in the second `&str`. Does not
/// take multiplicities into account.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `s.len()` + `t.len()`,
///       m = `t.len()`
///
/// # Examples
/// ```
/// use malachite_base::strings::string_is_subset;
///
/// assert_eq!(string_is_subset("o, well", "Hello, world!"), true);
/// assert_eq!(string_is_subset("MMM", "Mississippi"), true);
/// assert_eq!(string_is_subset("Hello, World!", "Hello, world!"), false);
/// assert_eq!(string_is_subset("j", "Mississippi"), false);
/// ```
pub fn string_is_subset(s: &str, t: &str) -> bool {
    let t_chars: HashSet<char> = t.chars().collect();
    s.chars().all(|c| t_chars.contains(&c))
}
