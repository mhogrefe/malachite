use num::basic::integers::PrimitiveInt;
use num::logic::traits::BitAccess;

fn _get_bit_unsigned<T: PrimitiveInt>(x: &T, index: u64) -> bool {
    index < T::WIDTH && *x & T::power_of_two(index) != T::ZERO
}

fn _set_bit_unsigned<T: PrimitiveInt>(x: &mut T, index: u64) {
    if index < T::WIDTH {
        *x |= T::power_of_two(index);
    } else {
        panic!(
            "Cannot set bit {} in non-negative value of width {}",
            index,
            T::WIDTH
        );
    }
}

fn _clear_bit_unsigned<T: PrimitiveInt>(x: &mut T, index: u64) {
    if index < T::WIDTH {
        *x &= !T::power_of_two(index);
    }
}

macro_rules! impl_bit_access_unsigned {
    ($t:ident) => {
        /// Provides functions for accessing and modifying individual bits of a primitive unsigned
        /// integer.
        impl BitAccess for $t {
            /// Determines whether the `index`th bit of a primitive unsigned integer, or the
            /// coefficient of $2^i$ in its binary expansion, is 0 or 1.
            ///
            /// `false` means 0 and `true` means 1. Getting bits beyond the type's width is allowed;
            /// those bits are false.
            ///
            /// Let
            /// $$
            /// n = \sum_{i=0}^\infty 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$; so finitely many of the bits are 1, and the
            /// rest are 0. Then $f(n, j) = (b_j = 1)$.
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Examples
            ///
            /// See the documentation of the `num::logic::bit_access` module.
            #[inline]
            fn get_bit(&self, index: u64) -> bool {
                _get_bit_unsigned(self, index)
            }

            /// Sets the `index`th bit of a primitive unsigned integer, or the coefficient of $2^i$
            /// in its binary expansion, to 1.
            ///
            /// Setting bits beyond the type's width is disallowed.
            ///
            /// Let
            /// $$
            /// n = \sum_{i=0}^{W-1} 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$ and $W$ is `$t::WIDTH`. Then
            /// $$
            /// f(n, j) = \sum_{i=0}^{W-1} 2^{c_i},
            /// $$
            /// where
            /// $$
            /// \\{c_0, c_1, c_2, \ldots, c_ {W-1}\\} =
            /// \\{b_0, b_1, b_2, \ldots, b_{j-1}, 1, b_ {j+1}, \ldots, b_ {W-1}\\}.
            /// $$
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if $i \geq W$, where $i$ is `index` and $W$ is `$t::WIDTH`.
            ///
            /// # Examples
            ///
            /// See the documentation of the `num::logic::bit_access` module.
            #[inline]
            fn set_bit(&mut self, index: u64) {
                _set_bit_unsigned(self, index)
            }

            /// Sets the `index`th bit of a primitive unsigned integer, or the coefficient of $2^i$
            /// in its binary expansion, to 0.
            ///
            /// Clearing bits beyond the type's width is allowed; since those bits are already
            /// false, clearing them does nothing.
            ///
            /// Let
            /// $$
            /// n = \sum_{i=0}^\infty 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$; so finitely many of the bits are 1, and the
            /// rest are 0. Then
            /// $$
            /// f(n, j) = \sum_{i=0}^\infty 2^{c_i},
            /// $$
            /// where
            /// $$
            /// \\{c_0, c_1, c_2, \ldots \\} =
            /// \\{b_0, b_1, b_2, \ldots, b_{j-1}, 0, b_ {j+1}, \ldots \\}.
            /// $$
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Examples
            ///
            /// See the documentation of the `num::logic::bit_access` module.
            #[inline]
            fn clear_bit(&mut self, index: u64) {
                _clear_bit_unsigned(self, index)
            }
        }
    };
}
apply_to_unsigneds!(impl_bit_access_unsigned);

fn _get_bit_signed<T: PrimitiveInt>(x: &T, index: u64) -> bool {
    if index < T::WIDTH {
        *x & (T::ONE << index) != T::ZERO
    } else {
        *x < T::ZERO
    }
}

fn _set_bit_signed<T: PrimitiveInt>(x: &mut T, index: u64) {
    if index < T::WIDTH {
        *x |= T::ONE << index;
    } else if *x >= T::ZERO {
        panic!(
            "Cannot set bit {} in non-negative value of width {}",
            index,
            T::WIDTH
        );
    }
}

fn _clear_bit_signed<T: PrimitiveInt>(x: &mut T, index: u64) {
    if index < T::WIDTH {
        *x &= !(T::ONE << index);
    } else if *x < T::ZERO {
        panic!(
            "Cannot clear bit {} in negative value of width {}",
            index,
            T::WIDTH
        );
    }
}

