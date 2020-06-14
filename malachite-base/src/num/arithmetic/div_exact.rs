use num::arithmetic::traits::{DivExact, DivExactAssign};

macro_rules! impl_div_exact {
    ($t:ident) => {
        impl DivExact for $t {
            type Output = $t;

            /// Divides a value by another value. The first value must be exactly divisible by the
            /// second. If it isn't, this function may crash or return a meaningless result.
            ///
            /// If you are unsure whether the division will be exact use `self / other` instead. If
            /// you're unsure and you want to know, use `self.div_mod(other)` and check whether the
            /// remainder is zero. If you want a function that panics if the division is not exact,
            /// use `self.div_round(other, RoundingMode::Exact)`.
            ///
            /// Time: Worst case O(1)
            ///
            /// Additional memory: Worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::DivExact;
            ///
            /// // 123 * 456 = 56088
            /// assert_eq!(56088u32.div_exact(456), 123);
            ///
            /// // -123 * -456 = 56088
            /// assert_eq!(56088i64.div_exact(-456), -123);
            /// ```
            #[inline]
            fn div_exact(self, other: $t) -> $t {
                self / other
            }
        }

        impl DivExactAssign for $t {
            /// Divides a value by another value in place. The value being assigned to must be
            /// exactly divisible by the value on the RHS. If it isn't, this function may crash or
            /// assign a meaningless value to the first value.
            ///
            /// If you are unsure whether the division will be exact use `self /= other` instead. If
            /// you're unsure and you want to know, use `self.div_assign_mod(other)` and check
            /// whether the remainder is zero. If you want a function that panics if the division is
            /// not exact, use `self.div_round_assign(other, RoundingMode::Exact)`.
            ///
            /// Time: Worst case O(1)
            ///
            /// Additional memory: Worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is zero or if `self` is `$t::MIN` and other is -1. May panic if
            /// `self` is not divisible by `other`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::DivExactAssign;
            ///
            /// // 123 * 456 = 56088
            /// let mut x = 56088u32;
            /// x.div_exact_assign(456);
            /// assert_eq!(x, 123);
            ///
            /// // -123 * -456 = 56088
            /// let mut x = 56088i64;
            /// x.div_exact_assign(-456);
            /// assert_eq!(x, -123);
            /// ```
            #[inline]
            fn div_exact_assign(&mut self, other: $t) {
                *self /= other;
            }
        }
    };
}
impl_div_exact!(u8);
impl_div_exact!(u16);
impl_div_exact!(u32);
impl_div_exact!(u64);
impl_div_exact!(u128);
impl_div_exact!(usize);
impl_div_exact!(i8);
impl_div_exact!(i16);
impl_div_exact!(i32);
impl_div_exact!(i64);
impl_div_exact!(i128);
impl_div_exact!(isize);
