use integer::Integer;
use traits::Assign;

/// Assigns a `u32` to `self`.
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::traits::Assign;
///
/// let mut x = Integer::from(-123);
/// x.assign(456);
/// assert_eq!(x.to_string(), "456");
/// ```
impl Assign<u32> for Integer {
    fn assign(&mut self, other: u32) {
        self.sign = true;
        self.abs.assign(other);
    }
}
