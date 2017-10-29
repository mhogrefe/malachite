use integer::Integer;
use malachite_base::traits::{Assign, NegAssign};

/// Assigns an `i64` to an `Integer`.
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
///     let mut x = Integer::from(-123);
///     x.assign(-456i64);
///     assert_eq!(x.to_string(), "-456");
/// }
/// ```
impl Assign<i64> for Integer {
    fn assign(&mut self, other: i64) {
        if other >= 0 {
            self.assign(other as u64);
        } else {
            self.assign(other.wrapping_neg() as u64);
            self.neg_assign();
        }
    }
}
