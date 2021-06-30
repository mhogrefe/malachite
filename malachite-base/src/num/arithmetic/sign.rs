use num::arithmetic::traits::Sign;
use std::cmp::Ordering;

macro_rules! impl_sign_primitive_int {
    ($t:ident) => {
        impl Sign for $t {
            /// Returns `Greater`, `Equal`, or `Less`, depending on whether `self` is positive,
            /// zero, or negative, respectively.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::Sign;
            /// use std::cmp::Ordering;
            ///
            /// assert_eq!(0u8.sign(), Ordering::Equal);
            /// assert_eq!(100u64.sign(), Ordering::Greater);
            /// assert_eq!((-100i16).sign(), Ordering::Less);
            /// ```
            #[inline]
            fn sign(&self) -> Ordering {
                self.cmp(&0)
            }
        }
    };
}
apply_to_primitive_ints!(impl_sign_primitive_int);

//TODO tests, demos, benches, props
macro_rules! impl_sign_primitive_float {
    ($t:ident) => {
        impl Sign for $t {
            /// Returns the sign of `self`.
            ///
            /// - Positive finite numbers, positive zero, and positive infinity have sign
            /// `Greater`.
            /// - Negative finite numbers, negative zero, and negative infinity have sign `Less`.
            /// - `NaN` has sign `Equal`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::Sign;
            /// use malachite_base::num::float::PrimitiveFloat;
            /// use std::cmp::Ordering;
            ///
            /// assert_eq!(0.0.sign(), Ordering::Greater);
            /// assert_eq!(1.0.sign(), Ordering::Greater);
            /// assert_eq!(f64::POSITIVE_INFINITY.sign(), Ordering::Greater);
            ///
            /// assert_eq!((-0.0).sign(), Ordering::Less);
            /// assert_eq!((-1.0).sign(), Ordering::Less);
            /// assert_eq!(f64::NEGATIVE_INFINITY.sign(), Ordering::Less);
            ///
            /// assert_eq!(f64::NAN.sign(), Ordering::Equal);
            /// ```
            #[inline]
            fn sign(&self) -> Ordering {
                if self.is_nan() {
                    Ordering::Equal
                } else if self.is_sign_positive() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        }
    };
}
apply_to_primitive_floats!(impl_sign_primitive_float);
