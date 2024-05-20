// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ShrRound;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{
    FromOtherTypeSlice, SplitInHalf, VecFromOtherType, VecFromOtherTypeSlice, WrappingFrom,
};
use crate::rounding_modes::RoundingMode::*;
use alloc::vec;
use alloc::vec::Vec;

const fn from_other_type_slice_ident<T: PrimitiveUnsigned>(xs: &[T]) -> T {
    if xs.is_empty() {
        T::ZERO
    } else {
        xs[0]
    }
}

macro_rules! impl_slice_traits_ident {
    ($a:ty) => {
        impl FromOtherTypeSlice<$a> for $a {
            /// Converts a slice of one type of value to a single value of the same type.
            ///
            /// $$
            /// f((x_k)_{k=0}^{n-1}) = \\begin{cases}
            ///     0 & \text{if} \\quad n = 0, \\\\
            ///     x_0 & \\text{otherwise},
            /// \\end{cases}
            /// $$
            /// where $W$ is the width of the type.
            ///
            /// The slice is interpreted as the base-$2^W$ digits of the value, in ascending order,
            /// where $W$ is the width of the type. If there's more than one element in the input
            /// slice, the value wraps and all elements past the first are ignored. This means that
            /// if the slice is empty, the output value is 0; otherwise, it's the first element of
            /// the slice.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::slice#from_other_type_slice).
            #[inline]
            fn from_other_type_slice(xs: &[$a]) -> Self {
                from_other_type_slice_ident(xs)
            }
        }

        impl VecFromOtherTypeSlice<$a> for $a {
            /// Converts a slice of one type of value to a [`Vec`] of the same type.
            ///
            /// In this case, it just converts the slice to a [`Vec`] the usual way.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
            ///
            /// # Examples
            /// See [here](super::slice#vec_from_other_type_slice).
            #[inline]
            fn vec_from_other_type_slice(xs: &[$a]) -> Vec<Self> {
                xs.to_vec()
            }
        }

        impl VecFromOtherType<$a> for $a {
            /// Converts a value of one type to a [`Vec`] of the same type.
            ///
            /// In this case, it just creates a one-element [`Vec`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::slice#vec_from_other_type).
            #[inline]
            fn vec_from_other_type(x: $a) -> Vec<Self> {
                ::alloc::vec![x]
            }
        }
    };
}

fn from_other_type_slice_large_to_small<
    A: PrimitiveUnsigned,
    B: PrimitiveUnsigned + WrappingFrom<A>,
>(
    xs: &[A],
) -> B {
    if xs.is_empty() {
        B::ZERO
    } else {
        B::wrapping_from(xs[0])
    }
}

fn vec_from_other_type_slice_large_to_small<
    A: PrimitiveUnsigned,
    B: PrimitiveUnsigned + WrappingFrom<A>,
>(
    xs: &[A],
) -> Vec<B> {
    let log_size_ratio = A::LOG_WIDTH - B::LOG_WIDTH;
    let mut out = ::alloc::vec![B::ZERO; xs.len() << log_size_ratio];
    for (chunk, &u) in out.chunks_exact_mut(1 << log_size_ratio).zip(xs.iter()) {
        let mut u = u;
        for x in chunk {
            *x = B::wrapping_from(u);
            u >>= B::WIDTH;
        }
    }
    out
}

fn vec_from_other_type_large_to_small<
    A: PrimitiveUnsigned,
    B: PrimitiveUnsigned + WrappingFrom<A>,
>(
    mut x: A,
) -> Vec<B> {
    let mut xs = ::alloc::vec![B::ZERO; 1 << (A::LOG_WIDTH - B::LOG_WIDTH)];
    for out in &mut xs {
        *out = B::wrapping_from(x);
        x >>= B::WIDTH;
    }
    xs
}

