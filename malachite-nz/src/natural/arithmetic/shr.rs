use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::vecs::vec_delete_left;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` right-shifted by a `Limb`, rounding down.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(1, `xs.len()` - `bits` / Limb::WIDTH)
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::shr::limbs_shr;
///
/// assert_eq!(limbs_shr(&[1], 1), &[0]);
/// assert_eq!(limbs_shr(&[3], 1), &[1]);
/// assert_eq!(limbs_shr(&[122, 456], 1), &[61, 228]);
/// assert_eq!(limbs_shr(&[123, 456], 1), &[61, 228]);
/// assert_eq!(limbs_shr(&[123, 455], 1), &[2147483709, 227]);
/// assert_eq!(limbs_shr(&[123, 456], 31), &[912, 0]);
/// assert_eq!(limbs_shr(&[123, 456], 32), &[456]);
/// assert_eq!(limbs_shr(&[123, 456], 100), Vec::<u32>::new());
/// assert_eq!(limbs_shr(&[256, 456], 8), &[3355443201, 1]);
/// assert_eq!(limbs_shr(&[u32::MAX, 1], 1), &[u32::MAX, 0]);
/// assert_eq!(limbs_shr(&[u32::MAX, u32::MAX], 32), &[u32::MAX]);
/// ```
///
/// This is mpn_rshift from mpn/generic/rshift.c, GMP 6.2.1, where the result is returned.
pub fn limbs_shr(xs: &[Limb], bits: u64) -> Vec<Limb> {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        Vec::new()
    } else {
        let mut out = xs[delete_count..].to_vec();
        let small_bits = bits & Limb::WIDTH_MASK;
        if small_bits != 0 {
            limbs_slice_shr_in_place(&mut out, small_bits);
        }
        out
    }
}

/// Interpreting a nonempty slice of `Limb`s as the limbs (in ascending order) of a `Natural`,
/// writes the limbs of the `Natural` right-shifted by a `Limb` to an output slice. The output slice
/// must be at least as long as the input slice. The `Limb` must be between 1 and `Limb::WIDTH` - 1,
/// inclusive. The carry, or the bits that are shifted past the width of the input slice, is
/// returned. The input slice should not only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty, `out` is shorter than `xs`, `bits` is 0, or `bits` is greater than or
/// equal to `Limb::WIDTH`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::shr::limbs_shr_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_shr_to_out(&mut out, &[123, 456], 1), 2147483648);
/// assert_eq!(out, &[61, 228, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_shr_to_out(&mut out, &[122, 455], 1), 0);
/// assert_eq!(out, &[2147483709, 227, 0]);
/// ```
///
/// This is mpn_rshift from mpn/generic/rshift.c, GMP 6.2.1.
pub fn limbs_shr_to_out(out: &mut [Limb], xs: &[Limb], bits: u64) -> Limb {
    let len = xs.len();
    assert_ne!(len, 0);
    assert_ne!(bits, 0);
    assert!(bits < Limb::WIDTH);
    assert!(out.len() >= len);
    let cobits = Limb::WIDTH - bits;
    let (xs_head, xs_tail) = xs.split_first().unwrap();
    let remaining_bits = xs_head << cobits;
    let mut previous_x = xs_head >> bits;
    let (out_last, out_init) = out[..len].split_last_mut().unwrap();
    for (out, x) in out_init.iter_mut().zip(xs_tail.iter()) {
        *out = previous_x | (x << cobits);
        previous_x = x >> bits;
    }
    *out_last = previous_x;
    remaining_bits
}

