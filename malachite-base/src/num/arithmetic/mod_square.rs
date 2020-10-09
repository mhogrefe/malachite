use num::arithmetic::traits::{ModPow, ModPowAssign};
use num::arithmetic::traits::{ModSquare, ModSquareAssign};

macro_rules! impl_mod_square {
    ($t:ident) => {
        impl ModSquare for $t {
            type Output = $t;

            /// Computes `self.square()` mod `m`. Assumes the input is already reduced mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModSquare;
            ///
            /// assert_eq!(2u8.mod_square(10), 4);
            /// assert_eq!(100u32.mod_square(497), 60);
            /// ```
            #[inline]
            fn mod_square(self, m: $t) -> $t {
                self.mod_pow(2, m)
            }
        }

        impl ModSquareAssign for $t {
            /// Replaces `self` with `self.square()` mod `m`. Assumes the input is already reduced
            /// mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModSquareAssign;
            ///
            /// let mut n = 2u8;
            /// n.mod_square_assign(10);
            /// assert_eq!(n, 4);
            ///
            /// let mut n = 100u32;
            /// n.mod_square_assign(497);
            /// assert_eq!(n, 60);
            /// ```
            #[inline]
            fn mod_square_assign(&mut self, m: $t) {
                self.mod_pow_assign(2, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_square);
