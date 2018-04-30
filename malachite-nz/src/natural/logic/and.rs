use malachite_base::limbs::limbs_set_zero;
use natural::Natural::{self, Large, Small};
use std::cmp::Ordering;
use std::ops::{BitAnd, BitAndAssign};

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of two `Natural`s, returns
/// a `Vec` of the limbs of the bitwise and of the `Natural`s. The length of the result is the
/// length of the smaller input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = min(`xs.len()`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::and::limbs_and;
///
/// assert_eq!(limbs_and(&[6, 7], &[1, 2, 3]), vec![0, 2]);
/// assert_eq!(limbs_and(&[100, 101, 102], &[102, 101, 100]), vec![100, 101, 100]);
/// ```
pub fn limbs_and(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    xs.iter().zip(ys.iter()).map(|(x, y)| x & y).collect()
}

fn limbs_and_same_length_to_out_no_check(out_limbs: &mut [u32], xs: &[u32], ys: &[u32]) {
    for i in 0..xs.len() {
        out_limbs[i] = xs[i] & ys[i];
    }
}

/// Interpreting two equal-length slices of `u32`s as the limbs (in ascending order) of two
/// `Natural`s, writes the limbs of the bitwise and of the `Natural`s to a specified slice. The
/// output slice must be at least as long as the length of one of the input slices.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths or if out_limbs is too short.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::and::limbs_and_same_length_to_out;
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_and_same_length_to_out(&mut out, &[6, 7], &[1, 2]);
/// assert_eq!(out, vec![0, 2, 10, 10]);
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_and_same_length_to_out(&mut out, &[100, 101, 102], &[102, 101, 100]);
/// assert_eq!(out, vec![100, 101, 100, 10]);
/// ```
pub fn limbs_and_same_length_to_out(out_limbs: &mut [u32], xs: &[u32], ys: &[u32]) {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(out_limbs.len() >= len);
    limbs_and_same_length_to_out_no_check(out_limbs, xs, ys);
}

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of two `Natural`s, writes
/// the limbs of the bitwise and of the `Natural`s to a specified slice. The output slice must be at
/// least as long as the longer input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Panics
/// Panics if out_limbs is too short.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::and::limbs_and_to_out;
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_and_to_out(&mut out, &[6, 7], &[1, 2, 3]);
/// assert_eq!(out, vec![0, 2, 0, 10]);
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_and_to_out(&mut out, &[100, 101, 102], &[102, 101, 100]);
/// assert_eq!(out, vec![100, 101, 100, 10]);
/// ```
pub fn limbs_and_to_out(out_limbs: &mut [u32], xs: &[u32], ys: &[u32]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        assert!(out_limbs.len() >= xs_len);
        limbs_and_same_length_to_out_no_check(out_limbs, &xs[0..ys_len], ys);
        limbs_set_zero(&mut out_limbs[ys_len..xs_len]);
    } else {
        assert!(out_limbs.len() >= ys_len);
        limbs_and_same_length_to_out_no_check(out_limbs, xs, &ys[0..xs_len]);
        limbs_set_zero(&mut out_limbs[xs_len..ys_len]);
    }
}

fn limbs_and_same_length_in_place_left_no_check(xs: &mut [u32], ys: &[u32]) {
    for i in 0..xs.len() {
        xs[i] &= ys[i];
    }
}

