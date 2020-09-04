use num::arithmetic::traits::XXXAddYYYIsZZZ;
use num::basic::integers::PrimitiveInt;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;

pub fn _xxx_add_yyy_is_zzz<T: PrimitiveUnsigned>(
    x_2: T,
    x_1: T,
    x_0: T,
    y_2: T,
    y_1: T,
    y_0: T,
) -> (T, T, T) {
    let (z_0, carry_1) = x_0.overflowing_add(y_0);
    let (mut z_1, mut carry_2) = x_1.overflowing_add(y_1);
    if carry_1 {
        carry_2 |= z_1.overflowing_add_assign(T::ONE);
    }
    let mut z_2 = x_2.wrapping_add(y_2);
    if carry_2 {
        z_2.wrapping_add_assign(T::ONE);
    }
    (z_2, z_1, z_0)
}

macro_rules! impl_xxx_add_yyy_is_zzz {
    ($t:ident) => {
        impl XXXAddYYYIsZZZ for $t {
            /// Adds two numbers, each composed of three `$t` values. The sum is returned as a
            /// triple of `$t` values. The more significant value always comes first. Addition is
            /// wrapping, and overflow is not indicated.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::XXXAddYYYIsZZZ;
            ///
            /// assert_eq!(
            ///     u64::xxx_add_yyy_is_zzz(0x12, 0x34, 0x56, 0x33, 0x33, 0x33),
            ///     (0x45, 0x67, 0x89)
            /// );
            /// assert_eq!(
            ///     u8::xxx_add_yyy_is_zzz(0x78, 0x9a, 0xbc, 0xde, 0xfe, 0xdc),
            ///     (0x57, 0x99, 0x98)
            /// );
            /// ```
            ///
            /// This is add_sssaaaaaa from longlong.h, FLINT Dev 1, where (sh, sm, sl) is returned.
            #[inline]
            fn xxx_add_yyy_is_zzz(
                x_2: $t,
                x_1: $t,
                x_0: $t,
                y_2: $t,
                y_1: $t,
                y_0: $t,
            ) -> ($t, $t, $t) {
                _xxx_add_yyy_is_zzz::<$t>(x_2, x_1, x_0, y_2, y_1, y_0)
            }
        }
    };
}

impl_xxx_add_yyy_is_zzz!(u8);
impl_xxx_add_yyy_is_zzz!(u16);
impl_xxx_add_yyy_is_zzz!(u32);
impl_xxx_add_yyy_is_zzz!(u64);
impl_xxx_add_yyy_is_zzz!(u128);

impl XXXAddYYYIsZZZ for usize {
    /// Adds two numbers, each composed of three `usize` values. The sum is returned as a triple of
    /// `usize` values. The more significant value always comes first. Addition is wrapping, and
    /// overflow is not indicated.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// This is add_sssaaaaaa from longlong.h, FLINT Dev 1, where (sh, sm, sl) is returned.
    fn xxx_add_yyy_is_zzz(
        x_2: usize,
        x_1: usize,
        x_0: usize,
        y_2: usize,
        y_1: usize,
        y_0: usize,
    ) -> (usize, usize, usize) {
        if usize::WIDTH == u32::WIDTH {
            let (z_2, z_1, z_0) = u32::xxx_add_yyy_is_zzz(
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
            let (z_2, z_1, z_0) = u64::xxx_add_yyy_is_zzz(
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