macro_rules! impl_slice_traits_large_to_small {
    ($a:ident, $b:ident) => {
        impl FromOtherTypeSlice<$a> for $b {
            /// Converts a slice of one type of unsigned integer to a single value of a smaller
            /// unsigned type.
            ///
            /// $$
            /// f((x_k)_{k=0}^{n-1}) = \\begin{cases}
            ///     0 & \text{if} \\quad n = 0, \\\\
            ///     y & \\text{otherwise},
            /// \\end{cases}
            /// $$
            /// where $0 \leq y < 2^W$, $x = y + k2^W$ for some integer $k$, and $W$ is the width of
            /// the output type.
            ///
            /// The slice is interpreted as the base-$2^W$ digits of the value, in ascending order,
            /// where $W$ is the width of the type. If the slice is empty, the output value is 0;
            /// otherwise, it consists of the least-significant bits of the first element of the
            /// slice.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::slice#from_other_type_slice).
            #[inline]
            fn from_other_type_slice(xs: &[$a]) -> Self {
                from_other_type_slice_large_to_small(xs)
            }
        }

        impl VecFromOtherTypeSlice<$a> for $b {
            /// Converts a slice of one type of unsigned integer to a [`Vec`] of a smaller unsigned
            /// type.
            ///
            /// Each value of the input slice will be broken up into several values in the output
            /// [`Vec`].
            ///
            /// Let $V$ be the the width of the input type and $W$ the width of the output type.
            ///
            /// $f((x_k)_ {k=0}^{n-1}) = (y_k)_ {k=0}^{m-1}$, where
            ///
            /// $$
            /// \sum_{j=0}^{n-1}2^{jV}x_j = \sum_{j=0}^{m-1}2^{jW}y_j,
            /// $$
            ///
            /// $y_j < 2^W$ for all $j$, and $m = 2^{V-W}n$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
            ///
            /// # Examples
            /// See [here](super::slice#vec_from_other_type_slice).
            #[inline]
            fn vec_from_other_type_slice(xs: &[$a]) -> Vec<Self> {
                vec_from_other_type_slice_large_to_small(xs)
            }
        }

        impl VecFromOtherType<$a> for $b {
            /// Converts a value of one type of unsigned integer to a [`Vec`] of a smaller unsigned
            /// type.
            ///
            /// The input value will be broken up into several values in the output [`Vec`].
            ///
            /// $f(x) = (y_k)_{k=0}^{m-1}$, where $x = \sum_{j=0}^{m-1}2^{jW}y_j$ and $m =
            /// 2^{V-W}n$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::slice#vec_from_other_type).
            #[inline]
            fn vec_from_other_type(x: $a) -> Vec<Self> {
                vec_from_other_type_large_to_small(x)
            }
        }
    };
}

fn from_other_type_slice_small_to_large<
    A: PrimitiveUnsigned,
    B: PrimitiveUnsigned + WrappingFrom<A>,
>(
    xs: &[A],
) -> B {
    let mut result = B::ZERO;
    let mut offset = 0;
    for &u in xs.iter().take(1 << (B::LOG_WIDTH - A::LOG_WIDTH)) {
        result |= B::wrapping_from(u) << offset;
        offset += A::WIDTH;
    }
    result
}

fn vec_from_other_type_slice_small_to_large<
    A: PrimitiveUnsigned,
    B: PrimitiveUnsigned + WrappingFrom<A>,
>(
    xs: &[A],
) -> Vec<B> {
    let log_size_ratio = B::LOG_WIDTH - A::LOG_WIDTH;
    let mut out = ::alloc::vec![B::ZERO; xs.len().shr_round(log_size_ratio, Ceiling).0];
    for (x, chunk) in out.iter_mut().zip(xs.chunks(1 << log_size_ratio)) {
        *x = from_other_type_slice_small_to_large(chunk);
    }
    out
}

fn vec_from_other_type_small_to_large<A, B: WrappingFrom<A>>(x: A) -> Vec<B> {
    ::alloc::vec![B::wrapping_from(x)]
}

macro_rules! impl_slice_traits_small_to_large {
    ($a:ident, $b:ident) => {
        impl FromOtherTypeSlice<$a> for $b {
            /// Converts a slice of one type of unsigned integer to a single value of a larger
            /// unsigned type.
            ///
            /// Let $V$ be the the width of the input type and $W$ the width of the output type.
            ///
            /// $f((x_k)_{k=0}^{n-1}) = y$, where $y < 2^W$ and
            ///
            /// $$
            /// y = k2^W + \sum_{j=0}^{n-1}2^{jV}x_j
            /// $$
            ///
            /// for some integer $k$.
            ///
            /// If the input slice contains more values than necessary to build an output value, the
            /// trailing values are ignored. If the input slice contains too few values to build an
            /// output value, the most-significant bits of the output value are set to 0.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::slice#from_other_type_slice).
            #[inline]
            fn from_other_type_slice(xs: &[$a]) -> Self {
                from_other_type_slice_small_to_large(xs)
            }
        }

        impl VecFromOtherTypeSlice<$a> for $b {
            /// Converts a slice of one type of unsigned integer to a [`Vec`] of a larger unsigned
            /// type.
            ///
            /// Adjacent chunks of values in the input slice will be joined into values of the
            /// output [`Vec`]. If the last few elements of the input slice don't make up a full
            /// chunk, the most-significant bits of the last output value are set to 0.
            ///
            /// Let $V$ be the the width of the input type and $W$ the width of the output type.
            ///
            /// $f((x_k)_ {k=0}^{n-1}) = (y_k)_ {k=0}^{m-1}$, where
            ///
            /// $$
            /// \sum_{j=0}^{n-1}2^{jV}x_j = \sum_{j=0}^{m-1}2^{jW}y_j,
            /// $$
            ///
            /// $y_j < 2^W$ for all $j$, and $m = \lceil n / 2^{W-V} \rceil$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
            ///
            /// # Examples
            /// See [here](super::slice#vec_from_other_type_slice).
            #[inline]
            fn vec_from_other_type_slice(xs: &[$a]) -> Vec<Self> {
                vec_from_other_type_slice_small_to_large(xs)
            }
        }

        impl VecFromOtherType<$a> for $b {
            /// Converts a value of one type of unsigned integer to a [`Vec`] of a larger unsigned
            /// type.
            ///
            /// The output [`Vec`] only contains one value. The least-significant bits of the output
            /// value contain the input value, and the most-significant bits are set to 0.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::slice#vec_from_other_type).
            #[inline]
            fn vec_from_other_type(x: $a) -> Vec<Self> {
                vec_from_other_type_small_to_large(x)
            }
        }
    };
}
apply_to_unsigneds!(impl_slice_traits_ident);

