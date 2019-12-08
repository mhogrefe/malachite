use std::cmp::max;
use std::ops::{BitAnd, BitAndAssign};

use malachite_base::limbs::{limbs_leading_zero_limbs, limbs_set_zero};
use malachite_base::num::arithmetic::traits::WrappingNegAssign;
use malachite_base::num::logic::traits::NotAssign;

use integer::Integer;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
/// negative of another, returns the limbs of the bitwise and of the `Integer`s. `xs` and `ys` may
/// not be empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.len() + ys.len()`, m = `xs.len()`
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::and_natural::limbs_and_pos_neg;
///
/// assert_eq!(limbs_and_pos_neg(&[1, 2], &[100, 200]), &[0, 2]);
/// assert_eq!(limbs_and_pos_neg(&[1, 2, 5], &[100, 200]), &[0, 2, 5]);
/// ```
///
/// This is mpz_and from mpz/and.c where res is returned, the first input is positive, and the
/// second is negative.
pub fn limbs_and_pos_neg(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        return Vec::new();
    } else if x_i >= ys_len {
        return xs.to_vec();
    }
    let max_i = max(x_i, y_i);
    let mut result_limbs = vec![0; max_i];
    result_limbs.push(
        xs[max_i]
            & if x_i <= y_i {
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

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
/// negative of another, writes the limbs of the bitwise and of the `Integer`s to an output slice.
/// `xs` and `ys` may not be empty or only contain zeros. The output slice must be at least as long
/// as the first input slice. `xs.len()` limbs will be written; if the number of significant limbs
/// of the result is lower, some of the written limbs will be zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len() + ys.len()`
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros, or if `out` is shorter than `xs`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::and_natural::limbs_and_pos_neg_to_out;
///
/// let mut result = vec![0, 0];
/// limbs_and_pos_neg_to_out(&mut result, &[1, 2], &[100, 200]);
/// assert_eq!(result, &[0, 2]);
///
/// let mut result = vec![10, 10, 10, 10];
/// limbs_and_pos_neg_to_out(&mut result, &[1, 2, 5], &[100, 200]);
/// assert_eq!(result, &[0, 2, 5, 10]);
/// ```
///
/// This is mpz_and from mpz/and.c where the first input is positive and the second is negative.
pub fn limbs_and_pos_neg_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= xs_len);
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        limbs_set_zero(&mut out[..xs_len]);
        return;
    } else if x_i >= ys_len {
        out[..xs_len].copy_from_slice(xs);
        return;
    }
    let max_i = max(x_i, y_i);
    limbs_set_zero(&mut out[..max_i]);
    out[max_i] = xs[max_i]
        & if x_i <= y_i {
            ys[max_i].wrapping_neg()
        } else {
            !ys[max_i]
        };
    for (z, (x, y)) in out[max_i + 1..]
        .iter_mut()
        .zip(xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter()))
    {
        *z = x & !y;
    }
    if xs_len > ys_len {
        out[ys_len..xs_len].copy_from_slice(&xs[ys_len..]);
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
/// negative of another, writes the limbs of the bitwise and of the `Integer`s to the first (left)
/// slice. `xs` and `ys` may not be empty or only contain zeros.
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
/// use malachite_nz::integer::logic::and_natural::limbs_and_pos_neg_in_place_left;
///
/// let mut xs = vec![1, 2];
/// limbs_and_pos_neg_in_place_left(&mut xs, &[100, 200]);
/// assert_eq!(xs, &[0, 2]);
///
/// let mut xs = vec![1, 2, 5];
/// limbs_and_pos_neg_in_place_left(&mut xs, &[100, 200]);
/// assert_eq!(xs, &[0, 2, 5]);
/// ```
///
/// This is mpz_and from mpz/and.c where res == op1, the first input is positive, and the second is
/// negative.
pub fn limbs_and_pos_neg_in_place_left(xs: &mut [Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        limbs_set_zero(xs);
        return;
    } else if x_i >= ys_len {
        return;
    }
    let max_i = max(x_i, y_i);
    limbs_set_zero(&mut xs[..max_i]);
    xs[max_i] &= if x_i <= y_i {
        ys[max_i].wrapping_neg()
    } else {
        !ys[max_i]
    };
    for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter()) {
        *x &= !y;
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
/// negative of another, writes the lowest min(`xs.len()`, `ys.len()`) limbs of the bitwise and of
/// the `Integer`s to the second (right) slice. `xs` and `ys` may not be empty or only contain
/// zeros. If `ys` is shorter than `xs`, the result may be too long to fit in `ys`. The extra limbs
/// in this case are just `xs[ys.len()..]`.
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
/// use malachite_nz::integer::logic::and_natural::limbs_slice_and_pos_neg_in_place_right;
///
/// let mut ys = vec![100, 200];
/// limbs_slice_and_pos_neg_in_place_right(&[1, 2], &mut ys);
/// assert_eq!(ys, &[0, 2]);
///
/// let mut ys = vec![100, 200];
/// limbs_slice_and_pos_neg_in_place_right(&[1, 2, 5], &mut ys);
/// // The result is missing the most-significant limb, which is 5
/// assert_eq!(ys, &[0, 2]);
/// ```
///
/// This is mpz_and from mpz/and.c where res == op2, the first input is positive, the second is
/// negative, and the length of op2 is not changed; instead, a carry is returned.
pub fn limbs_slice_and_pos_neg_in_place_right(xs: &[Limb], ys: &mut [Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len || x_i >= ys_len {
        limbs_set_zero(ys);
        return;
    }
    let max_i = max(x_i, y_i);
    limbs_set_zero(&mut ys[..max_i]);
    {
        let ys_max_i = &mut ys[max_i];
        if x_i <= y_i {
            ys_max_i.wrapping_neg_assign();
        } else {
            ys_max_i.not_assign();
        }
        *ys_max_i &= xs[max_i];
    }
    for (x, y) in xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter_mut()) {
        *y = !*y & x;
    }
}

/// Interpreting a slice of `Limb`s and a `Vec` of `Limb`s as the limbs (in ascending order) of one
/// `Integer` and the negative of another, writes the limbs of the bitwise and of the `Integer`s to
/// the `Vec`. `xs` and `ys` may not be empty or only contain zeros.
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
/// use malachite_nz::integer::logic::and_natural::limbs_vec_and_pos_neg_in_place_right;
///
/// let mut ys = vec![100, 200];
/// limbs_vec_and_pos_neg_in_place_right(&[1, 2], &mut ys);
/// assert_eq!(ys, &[0, 2]);
///
/// let mut ys = vec![100, 200];
/// limbs_vec_and_pos_neg_in_place_right(&[1, 2, 5], &mut ys);
/// assert_eq!(ys, &[0, 2, 5]);
///
/// let mut ys = vec![1, 2, 5];
/// limbs_vec_and_pos_neg_in_place_right(&[100, 200], &mut ys);
/// assert_eq!(ys, &[100, 200]);
/// ```
///
/// This is mpz_and from mpz/and.c where res == op2, the first input is positive and the second is
/// negative.
pub fn limbs_vec_and_pos_neg_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>) {
    limbs_slice_and_pos_neg_in_place_right(xs, ys);
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len > ys_len {
        let ys_len = ys.len();
        ys.extend(xs[ys_len..].iter());
    } else if xs_len < ys_len {
        ys.truncate(xs_len);
    }
}

/// Takes the bitwise and of an `Integer` and a `Natural`, taking both by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits() + other.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((Integer::from(-123) & Natural::from(456u32)).to_string(), "384");
/// assert_eq!((-Integer::trillion() & (Natural::trillion() + 1u32)).to_string(), "4096");
/// ```
impl BitAnd<Natural> for Integer {
    type Output = Natural;

    fn bitand(mut self, other: Natural) -> Natural {
        self &= other;
        self.abs
    }
}

/// Takes the bitwise and of an `Integer` and a `Natural`, taking the `Integer` by value and the
/// `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `self.significant_bits() + other.significant_bits()`, m = `other.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((Integer::from(-123) & &Natural::from(456u32)).to_string(), "384");
/// assert_eq!((-Integer::trillion() & &(Natural::trillion() + 1u32)).to_string(), "4096");
/// ```
impl<'a> BitAnd<&'a Natural> for Integer {
    type Output = Natural;

    fn bitand(mut self, other: &'a Natural) -> Natural {
        self &= other;
        self.abs.clone()
    }
}

/// Takes the bitwise and of an `Integer` and a `Natural`, taking the `Integer` by reference and the
/// `Natural` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.significant_bits() + ys.significant_bits()`, m = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((&Integer::from(-123) & Natural::from(456u32)).to_string(), "384");
/// assert_eq!((&-Integer::trillion() & (Natural::trillion() + 1u32)).to_string(), "4096");
/// ```
impl<'a> BitAnd<Natural> for &'a Integer {
    type Output = Natural;

    fn bitand(self, mut other: Natural) -> Natural {
        if self.sign {
            &self.abs & other
        } else {
            other.and_assign_pos_neg(&self.abs);
            other
        }
    }
}

/// Takes the bitwise and of an `Integer` and a `Natural`, taking both by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `self.significant_bits() + other.significant_bits()`,
///     m = `max(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((&Integer::from(-123) & &Natural::from(456u32)).to_string(), "384");
/// assert_eq!((&-Integer::trillion() & &(Natural::trillion() + 1u32)).to_string(), "4096");
/// ```
impl<'a, 'b> BitAnd<&'a Natural> for &'b Integer {
    type Output = Natural;

    fn bitand(self, other: &'a Natural) -> Natural {
        if self.sign {
            &self.abs & other
        } else {
            other.and_pos_neg(&self.abs)
        }
    }
}

/// Bitwise-ands an `Integer` with a `Natural` in place, taking the `Natural` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits() + other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::NegativeOne;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Integer::NEGATIVE_ONE;
///     x &= Natural::from(0x70ff_ffffu32);
///     x &= Natural::from(0x7ff0_ffffu32);
///     x &= Natural::from(0x7fff_f0ffu32);
///     x &= Natural::from(0x7fff_fff0u32);
///     assert_eq!(x, 0x70f0f0f0);
/// }
/// ```
impl BitAndAssign<Natural> for Integer {
    fn bitand_assign(&mut self, other: Natural) {
        if self.sign {
            self.abs.bitand_assign(other);
        } else {
            self.sign = true;
            self.abs.and_assign_neg_pos(other);
        }
    }
}

/// Bitwise-ands an `Integer` with a `Natural` in place, taking the `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.significant_bits() + ys.significant_bits()`, m = `other.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::NegativeOne;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Integer::NEGATIVE_ONE;
///     x &= &Natural::from(0x70ff_ffffu32);
///     x &= &Natural::from(0x7ff0_ffffu32);
///     x &= &Natural::from(0x7fff_f0ffu32);
///     x &= &Natural::from(0x7fff_fff0u32);
///     assert_eq!(x, 0x70f0f0f0);
/// }
/// ```
impl<'a> BitAndAssign<&'a Natural> for Integer {
    fn bitand_assign(&mut self, other: &'a Natural) {
        if self.sign {
            self.abs.bitand_assign(other);
        } else {
            self.sign = true;
            self.abs.and_assign_neg_pos_ref(other);
        }
    }
}

/// Takes the bitwise and of a `Natural` and an `Integer`, taking both by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits() + other.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((Natural::from(456u32) & Integer::from(-123)).to_string(), "384");
/// assert_eq!(((Natural::trillion() + 1u32) & -Integer::trillion()).to_string(), "4096");
/// ```
impl BitAnd<Integer> for Natural {
    type Output = Natural;

    #[inline]
    fn bitand(self, other: Integer) -> Natural {
        other & self
    }
}

/// Takes the bitwise and of a `Natural` and an `Integer`, taking the `Natural` by value and the
/// `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `self.significant_bits() + other.significant_bits()`, m = `other.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((Natural::from(456u32) & &Integer::from(-123)).to_string(), "384");
/// assert_eq!(((Natural::trillion() + 1u32) & &-Integer::trillion()).to_string(), "4096");
/// ```
impl<'a> BitAnd<&'a Integer> for Natural {
    type Output = Natural;

    #[inline]
    fn bitand(self, other: &'a Integer) -> Natural {
        other & self
    }
}

/// Takes the bitwise and of a `Natural` and an `Integer`, taking the `Natural` by reference and the
/// `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.significant_bits() + ys.significant_bits()`, m = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((&Natural::from(456u32) & Integer::from(-123)).to_string(), "384");
/// assert_eq!((&(Natural::trillion() + 1u32) & -Integer::trillion()).to_string(), "4096");
/// ```
impl<'a> BitAnd<Integer> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn bitand(self, other: Integer) -> Natural {
        other & self
    }
}

