use integer::Integer;

impl Integer {
    /// Converts an `Integer` to an `i32`, returning `None` if the `Integer` is outside the range of
    /// an `i32`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(format!("{:?}", Integer::from(123).to_i32()), "Some(123)");
    /// assert_eq!(format!("{:?}", Integer::from(-123).to_i32()), "Some(-123)");
    /// assert_eq!(format!("{:?}", Integer::trillion().to_i32()), "None");
    /// assert_eq!(format!("{:?}", (-Integer::trillion()).to_i32()), "None");
    /// ```
    pub fn to_i32(&self) -> Option<i32> {
        if *self >= i32::min_value() && *self <= i32::max_value() {
            Some(self.to_i32_wrapping())
        } else {
            None
        }
    }

    /// Converts an `Integer` to a `i32`, wrapping mod 2^(32).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(123).to_i32_wrapping().to_string(), "123");
    /// assert_eq!(Integer::from(-123).to_i32_wrapping().to_string(), "-123");
    /// assert_eq!(Integer::trillion().to_i32_wrapping().to_string(), "-727379968");
    /// assert_eq!((-Integer::trillion()).to_i32_wrapping().to_string(), "727379968");
    /// ```
    pub fn to_i32_wrapping(&self) -> i32 {
        self.to_u32_wrapping() as i32
    }
}
