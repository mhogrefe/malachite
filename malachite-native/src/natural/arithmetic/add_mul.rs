use integer::arithmetic::add_mul_u32::mpz_aorsmul_1;
use natural::arithmetic::add::mpn_add_in_place;
use natural::arithmetic::mul::mpn_mul;
use natural::arithmetic::sub::{mpn_sub_aba, mpn_sub_in_place};
use natural::comparison::ord::mpn_cmp_helper;
use natural::Natural::{self, Large, Small};
use std::cmp::{max, Ordering};
use traits::{AddMul, AddMulAssign};

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), taking `self`, b,
/// and c by value.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!(Natural::from(10u32).add_mul(Natural::from(3u32), Natural::from(4u32)), 22);
/// assert_eq!(Natural::from_str("1000000000000").unwrap()
///                     .add_mul(Natural::from(65536u32),
///                     Natural::from_str("1000000000000").unwrap()).to_string(),
///            "65537000000000000");
/// ```
impl<'a> AddMul<Natural, Natural> for Natural {
    type Output = Natural;

    fn add_mul(mut self, b: Natural, c: Natural) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), taking `self` and
/// b by value and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!(Natural::from(10u32).add_mul(Natural::from(3u32), &Natural::from(4u32)), 22);
/// assert_eq!(Natural::from_str("1000000000000").unwrap()
///                     .add_mul(Natural::from(65536u32),
///                     &Natural::from_str("1000000000000").unwrap()).to_string(),
///            "65537000000000000");
/// ```
impl<'a> AddMul<Natural, &'a Natural> for Natural {
    type Output = Natural;

