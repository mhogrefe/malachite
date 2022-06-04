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
