use integer::Integer;
use natural::Natural;
use platform::Limb;

impl From<Limb> for Integer {
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
    fn from(u: Limb) -> Integer {
        Integer {
            sign: true,
            abs: Natural::from(u),
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl From<u32> for Integer {
    #[inline]
    fn from(u: u32) -> Integer {
        Integer::from(Limb::from(u))
    }
}
