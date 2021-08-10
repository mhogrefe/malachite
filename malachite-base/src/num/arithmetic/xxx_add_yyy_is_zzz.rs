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
            /// Adds two numbers, each composed of three `Self` values, returning the sum as a
            /// triple of `Self` values.
            ///
            /// The more significant value always comes first. Addition is wrapping, and overflow
            /// is not indicated.
            ///
            /// $$
            /// f(x_2, x_1, x_0, y_2, y_1, y_0) = (z_2, z_1, z_0),
            /// $$
            /// where $W$ is `Self::WIDTH`,
            ///
            /// $x_2, x_1, x_0, y_2, y_1, y_0, z_2, z_1, z_0 < 2^W$, and
            /// $$
            /// (2^{2W}x_2 + 2^Wx_1 + x_0) + (2^{2W}y_2 + 2^Wy_1 + y_0)
            /// \equiv 2^{2W}z_2 + 2^Wz_1 + z_0 \mod 2^{3W}.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::xxx_add_yyy_is_zzz` module.
            ///
            /// This is add_sssaaaaaa from longlong.h, FLINT 2.7.1, where (sh, sm, sl) is returned.
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
    /// Adds two numbers, each composed of three `usize` values, returning the sum as a triple of
    /// `usize` values.
    ///
    /// The more significant value always comes first. Addition is wrapping, and overflow is not
    /// indicated.
    ///
    /// $$
    /// f(x_2, x_1, x_0, y_2, y_1, y_0) = (z_2, z_1, z_0),
    /// $$
    /// where $W$ is `Self::WIDTH`,
    ///
    /// $x_2, x_1, x_0, y_2, y_1, y_0, z_2, z_1, z_0 < 2^W$, and
    /// $$
    /// (2^{2W}x_2 + 2^Wx_1 + x_0) + (2^{2W}y_2 + 2^Wy_1 + y_0)
    /// \equiv 2^{2W}z_2 + 2^Wz_1 + z_0 \mod 2^{3W}.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See the documentation of the `num::arithmetic::xxx_add_yyy_is_zzz` module.
    ///
    /// This is add_sssaaaaaa from longlong.h, FLINT 2.7.1, where (sh, sm, sl) is returned.
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
