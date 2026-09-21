// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::assert_typst_compiles;
use itertools::Itertools;
use malachite_base::foer_sequences::FoerSequence;
use malachite_base::strings::typst::ToTypst;

#[test]
fn test_foer_sequence_to_typst() {
    let test = |non_repeating: &[u8], repeating: &[u8], out: &str| {
        assert_eq!(
            FoerSequence::from_slices(non_repeating, repeating)
                .to_typst()
                .to_string(),
            out
        );
    };
    test(&[], &[], "[]");
    test(&[1, 2], &[], "[1, 2]");
    test(&[], &[3, 4], "[overline(3 comma 4)]");
    test(&[1, 2], &[3, 4], "[1, 2, overline(3 comma 4)]");
    // a one-element repeating part is the ordinary repeating-decimal notation
    test(&[1], &[2], "[1, overline(2)]");
    test(&[], &[7], "[overline(7)]");
}

#[test]
fn foer_sequence_to_typst_compiles() {
    // Typst itself has the last word: a fragment it would reject cannot pass.
    let mut frags = Vec::new();
    for non_repeating in [&[][..], &[1][..], &[1, 2][..]] {
        for repeating in [&[][..], &[3][..], &[3, 4][..]] {
            frags.push(
                FoerSequence::<u8>::from_slices(non_repeating, repeating)
                    .to_typst()
                    .to_string(),
            );
        }
    }
    assert_typst_compiles(&frags);
}

#[test]
fn test_foer_sequence_to_typst_element_types() {
    // The elements' own fragments are used, whatever they are.
    assert_eq!(
        FoerSequence::from_slices(&['a'], &['β'])
            .to_typst()
            .to_string(),
        r#"["a", overline("β")]"#
    );
    assert_eq!(
        FoerSequence::from_slices(&[true], &[false])
            .to_typst()
            .to_string(),
        r#"["T", overline("F")]"#
    );
}

#[test]
fn test_foer_sequence_to_typst_is_injective() {
    // The vinculum is what keeps a repeating part from reading as a finite one.
    let fragments = [
        FoerSequence::<u8>::from_slices(&[], &[])
            .to_typst()
            .to_string(),
        FoerSequence::<u8>::from_slices(&[1, 2], &[])
            .to_typst()
            .to_string(),
        FoerSequence::<u8>::from_slices(&[], &[1, 2])
            .to_typst()
            .to_string(),
        FoerSequence::<u8>::from_slices(&[1], &[2])
            .to_typst()
            .to_string(),
        FoerSequence::<u8>::from_slices(&[1, 2], &[3])
            .to_typst()
            .to_string(),
    ];
    assert_eq!(fragments.iter().unique().count(), fragments.len());
}
