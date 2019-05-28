use std::cmp::max;
use std::ops::{BitOr, BitOrAssign};

use malachite_base::limbs::{limbs_leading_zero_limbs, limbs_set_zero};

use integer::Integer;
use natural::Natural::{self, Large, Small};
use platform::Limb;

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
/// `Integer`s, returns the limbs of the bitwise or of the `Integer`s. `xs` and `ys` may not be
/// empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.len() + ys.len()`, m = min(`xs.len()`, `ys.len()`)
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or::limbs_or_neg_neg;
///
/// assert_eq!(limbs_or_neg_neg(&[1, 2], &[100, 200]), &[1, 0]);
/// assert_eq!(limbs_or_neg_neg(&[1, 2, 5], &[100, 200]), &[1, 0]);
/// ```
///
/// This is mpz_ior from mpz/ior.c where res is returned and both inputs are negative.
pub fn limbs_or_neg_neg(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        return xs.to_vec();
    } else if x_i >= ys_len {
        return ys.to_vec();
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    let mut result_limbs = vec![0; min_i];
    if x_i > y_i {
        result_limbs.extend_from_slice(&ys[y_i..x_i]);
    } else if y_i > x_i {
        result_limbs.extend_from_slice(&xs[x_i..y_i]);
    }
    result_limbs.push(if x_i == y_i {
        ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1
    } else if x_i > y_i {
        (xs[x_i] - 1) & ys[x_i]
    } else {
        xs[y_i] & (ys[y_i] - 1)
    });
    result_limbs.extend(
        xs[max_i + 1..]
            .iter()
            .zip(ys[max_i + 1..].iter())
            .map(|(x, y)| x & y),
    );
    result_limbs
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
/// `Integer`s, writes the max(`xs.len()`, `ys.len()`) limbs of the bitwise or of the `Integer`s to
/// an output slice. `xs` and `ys` may not be empty or only contain zeros. The output slice must be
/// at least as long as the shorter input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len() + ys.len()`
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros, or if `out` is shorter than the
/// shorter of `xs` and `ys`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or::limbs_or_neg_neg_to_out;
///
/// let mut result = vec![10, 10];
/// limbs_or_neg_neg_to_out(&mut result, &[1, 2], &[100, 200]);
/// assert_eq!(result, &[1, 0]);
///
/// let mut result = vec![10, 10, 10, 10];
/// limbs_or_neg_neg_to_out(&mut result, &[1, 2, 5], &[100, 200]);
/// assert_eq!(result, &[1, 0, 10, 10]);
/// ```
///
/// This is mpz_ior from mpz/ior.c where both inputs are negative.
pub fn limbs_or_neg_neg_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= xs_len || out.len() >= ys_len);
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        out[..xs_len].copy_from_slice(xs);
        return;
    } else if x_i >= ys_len {
        out[..ys_len].copy_from_slice(ys);
        return;
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    limbs_set_zero(&mut out[..min_i]);
    if x_i > y_i {
        out[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
    } else if y_i > x_i {
        out[x_i..y_i].copy_from_slice(&xs[x_i..y_i]);
    }
    out[max_i] = if x_i == y_i {
        ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1
    } else if x_i > y_i {
        (xs[x_i] - 1) & ys[x_i]
    } else {
        xs[y_i] & (ys[y_i] - 1)
    };
    for (out, (x, y)) in out[max_i + 1..]
        .iter_mut()
        .zip(xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter()))
    {
        *out = x & y;
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
/// `Integer`s, writes the limbs of the bitwise or of the `Integer`s to the first (left) slice. `xs`
/// and `ys` may not be empty or only contain zeros. If the result has fewer significant limbs than
/// the left slice, the remaining limbs in the left slice are set to `Limb::MAX`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len() + ys.len()`
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or::limbs_slice_or_neg_neg_in_place_left;
///
/// let mut xs = vec![1, 2];
/// limbs_slice_or_neg_neg_in_place_left(&mut xs, &[100, 200]);
/// assert_eq!(xs, &[1, 0]);
///
/// let mut xs = vec![1, 2, 5];
/// limbs_slice_or_neg_neg_in_place_left(&mut xs, &[100, 200]);
/// assert_eq!(xs, &[1, 0, 0]);
/// ```
///
/// This is mpz_ior from mpz/ior.c where res == op1, both inputs are negative, and the length of op1
/// is not changed.
pub fn limbs_slice_or_neg_neg_in_place_left(xs: &mut [Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        return;
    } else if x_i >= ys_len {
        xs[..ys_len].copy_from_slice(ys);
        limbs_set_zero(&mut xs[ys_len..]);
        return;
    }
    let max_i = max(x_i, y_i);
    if x_i > y_i {
        xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
    }
    xs[max_i] = if x_i == y_i {
        ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1
    } else if x_i > y_i {
        (xs[x_i] - 1) & ys[x_i]
    } else {
        xs[y_i] & (ys[y_i] - 1)
    };
    for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter()) {
        *x &= y;
    }
    if xs_len > ys_len {
        limbs_set_zero(&mut xs[ys_len..]);
    }
}

/// Interpreting a slice of `Limb`s and a `Vec` of `Limb`s as the limbs (in ascending order) of the
/// negatives of two `Integer`s, writes the limbs of the bitwise or of the `Integer`s to the `Vec`.
/// `xs` and `ys` may not be empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len() + ys.len()`
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or::limbs_vec_or_neg_neg_in_place_left;
///
/// let mut xs = vec![1, 2];
/// limbs_vec_or_neg_neg_in_place_left(&mut xs, &[100, 200]);
/// assert_eq!(xs, &[1, 0]);
///
/// let mut xs = vec![100, 200];
/// limbs_vec_or_neg_neg_in_place_left(&mut xs, &[1, 2, 5]);
/// assert_eq!(xs, &[1, 0]);
///
/// let mut xs = vec![1, 2, 5];
/// limbs_vec_or_neg_neg_in_place_left(&mut xs, &[100, 200]);
/// assert_eq!(xs, &[1, 0]);
/// ```
///
/// This is mpz_ior from mpz/ior.c where res == op1 and both inputs are negative.
pub fn limbs_vec_or_neg_neg_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        return;
    } else if x_i >= ys_len {
        xs.truncate(ys_len);
        xs.copy_from_slice(ys);
        return;
    }
    let max_i = max(x_i, y_i);
    if x_i > y_i {
        xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
    }
    xs[max_i] = if x_i == y_i {
        ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1
    } else if x_i > y_i {
        (xs[x_i] - 1) & ys[x_i]
    } else {
        xs[y_i] & (ys[y_i] - 1)
    };
    for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter()) {
        *x &= y;
    }
    xs.truncate(ys_len);
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
/// `Integer`s, writes the lower min(`xs.len()`, `ys.len()`) limbs of the bitwise or of the
/// `Integer`s to the shorter slice (or the first one, if they are equally long). `xs` and `ys` may
/// not be empty or only contain zeros. Returns a `bool` which is `false` when the output is to the
/// first slice and `true` when it's to the second slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len() + ys.len()`
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or::limbs_or_neg_neg_in_place_either;
///
/// let mut xs = vec![1, 2];
/// let mut ys = vec![100, 200];
/// assert_eq!(limbs_or_neg_neg_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[1, 0]);
/// assert_eq!(ys, &[100, 200]);
///
/// let mut xs = vec![1, 2, 5];
/// let mut ys = vec![100, 200];
/// assert_eq!(limbs_or_neg_neg_in_place_either(&mut xs, &mut ys), true);
/// assert_eq!(xs, &[1, 2, 5]);
/// assert_eq!(ys, &[1, 0]);
///
/// let mut xs = vec![100, 200];
/// let mut ys = vec![1, 2, 5];
/// assert_eq!(limbs_or_neg_neg_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[1, 0]);
/// assert_eq!(ys, &[1, 2, 5]);
/// ```
///
/// This is mpz_ior from mpz/ior.c where both inputs are negative and the result is written to the
/// shorter input slice.
pub fn limbs_or_neg_neg_in_place_either(xs: &mut [Limb], ys: &mut [Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        return false;
    } else if x_i >= ys_len {
        return true;
    }
    let max_i = max(x_i, y_i);
    let boundary_limb = if x_i == y_i {
        ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1
    } else if x_i > y_i {
        (xs[x_i] - 1) & ys[x_i]
    } else {
        xs[y_i] & (ys[y_i] - 1)
    };
    if xs_len > ys_len {
        if y_i > x_i {
            ys[x_i..y_i].copy_from_slice(&xs[x_i..y_i]);
        }
        ys[max_i] = boundary_limb;
        for (y, x) in ys[max_i + 1..].iter_mut().zip(xs[max_i + 1..].iter()) {
            *y &= x;
        }
        true
    } else {
        if x_i > y_i {
            xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
        }
        xs[max_i] = boundary_limb;
        for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter()) {
            *x &= y;
        }
        false
    }
}

