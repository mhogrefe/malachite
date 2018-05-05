use integer::Integer;
use malachite_base::limbs::{limbs_set_zero, limbs_trailing_zero_limbs};
use natural::Natural::{self, Large, Small};
use std::cmp::max;
use std::ops::{BitAnd, BitAndAssign};
use std::u32;

pub fn limbs_and_pos_neg(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_trailing_zero_limbs(xs);
    let y_i = limbs_trailing_zero_limbs(ys);
    if y_i >= xs_len {
        return Vec::new();
    } else if x_i >= ys_len {
        return xs.to_vec();
    }
    let max_i = max(x_i, y_i);
    let mut result_limbs = vec![0; max_i];
    result_limbs.push(
        xs[max_i] & if x_i <= y_i {
            ys[max_i].wrapping_neg()
        } else {
            !ys[max_i]
        },
    );
    result_limbs.extend(
        xs[max_i + 1..]
            .iter()
            .zip(ys[max_i + 1..].iter())
            .map(|(&x, &y)| x & !y),
    );
    if xs_len > ys_len {
        result_limbs.extend_from_slice(&xs[ys_len..]);
    }
    result_limbs
}

pub fn limbs_and_pos_neg_in_place_left(xs: &mut [u32], ys: &[u32]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_trailing_zero_limbs(xs);
    let y_i = limbs_trailing_zero_limbs(ys);
    if y_i >= xs_len {
        limbs_set_zero(xs);
    } else if x_i >= ys_len {
        return;
    }
    let max_i = max(x_i, y_i);
    limbs_set_zero(&mut xs[0..max_i]);
    xs[max_i] &= if x_i <= y_i {
        ys[max_i].wrapping_neg()
    } else {
        !ys[max_i]
    };
    for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter()) {
        *x &= !y;
    }
}

pub fn limbs_slice_and_pos_neg_in_place_right(xs: &[u32], ys: &mut [u32]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_trailing_zero_limbs(xs);
    let y_i = limbs_trailing_zero_limbs(ys);
    if y_i >= xs_len {
        limbs_set_zero(ys);
    } else if x_i >= ys_len {
        limbs_set_zero(ys);
        return true;
    }
    let max_i = max(x_i, y_i);
    limbs_set_zero(&mut ys[0..max_i]);
    {
        let ys_max_i = &mut ys[max_i];
        if x_i <= y_i {
            *ys_max_i = ys_max_i.wrapping_neg();
        } else {
            *ys_max_i = !*ys_max_i;
        }
        *ys_max_i &= xs[max_i];
    }
    for (x, y) in xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter_mut()) {
        *y = !*y & x;
    }
    xs_len > ys_len
}

pub fn limbs_vec_and_pos_neg_in_place_right(xs: &[u32], ys: &mut Vec<u32>) {
    if limbs_slice_and_pos_neg_in_place_right(xs, ys) {
        let ys_len = ys.len();
        ys.extend(xs[ys_len..].iter());
    }
}

fn limbs_and_neg_neg_helper(input: u32, boundary_limb_seen: &mut bool) -> u32 {
    if *boundary_limb_seen {
        input
    } else {
        let result = input.wrapping_add(1);
        if result != 0 {
            *boundary_limb_seen = true;
        }
        result
    }
}

pub fn limbs_and_neg_neg(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_trailing_zero_limbs(xs);
    let y_i = limbs_trailing_zero_limbs(ys);
    if y_i >= xs_len {
        return ys.to_vec();
    } else if x_i >= ys_len {
        return xs.to_vec();
    }
    let max_i = max(x_i, y_i);
    let mut result_limbs = vec![0; max_i];
    let x = if x_i >= y_i {
        xs[max_i].wrapping_sub(1)
    } else {
        xs[max_i]
    };
    let y = if x_i <= y_i {
        ys[max_i].wrapping_sub(1)
    } else {
        ys[max_i]
    };
    let mut boundary_limb_seen = false;
    result_limbs.push(limbs_and_neg_neg_helper(x | y, &mut boundary_limb_seen));
    let xys = xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter());
    if boundary_limb_seen {
        result_limbs.extend(xys.map(|(&x, &y)| x | y));
    } else {
        for (&x, &y) in xys {
            result_limbs.push(limbs_and_neg_neg_helper(x | y, &mut boundary_limb_seen));
        }
    }
    if xs_len != ys_len {
        let zs = if xs_len > ys_len {
            &xs[ys_len..]
        } else {
            &ys[xs_len..]
        };
        if boundary_limb_seen {
            result_limbs.extend_from_slice(zs);
        } else {
            for &z in zs.iter() {
                result_limbs.push(limbs_and_neg_neg_helper(z, &mut boundary_limb_seen));
            }
        }
    }
    if !boundary_limb_seen {
        result_limbs.push(1);
    }
    result_limbs
}

