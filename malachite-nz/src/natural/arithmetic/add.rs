use natural::arithmetic::add_u32::{limbs_add_limb_to_out, limbs_slice_add_limb_in_place};
use natural::Natural::{self, Large, Small};
use std::cmp::max;
use std::ops::{Add, AddAssign};

// docs preserved
//TODO test
// mpn_add_nc from gmp-impl.h
pub fn mpn_add_nc(rp: &mut [u32], up: &[u32], vp: &[u32], ci: u32) -> u32 {
    let n = up.len();
    assert_eq!(vp.len(), n);
    let mut co = if limbs_add_same_length_to_out(rp, &up[..n], &vp[..n]) {
        1
    } else {
        0
    };
    co += if limbs_slice_add_limb_in_place(&mut rp[..n], ci) {
        1
    } else {
        0
    };
    co
}

// docs preserved
//TODO test
// mpn_add_nc from gmp-impl.h, rp == up
pub fn mpn_add_nc_in_place(rp: &mut [u32], vp: &[u32], ci: u32) -> u32 {
    let n = rp.len();
    assert_eq!(vp.len(), n);
    let mut co = if limbs_slice_add_same_length_in_place_left(&mut rp[..n], &vp[..n]) {
        1
    } else {
        0
    };
    co += if limbs_slice_add_limb_in_place(&mut rp[..n], ci) {
        1
    } else {
        0
    };
    co
}

fn add_and_carry(x: u32, y: u32, carry: &mut bool) -> u32 {
    let (sum, overflow) = x.overflowing_add(y);
    if *carry {
        *carry = overflow;
        let (sum, overflow) = sum.overflowing_add(1);
        *carry |= overflow;
        sum
    } else {
        *carry = overflow;
        sum
    }
}

// xs.len() >= ys_len
fn limbs_add_helper(
    xs: &[u32],
    ys_len: usize,
    mut result_limbs: Vec<u32>,
    carry: bool,
) -> Vec<u32> {
    result_limbs.extend_from_slice(&xs[ys_len..]);
    if carry && limbs_slice_add_limb_in_place(&mut result_limbs[ys_len..], 1) {
        result_limbs.push(1);
    }
    result_limbs
}

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of two `Natural`s, returns
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
pub fn limbs_add(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let mut result_limbs = Vec::with_capacity(max(xs_len, ys_len));
    let mut carry = false;
    for (&x, &y) in xs.iter().zip(ys.iter()) {
        result_limbs.push(add_and_carry(x, y, &mut carry));
    }
    if xs_len == ys_len {
        if carry {
            result_limbs.push(1);
        }
        result_limbs
    } else if xs_len > ys_len {
        limbs_add_helper(xs, ys_len, result_limbs, carry)
    } else {
        limbs_add_helper(ys, xs_len, result_limbs, carry)
    }
}

/// Interpreting two equal-length slices of `u32`s as the limbs (in ascending order) of two
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
/// Panics if `xs` and `ys` have different lengths or if `out_limbs` is too short.
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
pub fn limbs_add_same_length_to_out(out_limbs: &mut [u32], xs: &[u32], ys: &[u32]) -> bool {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(out_limbs.len() >= len);
    let mut carry = false;
    for i in 0..len {
        out_limbs[i] = add_and_carry(xs[i], ys[i], &mut carry);
    }
    carry
}

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of two `Natural`s, writes
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
/// Panics if `out_limbs` is too short.
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
pub fn limbs_add_to_out(out_limbs: &mut [u32], xs: &[u32], ys: &[u32]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let (min_len, max_len) = if xs_len <= ys_len {
        (xs_len, ys_len)
    } else {
        (ys_len, xs_len)
    };
    assert!(out_limbs.len() >= max_len);
    let carry = limbs_add_same_length_to_out(out_limbs, &xs[..min_len], &ys[..min_len]);
    if xs_len == ys_len {
        carry
    } else if xs_len > ys_len {
        if carry {
            limbs_add_limb_to_out(&mut out_limbs[ys_len..], &xs[ys_len..], 1)
        } else {
            out_limbs[ys_len..xs_len].copy_from_slice(&xs[ys_len..]);
            false
        }
    } else if carry {
        limbs_add_limb_to_out(&mut out_limbs[xs_len..], &ys[xs_len..], 1)
    } else {
        out_limbs[xs_len..ys_len].copy_from_slice(&ys[xs_len..]);
        false
    }
}

/// Interpreting two equal-length slices of `u32`s as the limbs (in ascending order) of two
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
pub fn limbs_slice_add_same_length_in_place_left(xs: &mut [u32], ys: &[u32]) -> bool {
    let xs_len = xs.len();
    assert_eq!(xs_len, ys.len());
    let mut carry = false;
    for i in 0..xs_len {
        xs[i] = add_and_carry(xs[i], ys[i], &mut carry);
    }
    carry
}

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of two `Natural`s, where the
/// length of the first slice is greater than or equal to the length of the second, writes the
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
pub fn limbs_slice_add_greater_in_place_left(xs: &mut [u32], ys: &[u32]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let carry = limbs_slice_add_same_length_in_place_left(&mut xs[..ys_len], ys);
    if xs_len == ys_len {
        carry
    } else if carry {
        limbs_slice_add_limb_in_place(&mut xs[ys_len..], 1)
    } else {
        false
    }
}

/// Interpreting a `Vec` of `u32`s and a slice of `u32`s as the limbs (in ascending order) of two
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
pub fn limbs_vec_add_in_place_left(xs: &mut Vec<u32>, ys: &[u32]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let carry = if xs_len >= ys_len {
        limbs_slice_add_greater_in_place_left(xs, ys)
    } else {
        let mut carry = limbs_slice_add_same_length_in_place_left(xs, &ys[..xs_len]);
        xs.extend_from_slice(&ys[xs_len..]);
        if carry {
            carry = limbs_slice_add_limb_in_place(&mut xs[xs_len..], 1);
        }
        carry
    };
    if carry {
        xs.push(1);
    }
}

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of two `Natural`s, writes
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
pub fn limbs_slice_add_in_place_either(xs: &mut Vec<u32>, ys: &mut Vec<u32>) -> (bool, bool) {
    if xs.len() >= ys.len() {
        (false, limbs_slice_add_greater_in_place_left(xs, ys))
    } else {
        (true, limbs_slice_add_greater_in_place_left(ys, xs))
    }
}

/// Interpreting two `Vec`s of `u32`s as the limbs (in ascending order) of two `Natural`s, writes
/// the limbs of the sum of the `Natural`s to the longer slice (or the first one, if they are
/// equally long). Returns a `bool` which is `false` when the output is to the first slice and
/// `true` when it's to the second slice.
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
pub fn limbs_vec_add_in_place_either(xs: &mut Vec<u32>, ys: &mut Vec<u32>) -> bool {
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
/// use malachite_base::num::Zero;
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
/// use malachite_base::num::Zero;
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
/// use malachite_base::num::Zero;
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
/// use malachite_base::num::Zero;
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
/// use malachite_base::num::Zero;
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
/// use malachite_base::num::Zero;
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