macro_rules! impl_bit_access_signed {
    ($t:ident) => {
        /// Provides functions for accessing and modifying the individual bits of a primitive signed
        /// integer.
        impl BitAccess for $t {
            /// Determines whether the `index`th bit of a primitive signed integer is 0 or 1.
            ///
            /// `false` means 0 and `true` means 1. Getting bits beyond the type's width is allowed;
            /// those bits are true if the value is negative, and false otherwise.
            ///
            /// If $n \geq 0$, let
            /// $$
            /// n = \sum_{i=0}^\infty 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$; so finitely many of the bits are 1, and the
            /// rest are 0. Then $f(n, i) = (b_i = 1)$.
            ///
            /// If $n < 0$, let
            /// $$
            /// 2^W = \sum_{i=0}^{W-1} 2^{b_i},
            /// $$
            /// where
            /// - $W$ is the type's width
            /// - for all $i$, $b_i\in \\{0, 1\\}$, and $b_i = 1$ for $i \geq W$.
            ///
            /// Then $f(n, j) = (b_j = 1)$.
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Examples
            ///
            /// See the documentation of the `num::logic::bit_access` module.
            #[inline]
            fn get_bit(&self, index: u64) -> bool {
                _get_bit_signed(self, index)
            }

            /// Sets the `index`th bit of a primitive signed integer to 1.
            ///
            /// Setting bits beyond the type's width is disallowed, if `self` is non-negative.
            ///
            /// If $n \geq 0$ and $j \neq W - 1$, let
            /// $$
            /// n = \sum_{i=0}^{W-1} 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$ and $W$ is `$t::WIDTH`. Then
            /// $$
            /// f(n, j) = \sum_{i=0}^{W-1} 2^{c_i},
            /// $$
            /// where
            /// $$
            /// \\{c_0, c_1, c_2, \ldots, c_ {W-1}\\} =
            /// \\{b_0, b_1, b_2, \ldots, b_{j-1}, 1, b_ {j+1}, \ldots, b_ {W-1}\\},
            /// $$
            /// and $i < W$.
            ///
            /// If $n < 0$ or $j = W - 1$, let
            /// $$
            /// 2^W + n = \sum_{i=0}^{W-1} 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$ and $W$ is `$t::WIDTH`. Then
            /// $$
            /// f(n, j) = \left ( \sum_{i=0}^{W-1} 2^{c_i} \right ) - 2^W,
            /// $$
            /// where
            /// $$
            /// \\{c_0, c_1, c_2, \ldots, c_ {W-1}\\} =
            /// \\{b_0, b_1, b_2, \ldots, b_{j-1}, 1, b_ {j+1}, \ldots, b_ {W-1}\\}.
            /// $$
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if $n \geq 0$ and $i \geq W$, where $n$ is `self`, $i$ is `index` and $W$ is
            /// `$t::WIDTH`.
            ///
            /// # Examples
            ///
            /// See the documentation of the `num::logic::bit_access` module.
            #[inline]
            fn set_bit(&mut self, index: u64) {
                _set_bit_signed(self, index)
            }

            /// Sets the `index`th bit of a primitive signed integer to 0.
            ///
            /// Clearing bits beyond the type's width is disallowed, if `self` is negative.
            ///
            /// If $n \geq 0$ or $j = W - 1$, let
            /// $$
            /// n = \sum_{i=0}^{W-1} 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$ and $W$ is `$t::WIDTH`. Then
            /// $$
            /// f(n, j) = \sum_{i=0}^{W-1} 2^{c_i},
            /// $$
            /// where
            /// $$
            /// \\{c_0, c_1, c_2, \ldots, c_ {W-1}\\} =
            /// \\{b_0, b_1, b_2, \ldots, b_{j-1}, 0, b_ {j+1}, \ldots, b_ {W-1}\\}.
            /// $$
            ///
            /// If $n < 0$ and $j \neq W - 1$, let
            /// $$
            /// 2^W + n = \sum_{i=0}^{W-1} 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$ and $W$ is `$t::WIDTH`. Then
            /// $$
            /// f(n, j) = \left ( \sum_{i=0}^{W-1} 2^{c_i} \right ) - 2^W,
            /// $$
            /// where
            /// $$
            /// \\{c_0, c_1, c_2, \ldots, c_ {W-1}\\} =
            /// \\{b_0, b_1, b_2, \ldots, b_{j-1}, 0, b_ {j+1}, \ldots, b_ {W-1}\\},
            /// $$
            /// and $i < W$.
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if $n < 0$ and $i \geq W$, where $n$ is `self`, $i$ is `index` and $W$ is
            /// `$t::WIDTH`.
            ///
            /// # Examples
            ///
            /// See the documentation of the `num::logic::bit_access` module.
            #[inline]
            fn clear_bit(&mut self, index: u64) {
                _clear_bit_signed(self, index)
            }
        }
    };
}
apply_to_signeds!(impl_bit_access_signed);