/// Takes the bitwise or of two `Integer`s, taking both by value.
///
/// Time: worst case O(m)
///
/// Additional memory: worst case O(n)
///
/// where m = `self.significant_bits() + other.significant_bits`,
///     n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((Integer::from(-123) | Integer::from(-456)).to_string(), "-67");
/// assert_eq!((-Integer::trillion() | -(Integer::trillion() + 1u32)).to_string(), "-999999995905");
/// ```
impl BitOr<Integer> for Integer {
    type Output = Integer;

    #[inline]
    fn bitor(mut self, other: Integer) -> Integer {
        self |= other;
        self
    }
}

/// Takes the bitwise or of two `Integer`s, taking the left `Integer` by value and the right
/// `Integer` by reference.
///
/// Time: worst case O(m)
///
/// Additional memory: worst case O(n)
///
/// where m = `self.significant_bits() + other.significant_bits`, n = `other.significant_bits`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((Integer::from(-123) | &Integer::from(-456)).to_string(), "-67");
/// assert_eq!((-Integer::trillion() | &-(Integer::trillion() + 1u32)).to_string(),
///     "-999999995905");
/// ```
impl<'a> BitOr<&'a Integer> for Integer {
    type Output = Integer;

    #[inline]
    fn bitor(mut self, other: &'a Integer) -> Integer {
        self |= other;
        self
    }
}

