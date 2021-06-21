use num::arithmetic::traits::XXXXAddYYYYIsZZZZ;
use num::basic::integers::PrimitiveInt;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;

#[allow(clippy::too_many_arguments)]
pub fn _xxxx_add_yyyy_is_zzzz<T: PrimitiveUnsigned>(
    x_3: T,
    x_2: T,
    x_1: T,
    x_0: T,
    y_3: T,
    y_2: T,
    y_1: T,
    y_0: T,
) -> (T, T, T, T) {
    let (z_0, carry_1) = x_0.overflowing_add(y_0);
    let (mut z_1, mut carry_2) = x_1.overflowing_add(y_1);
    if carry_1 {
        carry_2 |= z_1.overflowing_add_assign(T::ONE);
    }
    let (mut z_2, mut carry_3) = x_2.overflowing_add(y_2);
    if carry_2 {
        carry_3 |= z_2.overflowing_add_assign(T::ONE);
    }
    let mut z_3 = x_3.wrapping_add(y_3);
    if carry_3 {
        z_3.wrapping_add_assign(T::ONE);
    }
    (z_3, z_2, z_1, z_0)
}

macro_rules! impl_xxxx_add_yyyy_is_zzzz {
    ($t:ident) => {
        impl XXXXAddYYYYIsZZZZ for $t {
            /// Adds two numbers, each composed of four `$t` values. The sum is returned as a
            /// quadruple of `$t` values. The more significant value always comes first. Addition is
            /// wrapping, and overflow is not indicated.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::XXXXAddYYYYIsZZZZ;
            ///
            /// assert_eq!(
            ///     u64::xxxx_add_yyyy_is_zzzz(0x12, 0x34, 0x56, 0x78, 0x33, 0x33, 0x33, 0x33),
            ///     (0x45, 0x67, 0x89, 0xab)
            /// );
            /// assert_eq!(
            ///     u8::xxxx_add_yyyy_is_zzzz(0x78, 0x9a, 0xbc, 0xde, 0xfe, 0xdc, 0xba, 0x98),
            ///     (0x77, 0x77, 0x77, 0x76)
            /// );
            /// ```
            ///
            /// This is add_ssssaaaaaaaa from longlong.h, FLINT 2.7.1, where (s3, s2, s1, s0) is
            /// returned.
            #[inline]
            fn xxxx_add_yyyy_is_zzzz(
                x_3: $t,
                x_2: $t,
                x_1: $t,
                x_0: $t,
                y_3: $t,
                y_2: $t,
                y_1: $t,
                y_0: $t,
            ) -> ($t, $t, $t, $t) {
                _xxxx_add_yyyy_is_zzzz::<$t>(x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0)
            }
        }
    };
}

impl_xxxx_add_yyyy_is_zzzz!(u8);
impl_xxxx_add_yyyy_is_zzzz!(u16);
impl_xxxx_add_yyyy_is_zzzz!(u32);
impl_xxxx_add_yyyy_is_zzzz!(u64);
impl_xxxx_add_yyyy_is_zzzz!(u128);

impl XXXXAddYYYYIsZZZZ for usize {
    /// Adds two numbers, each composed of four `usize` values. The sum is returned as a quadruple
    /// of `usize` values. The more significant value always comes first. Addition is wrapping, and
    /// overflow is not indicated.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// This is add_ssssaaaaaaaa from longlong.h, FLINT 2.7.1, where (s3, s2, s1, s0) is returned.
    fn xxxx_add_yyyy_is_zzzz(
        x_3: usize,
        x_2: usize,
        x_1: usize,
        x_0: usize,
        y_3: usize,
        y_2: usize,
        y_1: usize,
        y_0: usize,
    ) -> (usize, usize, usize, usize) {
        if usize::WIDTH == u32::WIDTH {
            let (z_3, z_2, z_1, z_0) = u32::xxxx_add_yyyy_is_zzzz(
                u32::wrapping_from(x_3),
                u32::wrapping_from(x_2),
                u32::wrapping_from(x_1),
                u32::wrapping_from(x_0),
                u32::wrapping_from(y_3),
                u32::wrapping_from(y_2),
                u32::wrapping_from(y_1),
                u32::wrapping_from(y_0),
            );
            (
                usize::wrapping_from(z_3),
                usize::wrapping_from(z_2),
                usize::wrapping_from(z_1),
                usize::wrapping_from(z_0),
            )
        } else {
            let (z_3, z_2, z_1, z_0) = u64::xxxx_add_yyyy_is_zzzz(
                u64::wrapping_from(x_3),
                u64::wrapping_from(x_2),
                u64::wrapping_from(x_1),
                u64::wrapping_from(x_0),
                u64::wrapping_from(y_3),
                u64::wrapping_from(y_2),
                u64::wrapping_from(y_1),
                u64::wrapping_from(y_0),
            );
            (
                usize::wrapping_from(z_3),
                usize::wrapping_from(z_2),
                usize::wrapping_from(z_1),
                usize::wrapping_from(z_0),
            )
        }
    }
}
