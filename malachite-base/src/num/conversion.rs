use conversion::{CheckedFrom, WrappingFrom};

macro_rules! lossless_checked_from_impl {
    ($from:ty, $to:ty) => {
        impl CheckedFrom<$from> for $to {
            #[inline]
            fn checked_from(value: $from) -> Option<$to> {
                Some(value.into())
            }
        }
    };
}

macro_rules! lossy_checked_from_impl_a {
    ($from:ident, $to:ty) => {
        impl CheckedFrom<$from> for $to {
            #[allow(unused_comparisons)]
            #[inline]
            fn checked_from(value: $from) -> Option<$to> {
                let result = value as $to;
                if (result < 0) == (value < 0) && $from::from(result) == value {
                    Some(result)
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! lossy_checked_from_impl_b {
    ($from:ident, $to:ty) => {
        impl CheckedFrom<$from> for $to {
            #[allow(unused_comparisons, clippy::cast_lossless)]
            #[inline]
            fn checked_from(value: $from) -> Option<$to> {
                let result = value as $to;
                if (result < 0) == (value < 0) && result as $from == value {
                    Some(result)
                } else {
                    None
                }
            }
        }
    };
}

lossless_checked_from_impl!(u8, u8);
lossless_checked_from_impl!(u8, u16);
lossless_checked_from_impl!(u8, u32);
lossless_checked_from_impl!(u8, u64);
lossless_checked_from_impl!(u8, u128);
lossless_checked_from_impl!(u8, usize);
lossy_checked_from_impl_b!(u8, i8);
lossless_checked_from_impl!(u8, i16);
lossless_checked_from_impl!(u8, i32);
lossless_checked_from_impl!(u8, i64);
lossless_checked_from_impl!(u8, i128);
lossless_checked_from_impl!(u8, isize);
lossy_checked_from_impl_a!(u16, u8);
lossless_checked_from_impl!(u16, u16);
lossless_checked_from_impl!(u16, u32);
lossless_checked_from_impl!(u16, u64);
lossless_checked_from_impl!(u16, u128);
lossless_checked_from_impl!(u16, usize);
lossy_checked_from_impl_b!(u16, i8);
lossy_checked_from_impl_b!(u16, i16);
lossless_checked_from_impl!(u16, i32);
lossless_checked_from_impl!(u16, i64);
lossless_checked_from_impl!(u16, i128);
lossy_checked_from_impl_b!(u16, isize);
lossy_checked_from_impl_a!(u32, u8);
lossy_checked_from_impl_a!(u32, u16);
lossless_checked_from_impl!(u32, u32);
lossless_checked_from_impl!(u32, u64);
lossless_checked_from_impl!(u32, u128);
lossy_checked_from_impl_b!(u32, usize);
lossy_checked_from_impl_b!(u32, i8);
lossy_checked_from_impl_b!(u32, i16);
lossy_checked_from_impl_b!(u32, i32);
lossless_checked_from_impl!(u32, i64);
lossless_checked_from_impl!(u32, i128);
lossy_checked_from_impl_b!(u32, isize);
lossy_checked_from_impl_a!(u64, u8);
lossy_checked_from_impl_a!(u64, u16);
lossy_checked_from_impl_a!(u64, u32);
lossless_checked_from_impl!(u64, u64);
lossless_checked_from_impl!(u64, u128);
lossy_checked_from_impl_b!(u64, usize);
lossy_checked_from_impl_b!(u64, i8);
lossy_checked_from_impl_b!(u64, i16);
lossy_checked_from_impl_b!(u64, i32);
lossy_checked_from_impl_b!(u64, i64);
lossless_checked_from_impl!(u64, i128);
lossy_checked_from_impl_b!(u64, isize);
lossy_checked_from_impl_a!(u128, u8);
lossy_checked_from_impl_a!(u128, u16);
lossy_checked_from_impl_a!(u128, u32);
lossy_checked_from_impl_a!(u128, u64);
lossless_checked_from_impl!(u128, u128);
lossy_checked_from_impl_b!(u128, usize);
lossy_checked_from_impl_b!(u128, i8);
lossy_checked_from_impl_b!(u128, i16);
lossy_checked_from_impl_b!(u128, i32);
lossy_checked_from_impl_b!(u128, i64);
lossy_checked_from_impl_b!(u128, i128);
lossy_checked_from_impl_b!(u128, isize);
lossy_checked_from_impl_a!(usize, u8);
lossy_checked_from_impl_a!(usize, u16);
lossy_checked_from_impl_b!(usize, u32);
lossy_checked_from_impl_b!(usize, u64);
lossy_checked_from_impl_b!(usize, u128);
lossless_checked_from_impl!(usize, usize);
lossy_checked_from_impl_b!(usize, i8);
lossy_checked_from_impl_b!(usize, i16);
lossy_checked_from_impl_b!(usize, i32);
lossy_checked_from_impl_b!(usize, i64);
lossy_checked_from_impl_b!(usize, i128);
lossy_checked_from_impl_b!(usize, isize);
lossy_checked_from_impl_b!(i8, u8);
lossy_checked_from_impl_b!(i8, u16);
lossy_checked_from_impl_b!(i8, u32);
lossy_checked_from_impl_b!(i8, u64);
lossy_checked_from_impl_b!(i8, u128);
lossy_checked_from_impl_b!(i8, usize);
lossless_checked_from_impl!(i8, i8);
lossless_checked_from_impl!(i8, i16);
lossless_checked_from_impl!(i8, i32);
lossless_checked_from_impl!(i8, i64);
lossless_checked_from_impl!(i8, i128);
lossless_checked_from_impl!(i8, isize);
lossy_checked_from_impl_a!(i16, u8);
lossy_checked_from_impl_b!(i16, u16);
lossy_checked_from_impl_b!(i16, u32);
lossy_checked_from_impl_b!(i16, u64);
lossy_checked_from_impl_b!(i16, u128);
lossy_checked_from_impl_b!(i16, usize);
lossy_checked_from_impl_a!(i16, i8);
lossless_checked_from_impl!(i16, i16);
lossless_checked_from_impl!(i16, i32);
lossless_checked_from_impl!(i16, i64);
lossless_checked_from_impl!(i16, i128);
lossless_checked_from_impl!(i16, isize);
lossy_checked_from_impl_a!(i32, u8);
lossy_checked_from_impl_a!(i32, u16);
lossy_checked_from_impl_b!(i32, u32);
lossy_checked_from_impl_b!(i32, u64);
lossy_checked_from_impl_b!(i32, u128);
lossy_checked_from_impl_b!(i32, usize);
lossy_checked_from_impl_a!(i32, i8);
lossy_checked_from_impl_a!(i32, i16);
lossless_checked_from_impl!(i32, i32);
lossless_checked_from_impl!(i32, i64);
lossless_checked_from_impl!(i32, i128);
lossy_checked_from_impl_b!(i32, isize);
lossy_checked_from_impl_a!(i64, u8);
lossy_checked_from_impl_a!(i64, u16);
lossy_checked_from_impl_a!(i64, u32);
lossy_checked_from_impl_b!(i64, u64);
lossy_checked_from_impl_b!(i64, u128);
lossy_checked_from_impl_b!(i64, usize);
lossy_checked_from_impl_a!(i64, i8);
lossy_checked_from_impl_a!(i64, i16);
lossy_checked_from_impl_a!(i64, i32);
lossless_checked_from_impl!(i64, i64);
lossless_checked_from_impl!(i64, i128);
lossy_checked_from_impl_b!(i64, isize);
lossy_checked_from_impl_a!(i128, u8);
lossy_checked_from_impl_a!(i128, u16);
lossy_checked_from_impl_a!(i128, u32);
lossy_checked_from_impl_b!(i128, u64);
lossy_checked_from_impl_b!(i128, u128);
lossy_checked_from_impl_b!(i128, usize);
lossy_checked_from_impl_a!(i128, i8);
lossy_checked_from_impl_a!(i128, i16);
lossy_checked_from_impl_a!(i128, i32);
lossy_checked_from_impl_a!(i128, i64);
lossless_checked_from_impl!(i128, i128);
lossy_checked_from_impl_b!(i128, isize);
lossy_checked_from_impl_a!(isize, u8);
lossy_checked_from_impl_b!(isize, u16);
lossy_checked_from_impl_b!(isize, u32);
lossy_checked_from_impl_b!(isize, u64);
lossy_checked_from_impl_b!(isize, u128);
lossy_checked_from_impl_b!(isize, usize);
lossy_checked_from_impl_a!(isize, i8);
lossy_checked_from_impl_a!(isize, i16);
lossy_checked_from_impl_b!(isize, i32);
lossy_checked_from_impl_b!(isize, i64);
lossy_checked_from_impl_b!(isize, i128);
lossless_checked_from_impl!(isize, isize);

macro_rules! wrapping_impl_inner {
    ($from:ty, $to:ty) => {
        #[allow(clippy::cast_lossless)]
        impl WrappingFrom<$from> for $to {
            #[inline]
            fn wrapping_from(value: $from) -> $to {
                value as $to
            }
        }
    };
}

macro_rules! wrapping_impl {
    ($from:ty) => {
        wrapping_impl_inner!($from, u8);
        wrapping_impl_inner!($from, u16);
        wrapping_impl_inner!($from, u32);
        wrapping_impl_inner!($from, u64);
        wrapping_impl_inner!($from, u128);
        wrapping_impl_inner!($from, usize);
        wrapping_impl_inner!($from, i8);
        wrapping_impl_inner!($from, i16);
        wrapping_impl_inner!($from, i32);
        wrapping_impl_inner!($from, i64);
        wrapping_impl_inner!($from, i128);
        wrapping_impl_inner!($from, isize);
    };
}

wrapping_impl!(u8);
wrapping_impl!(u16);
wrapping_impl!(u32);
wrapping_impl!(u64);
wrapping_impl!(u128);
wrapping_impl!(usize);
wrapping_impl!(i8);
wrapping_impl!(i16);
wrapping_impl!(i32);
wrapping_impl!(i64);
wrapping_impl!(i128);
wrapping_impl!(isize);
