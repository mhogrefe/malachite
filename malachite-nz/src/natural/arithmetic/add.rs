use std::ops::{Add, AddAssign};

use malachite_base::num::traits::OverflowingAddAssign;

use natural::arithmetic::add_limb::{limbs_add_limb_to_out, limbs_slice_add_limb_in_place};
use natural::Natural::{self, Large, Small};
use platform::Limb;

fn add_and_carry(x: Limb, y: Limb, carry: &mut bool) -> Limb {
    let (mut sum, overflow) = x.overflowing_add(y);
    if *carry {
        *carry = overflow;
        *carry |= sum.overflowing_add_assign(1);
    } else {
        *carry = overflow;
    }
    sum
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, where
/// the first slice is at least as long as the second, returns a `Vec` of the limbs of the sum of
/// the `Natural`s.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is shorter than `ys`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add::limbs_add_greater;
///
/// assert_eq!(limbs_add_greater(&[1, 2, 3], &[6, 7]), &[7, 9, 3]);
/// assert_eq!(limbs_add_greater(&[100, 101, 0xffff_ffff], &[102, 101, 2]), &[202, 202, 1, 1]);
/// ```
///
/// This is mpn_add from gmp.h, where the first input is at least as long as the second, and the
/// output is returned.
pub fn limbs_add_greater(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let mut result_limbs = Vec::with_capacity(xs_len);
    let mut carry = false;
    for (&x, &y) in xs.iter().zip(ys.iter()) {
        result_limbs.push(add_and_carry(x, y, &mut carry));
    }
    if xs_len == ys_len {
        if carry {
            result_limbs.push(1);
        }
    } else {
        result_limbs.extend_from_slice(&xs[ys_len..]);
        if carry && limbs_slice_add_limb_in_place(&mut result_limbs[ys_len..], 1) {
            result_limbs.push(1);
        }
    }
    result_limbs
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
/// a `Vec` of the limbs of the sum of the `Natural`s.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add::limbs_add;
///
/// assert_eq!(limbs_add(&[6, 7], &[1, 2, 3]), &[7, 9, 3]);
/// assert_eq!(limbs_add(&[100, 101, 0xffff_ffff], &[102, 101, 2]), &[202, 202, 1, 1]);
/// ```
///
/// This is mpn_add from gmp.h, where the output is returned.
pub fn limbs_add(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    if xs.len() >= ys.len() {
        limbs_add_greater(xs, ys)
    } else {
        limbs_add_greater(ys, xs)
    }
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the `xs.len()` least-significant limbs of the sum of the `Natural`s to an
/// output slice. The output must be at least as long as one of the input slices. Returns whether
/// there is a carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths or if `out` is too short.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add::limbs_add_same_length_to_out;
///
/// let limbs = &mut [10, 10, 10, 10];
/// assert_eq!(limbs_add_same_length_to_out(limbs, &[6, 7], &[1, 2]), false);
/// assert_eq!(limbs, &[7, 9, 10, 10]);
///
/// let limbs = &mut [10, 10, 10, 10];
/// assert_eq!(limbs_add_same_length_to_out(limbs, &[100, 101, 0xffff_ffff], &[102, 101, 2]), true);
/// assert_eq!(limbs, &[202, 202, 1, 10]);
/// ```
///
/// This is mpn_add_n from gmp.h.
pub fn limbs_add_same_length_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(out.len() >= len);
    let mut carry = false;
    for i in 0..len {
        out[i] = add_and_carry(xs[i], ys[i], &mut carry);
    }
    carry
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, where
/// the first slice is at least as long as the second, writes the `xs.len()` least-significant limbs
/// of the sum of the `Natural`s to an output slice. The output must be at least as long as `xs`.
/// Returns whether there is a carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is shorter than `ys` or if `out` is too short.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add::limbs_add_greater_to_out;
///
/// let limbs = &mut [10, 10, 10, 10];
/// assert_eq!(limbs_add_greater_to_out(limbs, &[1, 2, 3], &[6, 7]), false);
/// assert_eq!(limbs, &[7, 9, 3, 10]);
///
/// let limbs = &mut [10, 10, 10, 10];
/// assert_eq!(limbs_add_greater_to_out(limbs, &[100, 101, 0xffff_ffff], &[102, 101, 2]), true);
/// assert_eq!(limbs, &[202, 202, 1, 10]);
/// ```
///
/// This is mpn_add from gmp.h, where the first input is at least as long as the second.
pub fn limbs_add_greater_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    assert!(out.len() >= xs_len);
    let carry = limbs_add_same_length_to_out(out, &xs[..ys_len], ys);
    if xs_len == ys_len {
        carry
    } else if carry {
        limbs_add_limb_to_out(&mut out[ys_len..], &xs[ys_len..], 1)
    } else {
        out[ys_len..xs_len].copy_from_slice(&xs[ys_len..]);
        false
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `max(xs.len(), ys.len())` least-significant limbs of the sum of the `Natural`s to an output
/// slice. The output must be at least as long as the longer input slice. Returns whether there is a
/// carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Panics
/// Panics if `out` is too short.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add::limbs_add_to_out;
///
/// let limbs = &mut [10, 10, 10, 10];
/// assert_eq!(limbs_add_to_out(limbs, &[6, 7], &[1, 2, 3]), false);
/// assert_eq!(limbs, &[7, 9, 3, 10]);
///
/// let limbs = &mut [10, 10, 10, 10];
/// assert_eq!(limbs_add_to_out(limbs, &[100, 101, 0xffff_ffff], &[102, 101, 2]), true);
/// assert_eq!(limbs, &[202, 202, 1, 10]);
/// ```
///
/// This is mpn_add from gmp.h.
pub fn limbs_add_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_add_greater_to_out(out, xs, ys)
    } else {
        limbs_add_greater_to_out(out, ys, xs)
    }
}

/// Given two slices of `Limb`s as the limbs `xs` and `ys`, where `xs` is at least as long as `ys`
/// and `in_size` is no greater than `ys.len()`, writes the `ys.len()` lowest limbs of the sum of
/// `xs[..in_size]` and `ys` to `xs`. Returns whether there is a carry.
///
/// For example,
/// `_limbs_add_to_out_aliased(&mut xs[..12], 7, &ys[0..10])`
/// would be equivalent to
/// `limbs_add_to_out(&mut xs[..12], &xs[..7], &ys[0..10])`
/// although the latter expression is not allowed because `xs` cannot be borrowed in that way.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Panics
/// Panics if `xs` is shorter than `ys` or `in_size` is greater than `ys.len()`.
///
/// This is mpn_add from gmp.h, where the second argument is at least as long as the first and the
/// output pointer is the same as the first input pointer.
pub fn _limbs_add_to_out_aliased(xs: &mut [Limb], in_size: usize, ys: &[Limb]) -> bool {
    let ys_len = ys.len();
    assert!(xs.len() >= ys_len);
    assert!(in_size <= ys_len);
    xs[in_size..ys_len].copy_from_slice(&ys[in_size..]);
    limbs_slice_add_same_length_in_place_left(&mut xs[..in_size], &ys[..in_size])
        && limbs_slice_add_limb_in_place(&mut xs[in_size..ys_len], 1)
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the `xs.len()` least-significant limbs of the sum of the `Natural`s to the
/// first (left) slice. Returns whether there is a carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
///
/// let xs = &mut [6, 7];
/// assert_eq!(limbs_slice_add_same_length_in_place_left(xs, &[1, 2]), false);
/// assert_eq!(xs, &[7, 9]);
///
/// let xs = &mut [100, 101, 0xffff_ffff];
/// assert_eq!(limbs_slice_add_same_length_in_place_left(xs, &[102, 101, 2]), true);
/// assert_eq!(xs, &[202, 202, 1]);
/// ```
///
/// This is mpn_add_n from gmp.h, where the output is written to the first input.
pub fn limbs_slice_add_same_length_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    assert_eq!(xs_len, ys.len());
    let mut carry = false;
    for i in 0..xs_len {
        xs[i] = add_and_carry(xs[i], ys[i], &mut carry);
    }
    carry
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, where
/// the length of the first slice is greater than or equal to the length of the second, writes the
/// `xs.len()` least-significant limbs of the sum of the `Natural`s to the first (left) slice.
/// Returns whether there is a carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is shorter than `ys`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add::limbs_slice_add_greater_in_place_left;
///
/// let xs = &mut [6, 7, 8];
/// assert_eq!(limbs_slice_add_greater_in_place_left(xs, &[1, 2]), false);
/// assert_eq!(xs, &[7, 9, 8]);
///
/// let xs = &mut [100, 101, 0xffff_ffff];
/// assert_eq!(limbs_slice_add_greater_in_place_left(xs, &[102, 101, 2]), true);
/// assert_eq!(xs, &[202, 202, 1]);
/// ```
///
/// This is mpn_add from gmp.h, where the first input is at least as long as the second, and the
/// output is written to the first input.
pub fn limbs_slice_add_greater_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let (xs_lo, xs_hi) = xs.split_at_mut(ys_len);
    let carry = limbs_slice_add_same_length_in_place_left(xs_lo, ys);
    if xs_len == ys_len {
        carry
    } else if carry {
        limbs_slice_add_limb_in_place(xs_hi, 1)
    } else {
        false
    }
}

/// Interpreting a `Vec` of `Limb`s and a slice of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the limbs of the sum of the `Natural`s to the first (left) slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`xs.len()`, `ys.len()`), m = max(1, ys.len() - xs.len())
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add::limbs_vec_add_in_place_left;
///
/// let mut xs = vec![6, 7];
/// limbs_vec_add_in_place_left(&mut xs, &[1, 2]);
/// assert_eq!(xs, &[7, 9]);
///
/// let mut xs = vec![100, 101, 0xffff_ffff];
/// limbs_vec_add_in_place_left(&mut xs, &[102, 101, 2]);
/// assert_eq!(xs, &[202, 202, 1, 1]);
/// ```
///
/// This is mpz_add from mpz/aors.h, where both inputs are non-negative and the output is written to
/// the first input.
pub fn limbs_vec_add_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let carry = if xs_len >= ys_len {
        limbs_slice_add_greater_in_place_left(xs, ys)
    } else {
        let (ys_lo, ys_hi) = ys.split_at(xs_len);
        let mut carry = limbs_slice_add_same_length_in_place_left(xs, ys_lo);
        xs.extend_from_slice(ys_hi);
        if carry {
            carry = limbs_slice_add_limb_in_place(&mut xs[xs_len..], 1);
        }
        carry
    };
    if carry {
        xs.push(1);
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `max(xs.len(), ys.len())` least-significant limbs of the sum of the `Natural`s to the longer
/// slice (or the first one, if they are equally long). Returns a pair of `bool`s. The first is
/// `false` when the output is to the first slice and `true` when it's to the second slice, and the
/// second is whether there is a carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add::limbs_slice_add_in_place_either;
///
/// let mut xs = vec![6, 7];
/// let mut ys = vec![1, 2, 3];
/// assert_eq!(limbs_slice_add_in_place_either(&mut xs, &mut ys), (true, false));
/// assert_eq!(xs, &[6, 7]);
/// assert_eq!(ys, &[7, 9, 3]);
///
/// let mut xs = vec![1, 2, 3];
/// let mut ys = vec![6, 7];
/// assert_eq!(limbs_slice_add_in_place_either(&mut xs, &mut ys), (false, false));
/// assert_eq!(xs, &[7, 9, 3]);
/// assert_eq!(ys, &[6, 7]);
///
/// let mut xs = vec![100, 101, 0xffff_ffff];
/// let mut ys = vec![102, 101, 2];
/// assert_eq!(limbs_slice_add_in_place_either(&mut xs, &mut ys), (false, true));
/// assert_eq!(xs, &[202, 202, 1]);
/// assert_eq!(ys, &[102, 101, 2]);
/// ```
///
/// This is mpn_add from gmp.h, where the output is written to the longer input.
pub fn limbs_slice_add_in_place_either(xs: &mut Vec<Limb>, ys: &mut Vec<Limb>) -> (bool, bool) {
    if xs.len() >= ys.len() {
        (false, limbs_slice_add_greater_in_place_left(xs, ys))
    } else {
        (true, limbs_slice_add_greater_in_place_left(ys, xs))
    }
}

/// Interpreting two `Vec`s of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the limbs of the sum of the `Natural`s to the longer slice (or the first one, if they are
/// equally long). Returns a `bool` which is `false` when the output is to the first `Vec` and
/// `true` when it's to the second `Vec`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add::limbs_vec_add_in_place_either;
///
/// let mut xs = vec![6, 7];
/// let mut ys = vec![1, 2, 3];
/// assert_eq!(limbs_vec_add_in_place_either(&mut xs, &mut ys), true);
/// assert_eq!(xs, &[6, 7]);
/// assert_eq!(ys, &[7, 9, 3]);
///
/// let mut xs = vec![1, 2, 3];
/// let mut ys = vec![6, 7];
/// assert_eq!(limbs_vec_add_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[7, 9, 3]);
/// assert_eq!(ys, &[6, 7]);
///
/// let mut xs = vec![100, 101, 0xffff_ffff];
/// let mut ys = vec![102, 101, 2];
/// assert_eq!(limbs_vec_add_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[202, 202, 1, 1]);
/// assert_eq!(ys, &[102, 101, 2]);
/// ```
///
/// This is mpz_add from mpz/aors.h, where both inputs are non-negative and the output is written to
/// the longer input.
pub fn limbs_vec_add_in_place_either(xs: &mut Vec<Limb>, ys: &mut Vec<Limb>) -> bool {
    if xs.len() >= ys.len() {
        if limbs_slice_add_greater_in_place_left(xs, ys) {
            xs.push(1);
        }
        false
    } else {
        if limbs_slice_add_greater_in_place_left(ys, xs) {
            ys.push(1);
        }
        true
    }
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the `xs.len()` least-significant limbs of the sum of the `Natural`s and a
/// carry (`false` is 0, `true` is 1) to an output slice. The output must be at least as long as one
/// of the input slices. Returns whether there is a carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths or if `out` is too short.
///
/// This is mpn_add_nc from gmp-impl.h, where rp and up are disjoint.
pub fn _limbs_add_same_length_with_carry_in_to_out(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    carry_in: bool,
) -> bool {
    let mut carry = limbs_add_same_length_to_out(out, xs, ys);
    if carry_in {
        carry |= limbs_slice_add_limb_in_place(&mut out[..xs.len()], 1);
    }
    carry
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the `xs.len()` least-significant limbs of the sum of the `Natural`s and a
/// carry (`false` is 0, `true` is 1) to the first (left) slice. Returns whether there is a carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// This is mpn_add_nc from gmp-impl.h, where rp is the same as up.
pub fn _limbs_add_same_length_with_carry_in_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    carry_in: bool,
) -> bool {
    let mut carry = limbs_slice_add_same_length_in_place_left(xs, ys);
    if carry_in {
        carry |= limbs_slice_add_limb_in_place(xs, 1);
    }
    carry
}

/// Adds a `Natural` to a `Natural`, taking both `Natural`s by value.
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
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::ZERO + Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Natural::from(123u32) + Natural::ZERO).to_string(), "123");
///     assert_eq!((Natural::from(123u32) + Natural::from(456u32)).to_string(), "579");
///     assert_eq!((Natural::trillion() + Natural::trillion() * 2).to_string(), "3000000000000");
/// }
/// ```
impl Add<Natural> for Natural {
    type Output = Natural;

    fn add(mut self, other: Natural) -> Natural {
        self += other;
        self
    }
}

/// Adds a `Natural` to a `Natural`, taking the left `Natural` by value and the right `Natural` by
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
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::ZERO + &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Natural::from(123u32) +&Natural::ZERO).to_string(), "123");
///     assert_eq!((Natural::from(123u32) +&Natural::from(456u32)).to_string(), "579");
///     assert_eq!((Natural::trillion() + &(Natural::trillion() * 2)).to_string(), "3000000000000");
/// }
/// ```
impl<'a> Add<&'a Natural> for Natural {
    type Output = Natural;

