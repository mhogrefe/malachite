use integer::Integer;
use traits::Assign;

/// Assigns an `i32` to `self`.
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::traits::Assign;
///
/// let mut x = Integer::from(456);
/// x.assign(-123);
/// assert_eq!(x.to_string(), "-123");
/// ```
impl Assign<i32> for Integer {
    fn assign(&mut self, other: i32) {
        self.sign = other >= 0;
        self.abs.assign(other.abs() as u32);
    }
}
