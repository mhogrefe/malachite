use integer::Integer;
use std::cmp::Ordering;

impl Integer {
    /// Returns the sign and limbs, or base-2<sup>32</sup> digits, of a `Natural`. The sign is
    /// `Ordering::Greater` if the `Natural` is positive, `Ordering::Equal` if it is zero (in which
    /// case the limbs are empty), and `Ordering::Less` if it is negative. The limbs are the limbs
    /// of the `Natural`'s absolute value. They are in ascending order, so that less significant
    /// limbs have lower indices in the output vector.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// This method is more efficient than `Natural::sign_and_limbs_desc`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.sign_and_limbs_asc(), (Ordering::Equal, vec![]));
    ///     assert_eq!(Integer::from(123).sign_and_limbs_asc(), (Ordering::Greater, vec![123]));
    ///     assert_eq!(Integer::from(-123).sign_and_limbs_asc(), (Ordering::Less, vec![123]));
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().sign_and_limbs_asc(),
    ///         (Ordering::Greater, vec![3567587328, 232]));
    ///     assert_eq!((-Integer::trillion()).sign_and_limbs_asc(),
    ///         (Ordering::Less, vec![3567587328, 232]));
    /// }
    /// ```
    pub fn sign_and_limbs_asc(&self) -> (Ordering, Vec<u32>) {
        (self.sign(), self.abs.to_limbs_asc())
    }

    /// Returns the sign and limbs, or base-2<sup>32</sup> digits, of a `Natural`. The sign is
    /// `Ordering::Greater` if the `Natural` is positive, `Ordering::Equal` if it is zero (in which
    /// case the limbs are empty), and `Ordering::Less` if it is negative. The limbs are the limbs
    /// of the `Natural`'s absolute value. They are in descending order, so that less significant
    /// limbs have higher indices in the output vector.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// This method is less efficient than `Natural::sign_and_limbs_asc`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.sign_and_limbs_desc(), (Ordering::Equal, vec![]));
    ///     assert_eq!(Integer::from(123).sign_and_limbs_desc(), (Ordering::Greater, vec![123]));
    ///     assert_eq!(Integer::from(-123).sign_and_limbs_desc(), (Ordering::Less, vec![123]));
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().sign_and_limbs_desc(),
    ///         (Ordering::Greater, vec![232, 3567587328]));
    ///     assert_eq!((-Integer::trillion()).sign_and_limbs_desc(),
    ///         (Ordering::Less, vec![232, 3567587328]));
    /// }
    /// ```
    pub fn sign_and_limbs_desc(&self) -> (Ordering, Vec<u32>) {
        (self.sign(), self.abs.to_limbs_desc())
    }
}