impl_slice_traits_large_to_small!(u16, u8);
impl_slice_traits_large_to_small!(u32, u8);
impl_slice_traits_large_to_small!(u32, u16);
impl_slice_traits_large_to_small!(u64, u8);
impl_slice_traits_large_to_small!(u64, u16);
impl_slice_traits_large_to_small!(u64, u32);
impl_slice_traits_large_to_small!(u128, u8);
impl_slice_traits_large_to_small!(u128, u16);
impl_slice_traits_large_to_small!(u128, u32);
impl_slice_traits_large_to_small!(u128, u64);
impl_slice_traits_large_to_small!(u128, usize);
impl_slice_traits_large_to_small!(usize, u8);
impl_slice_traits_large_to_small!(usize, u16);

impl_slice_traits_small_to_large!(u8, u16);
impl_slice_traits_small_to_large!(u8, u32);
impl_slice_traits_small_to_large!(u8, u64);
impl_slice_traits_small_to_large!(u8, u128);
impl_slice_traits_small_to_large!(u8, usize);
impl_slice_traits_small_to_large!(u16, u32);
impl_slice_traits_small_to_large!(u16, u64);
impl_slice_traits_small_to_large!(u16, u128);
impl_slice_traits_small_to_large!(u16, usize);
impl_slice_traits_small_to_large!(u32, u64);
impl_slice_traits_small_to_large!(u32, u128);
impl_slice_traits_small_to_large!(u64, u128);
impl_slice_traits_small_to_large!(usize, u128);

impl FromOtherTypeSlice<u32> for usize {
    /// Converts a slice of `u32`s to a single `usize`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// See [here](super::slice#from_other_type_slice).
    fn from_other_type_slice(xs: &[u32]) -> Self {
        if usize::WIDTH == u32::WIDTH {
            if xs.is_empty() {
                0
            } else {
                usize::wrapping_from(xs[0])
            }
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            let mut result = 0;
            let mut offset = 0;
            for &u in xs.iter().take(2) {
                result |= usize::wrapping_from(u) << offset;
                offset += u32::WIDTH;
            }
            result
        }
    }
}

impl VecFromOtherTypeSlice<u32> for usize {
    /// Converts a slice of [`u32`]s to a [`Vec`] of [`usize`]s.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
    ///
    /// See [here](super::slice#vec_from_other_type_slice).
    fn vec_from_other_type_slice(xs: &[u32]) -> Vec<Self> {
        let mut out;
        if usize::WIDTH == u32::WIDTH {
            out = vec![0; xs.len()];
            for (x, &u) in out.iter_mut().zip(xs.iter()) {
                *x = usize::wrapping_from(u);
            }
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            out = vec![0; xs.len().shr_round(1, Ceiling).0];
            for (x, chunk) in out.iter_mut().zip(xs.chunks(2)) {
                *x = usize::from_other_type_slice(chunk);
            }
        }
        out
    }
}

impl VecFromOtherType<u32> for usize {
    /// Converts a [`u32`] to a [`Vec`] of [`usize`]s.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// See [here](super::slice#vec_from_other_type).
    #[inline]
    fn vec_from_other_type(x: u32) -> Vec<Self> {
        vec![usize::wrapping_from(x)]
    }
}

impl FromOtherTypeSlice<u64> for usize {
    /// Converts a slice of [`u64`]s to a single [`usize`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// See [here](super::slice#from_other_type_slice).
    #[inline]
    fn from_other_type_slice(xs: &[u64]) -> Self {
        if xs.is_empty() {
            0
        } else {
            usize::wrapping_from(xs[0])
        }
    }
}

