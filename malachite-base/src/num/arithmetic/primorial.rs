// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{CheckedPrimorial, Primorial};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::conversion::traits::WrappingFrom;

const PRIMORIALS_U8: [u8; 5] = [1, 2, 6, 30, 210];
const PRIMORIALS_U16: [u16; 7] = [1, 2, 6, 30, 210, 2310, 30030];
const PRIMORIALS_U32: [u32; 10] = [1, 2, 6, 30, 210, 2310, 30030, 510510, 9699690, 223092870];
const PRIMORIALS_U64: [u64; 16] = [
    1,
    2,
    6,
    30,
    210,
    2310,
    30030,
    510510,
    9699690,
    223092870,
    6469693230,
    200560490130,
    7420738134810,
    304250263527210,
    13082761331670030,
    614889782588491410,
];
const PRIMORIALS_U128: [u128; 27] = [
    1,
    2,
    6,
    30,
    210,
    2310,
    30030,
    510510,
    9699690,
    223092870,
    6469693230,
    200560490130,
    7420738134810,
    304250263527210,
    13082761331670030,
    614889782588491410,
    32589158477190044730,
    1922760350154212639070,
    117288381359406970983270,
    7858321551080267055879090,
    557940830126698960967415390,
    40729680599249024150621323470,
    3217644767340672907899084554130,
    267064515689275851355624017992790,
    23768741896345550770650537601358310,
    2305567963945518424753102147331756070,
    232862364358497360900063316880507363070,
];

const PRIMORIAL_PRIMES_U8: [u64; 5] = [2, 3, 5, 7, 11];

const PRIMORIAL_PRIMES_U16: [u64; 7] = [2, 3, 5, 7, 11, 13, 17];

const PRIMORIAL_PRIMES_U32: [u64; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

const PRIMORIAL_PRIMES_U64: [u64; 16] =
    [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];

const PRIMORIAL_PRIMES_U128: [u64; 27] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103,
];

macro_rules! impl_primorials_a {
    ($t:ident, $ps:ident, $pps:ident) => {
        impl CheckedPrimorial for $t {
            /// Computes the primorial of a number: the product of all primes less than or equal to
            /// it.
            ///
            /// The
            /// [`checked_product_of_first_n_primes`](CheckedPrimorial::checked_product_of_first_n_primes)
            /// function is similar; it computes the primorial of the $n$th prime.
            ///
            /// If the input is too large, the function returns `None`.
            ///
            /// $$
            /// f(n) = \\begin{cases}
            ///     \operatorname{Some}(n\\#) & \text{if} \\quad n\\# < 2^W, \\\\
            ///     \operatorname{None} & \text{if} \\quad n\\# \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primorial#checked_primorial).
            #[inline]
            fn checked_primorial(n: u64) -> Option<$t> {
                let i = match $pps.binary_search(&n) {
                    Ok(i) => i + 1,
                    Err(i) => i,
                };
                $ps.get(i).copied()
            }

            /// Computes the product of the first $n$ primes.
            ///
            /// The [`checked_primorial`](CheckedPrimorial::checked_primorial) function is similar;
            /// it computes the product of all primes less than or equal to $n$.
            ///
            /// If the input is too large, the function returns `None`.
            ///
            /// $$
            /// f(n) = \\begin{cases}
            ///     \operatorname{Some}(p_n\\#) & \text{if} \\quad p_n\\# < 2^W, \\\\
            ///     \operatorname{None} & \text{if} \\quad p_n\\# \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primorial#checked_product_of_first_n_primes).
            #[inline]
            fn checked_product_of_first_n_primes(n: u64) -> Option<$t> {
                $ps.get(usize::try_from(n).ok()?).copied()
            }
        }
    };
}
impl_primorials_a!(u8, PRIMORIALS_U8, PRIMORIAL_PRIMES_U8);
impl_primorials_a!(u16, PRIMORIALS_U16, PRIMORIAL_PRIMES_U16);
impl_primorials_a!(u32, PRIMORIALS_U32, PRIMORIAL_PRIMES_U32);
impl_primorials_a!(u64, PRIMORIALS_U64, PRIMORIAL_PRIMES_U64);
impl_primorials_a!(u128, PRIMORIALS_U128, PRIMORIAL_PRIMES_U128);

