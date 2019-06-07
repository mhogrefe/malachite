use std::mem::swap;

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};

use natural::arithmetic::add::{limbs_add_greater, limbs_slice_add_greater_in_place_left};
use natural::arithmetic::mul::limbs_mul_to_out;
use natural::Natural::{self, Large, Small};
use platform::Limb;

/// Given the limbs `xs`, `ys` and `zs` of three `Natural`s a, b, and c, returns the limbs of
/// a + b * c. `xs` should be nonempty and `ys` and `zs` should have length at least 2. None of the
/// slices should have any trailing zeros. The result will have no trailing zeros.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(m + n * log(n))
///
/// where n = max(`ys.len()`, `zs.len()`)
///       m = `xs.len()`
///
/// # Panics
/// Panics if `ys` or `zs` are empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul::limbs_add_mul;
///
/// assert_eq!(limbs_add_mul(&[123, 456], &[123, 789], &[321, 654]),
///         &[39606, 334167, 516006]);
/// assert_eq!(limbs_add_mul(&[123, 456, 789, 987, 654], &[123, 789], &[321, 654]),
///         &[39606, 334167, 516795, 987, 654]);
/// ```
///
/// This is mpz_aorsmul from mpz/aorsmul.c, where w, x, and y are positive, sub is positive, and w
/// is returned instead of overwriting the first input.
pub fn limbs_add_mul(xs: &[Limb], ys: &[Limb], zs: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let mut product_size = ys.len() + zs.len();
    let mut product = vec![0; product_size];
    if limbs_mul_to_out(&mut product, ys, zs) == 0 {
        product_size -= 1;
        product.pop();
    }
    assert_ne!(*product.last().unwrap(), 0);
    if xs_len >= product_size {
        limbs_add_greater(xs, &product)
    } else {
        if limbs_slice_add_greater_in_place_left(&mut product, xs) {
            product.push(1);
        }
        product
    }
}

/// Given the limbs `xs`, `ys` and `zs` of three `Natural`s a, b, and c, computes a + b * c. The
/// limbs of the result are written to `xs`. `xs` should be nonempty and `ys` and `zs` should have
/// length at least 2. None of the slices should have any trailing zeros. The result will have no
/// trailing zeros.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`ys.len()`, `zs.len()`)
///       m = `xs.len()`
///
/// # Panics
/// Panics if `ys` or `zs` are empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul::limbs_add_mul_in_place_left;
///
/// let mut xs = vec![123, 456];
/// limbs_add_mul_in_place_left(&mut xs, &[123, 789], &[321, 654]);
/// assert_eq!(xs, &[39606, 334167, 516006]);
///
/// let mut xs = vec![123, 456, 789, 987, 654];
/// limbs_add_mul_in_place_left(&mut xs, &[123, 789], &[321, 654]);
/// assert_eq!(xs, &[39606, 334167, 516795, 987, 654]);
/// ```
///
/// This is mpz_aorsmul from mpz/aorsmul.c, where w, x, and y are positive and sub is positive.
pub fn limbs_add_mul_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], zs: &[Limb]) {
    let xs_len = xs.len();
    let mut product_size = ys.len() + zs.len();
    let mut product = vec![0; product_size];
    if limbs_mul_to_out(&mut product, ys, zs) == 0 {
        product_size -= 1;
        product.pop();
    }
    assert_ne!(*product.last().unwrap(), 0);
    if xs_len < product_size {
        swap(xs, &mut product);
    }
    if limbs_slice_add_greater_in_place_left(xs, &product) {
        xs.push(1);
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), taking `self`, b,
/// and c by value.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMul;
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
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMul;
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
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMul;
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
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMul;
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
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(m + n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMul;
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
        if let Small(small_a) = *self {
            b * c + small_a
        } else if let Small(small_b) = *b {
            self.add_mul(c, small_b)
        } else if let Small(small_c) = *c {
            self.add_mul(b, small_c)
        } else {
            if let Large(ref a_limbs) = *self {
                if let Large(ref b_limbs) = *b {
                    if let Large(ref c_limbs) = *c {
                        return Large(limbs_add_mul(a_limbs, b_limbs, c_limbs));
                    }
                }
            }
            unreachable!();
        }
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b and c by value.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
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
        } else if *self == 0 as Limb {
            *self = b * c;
        } else {
            let self_limbs = self.promote_in_place();
            if let Large(ref b_limbs) = b {
                if let Large(ref c_limbs) = c {
                    limbs_add_mul_in_place_left(self_limbs, b_limbs, c_limbs);
                }
            }
        }
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b by value and c by reference.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
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
        } else if *self == 0 as Limb {
            *self = b * c;
        } else {
            let self_limbs = self.promote_in_place();
            if let Large(ref b_limbs) = b {
                if let Large(ref c_limbs) = *c {
                    limbs_add_mul_in_place_left(self_limbs, b_limbs, c_limbs);
                }
            }
        }
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b by reference and c by value.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
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
        } else if *self == 0 as Limb {
            *self = b * c;
        } else {
            let self_limbs = self.promote_in_place();
            if let Large(ref b_limbs) = *b {
                if let Large(ref c_limbs) = c {
                    limbs_add_mul_in_place_left(self_limbs, b_limbs, c_limbs);
                }
            }
        }
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b and c by reference.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
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
        } else if *self == 0 as Limb {
            *self = b * c;
        } else {
            let self_limbs = self.promote_in_place();
            if let Large(ref b_limbs) = *b {
                if let Large(ref c_limbs) = *c {
                    limbs_add_mul_in_place_left(self_limbs, b_limbs, c_limbs);
                }
            }
        }
    }
}
