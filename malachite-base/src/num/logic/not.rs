use num::logic::traits::NotAssign;

macro_rules! impl_not {
    ($t:ident) => {
        impl NotAssign for $t {
            /// Replace a number with its bitwise negation.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::NotAssign;
            ///
            /// let mut x = 123u16;
            /// x.not_assign();
            /// assert_eq!(x, 65_412);
            /// ```
            #[inline]
            fn not_assign(&mut self) {
                *self = !*self;
            }
        }
    };
}

impl_not!(u8);
impl_not!(u16);
impl_not!(u32);
impl_not!(u64);
impl_not!(u128);
impl_not!(usize);
impl_not!(i8);
impl_not!(i16);
impl_not!(i32);
impl_not!(i64);
impl_not!(i128);
impl_not!(isize);
