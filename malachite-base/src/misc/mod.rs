/// This trait defines the minimum value of a type.
pub trait Min {
    /// The minimum value of `Self`.
    const MIN: Self;
}

/// This trait defines the minimum value of a type.
pub trait Max {
    /// The maximum value of `Self`.
    const MAX: Self;
}

/// This trait defines the name of a type. This is useful when constructing error messages in a
/// generic function.
#[allow(unknown_lints, const_static_lifetime)]
pub trait Named {
    /// The name of `Self`.
    const NAME: &'static str;
}

/// Implements `Named` for a type.
#[macro_export]
macro_rules! impl_named {
    ($t: ident) => {
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
            /// use malachite_base::misc::Named;
            ///
            /// fn main() {
            ///     assert_eq!(u8::NAME, "u8");
            ///     assert_eq!(i64::NAME, "i64");
            /// }
            /// ```
            const NAME: &'static str = stringify!($t);
        }
    }
}

/// This trait defines two functions, `increment` and `decrement`, that undo each other's effect,
/// and have the property that if a < b, b is reachable from a by a finite number of increments and
/// a is reachable from b by a finite number of decrements. If the type has a maximum value,
/// incrementing it should panic; if it has a minimum value, decrementing it should panic.
pub trait Walkable: Eq + Ord {
    /// Changes `self` to the smallest value greater than its old value. Panics if no greater value
    /// exists.
    fn increment(&mut self);

    /// Changes `self` to the greatest value smaller than its old value. Panics if no smaller value
    /// exists.
    fn decrement(&mut self);
}