/// Takes the bitwise or of two `Integer`s, taking the left `Integer` by reference and the right
/// `Integer` by value.
///
/// Time: worst case O(m)
///
/// Additional memory: worst case O(n)
///
/// where m = `self.significant_bits() + other.significant_bits`, n = `other.significant_bits`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((&Integer::from(-123) | Integer::from(-456)).to_string(), "-67");
/// assert_eq!((&-Integer::trillion() | -(Integer::trillion() + 1u32)).to_string(),
///     "-999999995905");
/// ```
impl<'a> BitOr<Integer> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn bitor(self, mut other: Integer) -> Integer {
        other |= self;
        other
    }
}

/// Takes the bitwise or of an `Integer` and a `Natural`, taking both by reference.
///
/// Time: worst case O(m)
///
/// Additional memory: worst case O(n)
///
/// where m = `self.significant_bits() + other.significant_bits`,
///     n = `max(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((&Integer::from(-123) | &Integer::from(-456)).to_string(), "-67");
/// assert_eq!((&-Integer::trillion() | &-(Integer::trillion() + 1u32)).to_string(),
///     "-999999995905");
/// ```
impl<'a, 'b> BitOr<&'a Integer> for &'b Integer {
    type Output = Integer;

    fn bitor(self, other: &'a Integer) -> Integer {
        match (self.sign, other.sign) {
            (true, true) => Integer {
                sign: true,
                abs: &self.abs | &other.abs,
            },
            (true, false) => Integer {
                sign: false,
                abs: self.abs.or_pos_neg(&other.abs),
            },
            (false, true) => Integer {
                sign: false,
                abs: other.abs.or_pos_neg(&self.abs),
            },
            (false, false) => Integer {
                sign: false,
                abs: self.abs.or_neg_neg(&other.abs),
            },
        }
    }
}