impl CheckedPrimorial for usize {
    /// Computes the primorial of a [`usize`]: the product of all primes less than or equal to it.
    ///
    /// The
    /// [`checked_product_of_first_n_primes`](CheckedPrimorial::checked_product_of_first_n_primes)
    /// function is similar; it computes the primorial of the $n$th prime.
    ///
    /// If the input is too large, the function returns `None`.
    ///
    /// $$
    /// f(n) = \\begin{cases}
    ///     \operatorname{Some}(n\\#) & \text{if} \\quad n\\# < 2^W, \\\\
    ///     \operatorname{None} & \text{if} \\quad n\\# \geq 2^W,
    /// \\end{cases}
    /// $$
    /// where $W$ is `usize::WIDTH`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::primorial#checked_primorial).
    #[inline]
    fn checked_primorial(n: u64) -> Option<usize> {
        match usize::WIDTH {
            u32::WIDTH => u32::checked_primorial(n).map(usize::wrapping_from),
            u64::WIDTH => u64::checked_primorial(n).map(usize::wrapping_from),
            _ => panic!("Unexpected usize width: {}", usize::WIDTH),
        }
    }

    /// Computes the product of the first $n$ primes.
    ///
    /// The [`checked_primorial`](CheckedPrimorial::checked_primorial) function is similar; it
    /// computes the product of all primes less than or equal to $n$.
    ///
    /// If the input is too large, the function returns `None`.
    ///
    /// $$
    /// f(n) = \\begin{cases}
    ///     \operatorname{Some}(p_n\\#) & \text{if} \\quad p_n\\# < 2^W, \\\\
    ///     \operatorname{None} & \text{if} \\quad p_n\\# \geq 2^W,
    /// \\end{cases}
    /// $$
    /// where $W$ is `usize::WIDTH`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::primorial#checked_product_of_first_n_primes).
    #[inline]
    fn checked_product_of_first_n_primes(n: u64) -> Option<usize> {
        match usize::WIDTH {
            u32::WIDTH => u32::checked_product_of_first_n_primes(n).map(usize::wrapping_from),
            u64::WIDTH => u64::checked_product_of_first_n_primes(n).map(usize::wrapping_from),
            _ => panic!("Unexpected usize width: {}", usize::WIDTH),
        }
    }
}

macro_rules! impl_primorials_b {
    ($t:ident) => {
        impl Primorial for $t {
            /// Computes the primorial of a number: the product of all primes less than or equal to
            /// it.
            ///
            /// The [`product_of_first_n_primes`](Primorial::product_of_first_n_primes) function is
            /// similar; it computes the primorial of the $n$th prime.
            ///
            /// If the input is too large, the function panics. For a function that returns `None`
            /// instead, try [`checked_primorial`](CheckedPrimorial::checked_primorial).
            ///
            /// $$
            /// f(n) = n\\# = \prod_{p \leq n \atop p \\ \\text {prime}} p.
            /// $$
            ///
            /// $n\\# = O(e^{(1+o(1))n})$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the output is too large to be represented.
            ///
            /// # Examples
            /// See [here](super::primorial#primorial).
            #[inline]
            fn primorial(n: u64) -> $t {
                $t::checked_primorial(n).unwrap()
            }

            /// Computes the product of the first $n$ primes.
            ///
            /// The [`primorial`](Primorial::primorial) function is similar; it computes the product
            /// of all primes less than or equal to $n$.
            ///
            /// If the input is too large, the function panics. For a function that returns `None`
            /// instead, try
            /// [`checked_product_of_first_n_primes`](CheckedPrimorial::checked_product_of_first_n_primes).
            ///
            /// $$
            /// f(n) = p_n\\# = \prod_{k=1}^n p_n,
            /// $$
            /// where $p_n$ is the $n$th prime number.
            ///
            /// $p_n\\# = O\left ( \left ( \frac{1}{e}k\log k\left ( \frac{\log k}{e^2}k \right
            /// )^{1/\log k} \right )^k \omega(1)\right )$.
            ///
            /// This asymptotic approximation is due to [Bart
            /// Michels](https://math.stackexchange.com/a/1594930).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the output is too large to be represented.
            ///
            /// # Examples
            /// See [here](super::primorial#product_of_first_n_primes).
            #[inline]
            fn product_of_first_n_primes(n: u64) -> $t {
                $t::checked_product_of_first_n_primes(n).unwrap()
            }
        }
    };
}
apply_to_unsigneds!(impl_primorials_b);
