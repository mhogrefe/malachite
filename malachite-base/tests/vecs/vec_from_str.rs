// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::nevers::Never;
use malachite_base::options::option_from_str;
use malachite_base::orderings::ordering_from_str;
use malachite_base::vecs::{vec_from_str, vec_from_str_custom};
use std::cmp::Ordering::*;
use std::fmt::Debug;
use std::str::FromStr;

#[allow(clippy::needless_pass_by_value)]
fn vec_from_str_helper<T: Debug + Eq + FromStr>(s: &str, out: Option<Vec<T>>) {
    assert_eq!(vec_from_str(s), out);
}

#[test]
fn test_vec_from_str() {
    vec_from_str_helper::<Never>("[]", Some(vec![]));
    vec_from_str_helper::<u32>("[5]", Some(vec![5]));
    vec_from_str_helper::<u32>("[5, 6, 7]", Some(vec![5, 6, 7]));
    vec_from_str_helper::<bool>("[false, false, true]", Some(vec![false, false, true]));
    vec_from_str_helper::<String>("[a, b]", Some(vec!["a".to_string(), "b".to_string()]));
    vec_from_str_helper::<String>("[a,  b]", Some(vec!["a".to_string(), " b".to_string()]));
    vec_from_str_helper::<String>("[a , b]", Some(vec!["a ".to_string(), "b".to_string()]));
    vec_from_str_helper::<String>("[a,, b]", Some(vec!["a,".to_string(), "b".to_string()]));
    vec_from_str_helper::<String>("[a ,,b]", Some(vec!["a ,,b".to_string()]));
    vec_from_str_helper::<String>("[a,b]", Some(vec!["a,b".to_string()]));
    vec_from_str_helper::<bool>("[", None);
    vec_from_str_helper::<bool>("", None);
    vec_from_str_helper::<bool>("abc", None);
    vec_from_str_helper::<bool>("[false, false, true", None);
    vec_from_str_helper::<bool>("[false, false,  true]", None);
    vec_from_str_helper::<bool>("[false, false, true,]", None);
    vec_from_str_helper::<bool>("[false, false, true] ", None);
    vec_from_str_helper::<bool>("[false, false, true ]", None);
    vec_from_str_helper::<bool>("[false, false, rue]", None);
}

#[allow(clippy::needless_pass_by_value)]
fn vec_from_str_custom_helper<T: Debug + Eq>(
    f: &dyn Fn(&str) -> Option<T>,
    s: &str,
    out: Option<Vec<T>>,
) {
    assert_eq!(vec_from_str_custom(f, s), out);
}

#[test]
fn test_vec_from_str_custom() {
    vec_from_str_custom_helper(
        &ordering_from_str,
        "[Less, Greater]",
        Some(vec![Less, Greater]),
    );
    vec_from_str_custom_helper(
        &option_from_str,
        "[Some(false), None]",
        Some(vec![Some(false), None]),
    );
    vec_from_str_custom_helper(
        &vec_from_str,
        "[[], [3], [2, 5]]",
        Some(vec![vec![], vec![3], vec![2, 5]]),
    );
    vec_from_str_custom_helper(&option_from_str::<bool>, "[Some(fals), None]", None);
    vec_from_str_custom_helper(&vec_from_str::<u32>, "[[], [3], [2, 5,]]", None);
    vec_from_str_custom_helper(&vec_from_str::<u32>, "[[], 3, [2, 5]]", None);
    vec_from_str_custom_helper(&vec_from_str::<u32>, "[[], [3], [true]]", None);
}