    #[inline]
    fn add(mut self, other: &'a Natural) -> Natural {
        self += other;
        self
    }
}

/// Adds a `Natural` to a `Natural`, taking the left `Natural` by reference and the right `Natural`
/// by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO + Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) + Natural::ZERO).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) + Natural::from(456u32)).to_string(), "579");
///     assert_eq!((&Natural::trillion() + Natural::trillion() * 2).to_string(), "3000000000000");
/// }
/// ```
impl<'a> Add<Natural> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn add(self, mut other: Natural) -> Natural {
        other += self;
        other
    }
}

/// Adds a `Natural` to a `Natural`, taking both `Natural`s by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `max(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO + &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) + &Natural::ZERO).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) + &Natural::from(456u32)).to_string(), "579");
///     assert_eq!((&Natural::trillion() + &(Natural::trillion() * 2)).to_string(),
///         "3000000000000");
/// }
/// ```
impl<'a, 'b> Add<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn add(self, other: &'a Natural) -> Natural {
        if self as *const Natural == other as *const Natural {
            self << 1
        } else {
            match (self, other) {
                (x, &Small(y)) => x + y,
                (&Small(x), y) => x + y,
                (&Large(ref xs), &Large(ref ys)) => Large(limbs_add(xs, ys)),
            }
        }
    }
}

