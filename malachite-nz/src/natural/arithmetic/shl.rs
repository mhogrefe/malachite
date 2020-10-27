use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShl, UnsignedAbs};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::vecs::vec_pad_left;

use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` left-shifted by a `Limb`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()` + `bits` / Limb::WIDTH
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shl::limbs_shl;
///
/// assert_eq!(limbs_shl(&[123, 456], 1), &[246, 912]);
/// assert_eq!(limbs_shl(&[123, 456], 31), &[2147483648, 61, 228]);
/// assert_eq!(limbs_shl(&[123, 456], 32), &[0, 123, 456]);
/// assert_eq!(limbs_shl(&[123, 456], 100), &[0, 0, 0, 1968, 7296]);
/// ```
///
/// This is mpn_lshift from mpn/generic/lshift.c, GMP 6.1.2, where the result is returned.
pub fn limbs_shl(xs: &[Limb], bits: u64) -> Vec<Limb> {
    let small_bits = bits & Limb::WIDTH_MASK;
    let mut out = vec![0; usize::exact_from(bits >> Limb::LOG_WIDTH)];
    if small_bits == 0 {
        out.extend_from_slice(xs);
    } else {
        let cobits = Limb::WIDTH - small_bits;
        let mut remaining_bits = 0;
        for x in xs {
            out.push((x << small_bits) | remaining_bits);
            remaining_bits = x >> cobits;
        }
        if remaining_bits != 0 {
            out.push(remaining_bits);
        }
    }
    out
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` left-shifted by a `Limb` to an output slice. The output slice must be at
/// least as long as the input slice. The `Limb` must be between 1 and `Limb::WIDTH` - 1, inclusive.
/// The carry, or the bits that are shifted past the width of the input slice, is returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `xs`, `bits` is 0, or `bits` is greater than or equal to
/// `Limb::WIDTH`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shl::limbs_shl_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_shl_to_out(&mut out, &[123, 456], 1), 0);
/// assert_eq!(out, &[246, 912, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_shl_to_out(&mut out, &[123, 456], 31), 228);
/// assert_eq!(out, &[2147483648, 61, 0]);
/// ```
///
/// This is mpn_lshift from mpn/generic/lshift.c, GMP 6.1.2.
pub fn limbs_shl_to_out(out: &mut [Limb], xs: &[Limb], bits: u64) -> Limb {
    assert_ne!(bits, 0);
    assert!(bits < Limb::WIDTH);
    let cobits = Limb::WIDTH - bits;
    let mut remaining_bits = 0;
    for (out, x) in out[..xs.len()].iter_mut().zip(xs.iter()) {
        *out = (x << bits) | remaining_bits;
        remaining_bits = x >> cobits;
    }
    remaining_bits
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` left-shifted by a `Limb` to the input slice. The `Limb` must be between 1
/// and `Limb::WIDTH` - 1, inclusive. The carry, or the bits that are shifted past the width of the
/// input slice, is returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shl::limbs_slice_shl_in_place;
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_slice_shl_in_place(&mut xs, 1), 0);
/// assert_eq!(xs, &[246, 912]);
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_slice_shl_in_place(&mut xs, 31), 228);
/// assert_eq!(xs, &[2147483648, 61]);
/// ```
///
/// This is mpn_lshift from mpn/generic/lshift.c, GMP 6.1.2, where rp == up.
pub fn limbs_slice_shl_in_place(xs: &mut [Limb], bits: u64) -> Limb {
    assert_ne!(bits, 0);
    assert!(bits < Limb::WIDTH);
    let cobits = Limb::WIDTH - bits;
    let mut remaining_bits = 0;
    for x in xs.iter_mut() {
        let previous_x = *x;
        *x = (previous_x << bits) | remaining_bits;
        remaining_bits = previous_x >> cobits;
    }
    remaining_bits
}

/// Interpreting a nonempty `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`,
/// writes the limbs of the `Natural` left-shifted by a `Limb` to the input `Vec`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.len()` + `bits` / Limb::WIDTH, m = `bits`
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shl::limbs_vec_shl_in_place;
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shl_in_place(&mut xs, 1);
/// assert_eq!(xs, &[246, 912]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_shl_in_place(&mut xs, 31);
/// assert_eq!(xs, &[2147483648, 61, 228]);
/// ```
///
/// This is mpn_lshift from mpn/generic/lshift.c, GMP 6.1.2, where rp == up and the carry is
/// appended to rp.
pub fn limbs_vec_shl_in_place(xs: &mut Vec<Limb>, bits: u64) {
    let small_bits = bits & Limb::WIDTH_MASK;
    let remaining_bits = if small_bits == 0 {
        0
    } else {
        limbs_slice_shl_in_place(xs, small_bits)
    };
    vec_pad_left(xs, usize::exact_from(bits >> Limb::LOG_WIDTH), 0);
    if remaining_bits != 0 {
        xs.push(remaining_bits);
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` left-shifted by a `Limb`, and complemented, to an output slice. The
/// output slice must be at least as long as the input slice. The `Limb` must be between 1 and
/// `Limb::WIDTH` - 1, inclusive. The carry, or the bits that are shifted past the width of the
/// input slice, is returned. The carry is not complemented.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `xs`, `xs` is empty, `bits` is 0, or `bits` is greater than or
/// equal to `Limb::WIDTH`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::shl::limbs_shl_with_complement_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_shl_with_complement_to_out(&mut out, &[123, 456], 1), 0);
/// assert_eq!(out, &[4294967049, 4294966383, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_shl_with_complement_to_out(&mut out, &[123, 456], 31), 228);
/// assert_eq!(out, &[2147483647, 4294967234, 0]);
/// ```
///
/// This is mpn_lshiftc from mpn/generic/mpn_lshiftc, GMP 6.1.2.
pub fn limbs_shl_with_complement_to_out(out: &mut [Limb], xs: &[Limb], bits: u64) -> Limb {
    let n = xs.len();
    assert_ne!(n, 0);
    assert_ne!(bits, 0);
    assert!(bits < Limb::WIDTH);
    let cobits = Limb::WIDTH - bits;
    let (xs_last, xs_init) = xs.split_last().unwrap();
    let remaining_bits = xs_last >> cobits;
    let mut previous_x = xs_last << bits;
    let (out_head, out_tail) = out[..n].split_first_mut().unwrap();
    for (out, x) in out_tail.iter_mut().rev().zip(xs_init.iter().rev()) {
        *out = !(previous_x | (x >> cobits));
        previous_x = x << bits;
    }
    *out_head = !previous_x;
    remaining_bits
}

fn _shl_ref_unsigned<T: Copy + Eq + Zero>(x: &Natural, bits: T) -> Natural
where
    u64: ExactFrom<T>,
    Limb: ArithmeticCheckedShl<T, Output = Limb>,
{
    match (x, bits) {
        (natural_zero!(), _) => x.clone(),
        (_, bits) if bits == T::ZERO => x.clone(),
        (Natural(Small(small)), bits) => {
            Natural(if let Some(shifted) = small.arithmetic_checked_shl(bits) {
                Small(shifted)
            } else {
                Large(limbs_shl(&[*small], u64::exact_from(bits)))
            })
        }
        (Natural(Large(ref limbs)), bits) => {
            Natural(Large(limbs_shl(limbs, u64::exact_from(bits))))
        }
    }
}

fn _shl_assign<T: Copy + Eq + Zero>(x: &mut Natural, bits: T)
where
    u64: ExactFrom<T>,
    Limb: ArithmeticCheckedShl<T, Output = Limb>,
{
    match (&mut *x, bits) {
        (natural_zero!(), _) => {}
        (_, bits) if bits == T::ZERO => {}
        (Natural(Small(ref mut small)), bits) => {
            if let Some(shifted) = small.arithmetic_checked_shl(bits) {
                *small = shifted;
            } else {
                *x = Natural(Large(limbs_shl(&[*small], u64::exact_from(bits))));
            }
        }
        (Natural(Large(ref mut limbs)), bits) => {
            limbs_vec_shl_in_place(limbs, u64::exact_from(bits));
        }
    }
}

macro_rules! impl_natural_shl_unsigned {
    ($t:ident) => {
        /// Shifts a `Natural` left (multiplies it by a power of 2), taking the `Natural` by value.
        ///
        /// Time: worst case O(n)
        ///
        /// Additional memory: worst case O(m)
        ///
        /// where n = `self.significant_bits()` + `bits`, m = `bits`
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::num::basic::traits::Zero;
        /// use malachite_nz::natural::Natural;
        ///
        /// assert_eq!((Natural::ZERO << 10u8).to_string(), "0");
        /// assert_eq!((Natural::from(123u32) << 2u16).to_string(), "492");
        /// assert_eq!((Natural::from(123u32) << 100u64).to_string(),
        ///     "155921023828072216384094494261248");
        /// ```
        impl Shl<$t> for Natural {
            type Output = Natural;

            #[inline]
            fn shl(mut self, bits: $t) -> Natural {
                self <<= bits;
                self
            }
        }
        /// Shifts a `Natural` left (multiplies it by a power of 2), taking the `Natural` by
        /// reference.
        ///
        /// Time: worst case O(n)
        ///
        /// Additional memory: worst case O(n)
        ///
        /// where n = `self.significant_bits()` + `bits`
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::num::basic::traits::Zero;
        /// use malachite_nz::natural::Natural;
        ///
        /// assert_eq!((&Natural::ZERO << 10u8).to_string(), "0");
        /// assert_eq!((&Natural::from(123u32) << 2u16).to_string(), "492");
        /// assert_eq!((&Natural::from(123u32) << 100u64).to_string(),
        ///     "155921023828072216384094494261248");
        /// ```
        impl<'a> Shl<$t> for &'a Natural {
            type Output = Natural;

            #[inline]
            fn shl(self, bits: $t) -> Natural {
                _shl_ref_unsigned(self, bits)
            }
        }

        /// Shifts a `Natural` left (multiplies it by a power of 2) in place.
        ///
        /// Time: worst case O(n)
        ///
        /// Additional memory: worst case O(m)
        ///
        /// where n = `self.significant_bits()` + `bits`, m = `bits`
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
        /// x <<= 1u8;
        /// x <<= 2u16;
        /// x <<= 3u32;
        /// x <<= 4u64;
        /// assert_eq!(x.to_string(), "1024");
        /// ```
        impl ShlAssign<$t> for Natural {
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                _shl_assign(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_natural_shl_unsigned);

fn _shl_ref_signed<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Natural,
    bits: S,
) -> Natural
where
    &'a Natural: Shl<U, Output = Natural> + Shr<U, Output = Natural>,
{
    if bits >= S::ZERO {
        x << bits.unsigned_abs()
    } else {
        x >> bits.unsigned_abs()
    }
}

fn _shl_assign_signed<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(x: &mut Natural, bits: S)
where
    Natural: ShlAssign<U> + ShrAssign<U>,
{
    if bits >= S::ZERO {
        *x <<= bits.unsigned_abs();
    } else {
        *x >>= bits.unsigned_abs();
    }
}

macro_rules! impl_natural_shl_signed {
    ($t:ident) => {
        impl Shl<$t> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2
            /// and takes the floor), taking the `Natural` by value.
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
            /// assert_eq!((Natural::ZERO << 10i8).to_string(), "0");
            /// assert_eq!((Natural::from(123u32) << 2i16).to_string(), "492");
            /// assert_eq!((Natural::from(123u32) << 100i32).to_string(),
            ///     "155921023828072216384094494261248");
            /// assert_eq!((Natural::ZERO << -10i64).to_string(), "0");
            /// assert_eq!((Natural::from(492u32) << -2i8).to_string(), "123");
            /// assert_eq!((Natural::trillion() << -10i16).to_string(), "976562500");
            /// ```
            #[inline]
            fn shl(mut self, bits: $t) -> Natural {
                self <<= bits;
                self
            }
        }

        impl<'a> Shl<$t> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2
            /// and takes the floor), taking the `Natural` by reference.
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
            /// assert_eq!((&Natural::ZERO << 10i8).to_string(), "0");
            /// assert_eq!((&Natural::from(123u32) << 2i16).to_string(), "492");
            /// assert_eq!((&Natural::from(123u32) << 100i32).to_string(),
            ///     "155921023828072216384094494261248");
            /// assert_eq!((&Natural::ZERO << -10i64).to_string(), "0");
            /// assert_eq!((&Natural::from(492u32) << -2i8).to_string(), "123");
            /// assert_eq!((&Natural::trillion() << -10i16).to_string(), "976562500");
            /// ```
            #[inline]
            fn shl(self, bits: $t) -> Natural {
                _shl_ref_signed(self, bits)
            }
        }

        impl ShlAssign<$t> for Natural {
            /// Shifts a `Natural` left (multiplies it by a power of 2 or divides it by a power of 2
            /// and takes the floor) in place.
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
            /// x <<= 1i8;
            /// x <<= 2i16;
            /// x <<= 3i32;
            /// x <<= 4i64;
            /// assert_eq!(x.to_string(), "1024");
            ///
            /// let mut x = Natural::from(1024u32);
            /// x <<= -1i8;
            /// x <<= -2i16;
            /// x <<= -3i32;
            /// x <<= -4i64;
            /// assert_eq!(x.to_string(), "1");
            /// ```
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                _shl_assign_signed(self, bits);
            }
        }
    };
}
apply_to_signeds!(impl_natural_shl_signed);
