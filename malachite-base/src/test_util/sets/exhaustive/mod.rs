use itertools::Itertools;
use std::collections::{BTreeSet, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

pub fn exhaustive_b_tree_sets_helper_helper<
    T: Clone + Debug + Ord,
    I: Iterator<Item = BTreeSet<T>>,
>(
    xss: I,
    out: &[BTreeSet<T>],
) {
    let xss = xss.take(20).collect_vec();
    assert_eq!(xss.into_iter().collect_vec().as_slice(), out);
}

pub fn exhaustive_b_tree_sets_small_helper_helper<
    T: Clone + Debug + Ord,
    I: Clone + Iterator<Item = BTreeSet<T>>,
>(
    xss: I,
    out_len: usize,
    out: &[BTreeSet<T>],
) {
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(xss_prefix.into_iter().collect_vec().as_slice(), out);
    assert_eq!(xss.count(), out_len);
}

pub fn exhaustive_hash_sets_helper_helper<
    T: Clone + Debug + Eq + Hash,
    I: Iterator<Item = HashSet<T>>,
>(
    xss: I,
    out: &[HashSet<T>],
) {
    let xss = xss.take(20).collect_vec();
    assert_eq!(xss.into_iter().collect_vec().as_slice(), out);
}

pub fn exhaustive_hash_sets_small_helper_helper<
    T: Clone + Debug + Eq + Hash,
    I: Clone + Iterator<Item = HashSet<T>>,
>(
    xss: I,
    out_len: usize,
    out: &[HashSet<T>],
) {
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(xss_prefix.into_iter().collect_vec().as_slice(), out);
    assert_eq!(xss.count(), out_len);
}
