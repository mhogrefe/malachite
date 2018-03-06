use integer::Integer;
use malachite_base::limbs::limbs_test_zero;
use malachite_base::num::Zero;
use natural::Natural;
use std::cmp::Ordering;

impl Integer {
    /// Converts a sign and a slice of limbs, or base-2<sup>32</sup> digits, to an `Integer`. The
    /// sign is `Ordering::Greater` if the `Natural` is positive, `Ordering::Equal` if it is zero
    /// (in which case the limbs, if any, must all equal 0), and `Ordering::Less` if it is negative.
    /// The limbs are in ascending order, so that less significant limbs have lower indices in the
    /// input slice.
    ///
    /// This function borrows `limbs`. If taking ownership of `limbs` is possible,
    /// `from_sign_and_owned_limbs_asc` is more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `limbs.len()`
    ///
    /// This method is more efficient than `from_sign_and_limbs_desc`.
    ///
    /// # Panics
    /// Panics if all limbs are zero but `sign` is not `Ordering::Equal`, or if not all limbs are
    /// zero but `sign` is `Ordering::Equal`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(Integer::from_sign_and_limbs_asc(Ordering::Equal, &[]).to_string(), "0");
    /// assert_eq!(Integer::from_sign_and_limbs_asc(Ordering::Greater, &[123]).to_string(), "123");
    /// assert_eq!(Integer::from_sign_and_limbs_asc(Ordering::Less, &[123]).to_string(), "-123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(
    ///     Integer::from_sign_and_limbs_asc(Ordering::Greater, &[3567587328, 232]).to_string(),
    ///     "1000000000000");
    /// assert_eq!(
    ///     Integer::from_sign_and_limbs_asc(Ordering::Less, &[3567587328, 232]).to_string(),
    ///     "-1000000000000");
    /// ```
    pub fn from_sign_and_limbs_asc(sign: Ordering, limbs: &[u32]) -> Integer {
        assert_eq!(
            limbs_test_zero(limbs),
            sign == Ordering::Equal,
            "sign should be Equal iff limbs only contains zeros. sign: {:?}, limbs: {:?}",
            sign,
            limbs
        );
        match sign {
            Ordering::Equal => Integer::ZERO,
            Ordering::Greater => Natural::from_limbs_asc(limbs).into_integer(),
            Ordering::Less => -Natural::from_limbs_asc(limbs),
        }
    }

    /// Converts a sign and a slice of limbs, or base-2<sup>32</sup> digits, to an `Integer`. The
    /// sign is `Ordering::Greater` if the `Natural` is positive, `Ordering::Equal` if it is zero
    /// (in which case the limbs, if any, must all equal 0), and `Ordering::Less` if it is negative.
    /// The limbs are in descending order, so that less significant limbs have higher indices in the
    /// input slice.
    ///
    /// This function borrows `limbs`. If taking ownership of `limbs` is possible,
    /// `from_sign_and_owned_limbs_desc` is more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `limbs.len()`
    ///
    /// This method is less efficient than `from_sign_and_limbs_asc`.
    ///
    /// # Panics
    /// Panics if all limbs are zero but `sign` is not `Ordering::Equal`, or if not all limbs are
    /// zero but `sign` is `Ordering::Equal`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(Integer::from_sign_and_limbs_desc(Ordering::Equal, &[]).to_string(), "0");
    /// assert_eq!(Integer::from_sign_and_limbs_desc(Ordering::Greater, &[123]).to_string(), "123");
    /// assert_eq!(Integer::from_sign_and_limbs_desc(Ordering::Less, &[123]).to_string(), "-123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(
    ///     Integer::from_sign_and_limbs_desc(Ordering::Greater, &[232, 3567587328]).to_string(),
    ///     "1000000000000");
    /// assert_eq!(
    ///     Integer::from_sign_and_limbs_desc(Ordering::Less, &[232, 3567587328]).to_string(),
    ///     "-1000000000000");
    /// ```
    pub fn from_sign_and_limbs_desc(sign: Ordering, limbs: &[u32]) -> Integer {
        assert_eq!(
            limbs.iter().all(|&limb| limb == 0),
            sign == Ordering::Equal,
            "sign should be Equal iff limbs only contains zeros. sign: {:?}, limbs: {:?}",
            sign,
            limbs
        );
        match sign {
            Ordering::Equal => Integer::ZERO,
            Ordering::Greater => Natural::from_limbs_desc(limbs).into_integer(),
            Ordering::Less => -Natural::from_limbs_desc(limbs),
        }
    }