/// Adds a `Natural` to a `Natural` in place, taking the `Natural` on the RHS by value.
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
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::ZERO;
///     x += Natural::trillion();
///     x += Natural::trillion() * 2;
///     x += Natural::trillion() * 3;
///     x += Natural::trillion() * 4;
///     assert_eq!(x.to_string(), "10000000000000");
/// }
/// ```
impl AddAssign<Natural> for Natural {
    fn add_assign(&mut self, other: Natural) {
        if let Small(y) = other {
            *self += y;
        } else if let Small(x) = *self {
            *self = other + x;
        } else if let Large(mut ys) = other {
            if let Large(ref mut xs) = *self {
                if limbs_vec_add_in_place_either(xs, &mut ys) {
                    *xs = ys;
                }
            }
        }
    }
}

/// Adds a `Natural` to a `Natural` in place, taking the `Natural` on the RHS by reference.
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
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::ZERO;
///     x += &Natural::trillion();
///     x += &(Natural::trillion() * 2);
///     x += &(Natural::trillion() * 3);
///     x += &(Natural::trillion() * 4);
///     assert_eq!(x.to_string(), "10000000000000");
/// }
/// ```
impl<'a> AddAssign<&'a Natural> for Natural {
    fn add_assign(&mut self, other: &'a Natural) {
        if self as *const Natural == other as *const Natural {
            *self <<= 1;
        } else if let Small(y) = *other {
            *self += y;
        } else if let Small(x) = *self {
            *self = other.clone() + x;
        } else if let Large(ref ys) = *other {
            if let Large(ref mut xs) = *self {
                limbs_vec_add_in_place_left(xs, ys);
            }
        }
    }
}
