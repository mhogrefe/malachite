/// Returns the number of ones in the binary representation of `self`.
pub trait CountOnes {
    fn count_ones(self) -> u32;
}

/// Returns the number of zeros in the binary representation of `self`.
pub trait CountZeros {
    fn count_zeros(self) -> u32;
}

/// Returns the number of leading zeros in the binary representation of `self`.
pub trait LeadingZeros {
    fn leading_zeros(self) -> u32;
}

/// Returns the number of trailing zeros in the binary representation of `self`.
pub trait TrailingZeros {
    fn trailing_zeros(self) -> u32;
}

/// Shifts the bits to the left by a specified amount, `n`, wrapping the truncated bits to the end
/// of the resulting value.
///
/// Please note this isn't the same operation as `<<`!
pub trait RotateLeft {
    fn rotate_left(self, n: u32) -> Self;
}

/// Shifts the bits to the right by a specified amount, `n`, wrapping the truncated bits to the end
/// of the resulting value.
///
/// Please note this isn't the same operation as `>>`!
pub trait RotateRight {
    fn rotate_right(self, n: u32) -> Self;
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

//TODO docs, test
pub trait BitScan {
    fn index_of_next_false_bit(self, starting_index: u64) -> Option<u64>;

    fn index_of_next_true_bit(self, starting_index: u64) -> Option<u64>;
}
