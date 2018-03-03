use integer::Integer;

//TODO properly implement, document, and test this
impl From<Integer> for u32 {
    fn from(i: Integer) -> u32 {
        i.to_u32().expect("Oops")
    }
}

impl From<Integer> for u64 {
    fn from(i: Integer) -> u64 {
        i.to_u64().expect("Oops")
    }
}

impl Integer {
    /// Converts an `Integer` to a `u32`, returning `None` if the `Integer` is negative or too
    /// large.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(format!("{:?}", Integer::from(123).to_u32()), "Some(123)");
    /// assert_eq!(format!("{:?}", Integer::from(-123).to_u32()), "None");
    /// assert_eq!(format!("{:?}", Integer::trillion().to_u32()), "None");
    /// assert_eq!(format!("{:?}", (-Integer::trillion()).to_u32()), "None");
    /// ```
    pub fn to_u32(&self) -> Option<u32> {
        match *self {
            Integer { sign: false, .. } => None,
            Integer {
                sign: true,
                ref abs,
            } => abs.to_u32(),
        }
    }

    /// Converts an `Integer` to a `u32`, wrapping mod 2<sup>32</sup>.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(123).to_u32_wrapping().to_string(), "123");
    /// assert_eq!(Integer::from(-123).to_u32_wrapping().to_string(), "4294967173");
    /// assert_eq!(Integer::trillion().to_u32_wrapping().to_string(), "3567587328");
    /// assert_eq!((-Integer::trillion()).to_u32_wrapping().to_string(), "727379968");
    /// ```
    pub fn to_u32_wrapping(&self) -> u32 {
        match *self {
            Integer {
                sign: true,
                ref abs,
            } => abs.to_u32_wrapping(),
            Integer {
                sign: false,
                ref abs,
            } => abs.to_u32_wrapping().wrapping_neg(),
        }
    }
}
