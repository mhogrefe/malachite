use natural::Natural::{self, Large, Small};
use natural::{get_lower, LIMB_BITS};
use std::cmp::max;
use std::mem::swap;
use traits::{AddMul, AddMulAssign};

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), taking `self` and b
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
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!(Natural::from(10u32).add_mul(Natural::from(3u32), 4), 22);
/// assert_eq!(Natural::from_str("1000000000000").unwrap()
///                     .add_mul(Natural::from(65536u32), 65536).to_string(),
///            "1004294967296");
/// ```
impl AddMul<Natural, u32> for Natural {
    type Output = Natural;

    fn add_mul(mut self, b: Natural, c: u32) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), taking `self` by
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
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(10u32)).add_mul(Natural::from(3u32), 4), 22);
/// assert_eq!((&Natural::from_str("1000000000000").unwrap())
///                     .add_mul(Natural::from(65536u32), 65536).to_string(),
///            "1004294967296");
/// ```
impl<'a> AddMul<Natural, u32> for &'a Natural {
    type Output = Natural;

    fn add_mul(self, mut b: Natural, c: u32) -> Natural {
        if c == 0 || b == 0 {
            return self.clone();
        }
        if c == 1 {
            b += self;
            return b;
        }
        if let Small(small_b) = b {
            if let Some(product) = small_b.checked_mul(c) {
                return self + product;
            }
        }
        {
            let mut b_limbs = b.promote_in_place();
            let old_len = b_limbs.len();
            b_limbs.resize(max(self.limb_count() as usize, old_len) + 1, 0);
            match self {
                &Small(small) => large_add_mul_u32_mut_b(&[small], &mut b_limbs, c),
                &Large(ref limbs) => large_add_mul_u32_mut_b(limbs, &mut b_limbs, c),
            }
        }
        b.trim();
        b
    }
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), taking `self` and b
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

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), in place, taking b by
/// value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `min(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMulAssign;
/// use std::str::FromStr;
///
/// let mut x = Natural::from(10u32);
/// x.add_mul_assign(Natural::from(3u32), 4);
/// assert_eq!(x, 22);
///
/// let mut x = Natural::from_str("1000000000000").unwrap();
/// x.add_mul_assign(Natural::from(65536u32), 65536);
/// assert_eq!(x.to_string(), "1004294967296");
/// ```
impl AddMulAssign<Natural, u32> for Natural {
    fn add_mul_assign(&mut self, mut b: Natural, c: u32) {
        if c == 0 || b == 0 {
            return;
        }
        if c == 1 {
            *self += b;
            return;
        }
        if let Small(small_b) = b {
            if let Some(product) = small_b.checked_mul(c) {
                *self += product;
                return;
            }
        }
        let self_limb_count = self.limb_count();
        let b_limb_count = b.limb_count();
        if self_limb_count >= b_limb_count {
            let mut self_limbs = self.promote_in_place();
            let old_len = self_limbs.len();
            self_limbs.resize(max(old_len, b.limb_count() as usize) + 1, 0);
            match b {
                Small(small) => large_add_mul_u32_mut_a(&mut self_limbs, &[small], c),
                Large(ref b_limbs) => large_add_mul_u32_mut_a(&mut self_limbs, b_limbs, c),
            }
        } else {
            {
                let mut b_limbs = b.promote_in_place();
                let old_len = b_limbs.len();
                b_limbs.resize(max(self.limb_count() as usize, old_len) + 1, 0);
                match self {
                    &mut Small(small) => large_add_mul_u32_mut_b(&[small], &mut b_limbs, c),
                    &mut Large(ref limbs) => large_add_mul_u32_mut_b(limbs, &mut b_limbs, c),
                }
            }
            swap(self, &mut b);
        }
        self.trim();
    }
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), in place, taking b by
/// reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `b.significant_bits()`
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
        {
            let mut self_limbs = self.promote_in_place();
            let old_len = self_limbs.len();
            self_limbs.resize(max(old_len, b.limb_count() as usize) + 1, 0);
            match b {
                &Small(small) => large_add_mul_u32_mut_a(&mut self_limbs, &[small], c),
                &Large(ref b_limbs) => large_add_mul_u32_mut_a(&mut self_limbs, b_limbs, c),
            }
        }
        self.trim();
    }
}

fn add_mul_and_carry(x: u32, y: u32, multiplicand: u64, carry: &mut u64) -> u32 {
    let sum = y as u64 * multiplicand + x as u64 + *carry;
    *carry = sum >> LIMB_BITS;
    get_lower(sum)
}

// xs.len() must be > ys.len() and the highest-order limb of xs must be 0.
pub(crate) fn large_add_mul_u32_mut_a(xs: &mut [u32], ys: &[u32], multiplicand: u32) {
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
}

// ys.len() must be > xs.len() and the highest-order limb of ys must be 0.
pub(crate) fn large_add_mul_u32_mut_b(xs: &[u32], ys: &mut [u32], multiplicand: u32) {
    let mut carry = 0;
    let mut xs_iter = xs.iter();
    let multiplicand = multiplicand as u64;
    for y in ys.iter_mut() {
        match xs_iter.next() {
            Some(x) => *y = add_mul_and_carry(*x, *y, multiplicand, &mut carry),
            None => *y = add_mul_and_carry(0, *y, multiplicand, &mut carry),
        }
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
