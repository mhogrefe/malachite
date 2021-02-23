/// A struct that allows for formatting a numeric type and rendering its digits in a specified base.
pub struct BaseFmtWrapper<T> {
    x: T,
    base: u64,
}

impl<T> BaseFmtWrapper<T> {
    /// Creates a new `BaseFmtWrapper`.
    ///
    /// # Worst-case complexity
    ///
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::string::BaseFmtWrapper;
    ///
    /// let x = BaseFmtWrapper::new(1000000000u32, 36);
    /// assert_eq!(format!("{}", x), "gjdgxs");
    /// assert_eq!(format!("{:#}", x), "GJDGXS");
    /// ```
    pub fn new(x: T, base: u64) -> Self {
        assert!((2..=36).contains(&base), "base out of range");
        BaseFmtWrapper { x, base }
    }

    /// Recovers the value from a `BaseFmtWrapper`.
    ///
    /// # Worst-case complexity
    ///
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::string::BaseFmtWrapper;
    ///
    /// assert_eq!(BaseFmtWrapper::new(1000000000u32, 36).unwrap(), 1000000000);
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn unwrap(self) -> T {
        self.x
    }
}

/// This module contains trait implementations for converting numbers to strings.
///
/// Here are usage examples of the macro-generated functions:
///
/// # Display::fmt for BaseFmtWrapper
/// ```
/// use malachite_base::num::conversion::string::BaseFmtWrapper;
///
/// let x = BaseFmtWrapper::new(1000000000u32, 36);
/// assert_eq!(format!("{}", x), "gjdgxs");
/// assert_eq!(format!("{:#}", x), "GJDGXS");
/// assert_eq!(format!("{:010}", x), "0000gjdgxs");
/// assert_eq!(format!("{:#010}", x), "0000GJDGXS");
///
/// let x = BaseFmtWrapper::new(-1000000000i32, 36);
/// assert_eq!(format!("{}", x), "-gjdgxs");
/// assert_eq!(format!("{:#}", x), "-GJDGXS");
/// assert_eq!(format!("{:010}", x), "-000gjdgxs");
/// assert_eq!(format!("{:#010}", x), "-000GJDGXS");
/// ```
///
/// # Debug::fmt for BaseFmtWrapper
/// ```
/// use malachite_base::num::conversion::string::BaseFmtWrapper;
///
/// let x = BaseFmtWrapper::new(1000000000u32, 36);
/// assert_eq!(format!("{:?}", x), "gjdgxs");
/// assert_eq!(format!("{:#?}", x), "GJDGXS");
/// assert_eq!(format!("{:010?}", x), "0000gjdgxs");
/// assert_eq!(format!("{:#010?}", x), "0000GJDGXS");
///
/// let x = BaseFmtWrapper::new(-1000000000i32, 36);
/// assert_eq!(format!("{:?}", x), "-gjdgxs");
/// assert_eq!(format!("{:#?}", x), "-GJDGXS");
/// assert_eq!(format!("{:010?}", x), "-000gjdgxs");
/// assert_eq!(format!("{:#010?}", x), "-000GJDGXS");
/// ```
///
/// # to_string_base
/// ```
/// use malachite_base::num::conversion::traits::ToStringBase;
///
/// assert_eq!(1000u16.to_string_base(2), "1111101000");
/// assert_eq!(1000u16.to_string_base(10), "1000");
/// assert_eq!(1000u16.to_string_base(36), "rs");
///
/// assert_eq!(1000i16.to_string_base(2), "1111101000");
/// assert_eq!(1000i16.to_string_base(10), "1000");
/// assert_eq!(1000i16.to_string_base(36), "rs");
///
/// assert_eq!((-1000i16).to_string_base(2), "-1111101000");
/// assert_eq!((-1000i16).to_string_base(10), "-1000");
/// assert_eq!((-1000i16).to_string_base(36), "-rs");
/// ```
///
/// # to_string_base_upper
/// ```
/// use malachite_base::num::conversion::traits::ToStringBase;
///
/// assert_eq!(1000u16.to_string_base_upper(2), "1111101000");
/// assert_eq!(1000u16.to_string_base_upper(10), "1000");
/// assert_eq!(1000u16.to_string_base_upper(36), "RS");
///
/// assert_eq!(1000i16.to_string_base_upper(2), "1111101000");
/// assert_eq!(1000i16.to_string_base_upper(10), "1000");
/// assert_eq!(1000i16.to_string_base_upper(36), "RS");
///
/// assert_eq!((-1000i16).to_string_base_upper(2), "-1111101000");
/// assert_eq!((-1000i16).to_string_base_upper(10), "-1000");
/// assert_eq!((-1000i16).to_string_base_upper(36), "-RS");
/// ```
pub mod to_string;
