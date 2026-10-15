// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use itertools::Itertools;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// The rows of a matrix with at least one row and one column, each as the strings `f` makes of its
// elements, joined by `element_separator`.
fn rows(
    m: &UnsignedMatrix<u64>,
    element_separator: &str,
    f: impl Fn(&u64) -> String,
) -> Vec<String> {
    m.elements
        .chunks(m.width())
        .map(|row| row.iter().map(&f).join(element_separator))
        .collect()
}

#[test]
fn test_to_typst_string() {
    let test = |s, out: &str| {
        assert_eq!(
            UnsignedMatrix::<u64>::from_str(s)
                .unwrap()
                .to_typst_string(),
            out
        );
    };
    test("[]_{0,0}", "mat()_(0 times 0)");
    test("[]_{0,4}", "mat()_(0 times 4)");
    test("[]_{4,0}", "mat()_(4 times 0)");
    test("[5]", "mat(5)");
    test("[1, 2, 3]", "mat(1, 2, 3)");
    test("[1; 2; 3]", "mat(1; 2; 3)");
    test("[1, 2; 3, 4]", "mat(1, 2; 3, 4)");
    test(
        "[18446744073709551615, 10]",
        "mat(18446744073709551615, 10)",
    );
}

#[test]
fn to_typst_string_properties() {
    unsigned_matrix_gen().test_properties(|m| {
        let typst = m.to_typst_string();
        if m.height() == 0 || m.width() == 0 {
            assert_eq!(typst, format!("mat()_({} times {})", m.height(), m.width()));
        } else {
            assert_eq!(
                typst,
                format!("mat({})", rows(&m, ", ", u64::to_string).join("; "))
            );
            // A `u64`'s fragment is its decimal digits, so the fragment is the `Display` output
            // with `mat(...)` in place of the brackets.
            let s = m.to_string();
            assert_eq!(typst, format!("mat({})", &s[1..s.len() - 1]));
        }
    });
}
