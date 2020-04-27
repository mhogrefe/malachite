use num::arithmetic::traits::ShrRound;
use num::basic::integers::PrimitiveInteger;
use num::basic::traits::Zero;
use num::conversion::traits::{
    FromOtherTypeSlice, HasHalf, JoinHalves, SplitInHalf, VecFromOtherType, VecFromOtherTypeSlice,
    WrappingFrom,
};
use round::RoundingMode;

macro_rules! impl_half_traits {
    ($t:ident, $ht: ident) => {
        /// Implements `HasHalf` for unsigned primitive integers.
        impl HasHalf for $t {
            /// The primitive integer type whose width is half of `Self`.
            type Half = $ht;
        }

        /// Implements `JoinHalves` for unsigned primitive integers.
        impl JoinHalves for $t {
            /// Joins two unsigned integers to form an unsigned integer with twice the width.
            /// `join_halves(upper, lower)`, where `upper` and `lower` are integers with w bits,
            /// yields an integer with 2w bits whose value is `upper` * 2<sup>w</sup> + `lower`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::JoinHalves;
            ///
            /// assert_eq!(u16::join_halves(1, 2), 258);
            /// assert_eq!(u32::join_halves(0xabcd, 0x1234), 0xabcd1234);
            /// ```
            #[inline]
            fn join_halves(upper: Self::Half, lower: Self::Half) -> Self {
                $t::from(upper) << $ht::WIDTH | $t::from(lower)
            }
        }

        /// Implements `SplitInHalf` for unsigned primitive integers.
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::conversion::traits::SplitInHalf;
        ///
        /// assert_eq!(258u16.split_in_half(), (1, 2));
        /// assert_eq!(0xabcd1234u32.split_in_half(), (0xabcd, 0x1234));
        /// ```
        impl SplitInHalf for $t {
            /// Extracts the lower, or least significant half, of and unsigned integer.
            /// `n.lower_half()`, where `n` is an integer with w bits, yields an integer with w/2
            /// bits whose value is `n` mod 2<sup>w/2</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::SplitInHalf;
            ///
            /// assert_eq!(258u16.lower_half(), 2);
            /// assert_eq!(0xabcd1234u32.lower_half(), 0x1234);
            /// ```
            #[inline]
            fn lower_half(&self) -> Self::Half {
                $ht::wrapping_from(*self)
            }

            /// Extracts the upper, or most significant half, of and unsigned integer.
            /// `n.upper_half()`, where `n` is an integer with w bits, yields an integer with w/2
            /// bits whose value is floor(`n` / 2<sup>w/2</sup>).
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::SplitInHalf;
            ///
            /// assert_eq!(258u16.upper_half(), 1);
            /// assert_eq!(0xabcd1234u32.upper_half(), 0xabcd);
            /// ```
            #[inline]
            fn upper_half(&self) -> Self::Half {
                $ht::wrapping_from(self >> $ht::WIDTH)
            }
        }
    };
}

impl_half_traits!(u16, u8);
impl_half_traits!(u32, u16);
impl_half_traits!(u64, u32);
impl_half_traits!(u128, u64);

macro_rules! impl_slice_traits_ident {
    ($a:ty) => {
        impl FromOtherTypeSlice<$a> for $a {
            /// Converts a slice of one type of value to a single value of the same type. If the
            /// slice is empty, the output value is 0; otherwise, it's the first element of the
            /// slice.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::FromOtherTypeSlice;
            ///
            /// let xs: &[u32] = &[];
            /// assert_eq!(u32::from_other_type_slice(xs), 0);
            /// assert_eq!(u32::from_other_type_slice(&[123u32, 456]), 123);
            /// ```
            #[inline]
            fn from_other_type_slice(slice: &[$a]) -> Self {
                if slice.is_empty() {
                    0
                } else {
                    slice[0]
                }
            }
        }

        impl VecFromOtherTypeSlice<$a> for $a {
            /// Converts a slice of one type of value to a `Vec` of the same type. In this case, it
            /// just converts the slice to a `Vec` the usual way.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `slice.len()`
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::VecFromOtherTypeSlice;
            ///
            /// assert_eq!(u32::vec_from_other_type_slice(&[123u32, 456]), vec![123, 456]);
            /// ```
            #[inline]
            fn vec_from_other_type_slice(slice: &[$a]) -> Vec<Self> {
                slice.to_vec()
            }
        }

        impl VecFromOtherType<$a> for $a {
            /// Converts a value of one type to a `Vec` of the same type. In this case, it just
            /// creates a one-element `Vec`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::VecFromOtherType;
            ///
            /// assert_eq!(u32::vec_from_other_type(123u32), vec![123]);
            /// ```
            #[inline]
            fn vec_from_other_type(value: $a) -> Vec<Self> {
                vec![value]
            }
        }
    };
}

