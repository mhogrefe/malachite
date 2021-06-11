use num::arithmetic::traits::{
    ModPow, ModPowAssign, ModPowPrecomputed, ModPowPrecomputedAssign, ModSquare, ModSquareAssign,
    ModSquarePrecomputed, ModSquarePrecomputedAssign,
};

macro_rules! impl_mod_square {
    ($t:ident) => {
        impl ModSquare for $t {
            type Output = $t;

            /// Computes `self.square()` mod `m`. Assumes the input is already reduced mod `m`.
            ///
            /// $f(x, m) = y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_square` module.
            #[inline]
            fn mod_square(self, m: $t) -> $t {
                self.mod_pow(2, m)
            }
        }

        impl ModSquareAssign for $t {
            /// Replaces `self` with `self.square()` mod `m`. Assumes the input is already reduced
            /// mod `m`.
            ///
            /// $x \gets y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_square` module.
            #[inline]
            fn mod_square_assign(&mut self, m: $t) {
                self.mod_pow_assign(2, m);
            }
        }

        impl ModSquarePrecomputed<u64, $t> for $t {
            /// Computes `self.square()` mod `m`. Assumes the input is already reduced mod `m`.
            ///
            /// Some precomputed data is provided; this speeds up computations involving several
            /// modular squarings with the same modulus. The precomputed data should be obtained
            /// using `precompute_mod_pow_data`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_square` module.
            #[inline]
            fn mod_square_precomputed(self, m: $t, data: &Self::Data) -> Self::Output {
                self.mod_pow_precomputed(2, m, data)
            }
        }

        impl ModSquarePrecomputedAssign<u64, $t> for $t {
            /// Replaces `self` with `self.square()` mod `m`. Assumes the input is already reduced
            /// mod `m`.
            ///
            /// Some precomputed data is provided; this speeds up computations involving several
            /// modular squarings with the same modulus. The precomputed data should be obtained
            /// using `precompute_mod_pow_data`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_square` module.
            #[inline]
            fn mod_square_precomputed_assign(&mut self, m: $t, data: &Self::Data) {
                self.mod_pow_precomputed_assign(2, m, data);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_square);