    /// Converts a sign and a slice of limbs, or base-2<sup>32</sup> digits, to an `Integer`. The
    /// sign is `Ordering::Greater` if the `Natural` is positive, `Ordering::Equal` if it is zero
    /// (in which case the limbs, if any, must all equal 0), and `Ordering::Less` if it is negative.
    /// The limbs are in ascending order, so that less significant limbs have lower indices in the
    /// input slice.
    ///
    /// This function takes ownership of `limbs`. If it's necessary to borrow `limbs` instead, use
    /// `from_sign_and_limbs_asc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `limbs.len()`
    ///
    /// This method is more efficient than `from_sign_and_owned_limbs_desc`.
    ///
    /// # Panics
    /// Panics if all limbs are zero but `sign` is not `Ordering::Equal`, or if not all limbs are
    /// zero but `sign` is `Ordering::Equal`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(Integer::from_sign_and_owned_limbs_asc(Ordering::Equal, vec![]).to_string(),
    ///     "0");
    /// assert_eq!(Integer::from_sign_and_owned_limbs_asc(Ordering::Greater, vec![123]).to_string(),
    ///     "123");
    /// assert_eq!(Integer::from_sign_and_owned_limbs_asc(Ordering::Less, vec![123]).to_string(),
    ///     "-123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(
    ///     Integer::from_sign_and_owned_limbs_asc(Ordering::Greater, vec![3567587328, 232])
    ///     .to_string(), "1000000000000");
    /// assert_eq!(
    ///     Integer::from_sign_and_owned_limbs_asc(Ordering::Less, vec![3567587328, 232])
    ///     .to_string(), "-1000000000000");
    /// ```
    pub fn from_sign_and_owned_limbs_asc(sign: Ordering, limbs: Vec<u32>) -> Integer {
        assert_eq!(
            limbs_test_zero(&limbs),
            sign == Ordering::Equal,
            "sign should be Equal iff limbs only contains zeros. sign: {:?}, limbs: {:?}",
            sign,
            limbs
        );
        match sign {
            Ordering::Equal => Integer::ZERO,
            Ordering::Greater => Natural::from_owned_limbs_asc(limbs).into_integer(),
            Ordering::Less => -Natural::from_owned_limbs_asc(limbs),
        }
    }

    /// Converts a sign and a slice of limbs, or base-2<sup>32</sup> digits, to an `Integer`. The
    /// sign is `Ordering::Greater` if the `Natural` is positive, `Ordering::Equal` if it is zero
    /// (in which case the limbs, if any, must all equal 0), and `Ordering::Less` if it is negative.
    /// The limbs are in descending order, so that less significant limbs have higher indices in the
    /// input slice.
    ///
    /// This function takes ownership of `limbs`. If it's necessary to borrow `limbs` instead, use
    /// `from_sign_and_limbs_desc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `limbs.len()`
    ///
    /// This method is less efficient than `from_sign_and_owned_limbs_asc`.
    ///
    /// # Panics
    /// Panics if all limbs are zero but `sign` is not `Ordering::Equal`, or if not all limbs are
    /// zero but `sign` is `Ordering::Equal`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!(Integer::from_sign_and_owned_limbs_desc(Ordering::Equal, vec![]).to_string(),
    ///     "0");
    /// assert_eq!(Integer::from_sign_and_owned_limbs_desc(Ordering::Greater, vec![123])
    ///     .to_string(), "123");
    /// assert_eq!(Integer::from_sign_and_owned_limbs_desc(Ordering::Less, vec![123]).to_string(),
    ///     "-123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(
    ///     Integer::from_sign_and_owned_limbs_desc(Ordering::Greater, vec![232, 3567587328])
    ///     .to_string(), "1000000000000");
    /// assert_eq!(
    ///     Integer::from_sign_and_owned_limbs_desc(Ordering::Less, vec![232, 3567587328])
    ///     .to_string(), "-1000000000000");
    /// ```
    pub fn from_sign_and_owned_limbs_desc(sign: Ordering, limbs: Vec<u32>) -> Integer {
        assert_eq!(
            limbs.iter().all(|&limb| limb == 0),
            sign == Ordering::Equal,
            "sign should be Equal iff limbs only contains zeros. sign: {:?}, limbs: {:?}",
            sign,
            limbs
        );
        match sign {
            Ordering::Equal => Integer::ZERO,
            Ordering::Greater => Natural::from_owned_limbs_desc(limbs).into_integer(),
            Ordering::Less => -Natural::from_owned_limbs_desc(limbs),
        }
    }
}
