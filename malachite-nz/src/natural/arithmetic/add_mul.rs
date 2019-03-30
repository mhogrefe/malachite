use integer::arithmetic::add_mul_limb::mpz_aorsmul_1;
use malachite_base::limbs::limbs_test_zero;
use malachite_base::num::{AddMul, AddMulAssign, NotAssign};
use natural::arithmetic::add::limbs_slice_add_greater_in_place_left;
use natural::arithmetic::mul::limbs_mul_greater_to_out;
use natural::arithmetic::sub::{limbs_sub_in_place_left, limbs_sub_same_length_in_place_right};
use natural::arithmetic::sub_limb::limbs_sub_limb_to_out;
use natural::comparison::ord::limbs_cmp;
use natural::Natural::{self, Large, Small};
use platform::Limb;
use std::cmp::{max, Ordering};

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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(10u32).add_mul(Natural::from(3u32), Natural::from(4u32)), 22);
///     assert_eq!(Natural::trillion().add_mul(Natural::from(0x1_0000u32),
///         Natural::trillion()).to_string(), "65537000000000000");
/// }
/// ```
impl<'a> AddMul<Natural, Natural> for Natural {
    type Output = Natural;

    #[inline]
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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(10u32).add_mul(Natural::from(3u32), &Natural::from(4u32)), 22);
///     assert_eq!(Natural::trillion().add_mul(Natural::from(0x1_0000u32),
///         &Natural::trillion()).to_string(), "65537000000000000");
/// }
/// ```
impl<'a> AddMul<Natural, &'a Natural> for Natural {
    type Output = Natural;

    #[inline]
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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(10u32).add_mul(&Natural::from(3u32), Natural::from(4u32)), 22);
///     assert_eq!(Natural::trillion().add_mul(&Natural::from(0x1_0000u32),
///         Natural::trillion()).to_string(), "65537000000000000");
/// }
/// ```
impl<'a> AddMul<&'a Natural, Natural> for Natural {
    type Output = Natural;

    #[inline]
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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(10u32).add_mul(&Natural::from(3u32), &Natural::from(4u32)), 22);
///     assert_eq!(Natural::trillion().add_mul(&Natural::from(0x1_0000u32),
///         &Natural::trillion()).to_string(), "65537000000000000");
/// }
/// ```
impl<'a, 'b> AddMul<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    #[inline]
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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::from(10u32)).add_mul(&Natural::from(3u32), &Natural::from(4u32)), 22);
///     assert_eq!((&Natural::trillion()).add_mul(&Natural::from(0x1_0000u32),
///         &Natural::trillion()).to_string(), "65537000000000000");
/// }
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
                let mut result_limbs = self.to_limbs_asc();
                if let Large(ref c_limbs) = *c {
                    let mut self_sign = false;
                    if let Large(ref b_limbs) = *b {
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
                    assert!(!self_sign);
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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMulAssign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(10u32);
///     x.add_mul_assign(Natural::from(3u32), Natural::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = Natural::trillion();
///     x.add_mul_assign(Natural::from(0x1_0000u32), Natural::trillion());
///     assert_eq!(x.to_string(), "65537000000000000");
/// }
/// ```
impl AddMulAssign<Natural, Natural> for Natural {
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
                    if let Large(ref b_limbs) = b {
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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMulAssign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(10u32);
///     x.add_mul_assign(Natural::from(3u32), &Natural::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = Natural::trillion();
///     x.add_mul_assign(Natural::from(0x1_0000u32), &Natural::trillion());
///     assert_eq!(x.to_string(), "65537000000000000");
/// }
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
                if let Large(ref c_limbs) = *c {
                    let mut self_sign = false;
                    if let Large(ref b_limbs) = b {
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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMulAssign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(10u32);
///     x.add_mul_assign(&Natural::from(3u32), Natural::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = Natural::trillion();
///     x.add_mul_assign(&Natural::from(0x1_0000u32), Natural::trillion());
///     assert_eq!(x.to_string(), "65537000000000000");
/// }
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
                    if let Large(ref b_limbs) = *b {
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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMulAssign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(10u32);
///     x.add_mul_assign(&Natural::from(3u32), &Natural::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = Natural::trillion();
///     x.add_mul_assign(&Natural::from(0x1_0000u32), &Natural::trillion());
///     assert_eq!(x.to_string(), "65537000000000000");
/// }
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
                if let Large(ref c_limbs) = *c {
                    let mut self_sign = false;
                    if let Large(ref b_limbs) = *b {
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
                    assert!(!self_sign);
                }
            }
            self.trim();
        }
    }
}

// expecting x and y both with non-zero high limbs
fn mpn_cmp_twosizes_lt(x: &[Limb], y: &[Limb]) -> bool {
    limbs_cmp(x, y) == Ordering::Less
}

fn mpn_sub_aba(a: &mut [Limb], b: &[Limb], len: usize) -> bool {
    let s1_len = b.len();
    assert!(s1_len >= len);
    assert!(a.len() >= s1_len);
    let borrow = limbs_sub_same_length_in_place_right(&b[..len], &mut a[..len]);
    if s1_len == len {
        borrow
    } else if borrow {
        limbs_sub_limb_to_out(&mut a[len..], &b[len..], 1)
    } else {
        a[len..s1_len].copy_from_slice(&b[len..]);
        false
    }
}

#[allow(unknown_lints, clippy::many_single_char_names)]
pub(crate) fn mpz_aorsmul(
    w_sign: &mut bool,
    w: &mut Vec<Limb>,
    x_sign: bool,
    x: &[Limb],
    y_sign: bool,
    y: &[Limb],
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
        // Nothing to add to, just set w=x*y. No w==x or w==y overlap here, since we know x,y != 0
        // but w == 0.
        let high = limbs_mul_greater_to_out(w, x, y);
        if high == 0 {
            tsize -= 1;
        }
        *w_sign = !sub && tsize != 0;
        return;
    }

    let mut t = vec![0; tsize];
    let high = limbs_mul_greater_to_out(&mut t, x, y);
    if high == 0 {
        tsize -= 1;
    }
    assert_ne!(t[tsize - 1], 0);
    if sub {
        if wsize < tsize {
            let c = if limbs_slice_add_greater_in_place_left(&mut w[..tsize], &t[..tsize]) {
                1
            } else {
                0
            };
            w[tsize] = c;
        } else {
            let c = if limbs_slice_add_greater_in_place_left(&mut w[..wsize], &t[..tsize]) {
                1
            } else {
                0
            };
            w[wsize] = c;
        }
    } else if mpn_cmp_twosizes_lt(&w[..wsize], &t[..tsize]) {
        if tsize != 0 {
            w_sign.not_assign();
        }
        assert!(!mpn_sub_aba(w, &t[..tsize], wsize));
    } else {
        assert!(!limbs_sub_in_place_left(&mut w[..wsize], &t[..tsize]));
    }
    if limbs_test_zero(w) {
        *w_sign = false;
    }
}
