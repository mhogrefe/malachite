// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Defines the name of a type. This is useful for constructing error messages in a generic
/// function.
pub trait Named {
    /// The name of `Self`.
    const NAME: &'static str;
}

/// Automatically implements [`Named`] for a type.
///
/// It doesn't work very well for types whose names contain several tokens, like `(u8, u8)`, `&str`,
/// or `Vec<bool>`.
///
/// # Examples
/// ```
/// use malachite_base::named::Named;
///
/// assert_eq!(u8::NAME, "u8");
/// assert_eq!(String::NAME, "String");
/// ```
#[macro_export]
macro_rules! impl_named {
    ($t:ident) => {
        impl Named for $t {
            /// The name of this type, as given by the [`stringify`] macro.
            ///
            /// See the documentation for [`impl_named`] for more details.
            const NAME: &'static str = stringify!($t);
        }
    };
}
