#[cfg(feature = "test_build")]
#[macro_export]
macro_rules! pub_test {
    ($( $body:tt )*) => {
        pub fn $( $body )*
    };
}

#[cfg(not(feature = "test_build"))]
#[macro_export]
macro_rules! pub_test {
    ($( $body:tt )*) => {
        fn $( $body )*
    };
}

#[cfg(feature = "test_build")]
#[macro_export]
macro_rules! pub_crate_test {
    ($( $body:tt )*) => {
        pub fn $( $body )*
    };
}

#[cfg(not(feature = "test_build"))]
#[macro_export]
macro_rules! pub_crate_test {
    ($( $body:tt )*) => {
        pub(crate) fn $( $body )*
    };
}

#[cfg(feature = "test_build")]
#[macro_export]
macro_rules! pub_const_test {
    ($( $body:tt )*) => {
        pub const fn $( $body )*
    };
}

#[cfg(not(feature = "test_build"))]
#[macro_export]
macro_rules! pub_const_test {
    ($( $body:tt )*) => {
        const fn $( $body )*
    };
}

#[cfg(feature = "test_build")]
#[macro_export]
macro_rules! pub_const_crate_test {
    ($( $body:tt )*) => {
        pub const fn $( $body )*
    };
}

#[cfg(not(feature = "test_build"))]
#[macro_export]
macro_rules! pub_const_crate_test {
    ($( $body:tt )*) => {
        pub(crate) const fn $( $body )*
    };
}
