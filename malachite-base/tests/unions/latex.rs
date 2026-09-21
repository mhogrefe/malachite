// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::Union3;
use itertools::Itertools;
use malachite_base::strings::latex::ToLatex;
use malachite_base::unions::Union2;

#[test]
fn test_union_to_latex() {
    // The variant's letter comes first, then the wrapped value's own fragment.
    assert_eq!(
        Union2::<u8, u8>::A(2).to_latex_string(),
        r"\text{A}\left(2\right)"
    );
    assert_eq!(
        Union2::<u8, u8>::B(2).to_latex_string(),
        r"\text{B}\left(2\right)"
    );
    // the variants may hold different types
    assert_eq!(
        Union2::<char, bool>::A('α').to_latex_string(),
        r"\text{A}\left(\alpha\right)"
    );
    assert_eq!(
        Union2::<char, bool>::B(true).to_latex_string(),
        r"\text{B}\left(\text{T}\right)"
    );
    // a union defined outside this crate gets the implementation too, since the macro writes it
    assert_eq!(
        Union3::<u8, u8, u8>::C(3).to_latex_string(),
        r"\text{C}\left(3\right)"
    );
    // unions nest, and hold the other collections
    assert_eq!(
        Union2::<Union2<u8, u8>, u8>::A(Union2::B(1)).to_latex_string(),
        r"\text{A}\left(\text{B}\left(1\right)\right)"
    );
    assert_eq!(
        Union2::<Vec<u8>, u8>::A(vec![1, 2]).to_latex_string(),
        r"\text{A}\left(\left[1, 2\right]\right)"
    );
}

#[test]
fn test_union_to_latex_is_injective() {
    // The letter is what keeps two variants holding equal values apart.
    let fragments = [
        Union2::<u8, u8>::A(2).to_latex_string(),
        Union2::<u8, u8>::B(2).to_latex_string(),
        Union2::<u8, u8>::A(3).to_latex_string(),
        Union3::<u8, u8, u8>::C(2).to_latex_string(),
    ];
    assert_eq!(fragments.iter().unique().count(), fragments.len());
}
