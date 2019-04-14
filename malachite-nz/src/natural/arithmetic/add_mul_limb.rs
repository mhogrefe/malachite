use malachite_base::num::{AddMul, AddMulAssign, PrimitiveInteger, SplitInHalf};
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::mul_limb::{limbs_mul_limb_to_out, limbs_slice_mul_limb_in_place};
use natural::Natural::{self, Large};
use platform::{DoubleLimb, Limb};

/// Given the limbs of two `Natural`s a and b, and a limb c, returns the limbs of a + b * c. `xs`
/// and `ys` should be nonempty and have no trailing zeros, and `limb` should be nonzero. The result
/// will have no trailing zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive, sub is positive, and w
/// is returned instead of overwriting the first input.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul_limb::limbs_add_mul_limb;
///
/// assert_eq!(limbs_add_mul_limb(&[123, 456], &[123], 4), &[615, 456]);
/// assert_eq!(limbs_add_mul_limb(&[123], &[0, 123], 4), &[123, 492]);
/// assert_eq!(limbs_add_mul_limb(&[123, 456], &[0, 123], 0xffff_ffff), &[123, 333, 123]);
/// ```
pub fn limbs_add_mul_limb(xs: &[Limb], ys: &[Limb], limb: Limb) -> Vec<Limb> {
    let mut result;
    if xs.len() >= ys.len() {
        result = xs.to_vec();
        limbs_vec_add_mul_limb_greater_in_place_left(&mut result, ys, limb);
    } else {
        result = ys.to_vec();
        limbs_vec_add_mul_limb_smaller_in_place_right(xs, &mut result, limb);
    }
    result
}

/// Given the limbs of two `Natural`s a and b, and a limb c, computes a + b * c. The lowest
/// `ys.len()` limbs of the result are written to `xs`, and the highest limb of b * c, plus the
/// carry-out from the addition, is returned. `xs` must be at least as long as `ys`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ys.len()`
///
/// This is mpn_addmul_1 from mpn/generic/addmul_1.c.
///
/// # Panics
/// Panics if `xs` is shorter than `ys`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul_limb::*;
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_slice_add_mul_limb_greater_in_place_left(xs, &[123], 4), 0);
/// assert_eq!(xs, &[615, 456]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_slice_add_mul_limb_greater_in_place_left(xs, &[123], 0xffff_ffff), 123);
/// assert_eq!(xs, &[0, 456]);
///
/// let xs = &mut [123, 0];
/// assert_eq!(limbs_slice_add_mul_limb_greater_in_place_left(xs, &[0, 123], 4), 0);
/// assert_eq!(xs, &[123, 492]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_slice_add_mul_limb_greater_in_place_left(xs, &[0, 123], 0xffff_ffff), 123);
/// assert_eq!(xs, &[123, 333]);
/// ```
pub fn limbs_slice_add_mul_limb_greater_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    limb: Limb,
) -> Limb {
    let ys_len = ys.len();
    assert!(xs.len() >= ys_len);
    let mut carry = 0;
    let limb_double = DoubleLimb::from(limb);
    for i in 0..ys_len {
        let limb_result = DoubleLimb::from(xs[i]) + DoubleLimb::from(ys[i]) * limb_double + carry;
        xs[i] = limb_result.lower_half();
        carry = limb_result >> Limb::WIDTH;
    }
    carry as Limb
}

/// Given the limbs of two `Natural`s a and b, and a limb c, computes a + b * c. The lowest limbs of
/// the result are written to `ys` and the highest limb is returned. `xs` must have the same length
/// as `ys`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive and have the same
/// lengths, sub is positive, the lowest limbs of the result are written to the second input rather
/// than the first, and the highest limb is returned.
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul_limb::*;
///
/// let ys = &mut [0, 123];
/// assert_eq!(limbs_slice_add_mul_limb_same_length_in_place_right(&[123, 0], ys, 4), 0);
/// assert_eq!(ys, &[123, 492]);
///
/// let ys = &mut [0, 123];
/// assert_eq!(limbs_slice_add_mul_limb_same_length_in_place_right(&[123, 456], ys, 0xffff_ffff),
///         123);
/// assert_eq!(ys, &[123, 333]);
/// ```
pub fn limbs_slice_add_mul_limb_same_length_in_place_right(
    xs: &[Limb],
    ys: &mut [Limb],
    limb: Limb,
) -> Limb {
    let xs_len = xs.len();
    assert_eq!(ys.len(), xs_len);
    let mut carry = 0;
    let limb_double = DoubleLimb::from(limb);
    for i in 0..xs_len {
        let limb_result = DoubleLimb::from(xs[i]) + DoubleLimb::from(ys[i]) * limb_double + carry;
        ys[i] = limb_result.lower_half();
        carry = limb_result >> Limb::WIDTH;
    }
    carry as Limb
}

