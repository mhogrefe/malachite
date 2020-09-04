use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwo, PowerOfTwo, RoundToMultipleOfPowerOfTwo, RoundToMultipleOfPowerOfTwoAssign,
    ShrRound, ShrRoundAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, LowMask};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::slices::{slice_set_zero, slice_test_zero};

use natural::arithmetic::add::limbs_slice_add_limb_in_place;
use natural::arithmetic::divisible_by_power_of_two::limbs_divisible_by_power_of_two;
use natural::logic::bit_access::limbs_get_bit;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` rounded down to a multiple of 2<sup>`pow`</sup>.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::round_to_multiple_of_power_of_two::*;
///
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_down(&[1], 1), &[0]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_down(&[3], 1), &[2]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_down(&[122, 456], 1), &[122, 456]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_down(&[123, 456], 1), &[122, 456]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_down(&[123, 455], 1), &[122, 455]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_down(&[123, 456], 31), &[0, 456]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_down(&[123, 456], 32), &[0, 456]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_down(&[123, 456], 100), Vec::<u32>::new());
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_down(&[256, 456], 8), &[256, 456]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_down(&[u32::MAX, 1], 1), &[u32::MAX - 1, 1]);
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two_down(&[u32::MAX, u32::MAX], 32),
///     &[0, u32::MAX]
/// );
/// ```
pub fn limbs_round_to_multiple_of_power_of_two_down(xs: &[Limb], pow: u64) -> Vec<Limb> {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    if clear_count >= xs_len {
        Vec::new()
    } else {
        let mut out = vec![0; xs_len];
        out[clear_count..].copy_from_slice(&xs[clear_count..]);
        let small_pow = pow & Limb::WIDTH_MASK;
        if small_pow != 0 {
            out[clear_count] &= !Limb::low_mask(small_pow);
        }
        out
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` rounded up to a multiple of 2<sup>`pow`</sup>. The limbs should not all
/// be zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(xs.len(), pow / Limb::WIDTH)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::round_to_multiple_of_power_of_two::*;
///
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_up(&[1], 1), &[2]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_up(&[3], 1), &[4]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_up(&[122, 456], 1), &[122, 456]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_up(&[123, 456], 1), &[124, 456]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_up(&[123, 455], 1), &[124, 455]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_up(&[123, 456], 31), &[2147483648, 456]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_up(&[123, 456], 32), &[0, 457]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_up(&[123, 456], 100), &[0, 0, 0, 16]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_up(&[256, 456], 8), &[256, 456]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_up(&[u32::MAX, 1], 1), &[0, 2]);
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two_up(&[u32::MAX, u32::MAX], 32),
///     &[0, 0, 1]
/// );
/// ```
pub fn limbs_round_to_multiple_of_power_of_two_up(xs: &[Limb], pow: u64) -> Vec<Limb> {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    let mut out;
    let small_pow = pow & Limb::WIDTH_MASK;
    if clear_count >= xs_len {
        out = vec![0; clear_count + 1];
        out[clear_count] = Limb::power_of_two(small_pow);
    } else {
        let (xs_lo, xs_hi) = xs.split_at(clear_count);
        let mut exact = slice_test_zero(xs_lo);
        out = vec![0; xs_len];
        let out_hi = &mut out[clear_count..];
        out_hi.copy_from_slice(xs_hi);
        if small_pow != 0 {
            let remainder = out_hi[0].mod_power_of_two(small_pow);
            if remainder != 0 {
                out_hi[0] -= remainder;
                exact = false;
            }
        }
        if !exact && limbs_slice_add_limb_in_place(out_hi, Limb::power_of_two(small_pow)) {
            out.push(1);
        }
    }
    out
}

fn limbs_round_to_multiple_of_power_of_two_half_integer_to_even(
    xs: &[Limb],
    pow: u64,
) -> Vec<Limb> {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    if clear_count >= xs_len {
        Vec::new()
    } else {
        let xs_hi = &xs[clear_count..];
        let mut out = vec![0; xs_len];
        let out_hi = &mut out[clear_count..];
        out_hi.copy_from_slice(xs_hi);
        let small_pow = pow & Limb::WIDTH_MASK;
        if small_pow != 0 {
            out_hi[0] &= !Limb::low_mask(small_pow);
        }
        if xs_hi[0].get_bit(small_pow)
            && limbs_slice_add_limb_in_place(out_hi, Limb::power_of_two(small_pow))
        {
            out.push(1);
        }
        out
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` rounded to the nearest multiple of 2<sup>`pow`</sup>. If the original
/// value is exactly between two multiples, it is rounded to the one whose `pow`th bit is zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`xs`.len(), `pow` / `Limb::WIDTH`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::round_to_multiple_of_power_of_two::*;
///
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_nearest(&[1], 1), &[0]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_nearest(&[3], 1), &[4]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_nearest(&[122, 456], 1), &[122, 456]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_nearest(&[123, 456], 1), &[124, 456]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_nearest(&[123, 455], 1), &[124, 455]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_nearest(&[123, 456], 31), &[0, 456]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_nearest(&[123, 456], 32), &[0, 456]);
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two_nearest(&[123, 456], 100),
///     Vec::<u32>::new()
/// );
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_nearest(&[256, 456], 8), &[256, 456]);
/// assert_eq!(limbs_round_to_multiple_of_power_of_two_nearest(&[u32::MAX, 1], 1), &[0, 2]);
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two_nearest(&[u32::MAX, u32::MAX], 32),
///     &[0, 0, 1]
/// );
/// ```
pub fn limbs_round_to_multiple_of_power_of_two_nearest(xs: &[Limb], pow: u64) -> Vec<Limb> {
    if pow == 0 {
        xs.to_vec()
    } else if !limbs_get_bit(xs, pow - 1) {
        limbs_round_to_multiple_of_power_of_two_down(xs, pow)
    } else if !limbs_divisible_by_power_of_two(xs, pow - 1) {
        limbs_round_to_multiple_of_power_of_two_up(xs, pow)
    } else {
        limbs_round_to_multiple_of_power_of_two_half_integer_to_even(xs, pow)
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the `Natural` rounded to a multiple of 2<sup>`pow`</sup>, using a specified rounding
/// format. If the original value is not already a multiple of the power of two, and the
/// `RoundingMode` is `Exact`, `None` is returned. The limbs should not all be zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`xs`.len(), `pow` / `Limb::WIDTH`)
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::rounding_modes::RoundingMode;
/// use malachite_nz::natural::arithmetic::round_to_multiple_of_power_of_two::*;
///
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two(&[1], 1, RoundingMode::Nearest),
///     Some(vec![0])
/// );
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two(&[3], 1, RoundingMode::Nearest),
///     Some(vec![4])
/// );
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two(&[122, 456], 1, RoundingMode::Floor),
///     Some(vec![122, 456])
/// );
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two(&[123, 456], 1, RoundingMode::Floor),
///     Some(vec![122, 456])
/// );
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two(&[123, 455], 1, RoundingMode::Floor),
///     Some(vec![122, 455])
/// );
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two(&[123, 456], 31, RoundingMode::Ceiling),
///     Some(vec![2147483648, 456])
/// );
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two(&[123, 456], 32, RoundingMode::Up),
///     Some(vec![0, 457])
/// );
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two(&[123, 456], 100, RoundingMode::Down),
///     Some(vec![])
/// );
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two(&[256, 456], 8, RoundingMode::Exact),
///     Some(vec![256, 456])
/// );
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two(&[u32::MAX, 1], 1, RoundingMode::Exact),
///     None
/// );
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two(&[u32::MAX, u32::MAX], 32, RoundingMode::Down),
///     Some(vec![0, u32::MAX])
/// );
/// ```
pub fn limbs_round_to_multiple_of_power_of_two(
    xs: &[Limb],
    pow: u64,
    rm: RoundingMode,
) -> Option<Vec<Limb>> {
    match rm {
        RoundingMode::Down | RoundingMode::Floor => {
            Some(limbs_round_to_multiple_of_power_of_two_down(xs, pow))
        }
        RoundingMode::Up | RoundingMode::Ceiling => {
            Some(limbs_round_to_multiple_of_power_of_two_up(xs, pow))
        }
        RoundingMode::Nearest => Some(limbs_round_to_multiple_of_power_of_two_nearest(xs, pow)),
        RoundingMode::Exact => {
            if limbs_divisible_by_power_of_two(xs, pow) {
                Some(xs.to_vec())
            } else {
                None
            }
        }
    }
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural`, rounded down to a multiple of 2<sup>`pow`</sup>, to the input `Vec`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::round_to_multiple_of_power_of_two::*;
///
/// let mut xs = vec![1];
/// limbs_round_to_multiple_of_power_of_two_down_in_place(&mut xs, 1);
/// assert_eq!(xs, &[0]);
///
/// let mut xs = vec![3];
/// limbs_round_to_multiple_of_power_of_two_down_in_place(&mut xs, 1);
/// assert_eq!(xs, &[2]);
///
/// let mut xs = vec![122, 456];
/// limbs_round_to_multiple_of_power_of_two_down_in_place(&mut xs, 1);
/// assert_eq!(xs, &[122, 456]);
///
/// let mut xs = vec![123, 456];
/// limbs_round_to_multiple_of_power_of_two_down_in_place(&mut xs, 1);
/// assert_eq!(xs, &[122, 456]);
///
/// let mut xs = vec![123, 455];
/// limbs_round_to_multiple_of_power_of_two_down_in_place(&mut xs, 1);
/// assert_eq!(xs, &[122, 455]);
///
/// let mut xs = vec![123, 456];
/// limbs_round_to_multiple_of_power_of_two_down_in_place(&mut xs, 31);
/// assert_eq!(xs, &[0, 456]);
///
/// let mut xs = vec![123, 456];
/// limbs_round_to_multiple_of_power_of_two_down_in_place(&mut xs, 32);
/// assert_eq!(xs, &[0, 456]);
///
/// let mut xs = vec![123, 456];
/// limbs_round_to_multiple_of_power_of_two_down_in_place(&mut xs, 100);
/// assert_eq!(xs, Vec::<u32>::new());
///
/// let mut xs = vec![256, 456];
/// limbs_round_to_multiple_of_power_of_two_down_in_place(&mut xs, 8);
/// assert_eq!(xs, &[256, 456]);
///
/// let mut xs = vec![u32::MAX, 1];
/// limbs_round_to_multiple_of_power_of_two_down_in_place(&mut xs, 1);
/// assert_eq!(xs, &[u32::MAX - 1, 1]);
///
/// let mut xs = vec![u32::MAX, u32::MAX];
/// limbs_round_to_multiple_of_power_of_two_down_in_place(&mut xs, 32);
/// assert_eq!(xs, &[0, u32::MAX]);
/// ```
pub fn limbs_round_to_multiple_of_power_of_two_down_in_place(xs: &mut Vec<Limb>, pow: u64) {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    if clear_count >= xs_len {
        xs.clear();
    } else {
        slice_set_zero(&mut xs[..clear_count]);
        let small_pow = pow & Limb::WIDTH_MASK;
        if small_pow != 0 {
            xs[clear_count] &= !Limb::low_mask(small_pow);
        }
    }
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural`, rounded up to a multiple of 2<sup>`pow`</sup>, to the input `Vec`. The
/// limbs should not all be zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`xs`.len(), `pow` / `Limb::WIDTH`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::round_to_multiple_of_power_of_two::*;
///
/// let mut xs = vec![1];
/// limbs_round_to_multiple_of_power_of_two_up_in_place(&mut xs, 1);
/// assert_eq!(xs, &[2]);
///
/// let mut xs = vec![3];
/// limbs_round_to_multiple_of_power_of_two_up_in_place(&mut xs, 1);
/// assert_eq!(xs, &[4]);
///
/// let mut xs = vec![122, 456];
/// limbs_round_to_multiple_of_power_of_two_up_in_place(&mut xs, 1);
/// assert_eq!(xs, &[122, 456]);
///
/// let mut xs = vec![123, 456];
/// limbs_round_to_multiple_of_power_of_two_up_in_place(&mut xs, 1);
/// assert_eq!(xs, &[124, 456]);
///
/// let mut xs = vec![123, 455];
/// limbs_round_to_multiple_of_power_of_two_up_in_place(&mut xs, 1);
/// assert_eq!(xs, &[124, 455]);
///
/// let mut xs = vec![123, 456];
/// limbs_round_to_multiple_of_power_of_two_up_in_place(&mut xs, 31);
/// assert_eq!(xs, &[2147483648, 456]);
///
/// let mut xs = vec![123, 456];
/// limbs_round_to_multiple_of_power_of_two_up_in_place(&mut xs, 32);
/// assert_eq!(xs, &[0, 457]);
///
/// let mut xs = vec![123, 456];
/// limbs_round_to_multiple_of_power_of_two_up_in_place(&mut xs, 100);
/// assert_eq!(xs, &[0, 0, 0, 16]);
///
/// let mut xs = vec![256, 456];
/// limbs_round_to_multiple_of_power_of_two_up_in_place(&mut xs, 8);
/// assert_eq!(xs, &[256, 456]);
///
/// let mut xs = vec![u32::MAX, 1];
/// limbs_round_to_multiple_of_power_of_two_up_in_place(&mut xs, 1);
/// assert_eq!(xs, &[0, 2]);
///
/// let mut xs = vec![u32::MAX, u32::MAX];
/// limbs_round_to_multiple_of_power_of_two_up_in_place(&mut xs, 32);
/// assert_eq!(xs, &[0, 0, 1]);
/// ```
pub fn limbs_round_to_multiple_of_power_of_two_up_in_place(xs: &mut Vec<Limb>, pow: u64) {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    let small_pow = pow & Limb::WIDTH_MASK;
    if clear_count >= xs_len {
        *xs = vec![0; clear_count + 1];
        xs[clear_count] = Limb::power_of_two(small_pow);
    } else {
        let (xs_lo, xs_hi) = xs.split_at_mut(clear_count);
        let mut exact = slice_test_zero(xs_lo);
        slice_set_zero(xs_lo);
        if small_pow != 0 {
            let remainder = xs_hi[0].mod_power_of_two(small_pow);
            if remainder != 0 {
                xs_hi[0] -= remainder;
                exact = false;
            }
        }
        if !exact && limbs_slice_add_limb_in_place(xs_hi, Limb::power_of_two(small_pow)) {
            xs.push(1);
        }
    }
}

fn limbs_round_to_multiple_of_power_of_two_half_integer_to_even_in_place(
    xs: &mut Vec<Limb>,
    pow: u64,
) {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    if clear_count >= xs_len {
        xs.clear();
    } else {
        let (xs_lo, xs_hi) = xs.split_at_mut(clear_count);
        if let Some(last) = xs_lo.last_mut() {
            *last = 0;
        }
        let small_pow = pow & Limb::WIDTH_MASK;
        if small_pow != 0 {
            xs_hi[0] &= !Limb::low_mask(small_pow);
        }
        if xs_hi[0].get_bit(small_pow)
            && limbs_slice_add_limb_in_place(xs_hi, Limb::power_of_two(small_pow))
        {
            xs.push(1);
        }
    }
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural`, rounded to the nearest multiple of 2<sup>`pow`</sup>, to the input
/// `Vec`. If the original value is exactly between two multiples, it is rounded to the one whose
/// `pow`th bit is zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`xs`.len(), `pow` / `Limb::WIDTH`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::round_to_multiple_of_power_of_two::*;
///
/// let mut xs = vec![1];
/// limbs_round_to_multiple_of_power_of_two_nearest_in_place(&mut xs, 1);
/// assert_eq!(xs, &[0]);
///
/// let mut xs = vec![3];
/// limbs_round_to_multiple_of_power_of_two_nearest_in_place(&mut xs, 1);
/// assert_eq!(xs, &[4]);
///
/// let mut xs = vec![122, 456];
/// limbs_round_to_multiple_of_power_of_two_nearest_in_place(&mut xs, 1);
/// assert_eq!(xs, &[122, 456]);
///
/// let mut xs = vec![123, 456];
/// limbs_round_to_multiple_of_power_of_two_nearest_in_place(&mut xs, 1);
/// assert_eq!(xs, &[124, 456]);
///
/// let mut xs = vec![123, 455];
/// limbs_round_to_multiple_of_power_of_two_nearest_in_place(&mut xs, 1);
/// assert_eq!(xs, &[124, 455]);
///
/// let mut xs = vec![123, 456];
/// limbs_round_to_multiple_of_power_of_two_nearest_in_place(&mut xs, 31);
/// assert_eq!(xs, &[0, 456]);
///
/// let mut xs = vec![123, 456];
/// limbs_round_to_multiple_of_power_of_two_nearest_in_place(&mut xs, 32);
/// assert_eq!(xs, &[0, 456]);
///
/// let mut xs = vec![123, 456];
/// limbs_round_to_multiple_of_power_of_two_nearest_in_place(&mut xs, 100);
/// assert_eq!(xs, Vec::<u32>::new());
///
/// let mut xs = vec![256, 456];
/// limbs_round_to_multiple_of_power_of_two_nearest_in_place(&mut xs, 8);
/// assert_eq!(xs, &[256, 456]);
///
/// let mut xs = vec![u32::MAX, 1];
/// limbs_round_to_multiple_of_power_of_two_nearest_in_place(&mut xs, 1);
/// assert_eq!(xs, &[0, 2]);
///
/// let mut xs = vec![u32::MAX, u32::MAX];
/// limbs_round_to_multiple_of_power_of_two_nearest_in_place(&mut xs, 32);
/// assert_eq!(xs, &[0, 0, 1]);
/// ```
pub fn limbs_round_to_multiple_of_power_of_two_nearest_in_place(xs: &mut Vec<Limb>, pow: u64) {
    if pow == 0 {
    } else if !limbs_get_bit(xs, pow - 1) {
        limbs_round_to_multiple_of_power_of_two_down_in_place(xs, pow);
    } else if !limbs_divisible_by_power_of_two(xs, pow - 1) {
        limbs_round_to_multiple_of_power_of_two_up_in_place(xs, pow);
    } else {
        limbs_round_to_multiple_of_power_of_two_half_integer_to_even_in_place(xs, pow);
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the `Natural` rounded to the nearest multiple of 2<sup>`pow`</sup> to the input `Vec`,
/// using a specified rounding format. If the original value is not already a multiple of the power
/// of two, and the `RoundingMode` is `Exact`, the value of `xs` becomes unspecified and `false` is
/// returned. Otherwise, `true` is returned. The limbs should not all be zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`xs`.len(), `pow` / `Limb::WIDTH`)
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::rounding_modes::RoundingMode;
/// use malachite_nz::natural::arithmetic::round_to_multiple_of_power_of_two::*;
///
/// let mut xs = vec![1];
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two_in_place(&mut xs, 1, RoundingMode::Nearest),
///     true
/// );
/// assert_eq!(xs, &[0]);
///
/// let mut xs = vec![3];
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two_in_place(&mut xs, 1, RoundingMode::Nearest),
///     true
/// );
/// assert_eq!(xs, &[4]);
///
/// let mut xs = vec![122, 456];
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two_in_place(&mut xs, 1, RoundingMode::Floor),
///     true
/// );
/// assert_eq!(xs, &[122, 456]);
///
/// let mut xs = vec![123, 456];
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two_in_place(&mut xs, 1, RoundingMode::Floor),
///     true
/// );
/// assert_eq!(xs, &[122, 456]);
///
/// let mut xs = vec![123, 455];
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two_in_place(&mut xs, 1, RoundingMode::Floor),
///     true
/// );
/// assert_eq!(xs, &[122, 455]);
///
/// let mut xs = vec![123, 456];
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two_in_place(&mut xs, 31, RoundingMode::Ceiling),
///     true
/// );
/// assert_eq!(xs, &[2147483648, 456]);
///
/// let mut xs = vec![123, 456];
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two_in_place(&mut xs, 32, RoundingMode::Up),
///     true
/// );
/// assert_eq!(xs, &[0, 457]);
///
/// let mut xs = vec![123, 456];
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two_in_place(&mut xs, 100, RoundingMode::Down),
///     true
/// );
/// assert_eq!(xs, Vec::<u32>::new());
///
/// let mut xs = vec![256, 456];
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two_in_place(&mut xs, 8, RoundingMode::Exact),
///     true
/// );
/// assert_eq!(xs, vec![256, 456]);
///
/// let mut xs = vec![u32::MAX, 1];
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two_in_place(&mut xs, 1, RoundingMode::Exact),
///     false
/// );
///
/// let mut xs = vec![u32::MAX, u32::MAX];
/// assert_eq!(
///     limbs_round_to_multiple_of_power_of_two_in_place(&mut xs, 32, RoundingMode::Down),
///     true
/// );
/// assert_eq!(xs, vec![0, u32::MAX]);
/// ```
pub fn limbs_round_to_multiple_of_power_of_two_in_place(
    xs: &mut Vec<Limb>,
    pow: u64,
    rm: RoundingMode,
) -> bool {
    match rm {
        RoundingMode::Down | RoundingMode::Floor => {
            limbs_round_to_multiple_of_power_of_two_down_in_place(xs, pow);
            true
        }
        RoundingMode::Up | RoundingMode::Ceiling => {
            limbs_round_to_multiple_of_power_of_two_up_in_place(xs, pow);
            true
        }
        RoundingMode::Nearest => {
            limbs_round_to_multiple_of_power_of_two_nearest_in_place(xs, pow);
            true
        }
        RoundingMode::Exact => limbs_divisible_by_power_of_two(xs, pow),
    }
}

impl RoundToMultipleOfPowerOfTwo<u64> for Natural {
    type Output = Natural;

    /// Rounds `self` to a multiple of a power of 2, according to a specified rounding mode, taking
    /// `self` by value.
    ///
    /// The following two expressions are equivalent:
    ///
    /// `x.round_to_multiple_of_power_of_two(pow, RoundingMode::Exact)`
    /// `{ assert!(x.divisible_by_power_of_two(pow)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = max(self.significant_bits(), pow)
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of two.
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOfTwo;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple_of_power_of_two(2, RoundingMode::Floor),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple_of_power_of_two(2, RoundingMode::Ceiling),
    ///     12
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple_of_power_of_two(2, RoundingMode::Down),
    ///     8
    /// );
    /// assert_eq!(Natural::from(10u32).round_to_multiple_of_power_of_two(2, RoundingMode::Up), 12);
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple_of_power_of_two(2, RoundingMode::Nearest),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(12u32).round_to_multiple_of_power_of_two(2, RoundingMode::Exact),
    ///     12
    /// );
    /// ```
    #[inline]
    fn round_to_multiple_of_power_of_two(mut self, pow: u64, rm: RoundingMode) -> Natural {
        self.round_to_multiple_of_power_of_two_assign(pow, rm);
        self
    }
}

impl<'a> RoundToMultipleOfPowerOfTwo<u64> for &'a Natural {
    type Output = Natural;

    /// Rounds `self` to a multiple of a power of 2, according to a specified rounding mode, taking
    /// `self` by reference.
    ///
    /// The following two expressions are equivalent:
    ///
    /// `x.round_to_multiple_of_power_of_two(pow, RoundingMode::Exact)`
    /// `{ assert!(x.divisible_by_power_of_two(pow)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = max(self.significant_bits(), pow)
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of two.
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOfTwo;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple_of_power_of_two(2, RoundingMode::Floor),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple_of_power_of_two(2, RoundingMode::Ceiling),
    ///     12
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple_of_power_of_two(2, RoundingMode::Down),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple_of_power_of_two(2, RoundingMode::Up),
    ///     12
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple_of_power_of_two(2, RoundingMode::Nearest),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Natural::from(12u32)).round_to_multiple_of_power_of_two(2, RoundingMode::Exact),
    ///     12
    /// );
    /// ```
    fn round_to_multiple_of_power_of_two(self, pow: u64, rm: RoundingMode) -> Natural {
        match (self, pow) {
            (_, 0) | (natural_zero!(), _) => self.clone(),
            (Natural(Small(small)), pow) => Natural::from(small.shr_round(pow, rm)) << pow,
            (Natural(Large(ref limbs)), pow) => {
                if let Some(result_limbs) = limbs_round_to_multiple_of_power_of_two(limbs, pow, rm)
                {
                    Natural::from_owned_limbs_asc(result_limbs)
                } else {
                    panic!("Rounding {} to multiple of 2^{} is not exact", self, pow);
                }
            }
        }
    }
}

impl RoundToMultipleOfPowerOfTwoAssign<u64> for Natural {
    /// Rounds `self` to a multiple of a power of 2, according to a specified rounding mode, in
    /// place.
    ///
    /// The following two expressions are equivalent:
    ///
    /// `x.round_to_multiple_of_power_of_two_assign(pow, RoundingMode::Exact);`
    /// `assert!(x.divisible_by_power_of_two(pow));`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = max(self.significant_bits(), pow)
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of two.
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOfTwoAssign;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(10u32);
    /// n.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Floor);
    /// assert_eq!(n, 8);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Ceiling);
    /// assert_eq!(n, 12);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Down);
    /// assert_eq!(n, 8);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Up);
    /// assert_eq!(n, 12);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Nearest);
    /// assert_eq!(n, 8);
    ///
    /// let mut n = Natural::from(12u32);
    /// n.round_to_multiple_of_power_of_two_assign(2, RoundingMode::Exact);
    /// assert_eq!(n, 12);
    /// ```
    fn round_to_multiple_of_power_of_two_assign(&mut self, pow: u64, rm: RoundingMode) {
        match (&mut *self, pow) {
            (_, 0) | (natural_zero!(), _) => {}
            (Natural(Small(ref mut small)), pow) => {
                small.shr_round_assign(pow, rm);
                *self <<= pow;
            }
            (Natural(Large(ref mut limbs)), pow) => {
                if limbs_round_to_multiple_of_power_of_two_in_place(limbs, pow, rm) {
                    self.trim();
                } else {
                    panic!("Rounding is not exact");
                }
            }
        }
    }
}
