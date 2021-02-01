use std::ops::Index;

/// Returns the number of ones in the binary representation of `self`.
pub trait CountOnes {
    fn count_ones(self) -> u64;
}

/// Returns the number of zeros in the binary representation of `self`.
pub trait CountZeros {
    fn count_zeros(self) -> u64;
}

/// Returns the number of leading zeros in the binary representation of `self`.
pub trait LeadingZeros {
    fn leading_zeros(self) -> u64;
}

/// Returns the number of trailing zeros in the binary representation of `self`.
pub trait TrailingZeros {
    fn trailing_zeros(self) -> u64;
}

/// Replaces a number with its bitwise negation.
pub trait NotAssign {
    fn not_assign(&mut self);
}

/// Provides a function to get the number of significant bits of `self`.
pub trait SignificantBits {
    /// The number of bits it takes to represent `self`.
    fn significant_bits(self) -> u64;
}

/// Returns the Hamming distance between `self` and `other`, or the number of bit flips needed to
/// turn `self` into `other`.
pub trait HammingDistance<RHS = Self> {
    fn hamming_distance(self, other: RHS) -> u64;
}

/// Returns the Hamming distance between `self` and `other`, or the number of bit flips needed to
/// turn `self` into `other`.
///
/// This trait allows for the possibility of the distance being undefined for some pairs of `self`
/// and `other`, in which case the `checked_hamming_distance` function should return `None`.
pub trait CheckedHammingDistance<RHS = Self> {
    fn checked_hamming_distance(self, other: RHS) -> Option<u64>;
}

/// Returns a value with the least significant `bits` bits on and the remaining bits off.
pub trait LowMask {
    fn low_mask(bits: u64) -> Self;
}

/// This trait defines functions that access or modify individual bits in a value, indexed by a
/// `u64`.
pub trait BitAccess {
    /// Determines whether the bit at `index` is true or false.
    fn get_bit(&self, index: u64) -> bool;

    /// Sets the bit at `index` to true.
    fn set_bit(&mut self, index: u64);

    /// Sets the bit at `index` to false.
    fn clear_bit(&mut self, index: u64);

    /// Sets the bit at `index` to whichever value `bit` is.
    ///
    /// # Worst-case complexity
    ///
    /// $T(n) = O(\max(T_S(n), T_C(n)))$,
    ///
    /// $M(n) = O(\max(M_S(n), M_C(n)))$
    ///
    /// where $T$ is time, $M$ is additional memory, $T_S$ and $M_S$ are the complexities of
    /// `Self::set_bit`, and $T_C$ and $M_C$ are the complexities of `Self::clear_bit`.
    ///
    /// # Panics
    /// See panics for `set_bit` and `assign_bit`.
    #[inline]
    fn assign_bit(&mut self, index: u64, bit: bool) {
        if bit {
            self.set_bit(index);
        } else {
            self.clear_bit(index);
        }
    }

    /// Sets the bit at `index` to the opposite of its original value.
    ///
    /// # Worst-case complexity
    ///
    /// $T(n) = O(T_G(n) + \max(T_S(n), T_C(n)))$,
    ///
    /// $M(n) = O(M_G(n) + \max(M_S(n), M_C(n)))$
    ///
    /// where $T$ is time, $M$ is additional memory, $T_G$ and $M_G$ are the complexities of
    /// `Self::get_bit`, $T_S$ and $M_S$ are the complexities of `Self::set_bit`, and $T_C$ and
    /// $M_C$ are the complexities of `Self::clear_bit`.
    ///
    /// # Panics
    /// See panics for `get_bit`, `set_bit` and `assign_bit`.
    #[inline]
    fn flip_bit(&mut self, index: u64) {
        if self.get_bit(index) {
            self.clear_bit(index);
        } else {
            self.set_bit(index);
        }
    }
}

/// This trait defines functions search for the next true or false bit, starting at a specified
/// index and searching in the more-significant direction.
pub trait BitScan {
    /// Finds the smallest index of a `false` bit that is greater than or equal to `start`.
    fn index_of_next_false_bit(self, start: u64) -> Option<u64>;

    /// Finds the smallest index of a `true` bit that is greater than or equal to `start`.
    fn index_of_next_true_bit(self, start: u64) -> Option<u64>;
}

/// This trait defines functions that access or modify blocks of adjacent bits in a value, where the
/// start (inclusive) and end (exclusive) indices of the block are specified.
pub trait BitBlockAccess: Sized {
    type Bits;

    /// Extracts a block of bits whose first index is `start` and last index is `end - 1`.
    fn get_bits(&self, start: u64, end: u64) -> Self::Bits;

    /// Extracts a block of bits whose first index is `start` and last index is `end - 1`, taking
    /// ownership of `self`.
    ///
    /// # Worst-case complexity
    /// For the default implementation, same as `Self::get_bits`.
    ///
    /// # Panics
    /// See panics for `Self::get_bits`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::logic::traits::BitBlockAccess;
    ///
    /// assert_eq!((-0x5433i16).get_bits_owned(4, 8), 0xc);
    /// assert_eq!((-0x5433i16).get_bits_owned(5, 9), 14);
    /// assert_eq!((-0x5433i16).get_bits_owned(5, 5), 0);
    /// assert_eq!((-0x5433i16).get_bits_owned(100, 104), 0xf);
    /// ```
    #[inline]
    fn get_bits_owned(self, start: u64, end: u64) -> Self::Bits {
        self.get_bits(start, end)
    }

    /// Assigns the least-significant `end - start` bits of `bits` to bits `start` (inclusive)
    /// through `end` (exclusive) of `self`.
    fn assign_bits(&mut self, start: u64, end: u64, bits: &Self::Bits);
}

/// This trait defines functions that express a value as a `Vec` of bits and read a value from a
/// slice of bits.
pub trait BitConvertible {
    /// Returns a `Vec` containing the bits of a value in ascending order: least- to most-
    /// significant.
    fn to_bits_asc(&self) -> Vec<bool>;

    /// Returns a `Vec` containing the bits of a value in descending order: most- to least-
    /// significant.
    fn to_bits_desc(&self) -> Vec<bool>;

    /// Converts an iterator of bits into a value. The input bits are in ascending order: least- to
    /// most-significant.
    fn from_bits_asc<I: Iterator<Item = bool>>(bits: I) -> Self;

    /// Converts an iterator of bits into a value. The input bits are in descending order: most- to
    /// least-significant.
    fn from_bits_desc<I: Iterator<Item = bool>>(bits: I) -> Self;
}

/// This trait defines an iterator over a value's bits.
pub trait BitIterable {
    type BitIterator: Iterator<Item = bool> + DoubleEndedIterator<Item = bool> + Index<u64>;

    /// Returns a double-ended iterator over a value's bits. The iterator ends after the value's
    /// most-significant bit.
    fn bits(self) -> Self::BitIterator;
}
