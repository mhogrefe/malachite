// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// These mirror the macros of the same names in `malachite_base`, and are defined here rather than
// imported for a specific reason: lints that report against the span of a generated item --
// `clippy::missing_const_for_fn` among them -- are silent when the macro comes from a *different*
// crate. Defining them locally puts every generated function back in view of those lints. Only the
// macros this crate uses are defined.
//
// Each pair promotes an item to `pub` under the `test_build` feature, so tests outside the crate
// can reach internals, and keeps the narrower visibility otherwise. The names read as
// `<visibility>_test_<kind>`, where the visibility is the one the item has *without* `test_build`
// and the kind is the item emitted, so `crate_test_const_fn` is a `const fn` and `crate_test_const`
// is a `const`.

#[cfg(feature = "test_build")]
macro_rules! private_test_fn {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub fn $name $( $body )*
    };
}

#[cfg(not(feature = "test_build"))]
macro_rules! private_test_fn {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        #[allow(dead_code)]
        $( #[$meta] )*
        fn $name $( $body )*
    };
}

#[cfg(feature = "test_build")]
macro_rules! crate_test_fn {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub fn $name $( $body )*
    };
}

#[cfg(not(feature = "test_build"))]
macro_rules! crate_test_fn {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        #[allow(dead_code)]
        $( #[$meta] )*
        pub(crate) fn $name $( $body )*
    };
}
