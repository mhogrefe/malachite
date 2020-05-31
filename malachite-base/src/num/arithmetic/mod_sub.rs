use num::arithmetic::traits::{ModSub, ModSubAssign};

macro_rules! impl_mod_sub {
    ($t:ident) => {
        impl ModSub for $t {
            type Output = $t;

            /// Computes `self - other` mod `m`. Assumes the inputs are already reduced mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModSub;
            ///
            /// assert_eq!(4u8.mod_sub(3, 5), 1);
            /// assert_eq!(7u32.mod_sub(9, 10), 8);
            /// ```
            ///
            /// This is nmod_sub from nmod_vec.h, FLINT Dev 1.
            #[inline]
            fn mod_sub(self, other: $t, m: $t) -> $t {
                let diff = self.wrapping_sub(other);
                if self < other {
                    m.wrapping_add(diff)
                } else {
                    diff
                }
            }
        }

        impl ModSubAssign for $t {
            /// Replaces `self` with `self - other` mod `m`. Assumes the inputs are already reduced
            /// mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModSubAssign;
            ///
            /// let mut n = 4u8;
            /// n.mod_sub_assign(3, 5);
            /// assert_eq!(n, 1);
            ///
            /// let mut n = 7u32;
            /// n.mod_sub_assign(9, 10);
            /// assert_eq!(n, 8);
            /// ```
            ///
            /// This is nmod_sub from nmod_vec.h, FLINT Dev 1, where the result is assigned to a.
            #[inline]
            fn mod_sub_assign(&mut self, other: $t, m: $t) {
                *self = self.mod_sub(other, m);
            }
        }
    };
}

impl_mod_sub!(u8);
impl_mod_sub!(u16);
impl_mod_sub!(u32);
impl_mod_sub!(u64);
impl_mod_sub!(u128);
impl_mod_sub!(usize);