/// Interpreting two equal-length slices of `u32`s as the limbs (in ascending order) of two
/// `Natural`s, writes the limbs of the bitwise and of the `Natural`s to the first (left) slice.
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
/// use malachite_nz::natural::logic::and::limbs_and_same_length_in_place_left;
///
/// let mut xs = vec![6, 7];
/// limbs_and_same_length_in_place_left(&mut xs, &[1, 2]);
/// assert_eq!(xs, vec![0, 2]);
///
/// let mut xs = vec![100, 101, 102];
/// limbs_and_same_length_in_place_left(&mut xs, &[102, 101, 100]);
/// assert_eq!(xs, vec![100, 101, 100]);
/// ```
pub fn limbs_and_same_length_in_place_left(xs: &mut [u32], ys: &[u32]) {
    assert_eq!(xs.len(), ys.len());
    limbs_and_same_length_in_place_left_no_check(xs, ys);
}

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of two `Natural`s, writes
/// the limbs of the bitwise and of the `Natural`s to the first (left) slice. If the second slice is
/// shorter than the first, then some of the most-significant bits of the first slice should become
/// zero. Rather than setting them to zero, this function optionally returns the length of the
/// significant part of the slice. The caller can decide whether to zero the rest. If `None` is
/// returned, the entire slice remains significant.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`xs.len()`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::and::limbs_slice_and_in_place_left;
///
/// let mut xs = vec![6, 7];
/// assert_eq!(limbs_slice_and_in_place_left(&mut xs, &[1, 2, 3]), None);
/// assert_eq!(xs, vec![0, 2]);
///
/// let mut xs = vec![1, 2, 3];
/// assert_eq!(limbs_slice_and_in_place_left(&mut xs, &[6, 7]), Some(2));
/// assert_eq!(xs, vec![0, 2, 3]);
///
/// let mut xs = vec![100, 101, 102];
/// assert_eq!(limbs_slice_and_in_place_left(&mut xs, &[102, 101, 100]), None);
/// assert_eq!(xs, vec![100, 101, 100]);
/// ```
pub fn limbs_slice_and_in_place_left(xs: &mut [u32], ys: &[u32]) -> Option<usize> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    match xs_len.cmp(&ys.len()) {
        Ordering::Equal => {
            limbs_and_same_length_in_place_left_no_check(xs, ys);
            None
        }
        Ordering::Greater => {
            limbs_and_same_length_in_place_left_no_check(&mut xs[0..ys_len], ys);
            Some(ys_len)
        }
        Ordering::Less => {
            limbs_and_same_length_in_place_left_no_check(xs, &ys[0..xs_len]);
            None
        }
    }
}

/// Interpreting a `Vec` of `u32`s and a slice of `u32`s as the limbs (in ascending order) of two
/// `Natural`s, writes the limbs of the bitwise and of the `Natural`s to the `Vec`. If the slice is
/// shorter than the `Vec`, then some of the most-significant bits of the `Vec` should become zero.
/// Rather than setting them to zero, this function truncates the `Vec`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`xs.len()`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::and::limbs_vec_and_in_place_left;
///
/// let mut xs = vec![6, 7];
/// limbs_vec_and_in_place_left(&mut xs, &[1, 2, 3]);
/// assert_eq!(xs, vec![0, 2]);
///
/// let mut xs = vec![1, 2, 3];
/// limbs_vec_and_in_place_left(&mut xs, &[6, 7]);
/// assert_eq!(xs, vec![0, 2]);
///
/// let mut xs = vec![100, 101, 102];
/// limbs_vec_and_in_place_left(&mut xs, &[102, 101, 100]);
/// assert_eq!(xs, vec![100, 101, 100]);
/// ```
pub fn limbs_vec_and_in_place_left(xs: &mut Vec<u32>, ys: &[u32]) {
    if let Some(truncate_size) = limbs_slice_and_in_place_left(xs, ys) {
        xs.truncate(truncate_size);
    }
}

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of two `Natural`s, takes
/// the limbs of the bitwise and of the `Natural`s and writes them to the shorter slice (or the
/// first one, if they are equally long). If the function writes to the first slice, it returns
/// `false`; otherwise, it returns `true`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`xs.len()`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::and::limbs_and_in_place_either;
///
/// let mut xs = vec![6, 7];
/// let mut ys = vec![1, 2, 3];
/// assert_eq!(limbs_and_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, vec![0, 2]);
/// assert_eq!(ys, vec![1, 2, 3]);
///
/// let mut xs = vec![1, 2, 3];
/// let mut ys = vec![6, 7];
/// assert_eq!(limbs_and_in_place_either(&mut xs, &mut ys), true);
/// assert_eq!(xs, vec![1, 2, 3]);
/// assert_eq!(ys, vec![0, 2]);
///
/// let mut xs = vec![100, 101, 102];
/// let mut ys = vec![102, 101, 100];
/// assert_eq!(limbs_and_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, vec![100, 101, 100]);
/// assert_eq!(ys, vec![102, 101, 100]);
/// ```
pub fn limbs_and_in_place_either(xs: &mut [u32], ys: &mut [u32]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    match xs_len.cmp(&ys_len) {
        Ordering::Equal => {
            limbs_and_same_length_in_place_left_no_check(xs, ys);
            false
        }
        Ordering::Less => {
            limbs_and_same_length_in_place_left_no_check(xs, &ys[0..xs_len]);
            false
        }
        Ordering::Greater => {
            limbs_and_same_length_in_place_left_no_check(ys, &xs[0..ys_len]);
            true
        }
    }
}

