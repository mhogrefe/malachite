// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::typst::ToTypst;
use malachite_base::tuples::typst::typst_tuple;
use malachite_base::typst_tuple;

#[test]
fn test_to_typst() {
    // The unit type has one value, so this assertion covers the whole domain.
    assert_eq!(().to_typst().to_string(), "()");
}

#[test]
fn test_tuple_to_typst() {
    assert_eq!((1u8,).to_typst().to_string(), "(1)");
    assert_eq!((1u8, 2u8).to_typst().to_string(), "(1, 2)");
    assert_eq!((1u8, 2u8, 3u8).to_typst().to_string(), "(1, 2, 3)");
    // every arity up to eight, which is as far as the implementations go
    assert_eq!((1u8, 2u8, 3u8, 4u8).to_typst().to_string(), "(1, 2, 3, 4)");
    assert_eq!(
        (1u8, 2u8, 3u8, 4u8, 5u8).to_typst().to_string(),
        "(1, 2, 3, 4, 5)"
    );
    assert_eq!(
        (1u8, 2u8, 3u8, 4u8, 5u8, 6u8).to_typst().to_string(),
        "(1, 2, 3, 4, 5, 6)"
    );
    assert_eq!(
        (1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8).to_typst().to_string(),
        "(1, 2, 3, 4, 5, 6, 7)"
    );
    assert_eq!(
        (1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8)
            .to_typst()
            .to_string(),
        "(1, 2, 3, 4, 5, 6, 7, 8)"
    );
}

#[test]
fn test_tuple_to_typst_element_types() {
    // The elements need not have the same type, and each writes its own fragment.
    assert_eq!(
        ('α', "hi", true).to_typst().to_string(),
        r#"("α", "hi", "T")"#
    );
    assert_eq!(
        (None::<u8>, Some(1u8), ()).to_typst().to_string(),
        "(bot, [1], ())"
    );
    // tuples nest, and hold the other collections
    assert_eq!(((1u8, 2u8), 3u8).to_typst().to_string(), "((1, 2), 3)");
    assert_eq!(
        (vec![1u8, 2], (3u8,)).to_typst().to_string(),
        "([1, 2], (3))"
    );
}

#[test]
fn test_tuple_to_typst_is_injective() {
    // Distinct tuples never share a fragment, and a tuple is never confused with a nesting of
    // shorter ones or with a sequence.
    let fragments = [
        (1u8, 2u8).to_typst().to_string(),
        (1u8, 2u8, 3u8).to_typst().to_string(),
        ((1u8, 2u8), 3u8).to_typst().to_string(),
        (1u8, (2u8, 3u8)).to_typst().to_string(),
        (1u8,).to_typst().to_string(),
        vec![1u8, 2].to_typst().to_string(),
        ().to_typst().to_string(),
    ];
    assert_eq!(fragments.iter().unique().count(), fragments.len());
}

#[test]
fn test_typst_tuple_macro() {
    // The macro agrees with the implementation for every arity that has one.
    assert_eq!(typst_tuple!(1u8), (1u8,).to_typst().to_string());
    assert_eq!(typst_tuple!(1u8, 2u8), (1u8, 2u8).to_typst().to_string());
    assert_eq!(
        typst_tuple!(1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8),
        (1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8)
            .to_typst()
            .to_string()
    );
    // and carries on past eight, where the orphan rule stops an outside crate
    assert_eq!(
        typst_tuple!(1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8),
        "(1, 2, 3, 4, 5, 6, 7, 8, 9)"
    );
    // it takes elements of different types, and a trailing comma
    assert_eq!(typst_tuple!('α', "hi", true,), r#"("α", "hi", "T")"#);
    // with no elements it gives the empty pair, which is not the unit type's fragment
    assert_eq!(typst_tuple!(), "()");
    // the function behind it takes the same elements as trait objects
    assert_eq!(typst_tuple(&[&1u8, &2u8]), "(1, 2)");
    assert_eq!(typst_tuple(&[]), "()");
}
