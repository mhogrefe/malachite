/// This trait defines the name of a type. This is useful when constructing error messages in a
/// generic function.
pub trait Named {
    /// The name of `Self`.
    const NAME: &'static str;
}

/// The name of a type, as given by the `stringify` macro.
///
/// It doesn't work very well for types whose names contain several tokens, like `(u8, u8)`, `&str`,
/// or `Vec<bool>`.
///
/// # Example
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::named::Named;
///
/// # // The `-> ()` is used to fool clippy into not falsely warning
/// # // `clippy::needless_doctest_main`.
/// fn main() -> () {
///     assert_eq!(u8::NAME, "u8");
///     assert_eq!(String::NAME, "String");
/// }
/// ```
#[macro_export]
macro_rules! impl_named {
    ($t:ident) => {
        impl Named for $t {
            /// The name of this type, as given by the `stringify` macro.
            ///
            /// See the documentation for the `impl_named` macro for more details.
            const NAME: &'static str = stringify!($t);
        }
    };
}
