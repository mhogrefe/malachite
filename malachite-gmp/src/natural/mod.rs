use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::Natural::*;
use std::mem;

/// A natural (non-negative) integer backed by [GMP](https://gmplib.org/).
///
/// This code uses Trevor Spiteri's
/// [`gmp_mpfr_sys`](https://tspiteri.gitlab.io/gmp-mpfr/gmp_mpfr_sys/index.html) low-level
/// bindings.
///
/// Any `Natural` small enough to fit into a `u32` is represented inline. Only integers outside this
/// range incur the costs of FFI and heap-allocation.
pub enum Natural {
    /// A small `Natural`.
    Small(u32),
    /// A large `Natural`.
    Large(mpz_t),
}

impl Natural {
    /// Creates a new `Natural` equal to 0.
    ///
    /// # Example
    /// ```
    /// use malachite_gmp::natural::Natural;
    ///
    /// assert_eq!(Natural::new().to_string(), "0");
    /// ```
    pub fn new() -> Natural {
        Small(0)
    }

    fn demote_if_small(&mut self) {
        if let Large(x) = *self {
            if unsafe { gmp::mpz_sizeinbase(&x, 2) } <= 32 {
                let s = (unsafe { gmp::mpz_get_ui(&x) }) as u32;
                *self = Small(s);
            }
        }
    }

    fn promote(&self) -> Natural {
        match *self {
            Small(x) => unsafe {
                let mut promoted: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_si(&mut promoted, x.into());
                Large(promoted)
            },
            ref x => x.clone(),
        }
    }

    /// Returns true iff `self` is valid. To be valid, `self` cannot be negative and can only be
    /// Large when it is at least 2^(32). All Naturals used outside this crate are valid, but
    /// temporary Naturals used inside may not be.
    pub fn is_valid(&self) -> bool {
        match *self {
            Small(_) => true,
            Large(ref x) => (unsafe { gmp::mpz_cmp_ui(x, u32::max_value().into()) }) > 0,
        }
    }
}

/// Creates a default `Natural` equal to 0.
///
/// # Example
/// ```
/// use malachite_gmp::natural::Natural;
///
/// assert_eq!(Natural::default().to_string(), "0");
/// ```
impl Default for Natural {
    fn default() -> Natural {
        Small(0)
    }
}

/// If `self` is large, clears the GMP-allocated memory.
impl Drop for Natural {
    fn drop(&mut self) {
        if let Large(ref mut x) = *self {
            unsafe {
                gmp::mpz_clear(x);
            }
        }
    }
}

fn get_lower(val: u64) -> u32 {
    (val & 0x0000_0000_ffff_ffff) as u32
}

fn get_upper(val: u64) -> u32 {
    ((val & 0xffff_ffff_0000_0000) >> 32) as u32
}

fn make_u64(upper: u32, lower: u32) -> u64 {
    (upper as u64) << 32 | (lower as u64)
}

pub enum LimbSize {
    U32,
    U64,
}

pub fn get_limb_size() -> LimbSize {
    let zero: gmp::limb_t = 0;
    match zero.leading_zeros() {
        32 => LimbSize::U32,
        64 => LimbSize::U64,
        _ => unreachable!(),
    }
}

macro_rules! mutate_with_possible_promotion {
    ($n: ident, $small: ident, $large: ident, $process_small: expr, $process_large: expr) => {
        if let Small(ref mut $small) = *$n {
            if let Some(x) = $process_small {
                *$small = x;
                return;
            }
        }
        if let Small(x) = *$n {
            unsafe {
                let mut promoted: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut promoted, x.into());
                *$n = Large(promoted);
            }
        }
        if let Large(ref mut $large) = *$n {
            $process_large
        }
    };
}

pub mod arithmetic {
    pub mod add_u32;
    pub mod is_power_of_two;
}
pub mod conversion;
pub mod comparison {
    pub mod eq_natural;
    pub mod hash;
    pub mod ord_natural;
    pub mod partial_eq_integer;
    pub mod partial_eq_u32;
    pub mod partial_ord_integer;
    pub mod partial_ord_u32;
}
pub mod logic {
    pub mod assign_limbs_le;
    pub mod get_bit;
    pub mod limb_count;
    pub mod limbs_le;
    pub mod set_bit;
    pub mod significant_bits;
}
