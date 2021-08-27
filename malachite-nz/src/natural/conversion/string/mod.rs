/// A struct that allows for formatting a numeric type and rendering its digits in a specified base.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct BaseFmtWrapper<T> {
    pub(crate) x: T,
    pub(crate) base: u64,
}

impl<T> BaseFmtWrapper<T> {
    /// Creates a new `BaseFmtWrapper`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::conversion::string::BaseFmtWrapper;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::from(1000000000u32);
    /// let x = BaseFmtWrapper::new(&n, 36);
    /// assert_eq!(format!("{}", x), "gjdgxs");
    /// assert_eq!(format!("{:#}", x), "GJDGXS");
    ///
    /// let n = Integer::from(-1000000000);
    /// let x = BaseFmtWrapper::new(&n, 36);
    /// assert_eq!(format!("{}", x), "-gjdgxs");
    /// assert_eq!(format!("{:#}", x), "-GJDGXS");
    /// ```
    pub fn new(x: T, base: u64) -> Self {
        assert!((2..=36).contains(&base), "base out of range");
        BaseFmtWrapper { x, base }
    }

    /// Recovers the value from a `BaseFmtWrapper`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::conversion::string::BaseFmtWrapper;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     BaseFmtWrapper::new(Natural::from(1000000000u32), 36).unwrap(),
    ///     1000000000
    /// );
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn unwrap(self) -> T {
        self.x
    }
}

pub mod from_string;
pub mod to_string;
