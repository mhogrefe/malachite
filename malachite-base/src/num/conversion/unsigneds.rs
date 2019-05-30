use num::basic::integers::PrimitiveInteger;
use num::conversion::traits::{FromU32Slice, HasHalf, JoinHalves, SplitInHalf, WrappingFrom};

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

//TODO doc and test
impl FromU32Slice for u8 {
    #[inline]
    fn from_u32_slice(slice: &[u32]) -> Self {
        assert!(!slice.is_empty());
        u8::wrapping_from(slice[0])
    }

    fn copy_from_u32_slice(out_slice: &mut [u8], in_slice: &[u32]) {
        let out_len = out_slice.len();
        assert!(out_len >= in_slice.len() << 2);
        let mut i = 0;
        for u in in_slice {
            let (upper, lower) = u.split_in_half();
            let (upper_upper, lower_upper) = upper.split_in_half();
            let (upper_lower, lower_lower) = lower.split_in_half();
            out_slice[i] = lower_lower;
            out_slice[i + 1] = upper_lower;
            out_slice[i + 2] = lower_upper;
            out_slice[i + 3] = upper_upper;
            i += 4;
        }
    }
}

//TODO doc and test
impl FromU32Slice for u16 {
    #[inline]
    fn from_u32_slice(slice: &[u32]) -> Self {
        assert!(!slice.is_empty());
        u16::wrapping_from(slice[0])
    }

    fn copy_from_u32_slice(out_slice: &mut [u16], in_slice: &[u32]) {
        let out_len = out_slice.len();
        assert!(out_len >= in_slice.len() << 1);
        let mut i = 0;
        for u in in_slice {
            let (upper, lower) = u.split_in_half();
            out_slice[i] = lower;
            out_slice[i + 1] = upper;
            i += 2;
        }
    }
}

//TODO doc and test
impl FromU32Slice for u32 {
    #[inline]
    fn from_u32_slice(slice: &[u32]) -> Self {
        assert!(!slice.is_empty());
        slice[0]
    }

    #[inline]
    fn copy_from_u32_slice(out_slice: &mut [u32], in_slice: &[u32]) {
        let out_len = out_slice.len();
        assert!(out_len >= in_slice.len());
        out_slice.copy_from_slice(&in_slice[..out_len]);
    }
}

//TODO doc and test
impl FromU32Slice for u64 {
    #[inline]
    fn from_u32_slice(slice: &[u32]) -> Self {
        assert!(slice.len() >= 2);
        u64::join_halves(slice[1], slice[0])
    }

    fn copy_from_u32_slice(out_slice: &mut [u64], in_slice: &[u32]) {
        let out_len = out_slice.len();
        assert!(out_len >= in_slice.len() >> 1);
        let mut i = 0;
        for out in out_slice.iter_mut() {
            *out = u64::join_halves(in_slice[i + 1], in_slice[i]);
            i += 2;
        }
    }
}

//TODO doc and test
impl FromU32Slice for u128 {
    #[inline]
    fn from_u32_slice(slice: &[u32]) -> Self {
        assert!(slice.len() >= 4);
        u128::join_halves(
            u64::join_halves(slice[3], slice[2]),
            u64::join_halves(slice[1], slice[0]),
        )
    }

    fn copy_from_u32_slice(out_slice: &mut [u128], in_slice: &[u32]) {
        let out_len = out_slice.len();
        assert!(out_len >= in_slice.len() >> 2);
        let mut i = 0;
        for out in out_slice.iter_mut() {
            *out = u128::join_halves(
                u64::join_halves(in_slice[i + 3], in_slice[i + 2]),
                u64::join_halves(in_slice[i + 1], in_slice[i]),
            );
            i += 4;
        }
    }
}

//TODO doc and test
impl FromU32Slice for usize {
    #[inline]
    fn from_u32_slice(slice: &[u32]) -> Self {
        match usize::WIDTH {
            u32::WIDTH => usize::wrapping_from(u32::from_u32_slice(slice)),
            u64::WIDTH => usize::wrapping_from(u64::from_u32_slice(slice)),
            _ => panic!("unexpected usize size: {}", usize::WIDTH),
        }
    }

    fn copy_from_u32_slice(out_slice: &mut [usize], in_slice: &[u32]) {
        match usize::WIDTH {
            u32::WIDTH => {
                let out_len = out_slice.len();
                assert!(out_len >= in_slice.len());
                for (out, &x) in out_slice.iter_mut().zip(in_slice.iter()) {
                    *out = usize::wrapping_from(x);
                }
            }
            u64::WIDTH => {
                let out_len = out_slice.len();
                assert!(out_len >= in_slice.len() >> 1);
                let mut i = 0;
                for out in out_slice.iter_mut() {
                    *out = usize::wrapping_from(u64::join_halves(in_slice[i + 1], in_slice[i]));
                    i += 2;
                }
            }
            _ => panic!("unexpected usize size: {}", usize::WIDTH),
        }
    }
}
