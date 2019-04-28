use std::collections::HashSet;

pub fn string_sort(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars.iter().collect()
}

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

pub fn string_is_subset(s: &str, t: &str) -> bool {
    let s_chars: HashSet<char> = s.chars().collect();
    let t_chars: HashSet<char> = t.chars().collect();
    s_chars.is_subset(&t_chars)
}
