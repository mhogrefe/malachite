use num::arithmetic::traits::{ModPowerOf2, UnsignedAbs};
use num::basic::integers::PrimitiveInt;
use num::conversion::traits::WrappingFrom;
use num::logic::traits::{BitBlockAccess, LeadingZeros};
use std::cmp::min;
use std::ops::Neg;

const ERROR_MESSAGE: &str = "Result exceeds width of output type";

fn get_bits_unsigned<T: ModPowerOf2<Output = T> + PrimitiveInt>(x: &T, start: u64, end: u64) -> T {
    assert!(start <= end);
    if start >= T::WIDTH {
        T::ZERO
    } else {
        (*x >> start).mod_power_of_2(end - start)
    }
}

fn assign_bits_unsigned<T: ModPowerOf2<Output = T> + PrimitiveInt>(
    x: &mut T,
    start: u64,
    end: u64,
    bits: &T,
) {
    assert!(start <= end);
    let width = T::WIDTH;
    let bits_width = end - start;
    let bits = bits.mod_power_of_2(bits_width);
    if bits != T::ZERO && LeadingZeros::leading_zeros(bits) < start {
        panic!("{}", ERROR_MESSAGE);
    } else if start < width {
        *x &= !(T::MAX.mod_power_of_2(min(bits_width, width - start)) << start);
        *x |= bits << start;
    }
}

macro_rules! impl_bit_block_access_unsigned {
    ($t:ident) => {
        impl BitBlockAccess for $t {
            type Bits = $t;

            /// Extracts a block of bits whose first index is `start` and last index is `end - 1`.
            ///
            /// The block of bits has the same type as the input. If `end` is greater than the
            /// type's width, the high bits of the result are all 0.
            ///
            /// Let
            /// $$
            /// n = \sum_{i=0}^\infty 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$; so finitely many of the bits are 1, and the
            /// rest are 0. Then
            /// $$
            /// f(n, p, q) = \sum_{i=p}^{q-1} 2^{b_{i-p}}.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `start < end`.
            ///
            /// # Examples
            /// See the documentation of the `num::logic::bit_block_access` module.
            #[inline]
            fn get_bits(&self, start: u64, end: u64) -> Self {
                get_bits_unsigned(self, start, end)
            }

            /// Assigns the least-significant `end - start` bits of `bits` to bits `start`
            /// (inclusive) through `end` (exclusive) of `self`.
            ///
            /// The block of bits has the same type as the input. If `bits` has fewer bits than
            /// `end - start`, the high bits are interpreted as 0. If `end` is greater than the
            /// type's width, the high bits of `bits` must be 0.
            ///
            /// Let
            /// $$
            /// n = \sum_{i=0}^{W-1} 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$ and $W$ is `$t::WIDTH`.
            /// Let
            /// $$
            /// m = \sum_{i=0}^k 2^{d_i},
            /// $$
            /// where for all $i$, $d_i\in \\{0, 1\\}$.
            /// Also, let $p, q \in \mathbb{N}$, where $d_i = 0$ for all $i \geq W + p$.
            ///
            /// Then
            /// $$
            /// f(n, p, q, m) = \sum_{i=0}^{W-1} 2^{c_i},
            /// $$
            /// where
            /// $$
            /// \\{c_0, c_1, c_2, \ldots, c_ {W-1}\\} =
            /// \\{b_0, b_1, b_2, \ldots, b_{p-1}, d_0, d_1, \ldots, d_{p-q-1}, b_q, \ldots,
            /// b_ {W-1}\\}.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `start < end`, or if `end > $t::WIDTH` and bits `$t::WIDTH - start`
            /// through `end - start` of `bits` are nonzero.
            ///
            /// # Examples
            /// See the documentation of the `num::logic::bit_block_access` module.
            #[inline]
            fn assign_bits(&mut self, start: u64, end: u64, bits: &Self::Bits) {
                assign_bits_unsigned(self, start, end, bits)
            }
        }
    };
}
apply_to_unsigneds!(impl_bit_block_access_unsigned);

fn get_bits_signed<T: ModPowerOf2<Output = U> + Neg<Output = T> + PrimitiveInt, U>(
    x: &T,
    start: u64,
    end: u64,
) -> U {
    assert!(start <= end);
    (if start >= T::WIDTH {
        -T::iverson(*x < T::ZERO)
    } else {
        *x >> start
    })
    .mod_power_of_2(end - start)
}

fn assign_bits_signed<
    T: PrimitiveInt + UnsignedAbs<Output = U> + WrappingFrom<U>,
    U: BitBlockAccess<Bits = U> + ModPowerOf2<Output = U> + PrimitiveInt,