/// Takes the bitwise and of two `Natural`s, taking both by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((Natural::from(123u32) & Natural::from(456u32)).to_string(), "72");
/// assert_eq!((Natural::trillion() & (Natural::trillion() - 1).unwrap()).to_string(),
///     "999999995904");
/// ```
impl BitAnd<Natural> for Natural {
    type Output = Natural;

    fn bitand(mut self, other: Natural) -> Natural {
        self &= other;
        self
    }
}

/// Takes the bitwise and of two `Natural`s, taking the left `Natural` by value and the right
/// `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((Natural::from(123u32) & &Natural::from(456u32)).to_string(), "72");
/// assert_eq!((Natural::trillion() & &(Natural::trillion() - 1).unwrap()).to_string(),
///     "999999995904");
/// ```
impl<'a> BitAnd<&'a Natural> for Natural {
    type Output = Natural;

    fn bitand(mut self, other: &'a Natural) -> Natural {
        self &= other;
        self
    }
}

/// Takes the bitwise and of two `Natural`s, taking the left `Natural` by reference and the right
/// `Natural` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits`
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((&Natural::from(123u32) & Natural::from(456u32)).to_string(), "72");
/// assert_eq!((&Natural::trillion() & (Natural::trillion() - 1).unwrap()).to_string(),
///     "999999995904");
/// ```
impl<'a> BitAnd<Natural> for &'a Natural {
    type Output = Natural;

    fn bitand(self, mut other: Natural) -> Natural {
        other &= self;
        other
    }
}

/// Takes the bitwise and of two `Natural`s, taking both `Natural`s by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `max(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((&Natural::from(123u32) & &Natural::from(456u32)).to_string(), "72");
/// assert_eq!((&Natural::trillion() & &(Natural::trillion() - 1).unwrap()).to_string(),
///     "999999995904");
/// ```
impl<'a, 'b> BitAnd<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn bitand(self, other: &'a Natural) -> Natural {
        match (self, other) {
            (x, &Small(y)) => Small(x & y),
            (&Small(x), y) => Small(x & y),
            (&Large(ref xs), &Large(ref ys)) => {
                let mut result = Large(limbs_and(xs, ys));
                result.trim();
                result
            }
        }
    }
}

/// Bitwise-ands a `Natural` with another `Natural` in place, taking the `Natural` on the RHS by
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
/// use malachite_nz::natural::Natural;
///
/// let mut x = Natural::from(0xffff_ffffu32);
/// x &= Natural::from(0xf0ff_ffffu32);
/// x &= Natural::from(0xfff0_ffffu32);
/// x &= Natural::from(0xffff_f0ffu32);
/// x &= Natural::from(0xffff_fff0u32);
/// assert_eq!(x, 0xf0f0_f0f0);
/// ```
impl BitAndAssign<Natural> for Natural {
    fn bitand_assign(&mut self, other: Natural) {
        if let Small(y) = other {
            *self &= y;
        } else if let Small(ref mut x) = *self {
            *x = &other & *x;
        } else if let Large(mut ys) = other {
            if let Large(ref mut xs) = *self {
                if limbs_and_in_place_either(xs, &mut ys) {
                    *xs = ys;
                }
            }
            self.trim();
        }
    }
}

/// Bitwise-ands a `Natural` with another `Natural` in place, taking the `Natural` on the RHS by
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
/// use malachite_nz::natural::Natural;
///
/// let mut x = Natural::from(0xffff_ffffu32);
/// x &= &Natural::from(0xf0ff_ffffu32);
/// x &= &Natural::from(0xfff0_ffffu32);
/// x &= &Natural::from(0xffff_f0ffu32);
/// x &= &Natural::from(0xffff_fff0u32);
/// assert_eq!(x, 0xf0f0_f0f0);
/// ```
impl<'a> BitAndAssign<&'a Natural> for Natural {
    fn bitand_assign(&mut self, other: &'a Natural) {
        if let Small(y) = *other {
            *self &= y;
        } else if let Small(ref mut x) = *self {
            *x = other & *x;
        } else if let Large(ref ys) = *other {
            if let Large(ref mut xs) = *self {
                limbs_vec_and_in_place_left(xs, ys);
            }
            self.trim();
        }
    }
}
