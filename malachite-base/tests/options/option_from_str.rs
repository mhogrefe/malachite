// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::nevers::Never;
use malachite_base::options::{option_from_str, option_from_str_custom};
use malachite_base::orderings::ordering_from_str;
use std::cmp::Ordering::*;
use std::fmt::Debug;
use std::str::FromStr;

#[allow(clippy::needless_pass_by_value)]
fn option_from_str_helper<T: Debug + Eq + FromStr>(s: &str, out: Option<Option<T>>) {
    assert_eq!(option_from_str(s), out);
}

#[test]
fn test_option_from_str() {
    option_from_str_helper::<bool>("Some(false)", Some(Some(false)));
    option_from_str_helper::<u32>("Some(5)", Some(Some(5)));
    option_from_str_helper::<Never>("None", Some(None));
    option_from_str_helper::<u32>("Some(hi)", None);
    option_from_str_helper::<bool>("abc", None);
}

#[allow(clippy::needless_pass_by_value)]
fn option_from_str_custom_helper<T: Debug + Eq>(
    f: &dyn Fn(&str) -> Option<T>,
    s: &str,
    out: Option<Option<T>>,
) {
    assert_eq!(option_from_str_custom(f, s), out);
}

#[test]
fn test_option_from_str_custom() {
    option_from_str_custom_helper(&ordering_from_str, "Some(Less)", Some(Some(Less)));
    option_from_str_custom_helper(
        &option_from_str,
        "Some(Some(false))",
        Some(Some(Some(false))),
    );
    option_from_str_custom_helper(&option_from_str::<bool>, "Some(None)", Some(Some(None)));
    option_from_str_custom_helper(&option_from_str::<bool>, "None", Some(None));
    option_from_str_custom_helper(&ordering_from_str, "Some(hi)", None);
    option_from_str_custom_helper(&ordering_from_str, "abc", None);
}
