use comparison::{Max, Min};
use conversion::{CheckedFrom, OverflowingFrom, SaturatingFrom, WrappingFrom};

macro_rules! identity_conversion {
    ($t:ty) => {
        impl CheckedFrom<$t> for $t {
            #[inline]
            fn checked_from(value: $t) -> Option<$t> {
                Some(value)
            }
        }

        impl WrappingFrom<$t> for $t {
            #[inline]
            fn wrapping_from(value: $t) -> $t {
                value
            }
        }

        impl SaturatingFrom<$t> for $t {
            #[inline]
            fn saturating_from(value: $t) -> $t {
                value
            }
        }

        impl OverflowingFrom<$t> for $t {
            #[inline]
            fn overflowing_from(value: $t) -> ($t, bool) {
                (value, false)
            }
        }
    };
}

macro_rules! lossless_conversion {
    ($a:ty, $b:ty) => {
        impl CheckedFrom<$a> for $b {
            #[inline]
            fn checked_from(value: $a) -> Option<$b> {
                Some(value.into())
            }
        }

        impl WrappingFrom<$a> for $b {
            #[inline]
            fn wrapping_from(value: $a) -> $b {
                value.into()
            }
        }

        impl SaturatingFrom<$a> for $b {
            #[inline]
            fn saturating_from(value: $a) -> $b {
                value.into()
            }
        }

        impl OverflowingFrom<$a> for $b {
            #[inline]
            fn overflowing_from(value: $a) -> ($b, bool) {
                (value.into(), false)
            }
        }
    };
}

macro_rules! lossy_conversion {
    ($a:ident, $b:ident) => {
        impl CheckedFrom<$a> for $b {
            #[allow(unused_comparisons, clippy::cast_lossless)]
            #[inline]
            fn checked_from(value: $a) -> Option<$b> {
                let result = value as $b;
                if (result < 0) == (value < 0) && result as $a == value {
                    Some(result)
                } else {
                    None
                }
            }
        }

        #[allow(clippy::cast_lossless)]
        impl WrappingFrom<$a> for $b {
            #[inline]
            fn wrapping_from(value: $a) -> $b {
                value as $b
            }
        }

        impl SaturatingFrom<$a> for $b {
            #[allow(unused_comparisons)]
            #[inline]
            fn saturating_from(value: $a) -> $b {
                if let Some(b_max) = $a::checked_from($b::MAX) {
                    if value >= b_max {
                        return $b::MAX;
                    }
                }
                if let Some(b_min) = $a::checked_from($b::MIN) {
                    if value <= b_min {
                        return $b::MIN;
                    }
                }
                value as $b
            }
        }

        impl OverflowingFrom<$a> for $b {
            #[allow(unused_comparisons, clippy::cast_lossless)]
            #[inline]
            fn overflowing_from(value: $a) -> ($b, bool) {
                let result = value as $b;
                if (result < 0) == (value < 0) && result as $a == value {
                    (result, false)
                } else {
                    (result, true)
                }
            }
        }
    };
}

macro_rules! proper_subset_conversion {
    ($a:ident, $b:ident) => {
        lossless_conversion!($a, $b);
        lossy_conversion!($b, $a);
    };
}

macro_rules! no_containment_conversion {
    ($a:ident, $b:ident) => {
        lossy_conversion!($a, $b);
        lossy_conversion!($b, $a);
    };
}

identity_conversion!(u8);
identity_conversion!(u16);
identity_conversion!(u32);
identity_conversion!(u64);
identity_conversion!(u128);
identity_conversion!(usize);
identity_conversion!(i8);
identity_conversion!(i16);
identity_conversion!(i32);
identity_conversion!(i64);
identity_conversion!(i128);
identity_conversion!(isize);

proper_subset_conversion!(u8, u16);
proper_subset_conversion!(u8, u32);
proper_subset_conversion!(u8, u64);
proper_subset_conversion!(u8, u128);
proper_subset_conversion!(u8, usize);
proper_subset_conversion!(u8, i16);
proper_subset_conversion!(u8, i32);
proper_subset_conversion!(u8, i64);
proper_subset_conversion!(u8, i128);
proper_subset_conversion!(u8, isize);
proper_subset_conversion!(u16, u32);
proper_subset_conversion!(u16, u64);
proper_subset_conversion!(u16, u128);
proper_subset_conversion!(u16, usize);
proper_subset_conversion!(u16, i32);
proper_subset_conversion!(u16, i64);
proper_subset_conversion!(u16, i128);
proper_subset_conversion!(u32, u64);
proper_subset_conversion!(u32, u128);
proper_subset_conversion!(u32, i64);
proper_subset_conversion!(u32, i128);
proper_subset_conversion!(u64, u128);
proper_subset_conversion!(u64, i128);
proper_subset_conversion!(i8, i16);
proper_subset_conversion!(i8, i32);
proper_subset_conversion!(i8, i64);
proper_subset_conversion!(i8, i128);
proper_subset_conversion!(i8, isize);
proper_subset_conversion!(i16, i32);
proper_subset_conversion!(i16, i64);
proper_subset_conversion!(i16, i128);
proper_subset_conversion!(i16, isize);
proper_subset_conversion!(i32, i64);
proper_subset_conversion!(i32, i128);
proper_subset_conversion!(i64, i128);

no_containment_conversion!(u8, i8);
no_containment_conversion!(u16, i8);
no_containment_conversion!(u16, i16);
no_containment_conversion!(u16, isize);
no_containment_conversion!(u32, usize);
no_containment_conversion!(u32, i8);
no_containment_conversion!(u32, i16);
no_containment_conversion!(u32, i32);
no_containment_conversion!(u32, isize);
no_containment_conversion!(u64, usize);
no_containment_conversion!(u64, i8);
no_containment_conversion!(u64, i16);
no_containment_conversion!(u64, i32);
no_containment_conversion!(u64, i64);
no_containment_conversion!(u64, isize);
no_containment_conversion!(u128, usize);
no_containment_conversion!(u128, i8);
no_containment_conversion!(u128, i16);
no_containment_conversion!(u128, i32);
no_containment_conversion!(u128, i64);
no_containment_conversion!(u128, i128);
no_containment_conversion!(u128, isize);
no_containment_conversion!(usize, i8);
no_containment_conversion!(usize, i16);
no_containment_conversion!(usize, i32);
no_containment_conversion!(usize, i64);
no_containment_conversion!(usize, i128);
no_containment_conversion!(usize, isize);
no_containment_conversion!(i32, isize);
no_containment_conversion!(i64, isize);
no_containment_conversion!(i128, isize);
