use num::arithmetic::traits::XXDivModYIsQR;
use num::basic::integers::PrimitiveInt;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::half::{wide_join_halves, wide_split_in_half};
use num::conversion::traits::WrappingFrom;
use num::conversion::traits::{HasHalf, JoinHalves, SplitInHalf};
use num::logic::traits::LeadingZeros;

fn implicit_xx_div_mod_y_is_qr<
    T: PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    x_1: T,
    x_0: T,
    y: T,
) -> (T, T) {
    assert!(x_1 < y);
    let (q, r) = DT::join_halves(x_1, x_0).div_mod(DT::from(y));
    (q.lower_half(), r.lower_half())
}

/// This is udiv_qrnnd_int from longlong.h, FLINT 2.7.1, where (q, r) is returned.
fn explicit_xx_div_mod_y_is_qr_normalized<T: PrimitiveUnsigned>(x_1: T, x_0: T, y: T) -> (T, T) {
    let (d_1, d_0) = wide_split_in_half(y);
    let (x_0_1, x_0_0) = wide_split_in_half(x_0);
    let mut q_1 = x_1 / d_1;
    let mut r_1 = x_1.wrapping_sub(q_1.wrapping_mul(d_1));
    let product = q_1.wrapping_mul(d_0);
    r_1 = wide_join_halves(r_1, x_0_1);
    if r_1 < product {
        q_1.wrapping_sub_assign(T::ONE);
        if !r_1.overflowing_add_assign(y) && r_1 < product {
            q_1.wrapping_sub_assign(T::ONE);
            r_1.wrapping_add_assign(y);
        }
    }
    r_1.wrapping_sub_assign(product);
    let mut q_0 = r_1 / d_1;
    let mut r_0 = r_1.wrapping_sub(q_0.wrapping_mul(d_1));
    let product = q_0.wrapping_mul(d_0);
    r_0 = wide_join_halves(r_0, x_0_0);
    if r_0 < product {
        q_0.wrapping_sub_assign(T::ONE);
        if !r_0.overflowing_add_assign(y) && r_0 < product {
            q_0.wrapping_sub_assign(T::ONE);
            r_0.wrapping_add_assign(y);
        }
    }
    r_0.wrapping_sub_assign(product);
    (wide_join_halves(q_1, q_0), r_0)
}

/// This is udiv_qrnnd from longlong.h, FLINT 2.7.1, where (q, r) is returned.
pub fn explicit_xx_div_mod_y_is_qr<T: PrimitiveUnsigned>(x_1: T, x_0: T, y: T) -> (T, T) {
    assert!(x_1 < y);
    let shift = LeadingZeros::leading_zeros(y);
    if shift == 0 {
        explicit_xx_div_mod_y_is_qr_normalized(x_1, x_0, y)
    } else {
        let (q, r) = explicit_xx_div_mod_y_is_qr_normalized(
            x_1 << shift | (x_0 >> (T::WIDTH - shift)),
            x_0 << shift,
            y << shift,
        );
        (q, r >> shift)
    }
}

macro_rules! implicit_xx_div_mod_is_qr {
    ($t:ident, $dt:ident) => {
        impl XXDivModYIsQR for $t {
            /// Computes the quotient and remainder of two numbers. The first is composed of two
            /// `Self` values, and the second of a single one.
            ///
            /// `x_1` must be less than `y`.
            ///
            /// $$
            /// f(x_1, x_0, y) = (q, r),
            /// $$
            /// where $W$ is `Self::WIDTH`,
            ///
            /// $x_1, x_0, y, q, r < 2^W$,
            ///
            /// $x_1, r < y$, and
            /// $$
            /// qy + r = 2^Wx_1 + x_0.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::xx_div_mod_y_is_qr` module.
            ///
            /// This is udiv_qrnnd from longlong.h, FLINT 2.7.1, where  (q, r) is returned.
            #[inline]
            fn xx_div_mod_y_is_qr(x_1: $t, x_0: $t, y: $t) -> ($t, $t) {
                implicit_xx_div_mod_y_is_qr::<$t, $dt>(x_1, x_0, y)
            }
        }
    };
}

implicit_xx_div_mod_is_qr!(u8, u16);
implicit_xx_div_mod_is_qr!(u16, u32);
implicit_xx_div_mod_is_qr!(u32, u64);
implicit_xx_div_mod_is_qr!(u64, u128);

impl XXDivModYIsQR for usize {
    /// Computes the quotient and remainder of two numbers. The first is composed of two `usize`
    /// values, and the second of a single one.
    ///
    /// `x_1` must be less than `y`.
    ///
    /// $$
    /// f(x_1, x_0, y) = (q, r),
    /// $$
    /// where $W$ is `Self::WIDTH`,
    ///
    /// $x_1, x_0, y, q, r < 2^W$,
    ///
    /// $x_1, r < y$, and
    /// $$
    /// qy + r = 2^Wx_1 + x_0.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See the documentation of the `num::arithmetic::xx_div_mod_y_is_qr` module.
    ///
    /// This is udiv_qrnnd from longlong.h, FLINT 2.7.1, where (q, r) is returned.
    fn xx_div_mod_y_is_qr(x_1: usize, x_0: usize, y: usize) -> (usize, usize) {
        if usize::WIDTH == u32::WIDTH {
            let (q, r) = u32::xx_div_mod_y_is_qr(
                u32::wrapping_from(x_1),
                u32::wrapping_from(x_0),
                u32::wrapping_from(y),
            );
            (usize::wrapping_from(q), usize::wrapping_from(r))
        } else {
            let (q, r) = u64::xx_div_mod_y_is_qr(
                u64::wrapping_from(x_1),
                u64::wrapping_from(x_0),
                u64::wrapping_from(y),
            );
            (usize::wrapping_from(q), usize::wrapping_from(r))
        }
    }
}

impl XXDivModYIsQR for u128 {
    /// Computes the quotient and remainder of two numbers. The first is composed of two `u128`
    /// values, and the second of a single one.
    ///
    /// `x_1` must be less than `y`.
    ///
    /// $$
    /// f(x_1, x_0, y) = (q, r),
    /// $$
    /// where $W$ is `Self::WIDTH`,
    ///
    /// $x_1, x_0, y, q, r < 2^W$,
    ///
    /// $x_1, r < y$, and
    /// $$
    /// qy + r = 2^Wx_1 + x_0.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See the documentation of the `num::arithmetic::xx_div_mod_y_is_qr` module.
    ///
    /// This is udiv_qrnnd from longlong.h, FLINT 2.7.1, where (q, r) is returned.
    #[inline]
    fn xx_div_mod_y_is_qr(x_1: u128, x_0: u128, y: u128) -> (u128, u128) {
        explicit_xx_div_mod_y_is_qr(x_1, x_0, y)
    }
}
