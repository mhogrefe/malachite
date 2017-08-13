use integer::Integer;
use natural::Natural;
use std::cmp::Ordering;

impl Integer {
    /// Converts a sign and a slice of limbs, or base-2^(32) digits, to an `Integer`. The sign is
    /// `Ordering::Greater` if the `Natural` is positive, `Ordering::Equal` if it is zero (in which
    /// case the limbs, if any, must all equal 0), and `Ordering::Less` if it is negative. The limbs
    /// are in little-endian order, so that less significant limbs have lower indices in the input
    /// slice. Although GMP may use 32- or 64-bit limbs internally, this method always takes 32-bit
    /// limbs.
    ///
    /// This method is more efficient than `from_sign_and_limbs_be`.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(Integer::from_sign_and_limbs_le(Ordering::Equal, &[]).to_string(), "0");
    /// assert_eq!(Integer::from_sign_and_limbs_le(Ordering::Greater, &[123]).to_string(), "123");
    /// assert_eq!(Integer::from_sign_and_limbs_le(Ordering::Less, &[123]).to_string(), "-123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(
    ///     Integer::from_sign_and_limbs_le(Ordering::Greater, &[3567587328, 232]).to_string(),
    ///     "1000000000000");
    /// assert_eq!(
    ///     Integer::from_sign_and_limbs_le(Ordering::Less, &[3567587328, 232]).to_string(),
    ///     "-1000000000000");
    /// ```
    pub fn from_sign_and_limbs_le(sign: Ordering, limbs: &[u32]) -> Integer {
        assert_eq!(
            limbs.iter().all(|&limb| limb == 0),
            sign == Ordering::Equal,
            "sign should be Equal iff limbs only contains zeros. sign: {:?}, limbs: {:?}",
            sign,
            limbs
        );
        match sign {
            Ordering::Equal => Integer::from(0),
            Ordering::Greater => Natural::from_limbs_le(limbs).into_integer(),
            Ordering::Less => -Natural::from_limbs_le(limbs),
        }
    }

    /// Converts a sign and a slice of limbs, or base-2^(32) digits, to an `Integer`. The sign is
    /// `Ordering::Greater` if the `Natural` is positive, `Ordering::Equal` if it is zero (in which
    /// case the limbs, if any, must all equal 0), and `Ordering::Less` if it is negative. The limbs
    /// are in big-endian order, so that less significant limbs have higher indices in the input
    /// slice. Although GMP may use 32- or 64-bit limbs internally, this method always takes 32-bit
    /// limbs.
    ///
    /// This method is less efficient than `from_sign_and_limbs_le`.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(Integer::from_sign_and_limbs_be(Ordering::Equal, &[]).to_string(), "0");
    /// assert_eq!(Integer::from_sign_and_limbs_be(Ordering::Greater, &[123]).to_string(), "123");
    /// assert_eq!(Integer::from_sign_and_limbs_be(Ordering::Less, &[123]).to_string(), "-123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(
    ///     Integer::from_sign_and_limbs_be(Ordering::Greater, &[232, 3567587328]).to_string(),
    ///     "1000000000000");
    /// assert_eq!(
    ///     Integer::from_sign_and_limbs_be(Ordering::Less, &[232, 3567587328]).to_string(),
    ///     "-1000000000000");
    /// ```
    pub fn from_sign_and_limbs_be(sign: Ordering, limbs: &[u32]) -> Integer {
        assert_eq!(
            limbs.iter().all(|&limb| limb == 0),
            sign == Ordering::Equal,
            "sign should be Equal iff limbs only contains zeros. sign: {:?}, limbs: {:?}",
            sign,
            limbs
        );
        match sign {
            Ordering::Equal => Integer::from(0),
            Ordering::Greater => Natural::from_limbs_be(limbs).into_integer(),
            Ordering::Less => -Natural::from_limbs_be(limbs),
        }
    }
}