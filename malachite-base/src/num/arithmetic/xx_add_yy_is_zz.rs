use num::arithmetic::traits::XXAddYYIsZZ;
use num::basic::integers::PrimitiveInt;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::{JoinHalves, SplitInHalf, WrappingFrom};

fn _implicit_xx_add_yy_is_zz<DT: JoinHalves + PrimitiveUnsigned + SplitInHalf>(
    x_1: DT::Half,
    x_0: DT::Half,
    y_1: DT::Half,
    y_0: DT::Half,
) -> (DT::Half, DT::Half) {
    DT::join_halves(x_1, x_0)
        .wrapping_add(DT::join_halves(y_1, y_0))
        .split_in_half()
}

pub fn _explicit_xx_add_yy_is_zz<T: PrimitiveUnsigned>(x_1: T, x_0: T, y_1: T, y_0: T) -> (T, T) {
    let (z_0, carry) = x_0.overflowing_add(y_0);
    let mut z_1 = x_1.wrapping_add(y_1);
    if carry {
        z_1.wrapping_add_assign(T::ONE);
    }
    (z_1, z_0)
}

macro_rules! implicit_xx_add_yy_is_zz {
    ($t:ident, $dt:ident) => {
        impl XXAddYYIsZZ for $t {
            /// Adds two numbers, each composed of two `$t` values. The sum is returned as a pair of
            /// `$t` values. The more significant value always comes first. Addition is wrapping,
            /// and overflow is not indicated.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::XXAddYYIsZZ;
            ///
            /// assert_eq!(u64::xx_add_yy_is_zz(0x12, 0x34, 0x33, 0x33), (0x45, 0x67));
            /// assert_eq!(u8::xx_add_yy_is_zz(0x78, 0x9a, 0xbc, 0xde), (0x35, 0x78));
            /// ```
            ///
            /// This is add_ssaaaa from longlong.h, GMP 6.2.1, where (sh, sl) is returned.
            #[inline]
            fn xx_add_yy_is_zz(x_1: $t, x_0: $t, y_1: $t, y_0: $t) -> ($t, $t) {
                _implicit_xx_add_yy_is_zz::<$dt>(x_1, x_0, y_1, y_0)
            }
        }
    };
}

implicit_xx_add_yy_is_zz!(u8, u16);
implicit_xx_add_yy_is_zz!(u16, u32);
implicit_xx_add_yy_is_zz!(u32, u64);
implicit_xx_add_yy_is_zz!(u64, u128);

impl XXAddYYIsZZ for usize {
    /// Adds two numbers, each composed of two `usize` values. The sum is returned as a pair of
    /// `usize` values. The more significant value always comes first. Addition is wrapping, and
    /// overflow is not indicated.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// This is add_ssaaaa from longlong.h, GMP 6.2.1, where (sh, sl) is returned.
    fn xx_add_yy_is_zz(x_1: usize, x_0: usize, y_1: usize, y_0: usize) -> (usize, usize) {
        if usize::WIDTH == u32::WIDTH {
            let (z_1, z_0) = u32::xx_add_yy_is_zz(
                u32::wrapping_from(x_1),
                u32::wrapping_from(x_0),
                u32::wrapping_from(y_1),
                u32::wrapping_from(y_0),
            );
            (usize::wrapping_from(z_1), usize::wrapping_from(z_0))
        } else {
            let (z_1, z_0) = u64::xx_add_yy_is_zz(
                u64::wrapping_from(x_1),
                u64::wrapping_from(x_0),
                u64::wrapping_from(y_1),
                u64::wrapping_from(y_0),
            );
            (usize::wrapping_from(z_1), usize::wrapping_from(z_0))
        }
    }
}

impl XXAddYYIsZZ for u128 {
    /// Adds two numbers, each composed of two `u128` values. The sum is returned as a pair of
    /// `u128` values. The more significant value always comes first. Addition is wrapping, and
    /// overflow is not indicated.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// This is add_ssaaaa from longlong.h, GMP 6.2.1, where (sh, sl) is returned.
    #[inline]
    fn xx_add_yy_is_zz(x_1: u128, x_0: u128, y_1: u128, y_0: u128) -> (u128, u128) {
        _explicit_xx_add_yy_is_zz(x_1, x_0, y_1, y_0)
    }
}