/// Interpreting a nonempty slice of `Limb`s as the limbs (in ascending order) of a `Natural`,
/// writes the limbs of the `Natural` right-shifted by a `Limb` to the input slice. The `Limb` must
/// be between 1 and `Limb::WIDTH` - 1, inclusive. The carry, or the bits that are shifted past the
/// width of the input slice, is returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty, `bits` is 0, or `bits` is greater than or equal to `Limb::WIDTH`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::shr::limbs_slice_shr_in_place;
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_slice_shr_in_place(&mut xs, 1), 2147483648);
/// assert_eq!(xs, &[61, 228]);
///
/// let mut xs = vec![122, 455];
/// assert_eq!(limbs_slice_shr_in_place(&mut xs, 1), 0);
/// assert_eq!(xs, &[2147483709, 227]);
/// ```
///
/// This is mpn_rshift from mpn/generic/rshift.c, GMP 6.2.1, where the rp == up.
pub fn limbs_slice_shr_in_place(xs: &mut [Limb], bits: u64) -> Limb {
    assert_ne!(bits, 0);
    assert!(bits < Limb::WIDTH);
    let len = xs.len();
    assert_ne!(len, 0);
    let cobits = Limb::WIDTH - bits;
    let mut x = xs[0];
    let remaining_bits = x << cobits;
    let mut previous_x = x >> bits;
    for i in 1..len {
        x = xs[i];
        xs[i - 1] = previous_x | (x << cobits);
        previous_x = x >> bits;
    }
    *xs.last_mut().unwrap() = previous_x;
    remaining_bits
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` right-shifted by a `Limb` to the input `Vec`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(1, `xs.len()` - `bits` / Limb::WIDTH)
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::shr::limbs_vec_shr_in_place;
///
/// let mut xs = vec![1];
/// limbs_vec_shr_in_place(&mut xs, 1);
/// assert_eq!(xs, &[0]);
///
/// let mut xs = vec![3];
/// limbs_vec_shr_in_place(&mut xs, 1);
/// assert_eq!(xs, &[1]);
///
/// let mut xs = vec![122, 456];
/// limbs_vec_shr_in_place(&mut xs, 1);
/// assert_eq!(xs, &[61, 228]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_in_place(&mut xs, 1);
/// assert_eq!(xs, &[61, 228]);
///
/// let mut xs = vec![123, 455];
/// limbs_vec_shr_in_place(&mut xs, 1);
/// assert_eq!(xs, &[2147483709, 227]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_in_place(&mut xs, 31);
/// assert_eq!(xs, &[912, 0]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_in_place(&mut xs, 32);
/// assert_eq!(xs, &[456]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shr_in_place(&mut xs, 100);
/// assert_eq!(xs, Vec::<u32>::new());
///
/// let mut xs = vec![256, 456];
/// limbs_vec_shr_in_place(&mut xs, 8);
/// assert_eq!(xs, &[3355443201, 1]);
///
/// let mut xs = vec![u32::MAX, 1];
/// limbs_vec_shr_in_place(&mut xs, 1);
/// assert_eq!(xs, &[u32::MAX, 0]);
///
/// let mut xs = vec![u32::MAX, u32::MAX];
/// limbs_vec_shr_in_place(&mut xs, 32);
/// assert_eq!(xs, &[u32::MAX]);
/// ```
///
/// This is mpn_rshift from mpn/generic/rshift.c, GMP 6.2.1, where rp == up and if cnt is
/// sufficiently large, limbs are removed from rp.
pub fn limbs_vec_shr_in_place(xs: &mut Vec<Limb>, bits: u64) {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        xs.clear();
    } else {
        let small_shift = bits & Limb::WIDTH_MASK;
        vec_delete_left(xs, delete_count);
        if small_shift != 0 {
            limbs_slice_shr_in_place(xs, small_shift);
        }
    }
}

fn shr_unsigned_ref<T: Copy + Eq + Ord + WrappingFrom<u64> + Zero>(x: &Natural, bits: T) -> Natural
where
    u64: ExactFrom<T>,
    Limb: Shr<T, Output = Limb>,
{
    match (x, bits) {
        (natural_zero!(), _) => x.clone(),
        (_, bits) if bits == T::ZERO => x.clone(),
        (Natural(Small(_)), bits) if bits >= T::wrapping_from(Limb::WIDTH) => Natural::ZERO,
        (Natural(Small(small)), bits) => Natural(Small(*small >> bits)),
        (Natural(Large(ref limbs)), bits) => {
            Natural::from_owned_limbs_asc(limbs_shr(limbs, u64::exact_from(bits)))
        }
    }
}

fn shr_assign_unsigned<T: Copy + Eq + Ord + WrappingFrom<u64> + Zero>(x: &mut Natural, bits: T)
where
    u64: ExactFrom<T>,
    Limb: ShrAssign<T>,
{
    match (&mut *x, bits) {
        (natural_zero!(), _) => {}
        (_, bits) if bits == T::ZERO => {}
        (Natural(Small(ref mut small)), bits) if bits >= T::wrapping_from(Limb::WIDTH) => {
            *small = 0;
        }
        (Natural(Small(ref mut small)), bits) => {
            *small >>= bits;
        }
        (Natural(Large(ref mut limbs)), bits) => {
            limbs_vec_shr_in_place(limbs, u64::exact_from(bits));
            x.trim();
        }
    }
}

