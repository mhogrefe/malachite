use malachite_base::num::traits::{
    CheckedSubMul, SplitInHalf, SubMul, SubMulAssign, WrappingAddAssign,
};

use natural::arithmetic::sub_mul::sub_mul_panic;
use natural::Natural;
use platform::{DoubleLimb, Limb};

// Multiply ys and limb, and subtract the ys.len() least significant limbs of the product from xs
// and write the result to xs. Return the most significant limb of the product, plus borrow-out from
// the subtraction. xs.len() >= ys.len().
pub fn limbs_sub_mul_limb_greater_in_place_left(xs: &mut [Limb], ys: &[Limb], limb: Limb) -> Limb {
    let ys_len = ys.len();
    assert!(xs.len() >= ys_len);
    let mut borrow = 0;
    let double_limb = DoubleLimb::from(limb);
    for i in 0..ys_len {
        let product = DoubleLimb::from(ys[i]) * double_limb;
        let (upper, mut lower) = product.split_in_half();
        lower.wrapping_add_assign(borrow);
        if lower < borrow {
            borrow = upper.wrapping_add(1);
        } else {
            borrow = upper;
        }
        let limb = xs[i];
        lower = limb.wrapping_sub(lower);
        if lower > limb {
            borrow.wrapping_add_assign(1);
        }
        xs[i] = lower;
    }
    borrow
}

impl SubMul<Natural, Limb> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` and b by value.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::from(15u32).sub_mul(Natural::from(3u32), 4).to_string(), "3");
    ///     assert_eq!(Natural::trillion().sub_mul(Natural::from(0x1_0000u32),
    ///         0x1_0000u32).to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: Natural, c: Limb) -> Natural {
        self.checked_sub_mul(b, c).expect("Cannot perform sub_mul")
    }
}

#[cfg(feature = "64_bit_limbs")]
impl SubMul<Natural, u32> for Natural {
    type Output = Natural;

    #[inline]
    fn sub_mul(self, b: Natural, c: u32) -> Natural {
        self.sub_mul(b, Limb::from(c))
    }
}

impl<'a> SubMul<&'a Natural, Limb> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` by value and b by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::from(15u32).sub_mul(&Natural::from(3u32), 4).to_string(), "3");
    ///     assert_eq!(Natural::trillion().sub_mul(&Natural::from(0x1_0000u32),
    ///         0x1_0000u32).to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: &'a Natural, c: Limb) -> Natural {
        self.checked_sub_mul(b, c).expect("Cannot perform sub_mul")
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SubMul<&'a Natural, u32> for Natural {
    type Output = Natural;

    #[inline]
    fn sub_mul(self, b: &'a Natural, c: u32) -> Natural {
        self.sub_mul(b, Limb::from(c))
    }
}

impl<'a> SubMul<Natural, Limb> for &'a Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` by reference and b by value.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!((&Natural::from(15u32)).sub_mul(Natural::from(3u32), 4).to_string(), "3");
    ///     assert_eq!((&Natural::trillion()).sub_mul(Natural::from(0x1_0000u32),
    ///         0x1_0000u32).to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: Natural, c: Limb) -> Natural {
        self.checked_sub_mul(b, c).expect("Cannot perform sub_mul")
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SubMul<Natural, u32> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn sub_mul(self, b: Natural, c: u32) -> Natural {
        self.sub_mul(b, Limb::from(c))
    }
}

impl<'a, 'b> SubMul<&'a Natural, Limb> for &'b Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` and b by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!((&Natural::from(15u32)).sub_mul(&Natural::from(3u32), 4).to_string(), "3");
    ///     assert_eq!((&Natural::trillion()).sub_mul(&Natural::from(0x1_0000u32),
    ///         0x1_0000u32).to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: &'a Natural, c: Limb) -> Natural {
        self.checked_sub_mul(b, c).unwrap_or_else(|| {
            sub_mul_panic(self, b, c);
        })
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a, 'b> SubMul<&'a Natural, u32> for &'b Natural {
    type Output = Natural;

    #[inline]
    fn sub_mul(self, b: &'a Natural, c: u32) -> Natural {
        self.sub_mul(b, Limb::from(c))
    }
}

impl SubMulAssign<Natural, Limb> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), in place,
    /// taking b by value.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::SubMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = Natural::from(15u32);
    ///     x.sub_mul_assign(Natural::from(3u32), 4);
    ///     assert_eq!(x, 3);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.sub_mul_assign(Natural::from(0x1_0000u32), 0x1_0000u32);
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul_assign(&mut self, b: Natural, c: Limb) {
        if self.sub_mul_assign_limb_no_panic(&b, c) {
            panic!("Natural sub_mul_assign cannot have a negative result");
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl SubMulAssign<Natural, u32> for Natural {
    #[inline]
    fn sub_mul_assign(&mut self, b: Natural, c: u32) {
        self.sub_mul_assign(b, Limb::from(c));
    }
}

impl<'a> SubMulAssign<&'a Natural, Limb> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), in place,
    /// taking b by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::SubMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = Natural::from(15u32);
    ///     x.sub_mul_assign(&Natural::from(3u32), 4);
    ///     assert_eq!(x, 3);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.sub_mul_assign(&Natural::from(0x1_0000u32), 0x1_0000u32);
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul_assign(&mut self, b: &'a Natural, c: Limb) {
        if self.sub_mul_assign_limb_no_panic(b, c) {
            panic!("Natural sub_mul_assign cannot have a negative result");
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SubMulAssign<&'a Natural, u32> for Natural {
    #[inline]
    fn sub_mul_assign(&mut self, b: &'a Natural, c: u32) {
        self.sub_mul_assign(b, Limb::from(c));
    }
}
