use integer::Integer;
use natural::Natural;
use std::cmp::Ordering;
use traits::Assign;

impl Integer {
    /// Converts an `Integer` to a `Natural`, taking the `Natural` by value. If the `Integer` is
    /// negative, `None` is returned.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(format!("{:?}", Integer::from(123).into_natural()), "Some(123)");
    /// assert_eq!(format!("{:?}", Integer::from(-123).into_natural()), "None");
    /// assert_eq!(format!("{:?}", Integer::from_str("1000000000000").unwrap().into_natural()),
    ///            "Some(1000000000000)");
    /// assert_eq!(format!("{:?}", Integer::from_str("-1000000000000").unwrap().into_natural()),
    ///            "None");
    /// ```
    pub fn into_natural(self) -> Option<Natural> {
        if self.sign() == Ordering::Less {
            None
        } else {
            let mut n = Natural::new();
            n.assign(self);
            Some(n)
        }
    }

    /// Converts an `Integer` to a `Natural`, taking the `Natural` by reference. If the `Integer` is
    /// negative, `None` is returned.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(format!("{:?}", Integer::from(123).to_natural()), "Some(123)");
    /// assert_eq!(format!("{:?}", Integer::from(-123).to_natural()), "None");
    /// assert_eq!(format!("{:?}", Integer::from_str("1000000000000").unwrap().to_natural()),
    ///            "Some(1000000000000)");
    /// assert_eq!(format!("{:?}", Integer::from_str("-1000000000000").unwrap().to_natural()),
    ///            "None");
    /// ```
    pub fn to_natural(&self) -> Option<Natural> {
        if self.sign() == Ordering::Less {
            None
        } else {
            let mut n = Natural::new();
            n.assign(self);
            Some(n)
        }
    }
}