macro_rules! impl_natural_shr_unsigned {
    ($t:ident) => {
        impl Shr<$t> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor), taking
            /// the `Natural` by value.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = max(1, `self.significant_bits()` - `bits`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((Natural::ZERO >> 10u8).to_string(), "0");
            /// assert_eq!((Natural::from(492u32) >> 2u32).to_string(), "123");
            /// assert_eq!((Natural::trillion() >> 10u64).to_string(), "976562500");
            /// ```
            #[inline]
            fn shr(mut self, bits: $t) -> Natural {
                self >>= bits;
                self
            }
        }

        impl<'a> Shr<$t> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor), taking
            /// the `Natural` by reference.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = max(1, `self.significant_bits()` - `bits`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((&Natural::ZERO >> 10u8).to_string(), "0");
            /// assert_eq!((&Natural::from(492u32) >> 2u32).to_string(), "123");
            /// assert_eq!((&Natural::trillion() >> 10u64).to_string(), "976562500");
            /// ```
            #[inline]
            fn shr(self, bits: $t) -> Natural {
                shr_unsigned_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for Natural {
            /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor) in place.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = max(1, `self.significant_bits()` - `bits`)
            ///
            /// # Examples
            /// ```
            /// use malachite_nz::natural::Natural;
            ///
            /// let mut x = Natural::from(1024u32);
            /// x >>= 1u8;
            /// x >>= 2u16;
            /// x >>= 3u32;
            /// x >>= 4u64;
            /// assert_eq!(x.to_string(), "1");
            /// ```
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_assign_unsigned(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_natural_shr_unsigned);

fn shr_signed_ref<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Natural,
    bits: S,
) -> Natural
where
    &'a Natural: Shl<U, Output = Natural> + Shr<U, Output = Natural>,
{
    if bits >= S::ZERO {
        x >> bits.unsigned_abs()
    } else {
        x << bits.unsigned_abs()
    }
}

fn shr_assign_signed<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(x: &mut Natural, bits: S)
where
    Natural: ShlAssign<U> + ShrAssign<U>,
{
    if bits >= S::ZERO {
        *x >>= bits.unsigned_abs();
    } else {
        *x <<= bits.unsigned_abs();
    }
}

macro_rules! impl_natural_shr_signed {
    ($t:ident) => {
        impl Shr<$t> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2), taking the `Natural` by value.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(`bits`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((Natural::ZERO >> 10i8).to_string(), "0");
            /// assert_eq!((Natural::from(492u32) >> 2i16).to_string(), "123");
            /// assert_eq!((Natural::trillion() >> 10i32).to_string(), "976562500");
            /// assert_eq!((Natural::ZERO >> -10i64).to_string(), "0");
            /// assert_eq!((Natural::from(123u32) >> -2i8).to_string(), "492");
            /// assert_eq!((Natural::from(123u32) >> -100i16).to_string(),
            ///     "155921023828072216384094494261248");
            /// ```
            #[inline]
            fn shr(mut self, bits: $t) -> Natural {
                self >>= bits;
                self
            }
        }

        impl<'a> Shr<$t> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2), taking the `Natural` by reference.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(`bits`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((&Natural::ZERO >> -10i8).to_string(), "0");
            /// assert_eq!((&Natural::from(123u32) >> -2i16).to_string(), "492");
            /// assert_eq!((&Natural::from(123u32) >> -100i32).to_string(),
            ///     "155921023828072216384094494261248");
            /// assert_eq!((&Natural::ZERO >> 10i64).to_string(), "0");
            /// assert_eq!((&Natural::from(492u32) >> 2i8).to_string(), "123");
            /// assert_eq!((&Natural::trillion() >> 10i16).to_string(), "976562500");
            /// ```
            #[inline]
            fn shr(self, bits: $t) -> Natural {
                shr_signed_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for Natural {
            /// Shifts a `Natural` right (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2) in place.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(`bits`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::One;
            /// use malachite_nz::natural::Natural;
            ///
            /// let mut x = Natural::ONE;
            /// x >>= -1i8;
            /// x >>= -2i16;
            /// x >>= -3i32;
            /// x >>= -4i64;
            /// assert_eq!(x.to_string(), "1024");
            ///
            /// let mut x = Natural::from(1024u32);
            /// x >>= 1i8;
            /// x >>= 2i16;
            /// x >>= 3i32;
            /// x >>= 4i64;
            /// assert_eq!(x.to_string(), "1");
            /// ```
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_assign_signed(self, bits);
            }
        }
    };
}
apply_to_signeds!(impl_natural_shr_signed);
