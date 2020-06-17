use num::arithmetic::traits::XMulYIsZZ;
use num::basic::integers::PrimitiveInteger;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::half::{wide_join_halves, wide_split_in_half, wide_upper_half};
use num::conversion::traits::{HasHalf, SplitInHalf, WrappingFrom};

fn _implicit_x_mul_y_is_zz<T, DT: PrimitiveUnsigned + SplitInHalf>(x: T, y: T) -> (T, T)
where
    DT: From<T> + HasHalf<Half = T>,
{
    (DT::from(x) * DT::from(y)).split_in_half()
}

pub fn _explicit_x_mul_y_is_zz<T: PrimitiveUnsigned>(x: T, y: T) -> (T, T) {
    let (x_1, x_0) = wide_split_in_half(x);
    let (y_1, y_0) = wide_split_in_half(y);
    let x_0_y_0 = x_0 * y_0;
    let mut x_0_y_1 = x_0 * y_1;
    let x_1_y_0 = x_1 * y_0;
    let mut x_1_y_1 = x_1 * y_1;
    let (x_0_y_0_1, x_0_y_0_0) = wide_split_in_half(x_0_y_0);
    x_0_y_1.wrapping_add_assign(x_0_y_0_1);
    if x_0_y_1.overflowing_add_assign(x_1_y_0) {
        x_1_y_1.wrapping_add_assign(T::power_of_two(T::WIDTH >> 1));
    }
    let z_1 = x_1_y_1.wrapping_add(wide_upper_half(x_0_y_1));
    let z_0 = wide_join_halves(x_0_y_1, x_0_y_0_0);
    (z_1, z_0)
}

macro_rules! implicit_x_mul_y_is_zz {
    ($t:ident, $dt:ident) => {
        impl XMulYIsZZ for $t {
            /// Multiplies two numbers, returning the product as a pair of `Self` values. The more
            /// significant value always comes first.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::XMulYIsZZ;
            ///
            /// assert_eq!(u64::x_mul_y_is_zz(15, 3), (0, 45));
            /// assert_eq!(u8::x_mul_y_is_zz(0x78, 0x9a), (0x48, 0x30));
            /// ```
            ///
            /// This is umul_ppmm from longlong.h, GMP 6.1.2, where (w1, w0) is returned.
            #[inline]
            fn x_mul_y_is_zz(x: $t, y: $t) -> ($t, $t) {
                _implicit_x_mul_y_is_zz::<$t, $dt>(x, y)
            }
        }
    };
}

implicit_x_mul_y_is_zz!(u8, u16);
implicit_x_mul_y_is_zz!(u16, u32);
implicit_x_mul_y_is_zz!(u32, u64);
implicit_x_mul_y_is_zz!(u64, u128);

impl XMulYIsZZ for usize {
    /// Multiplies two `usize`s, returning the product as a pair of `usize` values. The more
    /// significant value always comes first.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// This is umul_ppmm from longlong.h, GMP 6.1.2, where (w1, w0) is returned.
    fn x_mul_y_is_zz(x: usize, y: usize) -> (usize, usize) {
        if usize::WIDTH == u32::WIDTH {
            let (z_1, z_0) = u32::x_mul_y_is_zz(u32::wrapping_from(x), u32::wrapping_from(y));
            (usize::wrapping_from(z_1), usize::wrapping_from(z_0))
        } else {
            let (z_1, z_0) = u64::x_mul_y_is_zz(u64::wrapping_from(x), u64::wrapping_from(y));
            (usize::wrapping_from(z_1), usize::wrapping_from(z_0))
        }
    }
}

impl XMulYIsZZ for u128 {
    /// Multiplies two `u128`s, returning the product as a pair of `u128` values. The more
    /// significant value always comes first.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// This is umul_ppmm from longlong.h, GMP 6.1.2, where (w1, w0) is returned.
    #[inline]
    fn x_mul_y_is_zz(x: u128, y: u128) -> (u128, u128) {
        _explicit_x_mul_y_is_zz(x, y)
    }
}