macro_rules! impl_slice_traits_large_to_small {
    ($a:ident, $b:ident) => {
        impl FromOtherTypeSlice<$a> for $b {
            /// Converts a slice of one type of unsigned integer to a single value of a smaller
            /// unsigned type. If the slice is empty, the output value is 0; otherwise, it consists
            /// of the least-significant bits of the first element of the slice.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::FromOtherTypeSlice;
            ///
            /// assert_eq!(u8::from_other_type_slice(&[0xabcdu16, 0xef01]), 0xcd);
            /// ```
            #[inline]
            fn from_other_type_slice(slice: &[$a]) -> Self {
                if slice.is_empty() {
                    0
                } else {
                    $b::wrapping_from(slice[0])
                }
            }
        }

        impl VecFromOtherTypeSlice<$a> for $b {
            /// Converts a slice of one type of unsigned integer to a `Vec` of a smaller unsigned
            /// type. Each value of the input slice will be broken up into several values in the
            /// output `Vec`.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `slice.len()`
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::VecFromOtherTypeSlice;
            ///
            /// assert_eq!(
            ///     u8::vec_from_other_type_slice(&[0xcdabu16, 0x01ef, 0x4523, 0x8967]),
            ///     vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89]
            /// );
            /// ```
            fn vec_from_other_type_slice(slice: &[$a]) -> Vec<Self> {
                const LOG_SIZE_RATIO: u64 = $a::LOG_WIDTH - $b::LOG_WIDTH;
                const SIZE_RATIO: usize = 1 << LOG_SIZE_RATIO;
                let mut xs = vec![$b::ZERO; slice.len() << LOG_SIZE_RATIO];
                for (chunk, &u) in xs.chunks_exact_mut(SIZE_RATIO).zip(slice.iter()) {
                    let mut u = u;
                    for out in chunk {
                        *out = $b::wrapping_from(u);
                        u >>= $b::WIDTH;
                    }
                }
                xs
            }
        }

        impl VecFromOtherType<$a> for $b {
            /// Converts a value of one type of unsigned integer to a `Vec` of a smaller unsigned
            /// type. The input value will be broken up into several values in the output `Vec`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::VecFromOtherType;
            ///
            /// assert_eq!(u8::vec_from_other_type(0xcdabu16), vec![0xab, 0xcd]);
            /// ```
            #[inline]
            fn vec_from_other_type(mut value: $a) -> Vec<Self> {
                const SIZE_RATIO: usize = 1 << ($a::LOG_WIDTH - $b::LOG_WIDTH);
                let mut xs = vec![$b::ZERO; SIZE_RATIO];
                for out in xs.iter_mut() {
                    *out = $b::wrapping_from(value);
                    value >>= $b::WIDTH;
                }
                xs
            }
        }
    };
}