/// Takes the bitwise and of two `Integer`s, taking both by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((Integer::from(-123) & Integer::from(-456)).to_string(), "-512");
/// assert_eq!((-Integer::trillion() & -(Integer::trillion() + 1u32)).to_string(),
///     "-1000000004096");
/// ```
impl BitAnd<Integer> for Integer {
    type Output = Integer;

    fn bitand(mut self, other: Integer) -> Integer {
        self &= other;
        self
    }
}

/// Takes the bitwise and of two `Integer`s, taking the left `Integer` by value and the right
/// `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((Integer::from(-123) & &Integer::from(-456)).to_string(), "-512");
/// assert_eq!((-Integer::trillion() & &-(Integer::trillion() + 1u32)).to_string(),
///     "-1000000004096");
/// ```
impl<'a> BitAnd<&'a Integer> for Integer {
    type Output = Integer;

    fn bitand(mut self, other: &'a Integer) -> Integer {
        self &= other;
        self
    }
}

/// Takes the bitwise and of two `Integer`s, taking the left `Integer` by reference and the right
/// `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((&Integer::from(-123) & Integer::from(-456)).to_string(), "-512");
/// assert_eq!((&-Integer::trillion() & -(Integer::trillion() + 1u32)).to_string(),
///     "-1000000004096");
/// ```
impl<'a> BitAnd<Integer> for &'a Integer {
    type Output = Integer;

    fn bitand(self, mut other: Integer) -> Integer {
        other &= self;
        other
    }
}

/// Takes the bitwise and of two `Integer`s, taking both `Integer`s by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `max(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((&Integer::from(-123) & &Integer::from(-456)).to_string(), "-512");
/// assert_eq!((&-Integer::trillion() & &-(Integer::trillion() + 1u32)).to_string(),
///     "-1000000004096");
/// ```
impl<'a, 'b> BitAnd<&'a Integer> for &'b Integer {
    type Output = Integer;

    fn bitand(self, other: &'a Integer) -> Integer {
        match (self.sign, other.sign) {
            (true, true) => Integer {
                sign: true,
                abs: &self.abs & &other.abs,
            },
            (true, false) => Integer {
                sign: true,
                abs: self.abs.and_pos_neg(&other.abs),
            },
            (false, true) => Integer {
                sign: true,
                abs: other.abs.and_pos_neg(&self.abs),
            },
            (false, false) => Integer {
                sign: false,
                abs: self.abs.and_neg_neg(&other.abs),
            },
        }
    }
}

/// Bitwise-ands an `Integer` with another `Integer` in place, taking the `Integer` on the RHS by
/// value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::NegativeOne;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::NEGATIVE_ONE;
///     x &= Integer::from(0x70ff_ffff);
///     x &= Integer::from(0x7ff0_ffff);
///     x &= Integer::from(0x7fff_f0ff);
///     x &= Integer::from(0x7fff_fff0);
///     assert_eq!(x, 0x70f0f0f0);
/// }
/// ```
impl BitAndAssign<Integer> for Integer {
    fn bitand_assign(&mut self, other: Integer) {
        //TODO
        *self &= &other;
    }
}

/// Bitwise-ands an `Integer` with another `Integer` in place, taking the `Integer` on the RHS by
/// reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::NegativeOne;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::NEGATIVE_ONE;
///     x &= &Integer::from(0x70ff_ffff);
///     x &= &Integer::from(0x7ff0_ffff);
///     x &= &Integer::from(0x7fff_f0ff);
///     x &= &Integer::from(0x7fff_fff0);
///     assert_eq!(x, 0x70f0f0f0);
/// }
/// ```
impl<'a> BitAndAssign<&'a Integer> for Integer {
    fn bitand_assign(&mut self, other: &'a Integer) {
        //TODO
        *self = &*self & other;
    }
}

impl Natural {
    fn and_pos_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Small(y)) => self.and_pos_u32_neg(y.wrapping_neg()),
            (&Small(x), &Large(ref ys)) => Small(x & ys[0].wrapping_neg()),
            (&Large(ref xs), &Large(ref ys)) => {
                let mut result = Large(limbs_and_pos_neg(xs, ys));
                result.trim();
                result
            }
        }
    }

    fn and_neg_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Small(y)) => self.and_neg_u32_neg(y.wrapping_neg()),
            (&Small(x), _) => other.and_neg_u32_neg(x.wrapping_neg()),
            (&Large(ref xs), &Large(ref ys)) => {
                let mut result = Large(limbs_and_neg_neg(xs, ys));
                result.trim();
                result
            }
        }
    }
}
