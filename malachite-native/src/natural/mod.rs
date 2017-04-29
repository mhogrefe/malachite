use natural::Natural::*;

/// A natural (non-negative) integer.
///
/// Any `Natural` small enough to fit into an `u32` is represented inline. Only naturals outside
/// this range incur the costs of heap-allocation.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Natural {
    Small(u32),
    Large(Vec<u32>),
}

impl Natural {
    /// Creates a new `Natural` equal to 0.
    ///
    /// # Example
    /// ```
    /// use malachite_native::natural::Natural;
    ///
    /// assert_eq!(Natural::new().to_string(), "0");
    /// ```
    pub fn new() -> Natural {
        Small(0)
    }

    fn promote<'a>(&'a mut self) -> &'a mut Vec<u32> {
        if let &mut Small(x) = self {
            let xs = vec![x];
            *self = Large(xs);
        }
        if let &mut Large(ref mut xs) = self {
            xs
        } else {
            unreachable!();
        }
    }

    fn get_u32s_ref<'a>(&'a self) -> &'a Vec<u32> {
        if let Large(ref xs) = *self {
            xs
        } else {
            panic!("Can't get u32s from a Small Natural");
        }
    }

    fn trim(&mut self) {
        let mut demote = None;
        if let Large(ref mut xs) = *self {
            while !xs.is_empty() && xs[xs.len() - 1] == 0 {
                xs.pop();
            }
            if xs.len() == 1 {
                demote = Some(xs[0]);
            }
        }
        if let Some(x) = demote {
            *self = Small(x)
        }
    }

    /// Returns true iff `self` is valid. To be valid, `self` can only be Large when it is at least
    /// 2^(32), and cannot have leading zero limbs. All Naturals used outside this crate are valid,
    /// but temporary Naturals used inside may not be.
    pub fn is_valid(&self) -> bool {
        match self {
            &Small(_) => true,
            &Large(ref xs) => xs.len() > 1 && xs.last().unwrap() != &0,
        }
    }
}

/// Creates a default `Natural` equal to 0.
///
/// # Example
/// ```
/// use malachite_native::natural::Natural;
///
/// assert_eq!(Natural::default().to_string(), "0");
/// ```
impl Default for Natural {
    fn default() -> Natural {
        Small(0)
    }
}

fn make_u64(upper: u32, lower: u32) -> u64 {
    (upper as u64) << 32 | (lower as u64)
}

fn get_lower(val: u64) -> u32 {
    (val & 0x0000_0000_ffff_ffff) as u32
}

fn get_upper(val: u64) -> u32 {
    ((val & 0xffff_ffff_0000_0000) >> 32) as u32
}

pub mod arithmetic;
pub mod conversion;
pub mod comparison {
    pub mod partial_eq_u32;
}
pub mod logic {
    pub mod significant_bits;
}
