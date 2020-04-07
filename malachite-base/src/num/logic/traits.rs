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

/// Rotate a value's bits left or right.
pub trait Rotate {
    /// Shifts the bits to the left by a specified amount, `n`, wrapping the truncated bits to the
    /// end of the resulting value.
    ///
    /// Please note this isn't the same operation as `<<`!
    fn rotate_left(self, n: u64) -> Self;

    /// Shifts the bits to the right by a specified amount, `n`, wrapping the truncated bits to the
    /// end of the resulting value.
    ///
    /// Please note this isn't the same operation as `>>`!
    fn rotate_right(self, n: u64) -> Self;
}

/// Replaces a number with its bitwise negation.
pub trait NotAssign {
    fn not_assign(&mut self);
}

/// Provides a function to get the number of significant bits of `self`.
pub trait SignificantBits {
    /// The number of bits it takes to represent `self`. This is useful when benchmarking functions;
    /// the functions' inputs can be bucketed based on their number of significant bits.
    fn significant_bits(self) -> u64;
}

/// Returns the Hamming distance between `self` and `rhs`, or the number of bit flips needed to turn
/// `self` into `rhs`.
pub trait HammingDistance<RHS = Self> {
    fn hamming_distance(self, rhs: RHS) -> u64;
}

/// Returns the Hamming distance between `self` and `rhs`, or the number of bit flips needed to turn
/// `self` into `rhs`. This trait allows for the possibility of the distance being undefined for
/// some pairs of `self` and `rhs`, in which case the `checked_hamming_distance` function should
/// return `None`.
pub trait CheckedHammingDistance<RHS = Self> {
    fn checked_hamming_distance(self, rhs: RHS) -> Option<u64>;
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
    /// Time: worst case O(max(f(n), g(n))), where f(n) is the worst-case time complexity of
    ///     `Self::set_bit` and g(n) is the worst-case time complexity of `Self::clear_bit`.
    ///
    /// Additional memory: worst case O(max(f(n), g(n))), where f(n) is the worst-case
    ///     additional-memory complexity of `Self::set_bit` and g(n) is the worst-case
    ///     additional-memory complexity of `Self::clear_bit`.
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

    /// Sets the bit at `index` to the opposite of its previous value.
    ///
    /// Time: worst case O(f(n) + max(g(n), h(n))), where f(n) is the worst-case time complexity of
    ///     `Self::get_bit`, g(n) is the worst-case time complexity of `Self::set_bit`, and h(n) is
    ///     the worst-case time complexity of `Self::clear_bit`.
    ///
    /// Additional memory: worst case O(f(n) + max(g(n), h(n))), where f(n) is the worst-case
    ///     additional-memory complexity of `Self::get_bit`, g(n) is the worst-case
    ///     additional-memory complexity of `Self::set_bit`, and h(n) is the worst-case
    ///     additional-memory complexity of `Self::clear_bit`.
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
    /// Time: worst case O(f(n)), where f(n) is the worst-case time complexity of `Self::get_bits`.
    ///
    /// Additional memory: worst case O(f(n)), where f(n) is the worst-case additional-memory
    /// complexity of `Self::get_bits`.
    ///
    /// # Panics
    /// See panics for `get_bits`.
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

    /// Converts a slice of bits into a value. The input bits are in ascending order: least- to
    /// most-significant.
    fn from_bits_asc(bits: &[bool]) -> Self;

    /// Converts a slice of bits into a value. The input bits are in descending order: most- to
    /// least-significant.
    fn from_bits_desc(bits: &[bool]) -> Self;
}

/// This trait defines an iterator over a value's bits.
pub trait BitIterable {
    type BitIterator: Iterator<Item = bool> + DoubleEndedIterator<Item = bool> + Index<u64>;

    /// Returns a double-ended iterator over a value's bits. The iterator ends after the value's
    /// most-significant bit.
    fn bits(self) -> Self::BitIterator;
}

/// This trait defines functions that express a value as a `Vec` of digits and read a value from a
/// slice of bits, where the base is a power of two. The base-2 logarithm of the base is specified,
/// and the trait is parameterized by the digit type.
pub trait PowerOfTwoDigits<T> {
    /// Returns a `Vec` containing the digits of a value in ascending order: least- to most-
    /// significant. The base is 2<sup>`log_base`</sup>.
    fn to_power_of_two_digits_asc(&self, log_base: u64) -> Vec<T>;

    /// Returns a `Vec` containing the digits of a value in descending order: most- to least-
    /// significant. The base is 2<sup>`log_base`</sup>.
    fn to_power_of_two_digits_desc(&self, log_base: u64) -> Vec<T>;

    /// Converts a slice of digits into a value. The input digits are in ascending order: least- to
    /// most-significant. The base is 2<sup>`log_base`</sup>.
    fn from_power_of_two_digits_asc(log_base: u64, digits: &[T]) -> Self;

    /// Converts a slice of digits into a value. The input digits are in descending order: most- to
    /// least-significant. The base is 2<sup>`log_base`</sup>.
    fn from_power_of_two_digits_desc(log_base: u64, digits: &[T]) -> Self;
}

/// An iterator over a value's base-power-of-two digits.
pub trait PowerOfTwoDigitIterator<T>: Iterator<Item = T> + DoubleEndedIterator<Item = T> {
    fn get(&self, index: u64) -> T;
}

/// This trait defines an iterator over a value's base-power-of-two digits.
pub trait PowerOfTwoDigitIterable<T> {
    type PowerOfTwoDigitIterator: PowerOfTwoDigitIterator<T>;

    /// Returns a double-ended iterator over a value's digits in base 2<sup>`log_base`</sup>. The
    /// iterator ends after the value's most-significant digit.
    fn power_of_two_digits(self, log_base: u64) -> Self::PowerOfTwoDigitIterator;
}