/// Takes the bitwise and of a `Natural` and an `Integer`, taking both by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `self.significant_bits() + other.significant_bits()`,
///     m = `max(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((&Natural::from(456u32) & &Integer::from(-123)).to_string(), "384");
/// assert_eq!((&(Natural::trillion() + 1u32) & &-Integer::trillion()).to_string(), "4096");
/// ```
impl<'a, 'b> BitAnd<&'a Integer> for &'b Natural {
    type Output = Natural;

    #[inline]
    fn bitand(self, other: &'a Integer) -> Natural {
        other & self
    }
}

/// Bitwise-ands an `Integer` with a `Natural` in place, taking the `Natural` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits() + other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(0xffff_ffffu32);
///     x &= Integer::from(0x70ff_ffff);
///     x &= Integer::from(0x7ff0_ffff);
///     x &= Integer::from(0x7fff_f0ff);
///     x &= Integer::from(0x7fff_fff0);
///     assert_eq!(x, 0x70f0f0f0);
/// }
/// ```
impl BitAndAssign<Integer> for Natural {
    fn bitand_assign(&mut self, other: Integer) {
        if other.sign {
            self.bitand_assign(other.abs);
        } else {
            self.and_assign_pos_neg(&other.abs);
        }
    }
}

/// Bitwise-ands an `Integer` with a `Natural` in place, taking the `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.significant_bits() + ys.significant_bits()`, m = `other.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(0xffff_ffffu32);
///     x &= &Integer::from(0x70ff_ffff);
///     x &= &Integer::from(0x7ff0_ffff);
///     x &= &Integer::from(0x7fff_f0ff);
///     x &= &Integer::from(0x7fff_fff0);
///     assert_eq!(x, 0x70f0f0f0);
/// }
/// ```
impl<'a> BitAndAssign<&'a Integer> for Natural {
    fn bitand_assign(&mut self, other: &'a Integer) {
        if other.sign {
            self.bitand_assign(&other.abs);
        } else {
            self.and_assign_pos_neg(&other.abs);
        }
    }
}