/// Given the limbs of two `Natural`s a and b, and a limb c, writes the limbs of a + b * c to the
/// first (left) input, corresponding to the limbs of a. `xs` and `ys` should be nonempty and have
/// no trailing zeros, and `limb` should be nonzero. The result will have no trailing zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`xs.len()`, `ys.len()`)
///       m = max(1, `ys.len()` - `xs.len()`)
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive and sub is positive.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul_limb::limbs_vec_add_mul_limb_in_place_left;
///
/// let mut xs = vec![123, 456];
/// limbs_vec_add_mul_limb_in_place_left(&mut xs, &[123], 4);
/// assert_eq!(xs, &[615, 456]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_add_mul_limb_in_place_left(&mut xs, &[123], 0xffff_ffff);
/// assert_eq!(xs, &[0, 579]);
///
/// let mut xs = vec![123];
/// limbs_vec_add_mul_limb_in_place_left(&mut xs, &[0, 123], 4);
/// assert_eq!(xs, &[123, 492]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_add_mul_limb_in_place_left(&mut xs, &[0, 123], 0xffff_ffff);
/// assert_eq!(xs, &[123, 333, 123]);
/// ```
pub fn limbs_vec_add_mul_limb_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], limb: Limb) {
    let xs_len = xs.len();
    if xs_len >= ys.len() {
        limbs_vec_add_mul_limb_greater_in_place_left(xs, ys, limb);
    } else {
        let (ys_lo, ys_hi) = ys.split_at(xs_len);
        xs.resize(ys.len(), 0);
        let mut carry;
        {
            let (xs_lo, xs_hi) = xs.split_at_mut(xs_len);
            carry = limbs_mul_limb_to_out(xs_hi, ys_hi, limb);
            let inner_carry = limbs_slice_add_mul_limb_greater_in_place_left(xs_lo, ys_lo, limb);
            if inner_carry != 0 && limbs_slice_add_limb_in_place(xs_hi, inner_carry) {
                carry += 1;
            }
        }
        if carry != 0 {
            xs.push(carry);
        }
    }
}

// ys.len() > 0, xs.len() >= ys.len(), limb != 0
fn limbs_vec_add_mul_limb_greater_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], limb: Limb) {
    let carry = limbs_slice_add_mul_limb_greater_in_place_left(xs, ys, limb);
    let ys_len = ys.len();
    if carry != 0 {
        if xs.len() == ys_len {
            xs.push(carry);
        } else if limbs_slice_add_limb_in_place(&mut xs[ys_len..], carry) {
            xs.push(1);
        }
    }
}

/// Given the limbs of two `Natural`s a and b, and a limb c, writes the limbs of a + b * c to the
/// second (right) input, corresponding to the limbs of b. `xs` and `ys` should be nonempty and have
/// no trailing zeros, and `limb` should be nonzero. The result will have no trailing zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`xs.len()`, `ys.len()`)
///       m = max(1, `xs.len()` - `ys.len()`)
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive, sub is positive, and the
/// result is written to the second input rather than the first.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul_limb::limbs_vec_add_mul_limb_in_place_right;
///
/// let mut ys = vec![123];
/// limbs_vec_add_mul_limb_in_place_right(&[123, 456], &mut ys, 4);
/// assert_eq!(ys, &[615, 456]);
///
/// let mut ys = vec![123];
/// limbs_vec_add_mul_limb_in_place_right(&[123, 456], &mut ys, 0xffff_ffff);
/// assert_eq!(ys, &[0, 579]);
///
/// let mut ys = vec![0, 123];
/// limbs_vec_add_mul_limb_in_place_right(&[123], &mut ys, 4);
/// assert_eq!(ys, &[123, 492]);
///
/// let mut ys = vec![0, 123];
/// limbs_vec_add_mul_limb_in_place_right(&[123, 456], &mut ys, 0xffff_ffff);
/// assert_eq!(ys, &[123, 333, 123]);
/// ```
pub fn limbs_vec_add_mul_limb_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>, limb: Limb) {
    let ys_len = ys.len();
    if xs.len() >= ys_len {
        let carry = limbs_slice_add_mul_limb_same_length_in_place_right(&xs[..ys_len], ys, limb);
        ys.extend_from_slice(&xs[ys_len..]);
        if carry != 0 {
            if xs.len() == ys_len {
                ys.push(carry);
            } else if limbs_slice_add_limb_in_place(&mut ys[ys_len..], carry) {
                ys.push(1);
            }
        }
    } else {
        limbs_vec_add_mul_limb_smaller_in_place_right(xs, ys, limb);
    }
}

