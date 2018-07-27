use malachite_base::misc::{Named, Walkable};
use malachite_base::num::{NegativeOne, One, Two, Zero};
use natural::Natural::{self, Small};
use std::str::FromStr;

/// An integer.
///
/// Any `Integer` whose absolute value is small enough to fit into an `u32` is represented inline.
/// Only integers outside this range incur the costs of heap-allocation.
#[derive(Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Integer {
    pub(crate) sign: bool, // whether the Integer is non-negative
    pub(crate) abs: Natural,
}

impl Integer {
    /// Returns true iff `self` is valid. To be valid, its absolute value must be valid, and if the
    /// absolute value is zero, the sign must be true. All Integers must be valid.
    pub fn is_valid(&self) -> bool {
        self.abs.is_valid() && (self.sign || self.abs != 0)
    }

    pub fn trillion() -> Integer {
        Integer::from_str("1000000000000").unwrap()
    }
}

/// The constant 0.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Zero for Integer {
    const ZERO: Integer = Integer {
        sign: true,
        abs: Small(0),
    };
}

/// The constant 1.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl One for Integer {
    const ONE: Integer = Integer {
        sign: true,
        abs: Small(1),
    };
}

/// The constant 2.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Two for Integer {
    const TWO: Integer = Integer {
        sign: true,
        abs: Small(2),
    };
}

/// The constant -1.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl NegativeOne for Integer {
    const NEGATIVE_ONE: Integer = Integer {
        sign: false,
        abs: Small(1),
    };
}

/// Implement `Named` for `Integer`.
impl_named!(Integer);

impl Walkable for Integer {
    /// Increments `self`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::misc::Walkable;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     let mut i = Integer::from(10);
    ///     i.increment();
    ///     assert_eq!(i, 11);
    ///
    ///     let mut i = Integer::from(-5);
    ///     i.increment();
    ///     assert_eq!(i, -4);
    /// }
    /// ```
    fn increment(&mut self) {
        *self += 1u32;
    }

    /// Decrements `self`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::misc::Walkable;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     let mut i = Integer::from(10);
    ///     i.decrement();
    ///     assert_eq!(i, 9);
    ///
    ///     let mut i = Integer::from(-5);
    ///     i.decrement();
    ///     assert_eq!(i, -6);
    /// }
    /// ```
    fn decrement(&mut self) {
        *self -= 1u32;
    }
}

pub mod arithmetic {
    pub mod abs;
    pub mod add;
    pub mod add_i32;
    pub mod add_mul;
    pub mod add_mul_i32;
    pub mod add_mul_u32;
    pub mod add_u32;
    pub mod divisible_by_power_of_two;
    pub mod mod_power_of_two;
    pub mod mul;
    pub mod mul_i32;
    pub mod mul_u32;
    pub mod neg;
    pub mod parity;
    pub mod shl_i32;
    pub mod shl_u;
    pub mod shr_i32;
    pub mod shr_u;
    pub mod sub;
    pub mod sub_i32;
    pub mod sub_mul;
    pub mod sub_mul_i32;
    pub mod sub_mul_u32;
    pub mod sub_u32;
}
pub mod comparison {
    pub mod ord;
    pub mod ord_abs;
    pub mod partial_eq_i32;
    pub mod partial_eq_natural;
    pub mod partial_eq_u32;
    pub mod partial_ord_abs_i32;
    pub mod partial_ord_abs_natural;
    pub mod partial_ord_abs_u32;
    pub mod partial_ord_i32;
    pub mod partial_ord_natural;
    pub mod partial_ord_u32;
    pub mod sign;
}
pub mod conversion;
pub mod logic {
    pub mod and;
    pub mod and_i32;
    pub mod and_u32;
    pub mod bit_access;
    pub mod bit_scan;
    pub mod checked_count_ones;
    pub mod checked_count_zeros;
    pub mod checked_hamming_distance;
    pub mod checked_hamming_distance_i32;
    pub mod checked_hamming_distance_u32;
    pub mod not;
    pub mod or;
    pub mod or_i32;
    pub mod or_u32;
    pub mod significant_bits;
    pub mod trailing_zeros;
    pub mod xor;
    pub mod xor_i32;
    pub mod xor_u32;
}
