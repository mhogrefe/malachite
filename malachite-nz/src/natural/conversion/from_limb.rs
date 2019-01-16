use natural::Natural::{self, Small};
use platform::Limb;

/// Converts a `Limb` to a `Natural`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(Natural::from(123u32).to_string(), "123");
/// ```
impl From<Limb> for Natural {
    fn from(u: Limb) -> Natural {
        Small(u)
    }
}

//TODO test
#[cfg(feature = "64_bit_limbs")]
impl From<u32> for Natural {
    fn from(u: u32) -> Natural {
        Small(u64::from(u))
    }
}