macro_rules! impl_slice_traits_small_to_large {
    ($a:ident, $b:ident) => {
        impl FromOtherTypeSlice<$a> for $b {
            /// Converts a slice of one type of unsigned integer to a single value of a larger
            /// unsigned type. If the input slice contains more values than necessary to build an
            /// output value, the trailing values are ignored. If the input slice contains too few
            /// values to build an output value, the most-significant bits of the output value are
            /// set to 0.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::FromOtherTypeSlice;
            ///
            /// assert_eq!(u16::from_other_type_slice(&[0xabu8, 0xcd, 0xef]), 0xcdab);
            /// assert_eq!(u64::from_other_type_slice(&[0xabu8, 0xcd, 0xef]), 0xefcdab);
            /// ```
            fn from_other_type_slice(slice: &[$a]) -> Self {
                const SIZE_RATIO: usize = 1 << ($b::LOG_WIDTH - $a::LOG_WIDTH);
                let mut result = 0;
                let mut offset = 0;
                for &u in slice.iter().take(SIZE_RATIO) {
                    result |= $b::wrapping_from(u) << offset;
                    offset += $a::WIDTH;
                }
                result
            }
        }

        impl VecFromOtherTypeSlice<$a> for $b {
            /// Converts a slice of one type of unsigned integer to a `Vec` of a larger unsigned
            /// type. Adjacent chunks of values in the input slice will be joined into values of the
            /// output `Vec`. If the last few elements of the input slice don't make up a full
            /// chunk, the most-significant bits of the last output value are set to 0.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `slice.len()`
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::VecFromOtherTypeSlice;
            ///
            /// assert_eq!(
            ///     u16::vec_from_other_type_slice(&[0xabu8, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67]),
            ///     vec![0xcdab, 0x01ef, 0x4523, 0x67]
            /// );
            /// ```
            fn vec_from_other_type_slice(slice: &[$a]) -> Vec<Self> {
                const LOG_SIZE_RATIO: u64 = $b::LOG_WIDTH - $a::LOG_WIDTH;
                const SIZE_RATIO: usize = 1 << LOG_SIZE_RATIO;
                let mut xs =
                    vec![$b::ZERO; slice.len().shr_round(LOG_SIZE_RATIO, RoundingMode::Ceiling)];
                for (out, chunk) in xs.iter_mut().zip(slice.chunks(SIZE_RATIO)) {
                    *out = $b::from_other_type_slice(chunk);
                }
                xs
            }
        }

        impl VecFromOtherType<$a> for $b {
            /// Converts a value of one type of unsigned integer to a `Vec` of a larger unsigned
            /// type. The output `Vec` only contains one value. The least-significant bits of the
            /// output value contain the input value, and the most-significant bits are set to 0.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::VecFromOtherType;
            ///
            /// assert_eq!(u16::vec_from_other_type(0xabu8), vec![0xab]);
            /// ```
            #[inline]
            fn vec_from_other_type(value: $a) -> Vec<Self> {
                vec![$b::wrapping_from(value)]
            }
        }
    };
}

impl_slice_traits_ident!(u8);
impl_slice_traits_ident!(u16);
impl_slice_traits_ident!(u32);
impl_slice_traits_ident!(u64);
impl_slice_traits_ident!(u128);
impl_slice_traits_ident!(usize);

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
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn from_other_type_slice(slice: &[u32]) -> Self {
        if usize::WIDTH == u32::WIDTH {
            if slice.is_empty() {
                0
            } else {
                usize::wrapping_from(slice[0])
            }
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            let mut result = 0;
            let mut offset = 0;
            for &u in slice.iter().take(2) {
                result |= usize::wrapping_from(u) << offset;
                offset += u32::WIDTH;
            }
            result
        }
    }
}

impl VecFromOtherTypeSlice<u32> for usize {
    /// Converts a slice of `u32`s to a `Vec` of `usize`s.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `slice.len()`
    fn vec_from_other_type_slice(slice: &[u32]) -> Vec<Self> {
        let mut xs;
        if usize::WIDTH == u32::WIDTH {
            xs = vec![0; slice.len()];
            for (out, &u) in xs.iter_mut().zip(slice.iter()) {
                *out = usize::wrapping_from(u);
            }
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            xs = vec![0; slice.len().shr_round(1, RoundingMode::Ceiling)];
            for (out, chunk) in xs.iter_mut().zip(slice.chunks(2)) {
                *out = usize::from_other_type_slice(chunk);
            }
        }
        xs
    }
}

impl VecFromOtherType<u32> for usize {
    /// Converts a `u32` to a `Vec` of `usize`s.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    #[inline]
    fn vec_from_other_type(value: u32) -> Vec<Self> {
        vec![usize::wrapping_from(value)]
    }
}

impl FromOtherTypeSlice<u64> for usize {
    /// Converts a slice of `u64`s to a single `usize`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    #[inline]
    fn from_other_type_slice(slice: &[u64]) -> Self {
        if slice.is_empty() {
            0
        } else {
            usize::wrapping_from(slice[0])
        }
    }
}

