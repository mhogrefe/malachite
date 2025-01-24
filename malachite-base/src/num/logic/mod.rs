// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// [`BitAccess`](traits::BitAccess), a trait for getting and setting individual bits of a number.
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
/// [`BitBlockAccess`](traits::BitBlockAccess), a trait for getting and setting adjacent blocks of
/// bits in a number.
///
/// # get_bits
/// ```
/// use malachite_base::num::logic::traits::BitBlockAccess;
///
/// assert_eq!(0xabcdu16.get_bits(4, 8), 0xc);
/// assert_eq!(0xabcdu16.get_bits(12, 100), 0xa);
/// assert_eq!(0xabcdu16.get_bits(5, 9), 14);
/// assert_eq!(0xabcdu16.get_bits(5, 5), 0);
/// assert_eq!(0xabcdu16.get_bits(100, 200), 0);
///
/// assert_eq!((-0x5433i16).get_bits(4, 8), 0xc);
/// assert_eq!((-0x5433i16).get_bits(5, 9), 14);
/// assert_eq!((-0x5433i16).get_bits(5, 5), 0);
/// assert_eq!((-0x5433i16).get_bits(100, 104), 0xf);
/// ```
///
/// # assign_bits
/// ```
/// use malachite_base::num::logic::traits::BitBlockAccess;
///
/// let mut x = 0xab5du16;
/// x.assign_bits(4, 8, &0xc);
/// assert_eq!(x, 0xabcd);
///
/// let mut x = 0xabcdu16;
/// x.assign_bits(100, 200, &0);
/// assert_eq!(x, 0xabcd);
///
/// let mut x = 0xabcdu16;
/// x.assign_bits(0, 100, &0x1234);
/// assert_eq!(x, 0x1234);
///
/// let mut x = 0x2b5di16;
/// x.assign_bits(4, 8, &0xc);
/// assert_eq!(x, 0x2bcd);
///
/// let mut x = -0x5413i16;
/// x.assign_bits(4, 8, &0xc);
/// assert_eq!(x, -0x5433);
///
/// let mut x = -0x5433i16;
/// x.assign_bits(100, 104, &0xf);
/// assert_eq!(x, -0x5433);
/// ```
pub mod bit_block_access;
/// [`BitConvertible`](traits::BitConvertible), a trait for extracting all bits from a number or
/// constructing a number from bits.
///
/// # to_bits_asc
/// ```
/// use malachite_base::num::logic::traits::BitConvertible;
///
/// assert_eq!(0u8.to_bits_asc(), &[]);
/// assert_eq!(2u16.to_bits_asc(), &[false, true]);
/// assert_eq!(
///     123u32.to_bits_asc(),
///     &[true, true, false, true, true, true, true]
/// );
///
/// assert_eq!(0i8.to_bits_asc(), &[]);
/// assert_eq!(2i16.to_bits_asc(), &[false, true, false]);
/// assert_eq!(
///     (-123i32).to_bits_asc(),
///     &[true, false, true, false, false, false, false, true]
/// );
/// ```
///
/// # to_bits_desc
/// ```
/// use malachite_base::num::logic::traits::BitConvertible;
///
/// assert_eq!(0u8.to_bits_desc(), &[]);
/// assert_eq!(2u16.to_bits_desc(), &[true, false]);
/// assert_eq!(
///     123u32.to_bits_desc(),
///     &[true, true, true, true, false, true, true]
/// );
///
/// assert_eq!(0i8.to_bits_desc(), &[]);
/// assert_eq!(2i16.to_bits_desc(), &[false, true, false]);
/// assert_eq!(
///     (-123i32).to_bits_desc(),
///     &[true, false, false, false, false, true, false, true]
/// );
/// ```
///
/// # from_bits_asc
/// ```
/// use malachite_base::num::logic::traits::BitConvertible;
/// use std::iter::empty;
///
/// assert_eq!(u8::from_bits_asc(empty()), 0);
/// assert_eq!(u16::from_bits_asc([false, true, false].iter().cloned()), 2);
/// assert_eq!(
///     u32::from_bits_asc([true, true, false, true, true, true, true].iter().cloned()),
///     123
/// );
///
/// assert_eq!(i8::from_bits_asc(empty()), 0);
/// assert_eq!(i16::from_bits_asc([false, true, false].iter().cloned()), 2);
/// assert_eq!(
///     i32::from_bits_asc(
///         [true, false, true, false, false, false, false, true]
///             .iter()
///             .cloned()
///     ),
///     -123
/// );
/// ```
///
/// # from_bits_desc
/// ```
/// use malachite_base::num::logic::traits::BitConvertible;
/// use std::iter::empty;
///
/// assert_eq!(u8::from_bits_desc(empty()), 0);
/// assert_eq!(u16::from_bits_desc([false, true, false].iter().cloned()), 2);
/// assert_eq!(
///     u32::from_bits_desc([true, true, true, true, false, true, true].iter().cloned()),
///     123
/// );
///
/// assert_eq!(i8::from_bits_desc(empty()), 0);
/// assert_eq!(i16::from_bits_desc([false, true, false].iter().cloned()), 2);
/// assert_eq!(
///     i32::from_bits_desc(
///         [true, false, false, false, false, true, false, true]
///             .iter()
///             .cloned()
///     ),
///     -123
/// );
/// ```
pub mod bit_convertible;
/// [`BitIterable`](traits::BitIterable), a trait for producing a double-ended iterator over a
/// number's bits.
///
/// # bits
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::logic::traits::BitIterable;
///
/// assert!(0u8.bits().next().is_none());
/// // 105 = 1101001b
/// assert_eq!(
///     105u32.bits().collect_vec(),
///     &[true, false, false, true, false, true, true]
/// );
///
/// assert!(0u8.bits().next_back().is_none());
/// // 105 = 1101001b
/// assert_eq!(
///     105u32.bits().rev().collect_vec(),
///     &[true, true, false, true, false, false, true]
/// );
///
/// assert_eq!(0i8.bits().next(), None);
/// // 105 = 01101001b, with a leading false bit to indicate sign
/// assert_eq!(
///     105i32.bits().collect_vec(),
///     &[true, false, false, true, false, true, true, false]
/// );
/// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
/// assert_eq!(
///     (-105i32).bits().collect_vec(),
///     &[true, true, true, false, true, false, false, true]
/// );
///
/// assert_eq!(0i8.bits().next_back(), None);
/// // 105 = 01101001b, with a leading false bit to indicate sign
/// assert_eq!(
///     105i32.bits().rev().collect_vec(),
///     &[false, true, true, false, true, false, false, true]
/// );
/// // -105 = 10010111 in two's complement, with a leading true bit to indicate sign
/// assert_eq!(
///     (-105i32).bits().rev().collect_vec(),
///     &[true, false, false, true, false, true, true, true]
/// );
/// ```
pub mod bit_iterable;
/// [`BitScan`](traits::BitScan), a trait for finding the next `true` or `false` bit in a number
/// after a provided index.
///
/// # index_of_next_false_bit
/// ```
/// use malachite_base::num::logic::traits::BitScan;
///
/// assert_eq!(0xb00000000u64.index_of_next_false_bit(0), Some(0));
/// assert_eq!(0xb00000000u64.index_of_next_false_bit(20), Some(20));
/// assert_eq!(0xb00000000u64.index_of_next_false_bit(31), Some(31));
/// assert_eq!(0xb00000000u64.index_of_next_false_bit(32), Some(34));
/// assert_eq!(0xb00000000u64.index_of_next_false_bit(33), Some(34));
/// assert_eq!(0xb00000000u64.index_of_next_false_bit(34), Some(34));
/// assert_eq!(0xb00000000u64.index_of_next_false_bit(35), Some(36));
/// assert_eq!(0xb00000000u64.index_of_next_false_bit(100), Some(100));
///
/// assert_eq!((-0x500000000i64).index_of_next_false_bit(0), Some(0));
/// assert_eq!((-0x500000000i64).index_of_next_false_bit(20), Some(20));
/// assert_eq!((-0x500000000i64).index_of_next_false_bit(31), Some(31));
/// assert_eq!((-0x500000000i64).index_of_next_false_bit(32), Some(34));
/// assert_eq!((-0x500000000i64).index_of_next_false_bit(33), Some(34));
/// assert_eq!((-0x500000000i64).index_of_next_false_bit(34), Some(34));
/// assert_eq!((-0x500000000i64).index_of_next_false_bit(35), None);
/// assert_eq!((-0x500000000i64).index_of_next_false_bit(100), None);
/// ```
///
/// # index_of_next_true_bit
/// ```
/// use malachite_base::num::logic::traits::BitScan;
///
/// assert_eq!(0xb00000000u64.index_of_next_true_bit(0), Some(32));
/// assert_eq!(0xb00000000u64.index_of_next_true_bit(20), Some(32));
/// assert_eq!(0xb00000000u64.index_of_next_true_bit(31), Some(32));
/// assert_eq!(0xb00000000u64.index_of_next_true_bit(32), Some(32));
/// assert_eq!(0xb00000000u64.index_of_next_true_bit(33), Some(33));
/// assert_eq!(0xb00000000u64.index_of_next_true_bit(34), Some(35));
/// assert_eq!(0xb00000000u64.index_of_next_true_bit(35), Some(35));
/// assert_eq!(0xb00000000u64.index_of_next_true_bit(36), None);
/// assert_eq!(0xb00000000u64.index_of_next_true_bit(100), None);
///
/// assert_eq!((-0x500000000i64).index_of_next_true_bit(0), Some(32));
/// assert_eq!((-0x500000000i64).index_of_next_true_bit(20), Some(32));
/// assert_eq!((-0x500000000i64).index_of_next_true_bit(31), Some(32));
/// assert_eq!((-0x500000000i64).index_of_next_true_bit(32), Some(32));
/// assert_eq!((-0x500000000i64).index_of_next_true_bit(33), Some(33));
/// assert_eq!((-0x500000000i64).index_of_next_true_bit(34), Some(35));
/// assert_eq!((-0x500000000i64).index_of_next_true_bit(35), Some(35));
/// assert_eq!((-0x500000000i64).index_of_next_true_bit(36), Some(36));
/// assert_eq!((-0x500000000i64).index_of_next_true_bit(100), Some(100));
/// ```
pub mod bit_scan;
/// [`CountOnes`](traits::CountOnes), a trait for counting the number of ones in the binary
/// representation of a number.
pub mod count_ones;
/// [`CountZeros`](traits::CountZeros), a trait for counting the number of ones in the binary
/// representation of a number.
pub mod count_zeros;
/// [`HammingDistance`](traits::HammingDistance) and
/// [`CheckedHammingDistance`](traits::CheckedHammingDistance), traits for computing the Hamming
/// distance between two numbers.
///
/// # hamming_distance
/// ```
/// use malachite_base::num::logic::traits::HammingDistance;
///
/// assert_eq!(123u32.hamming_distance(456), 6);
/// assert_eq!(0u8.hamming_distance(255), 8);
/// ```
///
/// # checked_hamming_distance
/// ```
/// use malachite_base::num::logic::traits::CheckedHammingDistance;
///
/// assert_eq!(123i32.checked_hamming_distance(456), Some(6));
/// assert_eq!(0i8.checked_hamming_distance(127), Some(7));
/// assert_eq!(0i8.checked_hamming_distance(-1), None);
/// ```
pub mod hamming_distance;
/// [`LeadingZeros`](traits::LeadingZeros), a trait for determining the number of zeros that a
/// number starts with, when written in binary using $W$ bits, $W$ being the type width.
pub mod leading_zeros;
/// [`LowMask`](traits::LowMask), a trait for generating a low bit mask (a number in which only the
/// $k$ least-significant bits are 1).
///
/// # low_mask
/// ```
/// use malachite_base::num::logic::traits::LowMask;
///
/// assert_eq!(u16::low_mask(0), 0);
/// assert_eq!(u8::low_mask(3), 0x7);
/// assert_eq!(u8::low_mask(8), 0xff);
/// assert_eq!(u64::low_mask(40), 0xffffffffff);
///
/// assert_eq!(i16::low_mask(0), 0);
/// assert_eq!(i8::low_mask(3), 0x7);
/// assert_eq!(i8::low_mask(8), -1);
/// assert_eq!(i64::low_mask(40), 0xffffffffff);
/// ```
pub mod low_mask;
/// [`NotAssign`](traits::NotAssign), a trait for replacing a number with its bitwise negation.
///
/// # not_assign
/// ```
/// use malachite_base::num::logic::traits::NotAssign;
///
/// let mut x = 123u16;
/// x.not_assign();
/// assert_eq!(x, 65412);
/// ```
pub mod not;
/// [`SignificantBits`](traits::SignificantBits), a trait for determining how many significant bits
/// a number has.
///
/// # significant_bits
/// ```
/// use malachite_base::num::logic::traits::SignificantBits;
///
/// assert_eq!(0u8.significant_bits(), 0);
/// assert_eq!(100u64.significant_bits(), 7);
///
/// assert_eq!(0i8.significant_bits(), 0);
/// assert_eq!((-100i64).significant_bits(), 7);
/// ```
pub mod significant_bits;
/// [`TrailingZeros`](traits::TrailingZeros), a trait for determining the number of zeros that a
/// number ends with when written in binary.
pub mod trailing_zeros;
/// Various traits for performing logic or bitwise operations on numbers.
pub mod traits;