>(
    x: &mut T,
    start: u64,
    end: u64,
    bits: &U,
) {
    assert!(start <= end);
    if *x >= T::ZERO {
        let mut abs_x = x.unsigned_abs();
        abs_x.assign_bits(start, end, bits);
        if abs_x.get_highest_bit() {
            panic!("{}", ERROR_MESSAGE);
        }
        *x = T::wrapping_from(abs_x);
    } else {
        let width = T::WIDTH - 1;
        let bits_width = end - start;
        let bits = bits.mod_power_of_2(bits_width);
        let max = U::MAX;
        if bits_width > width + 1 {
            panic!("{}", ERROR_MESSAGE);
        } else if start >= width {
            if bits != max.mod_power_of_2(bits_width) {
                panic!("{}", ERROR_MESSAGE);
            }
        } else {
            let lower_width = width - start;
            if end > width && bits >> lower_width != max.mod_power_of_2(end - width) {
                panic!("{}", ERROR_MESSAGE);
            } else {
                *x &=
                    T::wrapping_from(!(max.mod_power_of_2(min(bits_width, lower_width)) << start));
                *x |= T::wrapping_from(bits << start);
            }
        }
    }
}

macro_rules! impl_bit_block_access_signed {
    ($u:ident, $s:ident) => {
        impl BitBlockAccess for $s {
            type Bits = $u;

            /// Extracts a block of bits whose first index is `start` and last index is `end - 1`.
            ///
            /// The type of the block of bits is the unsigned version of the input type. If `end` is
            /// greater than the type's width, the high bits of the result are all 0, or all 1,
            /// depending on the input value's sign; and if the input is negative and `end - start`
            /// is greater than the type's width, the function panics.
            ///
            /// If $n \geq 0$, let
            /// $$
            /// n = \sum_{i=0}^\infty 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$; so finitely many of the bits are 1, and the
            /// rest are 0. Then
            /// $$
            /// f(n, p, q) = \sum_{i=p}^{q-1} 2^{b_{i-p}}.
            /// $$
            ///
            /// If $n < 0$, let
            /// $$
            /// 2^W + n = \sum_{i=0}^{W-1} 2^{b_i},
            /// $$
            /// where
            /// - $W$ is the type's width
            /// - for all $i$, $b_i\in \\{0, 1\\}$, and $b_i = 1$ for $i \geq W$.
            /// Then
            /// $$
            /// f(n, p, q) = \sum_{i=p}^{q-1} 2^{b_{i-p}}.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `start < end` or `self < 0 && end - start > $s::WIDTH`.
            ///
            /// # Examples
            /// See the documentation of the `num::logic::bit_block_access` module.
            #[inline]
            fn get_bits(&self, start: u64, end: u64) -> Self::Bits {
                get_bits_signed(self, start, end)
            }

            /// Assigns the least-significant `end - start` bits of `bits` to bits `start`
            /// (inclusive) through `end` (exclusive) of `self`.
            ///
            /// The type of the block of bits is the unsigned version of the input type. If `bits`
            /// has fewer bits than `end - start`, the high bits are interpreted as 0 or 1,
            /// depending on the sign of `self`. If `end` is greater than the type's width, the high
            /// bits of `bits` must be 0 or 1, depending on the sign of `self`.
            ///
            /// The sign of `self` remains unchanged, since only a finite number of bits are changed
            /// and the sign is determined by the implied infinite prefix of bits.
            ///
            /// If $n \geq 0$ and $j \neq W - 1$, let
            /// $$
            /// n = \sum_{i=0}^{W-1} 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$ and $W$ is `$t::WIDTH`.
            /// Let
            /// $$
            /// m = \sum_{i=0}^k 2^{d_i},
            /// $$
            /// where for all $i$, $d_i\in \\{0, 1\\}$.
            /// Also, let $p, q \in \mathbb{N}$, where $d_i = 0$ for all $i \geq W + p - 1$. Then
            /// $$
            /// f(n, p, q, m) = \sum_{i=0}^{W-1} 2^{c_i},
            /// $$
            /// where
            /// $$
            /// \\{c_0, c_1, c_2, \ldots, c_ {W-1}\\} =
            /// \\{b_0, b_1, b_2, \ldots, b_{p-1}, d_0, d_1, \ldots, d_{p-q-1}, b_q, \ldots,
            /// b_ {W-1}\\}.
            /// $$
            ///
            /// If $n < 0$ or $j = W - 1$, let
            /// $$
            /// 2^W + n = \sum_{i=0}^{W-1} 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$ and $W$ is `$t::WIDTH`.
            /// Let
            /// $$
            /// m = \sum_{i=0}^k 2^{d_i},
            /// $$
            /// where for all $i$, $d_i\in \\{0, 1\\}$.
            /// Also, let $p, q \in \mathbb{N}$, where $d_i = 1$ for all $i \geq W + p - 1$. Then
            /// $$
            /// f(n, p, q, m) = \left ( \sum_{i=0}^{W-1} 2^{c_i} \right ) - 2^W,
            /// $$
            /// where
            /// $$
            /// \\{c_0, c_1, c_2, \ldots, c_ {W-1}\\} =
            /// \\{b_0, b_1, b_2, \ldots, b_{p-1}, d_0, d_1, \ldots, d_{p-q-1}, b_q, \ldots,
            /// b_ {W-1}\\}.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `start < end`, or if `end >= $s::WIDTH` and bits `$s::WIDTH - start`
            /// through `end - start` of `bits` are not equal to the original sign bit of `self`.
            ///
            /// # Examples
            /// See the documentation of the `num::logic::bit_block_access` module.
            #[inline]
            fn assign_bits(&mut self, start: u64, end: u64, bits: &Self::Bits) {
                assign_bits_signed(self, start, end, bits)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_bit_block_access_signed);