impl VecFromOtherTypeSlice<u64> for usize {
    /// Converts a slice of `u64`s to a `Vec` of `usize`s.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `slice.len()`
    #[allow(arithmetic_overflow)]
    fn vec_from_other_type_slice(slice: &[u64]) -> Vec<Self> {
        let mut xs;
        if usize::WIDTH == u32::WIDTH {
            xs = vec![0; slice.len() << 1];
            for (chunk, &u) in xs.chunks_exact_mut(2).zip(slice.iter()) {
                let mut u = u;
                for out in chunk {
                    *out = usize::wrapping_from(u);
                    u >>= usize::WIDTH;
                }
            }
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            xs = vec![0; slice.len()];
            for (out, &u) in xs.iter_mut().zip(slice.iter()) {
                *out = usize::wrapping_from(u);
            }
        }
        xs
    }
}

impl VecFromOtherType<u64> for usize {
    /// Converts a `u64` to a `Vec` of `usize`s.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn vec_from_other_type(value: u64) -> Vec<Self> {
        if usize::WIDTH == u32::WIDTH {
            let (upper, lower) = value.split_in_half();
            vec![usize::wrapping_from(lower), usize::wrapping_from(upper)]
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            vec![usize::wrapping_from(value)]
        }
    }
}

impl FromOtherTypeSlice<usize> for u32 {
    /// Converts a slice of `usize`s to a single `u32`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    #[inline]
    fn from_other_type_slice(slice: &[usize]) -> Self {
        if slice.is_empty() {
            0
        } else {
            u32::wrapping_from(slice[0])
        }
    }
}

impl VecFromOtherTypeSlice<usize> for u32 {
    /// Converts a slice of `usize`s to a `Vec` of `u32`s.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `slice.len()`
    #[allow(arithmetic_overflow)]
    fn vec_from_other_type_slice(slice: &[usize]) -> Vec<Self> {
        let mut xs;
        if usize::WIDTH == u32::WIDTH {
            xs = vec![0; slice.len()];
            for (out, &u) in xs.iter_mut().zip(slice.iter()) {
                *out = u32::wrapping_from(u);
            }
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            xs = vec![0; slice.len() << 1];
            for (chunk, &u) in xs.chunks_exact_mut(2).zip(slice.iter()) {
                let mut u = u;
                for out in chunk {
                    *out = u32::wrapping_from(u);
                    u >>= u32::WIDTH;
                }
            }
        }
        xs
    }
}

impl VecFromOtherType<usize> for u32 {
    /// Converts a `usize` to a `Vec` of `u32`s.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    #[allow(arithmetic_overflow)]
    fn vec_from_other_type(value: usize) -> Vec<Self> {
        if usize::WIDTH == u32::WIDTH {
            vec![u32::wrapping_from(value)]
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            let (upper, lower) = u64::wrapping_from(value).split_in_half();
            vec![lower, upper]
        }
    }
}

impl FromOtherTypeSlice<usize> for u64 {
    /// Converts a slice of `usize`s to a single `u64`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn from_other_type_slice(slice: &[usize]) -> Self {
        if usize::WIDTH == u32::WIDTH {
            let mut result = 0;
            let mut offset = 0;
            for &u in slice.iter().take(2) {
                result |= u64::wrapping_from(u) << offset;
                offset += usize::WIDTH;
            }
            result
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            if slice.is_empty() {
                0
            } else {
                u64::wrapping_from(slice[0])
            }
        }
    }
}

impl VecFromOtherTypeSlice<usize> for u64 {
    /// Converts a slice of `usize`s to a `Vec` of `u64`s.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `slice.len()`
    fn vec_from_other_type_slice(slice: &[usize]) -> Vec<Self> {
        let mut xs;
        if usize::WIDTH == u32::WIDTH {
            xs = vec![0; slice.len().shr_round(1, RoundingMode::Ceiling)];
            for (out, chunk) in xs.iter_mut().zip(slice.chunks(2)) {
                *out = u64::from_other_type_slice(chunk);
            }
        } else {
            assert_eq!(usize::WIDTH, u64::WIDTH);
            xs = vec![0; slice.len()];
            for (out, &u) in xs.iter_mut().zip(slice.iter()) {
                *out = u64::wrapping_from(u);
            }
        }
        xs
    }
}

impl VecFromOtherType<usize> for u64 {
    /// Converts a `usize` to a `Vec` of `u64`s.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    #[inline]
    fn vec_from_other_type(value: usize) -> Vec<Self> {
        vec![u64::wrapping_from(value)]
    }
}
