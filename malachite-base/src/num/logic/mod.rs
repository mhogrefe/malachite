/// This module contains functions for getting and setting individual bits.
///
/// Here are usage examples of the macro-generated functions:
///
/// # get_bit
/// ```
/// use malachite_base::num::logic::traits::BitAccess;
///
/// assert_eq!(123u8.get_bit(2), false);
/// assert_eq!(123u16.get_bit(3), true);
/// assert_eq!(123u32.get_bit(100), false);
/// assert_eq!(1000000000000u64.get_bit(12), true);
/// assert_eq!(1000000000000u64.get_bit(100), false);
///
/// assert_eq!(123i8.get_bit(2), false);
/// assert_eq!(123i16.get_bit(3), true);
/// assert_eq!(123i32.get_bit(100), false);
/// assert_eq!((-123i8).get_bit(0), true);
/// assert_eq!((-123i16).get_bit(1), false);
/// assert_eq!((-123i32).get_bit(100), true);
/// assert_eq!(1000000000000i64.get_bit(12), true);
/// assert_eq!(1000000000000i64.get_bit(100), false);
/// assert_eq!((-1000000000000i64).get_bit(12), true);
/// assert_eq!((-1000000000000i64).get_bit(100), true);
/// ```
///
/// # set_bit
/// ```
/// use malachite_base::num::logic::traits::BitAccess;
///
/// let mut x = 0u8;
/// x.set_bit(2);
/// x.set_bit(5);
/// x.set_bit(6);
/// assert_eq!(x, 100);
///
/// let mut x = 0i8;
/// x.set_bit(2);
/// x.set_bit(5);
/// x.set_bit(6);
/// assert_eq!(x, 100);
///
/// let mut x = -0x100i16;
/// x.set_bit(2);
/// x.set_bit(5);
/// x.set_bit(6);
/// assert_eq!(x, -156);
/// ```
///
/// # clear_bit
/// ```
/// use malachite_base::num::logic::traits::BitAccess;
///
/// let mut x = 0x7fu8;
/// x.clear_bit(0);
/// x.clear_bit(1);
/// x.clear_bit(3);
/// x.clear_bit(4);
/// assert_eq!(x, 100);
///
/// let mut x = 0x7fi8;
/// x.clear_bit(0);
/// x.clear_bit(1);
/// x.clear_bit(3);
/// x.clear_bit(4);
/// assert_eq!(x, 100);
///
/// let mut x = -156i16;
/// x.clear_bit(2);
/// x.clear_bit(5);
/// x.clear_bit(6);
/// assert_eq!(x, -256);
/// ```
///
/// # assign_bit
/// ```
/// use malachite_base::num::logic::traits::BitAccess;
///
/// let mut x = 0;
/// x.assign_bit(2, true);
/// x.assign_bit(5, true);
/// x.assign_bit(6, true);
/// assert_eq!(x, 100);
/// x.assign_bit(2, false);
/// x.assign_bit(5, false);
/// x.assign_bit(6, false);
/// assert_eq!(x, 0);
///
/// x.assign_bit(2, true);
/// x.assign_bit(5, true);
/// x.assign_bit(6, true);
/// assert_eq!(x, 100);
/// x.assign_bit(2, false);
/// x.assign_bit(5, false);
/// x.assign_bit(6, false);
/// assert_eq!(x, 0);
///
/// let mut x = -0x100i16;
/// x.assign_bit(2, true);
/// x.assign_bit(5, true);
/// x.assign_bit(6, true);
/// assert_eq!(x, -156);
/// x.assign_bit(2, false);
/// x.assign_bit(5, false);
/// x.assign_bit(6, false);
/// assert_eq!(x, -256);
/// ```
///
/// # flip_bit
/// ```
/// use malachite_base::num::logic::traits::BitAccess;
///
/// let mut x = 0u64;
/// x.flip_bit(10);
/// assert_eq!(x, 1024);
/// x.flip_bit(10);
/// assert_eq!(x, 0);
///
/// let mut x = 0i32;
/// x.flip_bit(10);
/// assert_eq!(x, 1024);
/// x.flip_bit(10);
/// assert_eq!(x, 0);
///
/// let mut x = -1i64;
/// x.flip_bit(10);
/// assert_eq!(x, -1025);
/// x.flip_bit(10);
/// assert_eq!(x, -1);
/// ```
pub mod bit_access;
pub mod bit_block_access;
pub mod bit_convertible;
pub mod bit_iterable;
pub mod bit_scan;
pub mod count_ones;
pub mod count_zeros;
pub mod hamming_distance;
pub mod leading_zeros;
pub mod low_mask;
pub mod not;
pub mod power_of_two_digit_iterable;
pub mod power_of_two_digits;
pub mod rotate;
pub mod significant_bits;
pub mod trailing_zeros;
pub mod traits;