impl VecFromOtherTypeSlice<u64> for usize {
    /// Converts a slice of [`u64`]s to a [`Vec`] of [`usize`]s.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
    ///
    /// See [here](super::slice#vec_from_other_type_slice).
    #[allow(arithmetic_overflow)]
    fn vec_from_other_type_slice(xs: &[u64]) -> Vec<Self> {
        let mut out;
        if usize::WIDTH == u32::WIDTH {
            out = ::alloc::vec![0; xs.len() << 1];
            for (chunk, &u) in out.chunks_exact_mut(2).zip(xs.iter()) {
                let mut u = u;
                for x in chunk {
                    *x = usize::wrapping_from(u);
                    u >>= usize::WIDTH;
                }
            }
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            out = ::alloc::vec![0; xs.len()];
            for (x, &u) in out.iter_mut().zip(xs.iter()) {
                *x = usize::wrapping_from(u);
            }
        }
        out
    }
}

impl VecFromOtherType<u64> for usize {
    /// Converts a [`u64`] to a [`Vec`] of [`usize`]s.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// See [here](super::slice#vec_from_other_type).
    fn vec_from_other_type(x: u64) -> Vec<Self> {
        if usize::WIDTH == u32::WIDTH {
            let (upper, lower) = x.split_in_half();
            ::alloc::vec![usize::wrapping_from(lower), usize::wrapping_from(upper)]
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            ::alloc::vec![usize::wrapping_from(x)]
        }
    }
}

impl FromOtherTypeSlice<usize> for u32 {
    /// Converts a slice of [`usize`]s to a single [`u32`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// See [here](super::slice#from_other_type_slice).
    #[inline]
    fn from_other_type_slice(xs: &[usize]) -> Self {
        if xs.is_empty() {
            0
        } else {
            u32::wrapping_from(xs[0])
        }
    }
}

impl VecFromOtherTypeSlice<usize> for u32 {
    /// Converts a slice of [`usize`]s to a [`Vec`] of [`u32`]s.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
    ///
    /// See [here](super::slice#vec_from_other_type_slice).
    #[allow(arithmetic_overflow)]
    fn vec_from_other_type_slice(xs: &[usize]) -> Vec<Self> {
        let mut out;
        if usize::WIDTH == u32::WIDTH {
            out = ::alloc::vec![0; xs.len()];
            for (x, &u) in out.iter_mut().zip(xs.iter()) {
                *x = u32::wrapping_from(u);
            }
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            out = ::alloc::vec![0; xs.len() << 1];
            for (chunk, &u) in out.chunks_exact_mut(2).zip(xs.iter()) {
                let mut u = u;
                for x in chunk {
                    *x = u32::wrapping_from(u);
                    u >>= u32::WIDTH;
                }
            }
        }
        out
    }
}

impl VecFromOtherType<usize> for u32 {
    /// Converts a [`usize`] to a [`Vec`] of [`u32`]s.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// See [here](super::slice#vec_from_other_type).
    #[allow(arithmetic_overflow)]
    fn vec_from_other_type(x: usize) -> Vec<Self> {
        if usize::WIDTH == u32::WIDTH {
            ::alloc::vec![u32::wrapping_from(x)]
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            let (upper, lower) = u64::wrapping_from(x).split_in_half();
            ::alloc::vec![lower, upper]
        }
    }
}

impl FromOtherTypeSlice<usize> for u64 {
    /// Converts a slice of [`usize`]s to a single [`u64`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// See [here](super::slice#from_other_type_slice).
    fn from_other_type_slice(xs: &[usize]) -> Self {
        if usize::WIDTH == u32::WIDTH {
            let mut result = 0;
            let mut offset = 0;
            for &u in xs.iter().take(2) {
                result |= u64::wrapping_from(u) << offset;
                offset += usize::WIDTH;
            }
            result
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            if xs.is_empty() {
                0
            } else {
                u64::wrapping_from(xs[0])
            }
        }
    }
}

impl VecFromOtherTypeSlice<usize> for u64 {
    /// Converts a slice of [`usize`]s to a [`Vec`] of [`u64`]s.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
    ///
    /// See [here](super::slice#vec_from_other_type_slice).
    fn vec_from_other_type_slice(xs: &[usize]) -> Vec<Self> {
        let mut out;
        if usize::WIDTH == u32::WIDTH {
            out = ::alloc::vec![0; xs.len().shr_round(1, Ceiling).0];
            for (x, chunk) in out.iter_mut().zip(xs.chunks(2)) {
                *x = u64::from_other_type_slice(chunk);
            }
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            out = ::alloc::vec![0; xs.len()];
            for (x, &u) in out.iter_mut().zip(xs.iter()) {
                *x = u64::wrapping_from(u);
            }
        }
        out
    }
}

impl VecFromOtherType<usize> for u64 {
    /// Converts a [`usize`] to a [`Vec`] of [`u64`]s.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// See [here](super::slice#vec_from_other_type).
    #[inline]
    fn vec_from_other_type(x: usize) -> Vec<Self> {
        ::alloc::vec![u64::wrapping_from(x)]
    }
}
