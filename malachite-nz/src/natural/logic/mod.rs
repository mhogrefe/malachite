// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Bitwise and of [`Natural`](super::Natural)s.
pub mod and;
/// An implementation of [`BitAccess`](malachite_base::num::logic::traits::BitAccess), a trait for
/// getting and setting individual bits of a number.
pub mod bit_access;
/// An implementation of [`BitBlockAccess`](malachite_base::num::logic::traits::BitBlockAccess), a
/// trait for getting and setting adjacent blocks of bits in a number.
pub mod bit_block_access;
/// An implementation of [`BitConvertible`](malachite_base::num::logic::traits::BitConvertible), a
/// trait for extracting all bits from a number or constructing a number from bits.
pub mod bit_convertible;
/// An implementation of [`BitIterable`](malachite_base::num::logic::traits::BitIterable), a trait
/// for producing a double-ended iterator over a number's bits.
pub mod bit_iterable;
/// An implementation of [`BitScan`](malachite_base::num::logic::traits::BitScan), a trait for
/// finding the next `true` or `false` bit in a number after a provided index.
pub mod bit_scan;
/// An implementation of [`CountOnes`](malachite_base::num::logic::traits::CountOnes), a trait for
/// counting the number of ones in the binary representation of a number.
pub mod count_ones;
/// An implementation of [`HammingDistance`](malachite_base::num::logic::traits::HammingDistance), a
/// trait for computing the Hamming distance between two numbers.
pub mod hamming_distance;
/// An implementation of [`LowMask`](malachite_base::num::logic::traits::LowMask), a trait for
/// generating a low bit mask (a number in which only the $k$ least-significant bits are 1).
pub mod low_mask;
/// Bitwise negation of [`Natural`](super::Natural)s.
pub mod not;
/// Bitwise or of [`Natural`](super::Natural)s.
pub mod or;
/// An implementation of [`SignificantBits`](malachite_base::num::logic::traits::SignificantBits), a
/// trait for determining how many significant bits a number has.
pub mod significant_bits;
/// An implementation of [`TrailingZeros`](malachite_base::num::logic::traits::TrailingZeros), a
/// trait for determining the number of zeros that a number ends with when written in binary.
pub mod trailing_zeros;
/// Bitwise xor of [`Natural`](super::Natural)s.
pub mod xor;
