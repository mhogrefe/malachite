use integer::Integer;
use malachite_base::misc::CheckedFrom;
use natural::Natural;

impl CheckedFrom<Integer> for Natural {
    /// Converts an `Integer` to a `Natural`, taking the `Natural` by value. If the `Integer` is
    /// negative, `None` is returned.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::misc::CheckedFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", Natural::checked_from(Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", Natural::checked_from(Integer::from(-123))), "None");
    ///     assert_eq!(format!("{:?}", Natural::checked_from(Integer::trillion())),
    ///         "Some(1000000000000)");
    ///     assert_eq!(format!("{:?}", Natural::checked_from(-Integer::trillion())), "None");
    /// }
    /// ```
    fn checked_from(value: Integer) -> Option<Natural> {
        match value {
            Integer { sign: false, .. } => None,
            Integer { sign: true, abs } => Some(abs),
        }
    }
}

impl<'a> CheckedFrom<&'a Integer> for Natural {
    /// Converts an `Integer` to a `Natural`, taking the `Natural` by reference. If the `Integer` is
    /// negative, `None` is returned.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::misc::CheckedFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", Natural::checked_from(&Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", Natural::checked_from(&Integer::from(-123))), "None");
    ///     assert_eq!(format!("{:?}", Natural::checked_from(&Integer::trillion())),
    ///         "Some(1000000000000)");
    ///     assert_eq!(format!("{:?}", Natural::checked_from(&(-Integer::trillion()))), "None");
    /// }
    /// ```
    fn checked_from(value: &'a Integer) -> Option<Natural> {
        match *value {
            Integer { sign: false, .. } => None,
            Integer {
                sign: true,
                ref abs,
            } => Some(abs.clone()),
        }
    }
}
