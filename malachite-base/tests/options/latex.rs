// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::chars::exhaustive::exhaustive_chars;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::options::exhaustive::exhaustive_options;
use malachite_base::strings::latex::ToLatex;
use malachite_base::test_util::generators::option_unsigned_gen;

#[test]
fn test_option_to_latex() {
    assert_eq!(None::<u8>.to_latex().to_string(), r"\bot");
    assert_eq!(None::<char>.to_latex().to_string(), r"\bot");
    assert_eq!(None::<&str>.to_latex().to_string(), r"\bot");
    assert_eq!(Some(0u8).to_latex().to_string(), r"\left[0\right]");
    assert_eq!(Some(5u8).to_latex().to_string(), r"\left[5\right]");
    assert_eq!(Some(-5i32).to_latex().to_string(), r"\left[-5\right]");
    assert_eq!(Some('α').to_latex().to_string(), r"\left[\alpha\right]");
    assert_eq!(Some("hi").to_latex().to_string(), r"\left[\text{hi}\right]");
    assert_eq!(Some(true).to_latex().to_string(), r"\left[\text{T}\right]");
    assert_eq!(Some(()).to_latex().to_string(), r"\left[()\right]");
    // nesting
    assert_eq!(
        Some(None::<u8>).to_latex().to_string(),
        r"\left[\bot\right]"
    );
    assert_eq!(
        Some(Some(5u8)).to_latex().to_string(),
        r"\left[\left[5\right]\right]"
    );
    assert_eq!(
        Some(Some(None::<u8>)).to_latex().to_string(),
        r"\left[\left[\bot\right]\right]"
    );
}

#[test]
fn test_option_to_latex_is_injective_over_nesting() {
    // The brackets exist so that distinct `Option`s never share a fragment. Without them every one
    // of these would collapse onto the same string.
    let fragments = [
        None::<Option<Option<u8>>>.to_latex().to_string(),
        Some(None::<Option<u8>>).to_latex().to_string(),
        Some(Some(None::<u8>)).to_latex().to_string(),
        Some(Some(Some(0u8))).to_latex().to_string(),
    ];
    assert_eq!(fragments.iter().unique().count(), fragments.len());
}

// Every fragment is either `\bot` or a `\left[...\right]`-bracketed copy of the inner value's
// fragment.
fn check<T: ToLatex>(o: &Option<T>) {
    let s = o.to_latex().to_string();
    match o {
        None => assert_eq!(s, r"\bot"),
        Some(x) => {
            let inner = x.to_latex().to_string();
            assert_eq!(s, format!("\\left[{inner}\\right]"));
            assert_ne!(s, inner);
            assert_ne!(s, r"\bot");
        }
    }
}

#[test]
fn option_to_latex_properties() {
    fn helper<T: PrimitiveUnsigned + ToLatex>() {
        option_unsigned_gen::<T>().test_properties(|o| check(&o));
    }
    apply_fn_to_unsigneds!(helper);

    // `Option<Option<_>>` and a non-numeric element type have no generator of their own, and a
    // missing bracket would show up in the nested case, so they are covered directly.
    for o in exhaustive_options(exhaustive_options(exhaustive_unsigneds::<u8>())).take(300) {
        check(&o);
    }
    for o in exhaustive_options(exhaustive_chars()).take(300) {
        check(&o);
    }
}