/// Bitwise-ors an `Integer` with another `Integer` in place, taking the `Integer` on the RHS by
/// value.
///
/// Time: worst case O(m)
///
/// Additional memory: worst case O(n)
///
/// where m = `self.significant_bits() + other.significant_bits`,
///     n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x |= Integer::from(0x0000_000f);
///     x |= Integer::from(0x0000_0f00);
///     x |= Integer::from(0x000f_0000);
///     x |= Integer::from(0x0f00_0000);
///     assert_eq!(x, 0x0f0f_0f0f);
/// }
/// ```
impl BitOrAssign<Integer> for Integer {
    fn bitor_assign(&mut self, other: Integer) {
        match (self.sign, other.sign) {
            (true, true) => self.abs.bitor_assign(other.abs),
            (true, false) => {
                self.sign = false;
                self.abs.or_assign_pos_neg(other.abs)
            }
            (false, true) => self.abs.or_assign_neg_pos(other.abs),
            (false, false) => self.abs.or_assign_neg_neg(other.abs),
        }
    }
}

/// Bitwise-ors an `Integer` with another `Integer` in place, taking the `Integer` on the RHS by
/// reference.
///
/// Time: worst case O(m)
///
/// Additional memory: worst case O(n)
///
/// where m = `self.significant_bits() + other.significant_bits`, n = `other.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x |= &Integer::from(0x0000_000f);
///     x |= &Integer::from(0x0000_0f00);
///     x |= &Integer::from(0x000f_0000);
///     x |= &Integer::from(0x0f00_0000);
///     assert_eq!(x, 0x0f0f_0f0f);
/// }
/// ```
impl<'a> BitOrAssign<&'a Integer> for Integer {
    fn bitor_assign(&mut self, other: &'a Integer) {
        match (self.sign, other.sign) {
            (true, true) => self.abs.bitor_assign(&other.abs),
            (true, false) => {
                self.sign = false;
                self.abs.or_assign_pos_neg_ref(&other.abs)
            }
            (false, true) => self.abs.or_assign_neg_pos_ref(&other.abs),
            (false, false) => self.abs.or_assign_neg_neg_ref(&other.abs),
        }
    }
}

impl Natural {
    fn or_assign_neg_neg_ref(&mut self, other: &Natural) {
        if let Small(y) = *other {
            self.or_assign_neg_limb_neg(y.wrapping_neg());
        } else if let Small(x) = *self {
            *self = other.or_neg_limb_neg(x.wrapping_neg());
        } else if let Large(ref ys) = *other {
            if let Large(ref mut xs) = *self {
                limbs_vec_or_neg_neg_in_place_left(xs, ys);
            }
            self.trim();
        }
    }

    fn or_assign_neg_neg(&mut self, other: Natural) {
        if let Small(y) = other {
            self.or_assign_neg_limb_neg(y.wrapping_neg());
        } else if let Small(x) = *self {
            *self = other;
            self.or_assign_neg_limb_neg(x.wrapping_neg());
        } else if let Large(mut ys) = other {
            if let Large(ref mut xs) = *self {
                if limbs_or_neg_neg_in_place_either(xs, &mut ys) {
                    *xs = ys;
                }
            }
            self.trim();
        }
    }

    fn or_neg_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Small(y)) => self.or_neg_limb_neg(y.wrapping_neg()),
            (&Small(x), _) => other.or_neg_limb_neg(x.wrapping_neg()),
            (&Large(ref xs), &Large(ref ys)) => {
                let mut result = Large(limbs_or_neg_neg(xs, ys));
                result.trim();
                result
            }
        }
    }
}
