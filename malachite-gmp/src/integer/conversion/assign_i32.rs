use integer::Integer::{self, Small};
use malachite_base::traits::Assign;

/// Assigns an `i32` to an `Integer`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Assign;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(456);
///     x.assign(-123);
///     assert_eq!(x.to_string(), "-123");
/// }
/// ```
impl Assign<i32> for Integer {
    fn assign(&mut self, other: i32) {
        *self = Small(other);
    }
}
