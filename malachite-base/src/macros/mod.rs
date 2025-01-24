// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[doc(hidden)]
#[cfg(feature = "test_build")]
#[macro_export]
macro_rules! pub_test {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub fn $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(not(feature = "test_build"))]
#[macro_export]
macro_rules! pub_test {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        fn $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(feature = "test_build")]
#[macro_export]
macro_rules! pub_crate_test {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub fn $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(not(feature = "test_build"))]
#[macro_export]
macro_rules! pub_crate_test {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub(crate) fn $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(feature = "test_build")]
#[macro_export]
macro_rules! pub_const_test {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub const fn $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(not(feature = "test_build"))]
#[macro_export]
macro_rules! pub_const_test {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        const fn $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(feature = "test_build")]
#[macro_export]
macro_rules! pub_const_crate_test {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub const fn $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(not(feature = "test_build"))]
#[macro_export]
macro_rules! pub_const_crate_test {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub(crate) const fn $name $( $body )*
    };
}