    fn add_mul(mut self, b: Natural, c: &'a Natural) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), taking `self` and
/// c by value and b by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!(Natural::from(10u32).add_mul(&Natural::from(3u32), Natural::from(4u32)), 22);
/// assert_eq!(Natural::from_str("1000000000000").unwrap()
///                     .add_mul(&Natural::from(65536u32),
///                     Natural::from_str("1000000000000").unwrap()).to_string(),
///            "65537000000000000");
/// ```
impl<'a> AddMul<&'a Natural, Natural> for Natural {
    type Output = Natural;

    fn add_mul(mut self, b: &'a Natural, c: Natural) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), taking `self` by
/// value and b and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!(Natural::from(10u32).add_mul(&Natural::from(3u32), &Natural::from(4u32)), 22);
/// assert_eq!(Natural::from_str("1000000000000").unwrap()
///                     .add_mul(&Natural::from(65536u32),
///                     &Natural::from_str("1000000000000").unwrap()).to_string(),
///            "65537000000000000");
/// ```
impl<'a, 'b> AddMul<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    fn add_mul(mut self, b: &'a Natural, c: &'b Natural) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), taking `self`, b,
/// and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(10u32)).add_mul(&Natural::from(3u32), &Natural::from(4u32)), 22);
/// assert_eq!((&Natural::from_str("1000000000000").unwrap())
///                     .add_mul(&Natural::from(65536u32),
///                     &Natural::from_str("1000000000000").unwrap()).to_string(),
///             "65537000000000000");
/// assert_eq!((&Natural::from_str("0").unwrap())
///                     .add_mul(&Natural::from_str("1000000000000").unwrap(),
///                     &Natural::from_str("1000000000000").unwrap()).to_string(),
///             "1000000000000000000000000");
/// ```
impl<'a, 'b, 'c> AddMul<&'a Natural, &'b Natural> for &'c Natural {
    type Output = Natural;

    fn add_mul(self, b: &'a Natural, c: &'b Natural) -> Natural {
        if let Small(small_b) = *b {
            self.add_mul(c, small_b)
        } else if let Small(small_c) = *c {
            self.add_mul(b, small_c)
        } else if *self == 0 {
            b * c
        } else {
            let mut result = {
                let mut result_limbs = self.to_limbs_le();
                if let &Large(ref c_limbs) = c {
                    let mut self_sign = false;
                    match b {
                        &Small(small_b) => {
                            mpz_aorsmul(
                                &mut self_sign,
                                &mut result_limbs,
                                false,
                                &[small_b],
                                false,
                                c_limbs,
                                true,
                            );
                        }
                        &Large(ref b_limbs) => {
                            mpz_aorsmul(
                                &mut self_sign,
                                &mut result_limbs,
                                false,
                                b_limbs,
                                false,
                                c_limbs,
                                true,
                            );
                        }
                    }
                    assert!(!self_sign, "{} {} {}", self, b, c);
                }
                Large(result_limbs)
            };
            result.trim();
            result
        }
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b and c by value.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMulAssign;
/// use std::str::FromStr;
///
/// let mut x = Natural::from(10u32);
/// x.add_mul_assign(Natural::from(3u32), Natural::from(4u32));
/// assert_eq!(x, 22);
///
/// let mut x = Natural::from_str("1000000000000").unwrap();
/// x.add_mul_assign(Natural::from(65536u32), Natural::from_str("1000000000000").unwrap());
/// assert_eq!(x.to_string(), "65537000000000000");
/// ```
impl<'a> AddMulAssign<Natural, Natural> for Natural {
    fn add_mul_assign(&mut self, b: Natural, c: Natural) {
        if let Small(small_b) = b {
            self.add_mul_assign(c, small_b);
        } else if let Small(small_c) = c {
            self.add_mul_assign(b, small_c);
        } else if *self == 0 {
            *self = b * c;
        } else {
            {
                let self_limbs = self.promote_in_place();
                if let Large(ref c_limbs) = c {
                    let mut self_sign = false;
                    match b {
                        Small(small_b) => {
                            mpz_aorsmul(
                                &mut self_sign,
                                self_limbs,
                                false,
                                &[small_b],
                                false,
                                c_limbs,
                                true,
                            );
                        }
                        Large(ref b_limbs) => {
                            mpz_aorsmul(
                                &mut self_sign,
                                self_limbs,
                                false,
                                b_limbs,
                                false,
                                c_limbs,
                                true,
                            );
                        }
                    }
                    assert!(!self_sign);
                }
            }
            self.trim();
        }
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b by value and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMulAssign;
/// use std::str::FromStr;
///
/// let mut x = Natural::from(10u32);
/// x.add_mul_assign(Natural::from(3u32), &Natural::from(4u32));
/// assert_eq!(x, 22);
///
/// let mut x = Natural::from_str("1000000000000").unwrap();
/// x.add_mul_assign(Natural::from(65536u32), &Natural::from_str("1000000000000").unwrap());
/// assert_eq!(x.to_string(), "65537000000000000");
/// ```
impl<'a> AddMulAssign<Natural, &'a Natural> for Natural {
    fn add_mul_assign(&mut self, b: Natural, c: &'a Natural) {
        if let Small(small_b) = b {
            self.add_mul_assign(c, small_b);
        } else if let Small(small_c) = *c {
            self.add_mul_assign(b, small_c);
        } else if *self == 0 {
            *self = b * c;
        } else {
            {
                let self_limbs = self.promote_in_place();
                if let &Large(ref c_limbs) = c {
                    let mut self_sign = false;
                    match b {
                        Small(small_b) => {
                            mpz_aorsmul(
                                &mut self_sign,
                                self_limbs,
                                false,
                                &[small_b],
                                false,
                                c_limbs,
                                true,
                            );
                        }
                        Large(ref b_limbs) => {
                            mpz_aorsmul(
                                &mut self_sign,
                                self_limbs,
                                false,
                                b_limbs,
                                false,
                                c_limbs,
                                true,
                            );
                        }
                    }
                    assert!(!self_sign);
                }
            }
            self.trim();
        }
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b by reference and c by value.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMulAssign;
/// use std::str::FromStr;
///
/// let mut x = Natural::from(10u32);
/// x.add_mul_assign(&Natural::from(3u32), Natural::from(4u32));
/// assert_eq!(x, 22);
///
/// let mut x = Natural::from_str("1000000000000").unwrap();
/// x.add_mul_assign(&Natural::from(65536u32), Natural::from_str("1000000000000").unwrap());
/// assert_eq!(x.to_string(), "65537000000000000");
/// ```
impl<'a> AddMulAssign<&'a Natural, Natural> for Natural {
    fn add_mul_assign(&mut self, b: &'a Natural, c: Natural) {
        if let Small(small_b) = *b {
            self.add_mul_assign(c, small_b);
        } else if let Small(small_c) = c {
            self.add_mul_assign(b, small_c);
        } else if *self == 0 {
            *self = b * c;
        } else {
            {
                let self_limbs = self.promote_in_place();
                if let Large(ref c_limbs) = c {
                    let mut self_sign = false;
                    match b {
                        &Small(small_b) => {
                            mpz_aorsmul(
                                &mut self_sign,
                                self_limbs,
                                false,
                                &[small_b],
                                false,
                                c_limbs,
                                true,
                            );
                        }
                        &Large(ref b_limbs) => {
                            mpz_aorsmul(
                                &mut self_sign,
                                self_limbs,
                                false,
                                b_limbs,
                                false,
                                c_limbs,
                                true,
                            );
                        }
                    }
                    assert!(!self_sign);
                }
            }
            self.trim();
        }
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMulAssign;
/// use std::str::FromStr;
///
/// let mut x = Natural::from(10u32);
/// x.add_mul_assign(&Natural::from(3u32), &Natural::from(4u32));
/// assert_eq!(x, 22);
///
/// let mut x = Natural::from_str("1000000000000").unwrap();
/// x.add_mul_assign(&Natural::from(65536u32), &Natural::from_str("1000000000000").unwrap());
/// assert_eq!(x.to_string(), "65537000000000000");
/// ```
impl<'a, 'b> AddMulAssign<&'a Natural, &'b Natural> for Natural {
    fn add_mul_assign(&mut self, b: &'a Natural, c: &'b Natural) {
        if let Small(small_b) = *b {
            self.add_mul_assign(c, small_b);
        } else if let Small(small_c) = *c {
            self.add_mul_assign(b, small_c);
        } else if *self == 0 {
            *self = b * c;
        } else {
            {
                let self_limbs = self.promote_in_place();
                if let &Large(ref c_limbs) = c {
                    let mut self_sign = false;
                    match b {
                        &Small(small_b) => {
                            mpz_aorsmul(
                                &mut self_sign,
                                self_limbs,
                                false,
                                &[small_b],
                                false,
                                c_limbs,
                                true,
                            );
                        }
                        &Large(ref b_limbs) => {
                            mpz_aorsmul(
                                &mut self_sign,
                                self_limbs,
                                false,
                                b_limbs,
                                false,
                                c_limbs,
                                true,
                            );
                        }
                    }
                    assert!(!self_sign);
                }
            }
            self.trim();
        }
    }
}

// expecting x and y both with non-zero high limbs
fn mpn_cmp_twosizes_lt(x: &[u32], y: &[u32]) -> bool {
    mpn_cmp_helper(x, y) == Ordering::Less
}

fn mpz_aorsmul(
    w_sign: &mut bool,
    w: &mut Vec<u32>,
    x_sign: bool,
    x: &[u32],
    y_sign: bool,
    y: &[u32],
    mut sub: bool,
) {
    // make x the bigger of the two
    let (x, y) = if y.len() > x.len() { (y, x) } else { (x, y) };
    let xsize = x.len();
    let ysize = y.len();

    // w unaffected if x == 0 or y == 0
    if ysize == 0 {
        return;
    }
    sub ^= y_sign;
    // use mpn_addmul_1/mpn_submul_1 if possible
    if y_sign && ysize == 1 {
        mpz_aorsmul_1(w_sign, w, x_sign, x, y[0], sub);
        return;
    }
    sub ^= x_sign;
    sub ^= *w_sign;
    let wsize = w.len();
    let mut tsize = xsize + ysize;
    w.resize(max(wsize, tsize) + 1, 0);

    if wsize == 0 {
        // Nothing to add to, just set w=x*y.  No w==x or w==y overlap here, since we know x,y != 0
        // but w == 0.
        let high = mpn_mul(w, &x[0..xsize], &y[0..ysize]);
        if high == 0 {
            tsize -= 1;
        }
        *w_sign = sub || tsize == 0;
        return;
    }

    let mut t = vec![0; tsize];
    let high = mpn_mul(&mut t, &x[0..xsize], &y[0..ysize]);
    if high == 0 {
        tsize -= 1;
    }
    assert_ne!(t[tsize - 1], 0);
    if sub {
        if wsize < tsize {
            let c = if mpn_add_in_place(&mut w[0..tsize], &t[0..tsize]) {
                1
            } else {
                0
            };
            w[tsize] = c;
        } else {
            let c = if mpn_add_in_place(&mut w[0..wsize], &t[0..tsize]) {
                1
            } else {
                0
            };
            w[wsize] = c;
        }
    } else {
        if mpn_cmp_twosizes_lt(&w[0..wsize], &t[0..tsize]) {
            if tsize != 0 {
                *w_sign = !*w_sign;
            }
            assert!(!mpn_sub_aba(w, &t[0..tsize], wsize));
        } else {
            assert!(!mpn_sub_in_place(&mut w[0..wsize], &t[0..tsize]));
        }
    }
}