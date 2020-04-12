use num::arithmetic::traits::{ModAdd, ModAddAssign};

macro_rules! impl_mod_add {
    ($t:ident) => {
        impl ModAdd for $t {
            type Output = $t;

            /// Computes `self + rhs` mod `m`. Assumes the inputs are already reduced mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModAdd;
            ///
            /// assert_eq!(0u8.mod_add(3, 5), 3);
            /// assert_eq!(7u32.mod_add(5, 10), 2);
            /// ```
            ///
            /// This is nmod_add from nmod_vec.h, FLINT Dev 1.
            #[inline]
            fn mod_add(self, rhs: $t, m: $t) -> $t {
                let neg = m - self;
                if neg > rhs {
                    self + rhs
                } else {
                    rhs - neg
                }
            }
        }

        impl ModAddAssign for $t {
            /// Computes `self + rhs` mod `m`. Assumes the inputs are already reduced mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModAddAssign;
            ///
            /// let mut n = 0u8;
            /// n.mod_add_assign(3, 5);
            /// assert_eq!(n, 3);
            ///
            /// let mut n = 7u32;
            /// n.mod_add_assign(5, 10);
            /// assert_eq!(n, 2);
            /// ```
            ///
            /// This is nmod_add from nmod_vec.h, FLINT Dev 1, where the result is assigned to a.
            #[inline]
            fn mod_add_assign(&mut self, rhs: $t, m: $t) {
                let neg = m - *self;
                if neg > rhs {
                    *self += rhs;
                } else {
                    *self = rhs - neg;
                }
            }
        }
    };
}

impl_mod_add!(u8);
impl_mod_add!(u16);
impl_mod_add!(u32);
impl_mod_add!(u64);
impl_mod_add!(u128);
impl_mod_add!(usize);