impl Natural {
    pub(crate) fn and_assign_pos_neg(&mut self, other: &Natural) {
        if let Natural(Small(y)) = *other {
            self.and_assign_pos_limb_neg(y.wrapping_neg());
        } else if let Natural(Small(ref mut x)) = *self {
            if let Natural(Large(ref ys)) = *other {
                *x &= ys[0].wrapping_neg();
            }
        } else if let Natural(Large(ref ys)) = *other {
            if let Natural(Large(ref mut xs)) = *self {
                limbs_and_pos_neg_in_place_left(xs, ys);
            }
            self.trim();
        }
    }

    pub(crate) fn and_assign_neg_pos(&mut self, mut other: Natural) {
        other.and_assign_pos_neg(self);
        *self = other;
    }

    pub(crate) fn and_assign_neg_pos_ref(&mut self, other: &Natural) {
        let new_self_value = if let Natural(Small(x)) = *self {
            let mut new_self_value = other.clone();
            new_self_value.and_assign_pos_limb_neg(x.wrapping_neg());
            Some(new_self_value)
        } else if let Natural(Small(ref y)) = *other {
            let x = if let Natural(Large(ref xs)) = *self {
                xs[0].wrapping_neg() & *y
            } else {
                unreachable!()
            };
            *self = Natural(Small(x));
            None
        } else if let Natural(Large(ref ys)) = *other {
            if let Natural(Large(ref mut xs)) = *self {
                limbs_vec_and_pos_neg_in_place_right(ys, xs);
            }
            self.trim();
            None
        } else {
            None
        };
        if let Some(new_self_value) = new_self_value {
            *self = new_self_value;
        }
    }

    pub(crate) fn and_pos_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Natural(Small(y))) => self.and_pos_limb_neg(y.wrapping_neg()),
            (&Natural(Small(x)), &Natural(Large(ref ys))) => {
                Natural(Small(x & ys[0].wrapping_neg()))
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                let mut result = Natural(Large(limbs_and_pos_neg(xs, ys)));
                result.trim();
                result
            }
        }
    }
}
