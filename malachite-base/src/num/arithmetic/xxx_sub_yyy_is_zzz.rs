use num::arithmetic::traits::XXXSubYYYIsZZZ;
use num::basic::integers::PrimitiveInt;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;

pub fn _xxx_sub_yyy_is_zzz<T: PrimitiveUnsigned>(
    x_2: T,
    x_1: T,
    x_0: T,
    y_2: T,
    y_1: T,
    y_0: T,
) -> (T, T, T) {
    let (z_0, borrow_1) = x_0.overflowing_sub(y_0);
    let (mut z_1, mut borrow_2) = x_1.overflowing_sub(y_1);
    if borrow_1 {
        borrow_2 |= z_1.overflowing_sub_assign(T::ONE);
    }
    let mut z_2 = x_2.wrapping_sub(y_2);
    if borrow_2 {
        z_2.wrapping_sub_assign(T::ONE);
    }
    (z_2, z_1, z_0)
}

macro_rules! impl_xxx_sub_yyy_is_zzz {
    ($t:ident) => {
        impl XXXSubYYYIsZZZ for $t {
            /// Subtracts two numbers, each composed of three `$t` values. The difference is
            /// returned as a triple of `$t` values. The more significant value always comes first.
            /// Subtraction is wrapping, and overflow is not indicated.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::XXXSubYYYIsZZZ;
            ///
            /// assert_eq!(
            ///     u64::xxx_sub_yyy_is_zzz(0x67, 0x89, 0xab, 0x33, 0x33, 0x33),
            ///     (0x34, 0x56, 0x78)
            /// );
            /// assert_eq!(
            ///     u8::xxx_sub_yyy_is_zzz(0x78, 0x9a, 0xbc, 0xde, 0xfe, 0xdc),
            ///     (0x99, 0x9b, 0xe0)
            /// );
            /// ```
            ///
            /// This is sub_dddmmmsss from longlong.h, FLINT 2.7.1, where (dh, dm, dl) is returned.
            #[inline]
            fn xxx_sub_yyy_is_zzz(
                x_2: $t,
                x_1: $t,
                x_0: $t,
                y_2: $t,
                y_1: $t,
                y_0: $t,
            ) -> ($t, $t, $t) {
                _xxx_sub_yyy_is_zzz::<$t>(x_2, x_1, x_0, y_2, y_1, y_0)
            }
        }
    };
}

impl_xxx_sub_yyy_is_zzz!(u8);
impl_xxx_sub_yyy_is_zzz!(u16);
impl_xxx_sub_yyy_is_zzz!(u32);
impl_xxx_sub_yyy_is_zzz!(u64);
impl_xxx_sub_yyy_is_zzz!(u128);

impl XXXSubYYYIsZZZ for usize {
    /// Subtracts two numbers, each composed of three `usize` values. The difference is returned as
    /// a triple of `usize` values. The more significant value always comes first. Subtraction is
    /// wrapping, and overflow is not indicated.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// This is sub_dddmmmsss from longlong.h, FLINT 2.7.1, where (dh, dm, dl) is returned.
    fn xxx_sub_yyy_is_zzz(
        x_2: usize,
        x_1: usize,
        x_0: usize,
        y_2: usize,
        y_1: usize,
        y_0: usize,
    ) -> (usize, usize, usize) {
        if usize::WIDTH == u32::WIDTH {
            let (z_2, z_1, z_0) = u32::xxx_sub_yyy_is_zzz(
                u32::wrapping_from(x_2),
                u32::wrapping_from(x_1),
                u32::wrapping_from(x_0),
                u32::wrapping_from(y_2),
                u32::wrapping_from(y_1),
                u32::wrapping_from(y_0),
            );
            (
                usize::wrapping_from(z_2),
                usize::wrapping_from(z_1),
                usize::wrapping_from(z_0),
            )
        } else {
            let (z_2, z_1, z_0) = u64::xxx_sub_yyy_is_zzz(
                u64::wrapping_from(x_2),
                u64::wrapping_from(x_1),
                u64::wrapping_from(x_0),
                u64::wrapping_from(y_2),
                u64::wrapping_from(y_1),
                u64::wrapping_from(y_0),
            );
            (
                usize::wrapping_from(z_2),
                usize::wrapping_from(z_1),
                usize::wrapping_from(z_0),
            )
        }
    }
}
