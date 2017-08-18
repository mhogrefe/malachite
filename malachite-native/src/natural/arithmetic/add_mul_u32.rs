use natural::Natural::{self, Large, Small};
use natural::{get_lower, LIMB_BITS};
use traits::{AddMul, AddMulAssign};

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), taking `self` by
/// value.
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!(Natural::from(10u32).add_mul(&Natural::from(3u32), 4), 22);
/// assert_eq!(Natural::from_str("1000000000000").unwrap()
///                     .add_mul(&Natural::from(65536u32), 65536).to_string(),
///            "1004294967296");
/// ```
impl<'a> AddMul<&'a Natural, u32> for Natural {
    type Output = Natural;

    fn add_mul(mut self, b: &'a Natural, c: u32) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), taking `self` by
/// reference.
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(10u32)).add_mul(&Natural::from(3u32), 4), 22);
/// assert_eq!((&Natural::from_str("1000000000000").unwrap())
///                     .add_mul(&Natural::from(65536u32), 65536).to_string(),
///             "1004294967296");
/// ```
impl<'a, 'b> AddMul<&'a Natural, u32> for &'b Natural {
    type Output = Natural;

    fn add_mul(self, b: &'a Natural, c: u32) -> Natural {
        if c == 0 || *b == 0 {
            return self.clone();
        }
        if c == 1 {
            return self + b;
        }
        if let Small(small_b) = *b {
            if let Some(product) = small_b.checked_mul(c) {
                return self + product;
            }
        }
        Large(match (self, b) {
            (&Small(small_self), &Small(small_b)) => {
                large_add_mul_u32(&[small_self], &[small_b], c)
            }
            (&Small(small_self), &Large(ref b_limbs)) => {
                large_add_mul_u32(&[small_self], b_limbs, c)
            }
            (&Large(ref self_limbs), &Small(small_b)) => {
                large_add_mul_u32(self_limbs, &[small_b], c)
            }
            (&Large(ref self_limbs), &Large(ref b_limbs)) => {
                large_add_mul_u32(self_limbs, b_limbs, c)
            }
        })
    }
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), in place.
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMulAssign;
/// use std::str::FromStr;
///
/// let mut x = Natural::from(10u32);
/// x.add_mul_assign(&Natural::from(3u32), 4);
/// assert_eq!(x, 22);
///
/// let mut x = Natural::from_str("1000000000000").unwrap();
/// x.add_mul_assign(&Natural::from(65536u32), 65536);
/// assert_eq!(x.to_string(), "1004294967296");
/// ```
impl<'a> AddMulAssign<&'a Natural, u32> for Natural {
    fn add_mul_assign(&mut self, b: &'a Natural, c: u32) {
        if c == 0 || *b == 0 {
            return;
        }
        if c == 1 {
            *self += b;
            return;
        }
        if let Small(small_b) = *b {
            if let Some(product) = small_b.checked_mul(c) {
                *self += product;
                return;
            }
        }
        let mut self_limbs = self.promote_in_place();
        match b {
            &Small(small) => large_add_mul_u32_in_place(&mut self_limbs, &[small], c),
            &Large(ref b_limbs) => large_add_mul_u32_in_place(&mut self_limbs, b_limbs, c),
        }
    }
}

fn add_mul_and_carry(x: u32, y: u32, multiplicand: u64, carry: &mut u64) -> u32 {
    let sum = y as u64 * multiplicand + x as u64 + *carry;
    *carry = sum >> LIMB_BITS;
    get_lower(sum)
}

fn large_add_mul_u32_in_place(xs: &mut Vec<u32>, ys: &[u32], multiplicand: u32) {
    let mut carry = 0;
    let mut ys_iter = ys.iter();
    let multiplicand = multiplicand as u64;
    for x in xs.iter_mut() {
        match ys_iter.next() {
            Some(y) => *x = add_mul_and_carry(*x, *y, multiplicand, &mut carry),
            None if carry != 0 => *x = add_mul_and_carry(*x, 0, multiplicand, &mut carry),
            None => break,
        }
    }
    for y in ys_iter {
        xs.push(add_mul_and_carry(0, *y, multiplicand, &mut carry));
    }
    if carry != 0 {
        xs.push(carry as u32);
    }
}

fn large_add_mul_u32(xs: &[u32], ys: &[u32], multiplicand: u32) -> Vec<u32> {
    let mut result_limbs = Vec::with_capacity(xs.len());
    let mut carry = 0;
    let mut ys_iter = ys.iter();
    let multiplicand = multiplicand as u64;
    for &x in xs.iter() {
        result_limbs.push(match ys_iter.next() {
            Some(&y) => add_mul_and_carry(x, y, multiplicand, &mut carry),
            None if carry != 0 => add_mul_and_carry(x, 0, multiplicand, &mut carry),
            None => x,
        });
    }
    for y in ys_iter {
        result_limbs.push(add_mul_and_carry(0, *y, multiplicand, &mut carry));
    }
    if carry != 0 {
        result_limbs.push(carry as u32);
    }
    result_limbs
}
