use integer::Integer;
use natural::Natural;

impl From<Natural> for Integer {
    /// Converts a `Natural` to an `Integer`, taking the `Natural` by value.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Integer::from(Natural::from(123u32)).to_string(), "123");
    /// assert_eq!(
    ///     Integer::from(Natural::trillion()).to_string(),
    ///     "1000000000000"
    /// );
    /// ```
    fn from(value: Natural) -> Integer {
        Integer {
            sign: true,
            abs: value,
        }
    }
}

impl<'a> From<&'a Natural> for Integer {
    /// Converts a `Natural` to an `Integer`, taking the `Natural` by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Integer::from(&Natural::from(123u32)).to_string(), "123");
    /// assert_eq!(
    ///     Integer::from(&Natural::trillion()).to_string(),
    ///     "1000000000000"
    /// );
    /// ```
    fn from(value: &'a Natural) -> Integer {
        Integer {
            sign: true,
            abs: value.clone(),
        }
    }
}
