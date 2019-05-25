use std::fmt::Display;

use malachite_base::num::traits::{CheckedSubMul, SubMul, SubMulAssign};

use natural::Natural;

pub(crate) fn sub_mul_panic<S: Display, T: Display, U: Display>(a: S, b: T, c: U) -> ! {
    panic!("Cannot perform sub_mul. a: {}, b: {}, c: {}", a, b, c);
}

impl SubMul<Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self`, b, and c by value.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
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
    ///     assert_eq!(
    ///         Natural::from(20u32).sub_mul(Natural::from(3u32), Natural::from(4u32))
    ///         .to_string(), "8");
    ///     assert_eq!(Natural::trillion().sub_mul(
    ///         Natural::from(0x1_0000u32), Natural::from(0x1_0000u32)).to_string(),
    ///         "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: Natural, c: Natural) -> Natural {
        self.sub_mul(&b, &c)
    }
}

impl<'a> SubMul<Natural, &'a Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self` and b by value and c by reference.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
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
    ///     assert_eq!(
    ///         Natural::from(20u32).sub_mul(Natural::from(3u32), &Natural::from(4u32))
    ///         .to_string(), "8");
    ///     assert_eq!(Natural::trillion().sub_mul(
    ///         Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32)).to_string(),
    ///         "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: Natural, c: &'a Natural) -> Natural {
        self.sub_mul(&b, c)
    }
}

impl<'a> SubMul<&'a Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self` and c value and b by reference.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
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
    ///     assert_eq!(
    ///         Natural::from(20u32).sub_mul(&Natural::from(3u32), Natural::from(4u32))
    ///         .to_string(), "8");
    ///     assert_eq!(Natural::trillion().sub_mul(
    ///         &Natural::from(0x1_0000u32), Natural::from(0x1_0000u32)).to_string(),
    ///         "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: &'a Natural, c: Natural) -> Natural {
        self.sub_mul(b, &c)
    }
}

impl<'a, 'b> SubMul<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self` by value and b and c by reference.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
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
    ///     assert_eq!(
    ///         Natural::from(20u32).sub_mul(&Natural::from(3u32), &Natural::from(4u32))
    ///         .to_string(), "8");
    ///     assert_eq!(Natural::trillion().sub_mul(
    ///         &Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32)).to_string(),
    ///         "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: &'a Natural, c: &'b Natural) -> Natural {
        self.checked_sub_mul(b, c)
            .expect("Natural sub_mul_assign cannot have a negative result")
    }
}

impl<'a, 'b, 'c> SubMul<&'a Natural, &'b Natural> for &'c Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self`, b, and c by reference.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
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
    ///     assert_eq!(
    ///         (&Natural::from(20u32)).sub_mul(&Natural::from(3u32), &Natural::from(4u32))
    ///         .to_string(), "8");
    ///     assert_eq!((&Natural::trillion()).sub_mul(
    ///         &Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32)).to_string(),
    ///         "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: &'a Natural, c: &'b Natural) -> Natural {
        self.checked_sub_mul(b, c).unwrap_or_else(|| {
            sub_mul_panic(self, b, c);
        })
    }
}

impl SubMulAssign<Natural, Natural> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), in
    /// place, taking b and c by value.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
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
    ///     let mut x = Natural::from(20u32);
    ///     x.sub_mul_assign(Natural::from(3u32), Natural::from(4u32));
    ///     assert_eq!(x, 8);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.sub_mul_assign(Natural::from(0x1_0000u32), Natural::from(0x1_0000u32));
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul_assign(&mut self, b: Natural, c: Natural) {
        self.sub_mul_assign(&b, &c);
    }
}

impl<'a> SubMulAssign<Natural, &'a Natural> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), in
    /// place, taking b by value and c by reference.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
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
    ///     let mut x = Natural::from(20u32);
    ///     x.sub_mul_assign(Natural::from(3u32), &Natural::from(4u32));
    ///     assert_eq!(x, 8);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.sub_mul_assign(Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32));
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul_assign(&mut self, b: Natural, c: &'a Natural) {
        self.sub_mul_assign(&b, c);
    }
}

impl<'a> SubMulAssign<&'a Natural, Natural> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), in
    /// place, taking b by reference and c by value.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
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
    ///     let mut x = Natural::from(20u32);
    ///     x.sub_mul_assign(&Natural::from(3u32), Natural::from(4u32));
    ///     assert_eq!(x, 8);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.sub_mul_assign(&Natural::from(0x1_0000u32), Natural::from(0x1_0000u32));
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul_assign(&mut self, b: &'a Natural, c: Natural) {
        self.sub_mul_assign(b, &c);
    }
}

impl<'a, 'b> SubMulAssign<&'a Natural, &'b Natural> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), in
    /// place, taking b and c by reference.
    ///
    /// Time: TODO
    ///
    /// Additional memory: TODO
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
    ///     let mut x = Natural::from(20u32);
    ///     x.sub_mul_assign(&Natural::from(3u32), &Natural::from(4u32));
    ///     assert_eq!(x, 8);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.sub_mul_assign(&Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32));
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul_assign(&mut self, b: &'a Natural, c: &'b Natural) {
        if self.sub_mul_assign_no_panic(b, c) {
            panic!("Natural sub_mul_assign cannot have a negative result");
        }
    }
}
