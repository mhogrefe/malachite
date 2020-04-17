use num::arithmetic::traits::{ModNeg, ModNegAssign};

macro_rules! impl_mod_neg {
    ($t:ident) => {
        impl ModNeg for $t {
            type Output = $t;

            /// Computes `-self` mod `m`. Assumes the input is already reduced mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModNeg;
            ///
            /// assert_eq!(0u8.mod_neg(5), 0);
            /// assert_eq!(7u32.mod_neg(10), 3);
            /// assert_eq!(100u16.mod_neg(101), 1);
            /// ```
            ///
            /// This is nmod_neg from nmod_vec.h, FLINT Dev 1.
            #[inline]
            fn mod_neg(self, m: $t) -> $t {
                if self == 0 {
                    0
                } else {
                    m - self
                }
            }
        }

        impl ModNegAssign for $t {
            /// Replaces `self` with `-self` mod `m`. Assumes the input is already reduced mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModNegAssign;
            ///
            /// let mut n = 0u8;
            /// n.mod_neg_assign(5);
            /// assert_eq!(n, 0);
            ///
            /// let mut n = 7u32;
            /// n.mod_neg_assign(10);
            /// assert_eq!(n, 3);
            ///
            /// let mut n = 100u16;
            /// n.mod_neg_assign(101);
            /// assert_eq!(n, 1);
            /// ```
            ///
            /// This is nmod_neg from nmod_vec.h, FLINT Dev 1, where the output is assign to a.
            #[inline]
            fn mod_neg_assign(&mut self, m: $t) {
                if *self != 0 {
                    *self = m - *self;
                }
            }
        }
    };
}

impl_mod_neg!(u8);
impl_mod_neg!(u16);
impl_mod_neg!(u32);
impl_mod_neg!(u64);
impl_mod_neg!(u128);
impl_mod_neg!(usize);
