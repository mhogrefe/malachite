use integer::Integer;
use natural::Natural;
use platform::Limb;

/// Converts a `Limb` to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(Integer::from(123).to_string(), "123");
/// ```
impl From<Limb> for Integer {
    fn from(u: Limb) -> Integer {
        Integer {
            sign: true,
            abs: Natural::from(u),
        }
    }
}
