use malachite_base::misc::{Min, Named, Walkable};
use malachite_base::num::{One, Two, Zero};
use natural::Natural::*;
use std::str::FromStr;

/// A natural (non-negative) integer.
///
/// Any `Natural` small enough to fit into an `u32` is represented inline. Only naturals outside
/// this range incur the costs of heap-allocation.
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Natural {
    Small(u32),
    Large(Vec<u32>),
}

impl Natural {
    fn demote_if_small(&mut self) {
        let mut demoted_value = None;
        if let Large(ref limbs) = *self {
            let limb_count = limbs.len();
            match limb_count {
                0 => demoted_value = Some(0),
                1 => demoted_value = Some(limbs[0]),
                _ => {}
            }
        }
        if let Some(small) = demoted_value {
            *self = Small(small);
        }
    }

    pub(crate) fn promote_in_place(&mut self) -> &mut Vec<u32> {
        if let Small(x) = *self {
            *self = Large(vec![x]);
        }
        if let Large(ref mut xs) = *self {
            xs
        } else {
            unreachable!();
        }
    }

    pub(crate) fn trim(&mut self) {
        if let Large(ref mut xs) = *self {
            while !xs.is_empty() && xs.last().unwrap() == &0 {
                xs.pop();
            }
        }
        self.demote_if_small();
    }

    /// Returns true iff `self` is valid. To be valid, `self` can only be Large when it is at least
    /// 2<sup>32</sup>, and cannot have leading zero limbs. All Naturals must be valid.
    pub fn is_valid(&self) -> bool {
        match *self {
            Small(_) => true,
            Large(ref xs) => xs.len() > 1 && xs.last().unwrap() != &0,
        }
    }

    pub fn trillion() -> Natural {
        Natural::from_str("1000000000000").unwrap()
    }

    //TODO test
    pub fn count_ones(&self) -> u64 {
        match *self {
            Small(small) => small.count_ones().into(),
            Large(ref limbs) => limbs.iter().map(|limb| u64::from(limb.count_ones())).sum(),
        }
    }
}

/// The constant 0.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Zero for Natural {
    const ZERO: Natural = Small(0);
}

/// The constant 1.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl One for Natural {
    const ONE: Natural = Small(1);
}

/// The constant 2.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Two for Natural {
    const TWO: Natural = Small(2);
}

/// The minimum value of a `Natural`, 0.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Min for Natural {
    const MIN: Natural = Small(0);
}

/// Implement `Named` for `Natural`.
impl_named!(Natural);

impl Walkable for Natural {
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
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut i = Natural::from(10u32);
    ///     i.increment();
    ///     assert_eq!(i, 11);
    /// }
    /// ```
    fn increment(&mut self) {
        *self += 1;
    }

    /// Decrements `self`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `self` == 0`.
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::misc::Walkable;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut i = Natural::from(10u32);
    ///     i.decrement();
    ///     assert_eq!(i, 9);
    /// }
    /// ```
    fn decrement(&mut self) {
        *self -= 1;
    }
}

macro_rules! mutate_with_possible_promotion {
    ($n: ident, $small: ident, $large: ident, $process_small: expr, $process_large: expr) => {
        if let Small(ref mut $small) = *$n {
            if let Some(small_result) = $process_small {
                *$small = small_result;
                return;
            }
        }
        if let Small(x) = *$n {
            *$n = Large(vec![x]);
        }
        if let Large(ref mut $large) = *$n {
            $process_large
        }
    };
}

pub mod arithmetic;
pub mod conversion;
pub mod comparison {
    pub mod ord;
    pub mod partial_eq_u32;
    pub mod partial_ord_u32;
}
pub mod logic {
    pub mod bit_access;
    pub mod not;
    pub mod significant_bits;
    pub mod trailing_zeros;
}
pub mod random {
    pub mod random_natural_below;
    pub mod random_natural_up_to_bits;
    pub mod random_natural_with_bits;
    pub mod special_random_natural_below;
    pub mod special_random_natural_up_to_bits;
    pub mod special_random_natural_with_bits;
}
