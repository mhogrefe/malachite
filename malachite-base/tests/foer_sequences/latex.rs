// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::foer_sequences::FoerSequence;
use malachite_base::strings::latex::ToLatex;

#[test]
fn test_foer_sequence_to_latex() {
    let test = |non_repeating: &[u8], repeating: &[u8], out: &str| {
        assert_eq!(
            FoerSequence::from_slices(non_repeating, repeating).to_latex_string(),
            out
        );
    };
    test(&[], &[], r"\left[\right]");
    test(&[1, 2], &[], r"\left[1, 2\right]");
    test(&[], &[3, 4], r"\left[\overline{3, 4}\right]");
    test(&[1, 2], &[3, 4], r"\left[1, 2, \overline{3, 4}\right]");
    // a one-element repeating part is the ordinary repeating-decimal notation
    test(&[1], &[2], r"\left[1, \overline{2}\right]");
    test(&[], &[7], r"\left[\overline{7}\right]");
}

#[test]
fn test_foer_sequence_to_latex_element_types() {
    // The elements' own fragments are used, whatever they are.
    assert_eq!(
        FoerSequence::from_slices(&['a'], &['β']).to_latex_string(),
        r"\left[\text{a}, \overline{\beta}\right]"
    );
    assert_eq!(
        FoerSequence::from_slices(&[true], &[false]).to_latex_string(),
        r"\left[\text{T}, \overline{\text{F}}\right]"
    );
}

#[test]
fn test_foer_sequence_to_latex_is_injective() {
    // The vinculum is what keeps a repeating part from reading as a finite one.
    let fragments = [
        FoerSequence::<u8>::from_slices(&[], &[]).to_latex_string(),
        FoerSequence::<u8>::from_slices(&[1, 2], &[]).to_latex_string(),
        FoerSequence::<u8>::from_slices(&[], &[1, 2]).to_latex_string(),
        FoerSequence::<u8>::from_slices(&[1], &[2]).to_latex_string(),
        FoerSequence::<u8>::from_slices(&[1, 2], &[3]).to_latex_string(),
    ];
    assert_eq!(fragments.iter().unique().count(), fragments.len());
}
