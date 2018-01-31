use malachite_base::num::{One, Two, Zero};
use natural::Natural::*;
use std::str::FromStr;

pub const LOG_LIMB_BITS: u32 = 5;
pub const LIMB_BITS: u32 = 1 << LOG_LIMB_BITS;
pub const LIMB_BITS_MASK: u32 = LIMB_BITS - 1;

/// A natural (non-negative) integer.
///
/// Any `Natural` small enough to fit into an `u32` is represented inline. Only naturals outside
/// this range incur the costs of heap-allocation.
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Natural {
    Small(u32),
    Large(Vec<u32>),
}

// Test s and return whether the operand is zero.
pub fn mpn_zero_p(s: &[u32]) -> bool {
    s.iter().all(|&x| x == 0)
}

// Zero r.
pub fn mpn_zero(r: &mut [u32]) {
    for x in r.iter_mut() {
        *x = 0;
    }
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

fn pad_left<T: Clone>(vec: &mut Vec<T>, pad_size: usize, value: T) {
    let old_len = vec.len();
    vec.resize(old_len + pad_size, value);
    for i in (0..old_len).rev() {
        vec.swap(i, i + pad_size);
    }
}

fn delete_left<T>(vec: &mut Vec<T>, delete_size: usize) {
    assert!(vec.len() >= delete_size);
    let remaining_size = vec.len() - delete_size;
    for i in 0..remaining_size {
        vec.swap(i, i + delete_size);
    }
    vec.truncate(remaining_size);
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
    pub mod from_limbs;
    pub mod limb_count;
    pub mod limbs;
    pub mod not;
    pub mod significant_bits;
    pub mod trailing_zeros;
}
pub mod random;