// xs.len() > 0, xs.len() < ys.len(), limb != 0
fn limbs_vec_add_mul_limb_smaller_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>, limb: Limb) {
    let mut carry;
    {
        let (ys_lo, ys_hi) = ys.split_at_mut(xs.len());
        carry = limbs_slice_mul_limb_in_place(ys_hi, limb);
        let inner_carry = limbs_slice_add_mul_limb_same_length_in_place_right(xs, ys_lo, limb);
        if inner_carry != 0 && limbs_slice_add_limb_in_place(ys_hi, inner_carry) {
            carry += 1;
        }
    }
    if carry != 0 {
        ys.push(carry);
    }
}

/// Given the limbs of two `Natural`s a and b, and a limb c, writes the limbs of a + b * c to
/// whichever input is longer. If the result is written to the first input, `false` is returned; if
/// to the second, `true` is returned. `xs` and `ys` should be nonempty and have no trailing zeros,
/// and `limb` should be nonzero. The result will have no trailing zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive, sub is positive, and the
/// result is written to the longer input.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul_limb::limbs_vec_add_mul_limb_in_place_either;
///
/// let mut xs = vec![123, 456];
/// let mut ys = vec![123];
/// assert_eq!(limbs_vec_add_mul_limb_in_place_either(&mut xs, &mut ys, 4), false);
/// assert_eq!(xs, &[615, 456]);
/// assert_eq!(ys, &[123]);
///
/// let mut xs = vec![123, 456];
/// let mut ys = vec![123];
/// assert_eq!(limbs_vec_add_mul_limb_in_place_either(&mut xs, &mut ys, 0xffff_ffff), false);
/// assert_eq!(xs, &[0, 579]);
/// assert_eq!(ys, &[123]);
///
/// let mut xs = vec![123];
/// let mut ys = vec![0, 123];
/// assert_eq!(limbs_vec_add_mul_limb_in_place_either(&mut xs, &mut ys, 4), true);
/// assert_eq!(xs, &[123]);
/// assert_eq!(ys, &[123, 492]);
///
/// let mut xs = vec![123, 456];
/// let mut ys = vec![0, 123];
/// assert_eq!(limbs_vec_add_mul_limb_in_place_either(&mut xs, &mut ys, 0xffff_ffff), false);
/// assert_eq!(xs, &[123, 333, 123]);
/// assert_eq!(ys, &[0, 123]);
/// ```
pub fn limbs_vec_add_mul_limb_in_place_either(
    xs: &mut Vec<Limb>,
    ys: &mut Vec<Limb>,
    limb: Limb,
) -> bool {
    if xs.len() >= ys.len() {
        limbs_vec_add_mul_limb_greater_in_place_left(xs, ys, limb);
        false
    } else {
        limbs_vec_add_mul_limb_smaller_in_place_right(xs, ys, limb);
        true
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), taking `self` and b
/// by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `min(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(10u32).add_mul(Natural::from(3u32), 4), 22);
///     assert_eq!(Natural::trillion().add_mul(Natural::from(0x1_0000u32), 0x1_0000u32).to_string(),
///                "1004294967296");
/// }
/// ```
impl AddMul<Natural, Limb> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(mut self, b: Natural, c: Limb) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl AddMul<Natural, u32> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(self, b: Natural, c: u32) -> Natural {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), taking `self` by
/// value and b by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `b.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(10u32).add_mul(&Natural::from(3u32), 4), 22);
///     assert_eq!(Natural::trillion().add_mul(&Natural::from(0x1_0000u32),
///         0x1_0000u32).to_string(), "1004294967296");
/// }
/// ```
impl<'a> AddMul<&'a Natural, Limb> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(mut self, b: &'a Natural, c: Limb) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> AddMul<&'a Natural, u32> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(self, b: &'a Natural, c: u32) -> Natural {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), taking `self` by
/// reference and b by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::from(10u32)).add_mul(Natural::from(3u32), 4), 22);
///     assert_eq!((&Natural::trillion()).add_mul(Natural::from(0x1_0000u32),
///         0x1_0000u32).to_string(), "1004294967296");
/// }
/// ```
impl<'a> AddMul<Natural, Limb> for &'a Natural {
    type Output = Natural;

    fn add_mul(self, mut b: Natural, c: Limb) -> Natural {
        if c == 0 || b == 0 as Limb {
            return self.clone();
        }
        if c == 1 {
            return self + b;
        }
        let fallback = match (self, &mut b) {
            (Large(ref a_limbs), Large(ref mut b_limbs)) => {
                limbs_vec_add_mul_limb_in_place_right(a_limbs, b_limbs, c);
                false
            }
            _ => true,
        };
        if fallback {
            self + b * c
        } else {
            b
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> AddMul<Natural, u32> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(self, b: Natural, c: u32) -> Natural {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), taking `self` and b
/// by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `max(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::from(10u32)).add_mul(&Natural::from(3u32), 4), 22);
///     assert_eq!((&Natural::trillion()).add_mul(&Natural::from(0x1_0000u32),
///         0x1_0000u32).to_string(), "1004294967296");
/// }
/// ```
impl<'a, 'b> AddMul<&'a Natural, Limb> for &'b Natural {
    type Output = Natural;

    fn add_mul(self, b: &'a Natural, c: Limb) -> Natural {
        if c == 0 || *b == 0 as Limb {
            return self.clone();
        }
        if c == 1 {
            return self + b;
        }
        match (self, b) {
            (Large(ref a_limbs), Large(ref b_limbs)) => {
                Large(limbs_add_mul_limb(a_limbs, b_limbs, c))
            }
            _ => self + b * c,
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a, 'b> AddMul<&'a Natural, u32> for &'b Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(self, b: &'a Natural, c: u32) -> Natural {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), in place, taking b
/// by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `min(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMulAssign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(10u32);
///     x.add_mul_assign(Natural::from(3u32), 4);
///     assert_eq!(x, 22);
///
///     let mut x = Natural::trillion();
///     x.add_mul_assign(Natural::from(0x1_0000u32), 0x1_0000u32);
///     assert_eq!(x.to_string(), "1004294967296");
/// }
/// ```
impl AddMulAssign<Natural, Limb> for Natural {
    fn add_mul_assign(&mut self, mut b: Natural, c: Limb) {
        if c == 0 || b == 0 as Limb {
            return;
        }
        if c == 1 {
            *self += b;
            return;
        }
        let (fallback, right) = match (&mut *self, &mut b) {
            (&mut Large(ref mut a_limbs), &mut Large(ref mut b_limbs)) => (
                false,
                limbs_vec_add_mul_limb_in_place_either(a_limbs, b_limbs, c),
            ),
            _ => (true, false),
        };
        if fallback {
            *self += b * c;
        } else if right {
            *self = b;
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl AddMulAssign<Natural, u32> for Natural {
    #[inline]
    fn add_mul_assign(&mut self, b: Natural, c: u32) {
        self.add_mul_assign(b, Limb::from(c));
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), in place, taking b
/// by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `b.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMulAssign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(10u32);
///     x.add_mul_assign(&Natural::from(3u32), 4);
///     assert_eq!(x, 22);
///
///     let mut x = Natural::trillion();
///     x.add_mul_assign(&Natural::from(0x1_0000u32), 0x1_0000u32);
///     assert_eq!(x.to_string(), "1004294967296");
/// }
/// ```
impl<'a> AddMulAssign<&'a Natural, Limb> for Natural {
    fn add_mul_assign(&mut self, b: &'a Natural, c: Limb) {
        if c == 0 || *b == 0 as Limb {
            return;
        }
        if c == 1 {
            *self += b;
            return;
        }
        let fallback = match (&mut *self, b) {
            (&mut Large(ref mut a_limbs), &Large(ref b_limbs)) => {
                limbs_vec_add_mul_limb_in_place_left(a_limbs, b_limbs, c);
                false
            }
            _ => true,
        };
        if fallback {
            *self += b * c;
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> AddMulAssign<&'a Natural, u32> for Natural {
    #[inline]
    fn add_mul_assign(&mut self, b: &'a Natural, c: u32) {
        self.add_mul_assign(b, Limb::from(c));
    }
}
