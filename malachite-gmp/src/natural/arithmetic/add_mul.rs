use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::Natural::{self, Large, Small};
use std::mem;
use malachite_base::traits::{AddMul, AddMulAssign};

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), taking `self`, b,
/// and c by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMul;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(10u32).add_mul(Natural::from(3u32), Natural::from(4u32)), 22);
///     assert_eq!(Natural::trillion().add_mul(Natural::from(0x1_0000u32),
///         Natural::trillion()).to_string(), "65537000000000000");
/// }
/// ```
impl AddMul<Natural, Natural> for Natural {
    type Output = Natural;

    fn add_mul(mut self, b: Natural, c: Natural) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), taking `self` and
/// b by value and c by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMul;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(10u32).add_mul(Natural::from(3u32), &Natural::from(4u32)), 22);
///     assert_eq!(Natural::trillion().add_mul(Natural::from(0x1_0000u32),
///         &Natural::trillion()).to_string(), "65537000000000000");
/// }
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
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMul;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(10u32).add_mul(&Natural::from(3u32), Natural::from(4u32)), 22);
///     assert_eq!(Natural::trillion().add_mul(&Natural::from(0x1_0000u32),
///         Natural::trillion()).to_string(), "65537000000000000");
/// }
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
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMul;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(10u32).add_mul(&Natural::from(3u32), &Natural::from(4u32)), 22);
///     assert_eq!(Natural::trillion().add_mul(&Natural::from(0x1_0000u32),
///         &Natural::trillion()).to_string(), "65537000000000000");
/// }
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
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMul;
/// use malachite_gmp::natural::Natural;
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
        } else {
            unsafe {
                let mut result: mpz_t = mem::uninitialized();
                match *self {
                    Small(small) => gmp::mpz_init_set_ui(&mut result, small.into()),
                    Large(ref large) => gmp::mpz_init_set(&mut result, large),
                }
                if let Large(ref large_b) = *b {
                    if let &Large(ref large_c) = c {
                        gmp::mpz_addmul(&mut result, large_b, large_c)
                    } else {
                        unreachable!()
                    }
                }
                Large(result)
            }
        }
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b and c by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMulAssign;
/// use malachite_gmp::natural::Natural;
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
        self.add_mul_assign(&b, &c);
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b by value and c by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMulAssign;
/// use malachite_gmp::natural::Natural;
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
        self.add_mul_assign(&b, c);
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b by reference and c by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMulAssign;
/// use malachite_gmp::natural::Natural;
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
        self.add_mul_assign(b, &c);
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b and c by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMulAssign;
/// use malachite_gmp::natural::Natural;
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
        } else {
            let large_self = self.promote_in_place();
            unsafe {
                if let Large(ref large_b) = *b {
                    if let Large(ref large_c) = *c {
                        gmp::mpz_addmul(large_self, large_b, large_c)
                    }
                }
            }
        }
    }
}
