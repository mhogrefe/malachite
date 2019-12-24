/// This trait defines the name of a type. This is useful when constructing error messages in a
/// generic function.
pub trait Named {
    /// The name of `Self`.
    const NAME: &'static str;
}

/// Implements `Named` for a type.
#[macro_export]
macro_rules! impl_named {
    ($t:ident) => {
        impl Named for $t {
            /// Returns the name of a type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// #[macro_use]
            /// extern crate malachite_base;
            ///
            /// use malachite_base::named::Named;
            ///
            /// // Fool clippy into not falsely warning clippy::needless_doctest_main
            /// fn main() -> () {
            ///     assert_eq!(u8::NAME, "u8");
            ///     assert_eq!(i64::NAME, "i64");
            /// }
            /// ```
            const NAME: &'static str = stringify!($t);
        }
    };
}
