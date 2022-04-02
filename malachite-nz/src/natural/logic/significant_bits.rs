use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use natural::InnerNatural::{Large, Small};
use natural::Natural;

// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, returns the
// smallest number of bits necessary to represent that `Natural`. 0 has zero significant bits. When
// the `Natural` is nonzero, this is equal to 1 + floor(log<sub>2</sub>(`self`)).
//
// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
//
// Time: worst case O(1)
//
// Additional memory: worst case O(1)
//
// # Panics
// Panics if `xs` is empty.
//
// This is mpz_sizeinbase from mpz/sizeinbase.c, GMP 6.1.2, where x is non-negative and base is 2.
pub_crate_test! {limbs_significant_bits<T: PrimitiveUnsigned>(xs: &[T]) -> u64 {
    ((u64::wrapping_from(xs.len()) - 1) << T::LOG_WIDTH) + xs.last().unwrap().significant_bits()
}}

impl<'a> SignificantBits for &'a Natural {
    /// Returns the smallest number of bits necessary to represent a `Natural`. 0 has zero
    /// significant bits. When `self` is nonzero, this is equal to
    /// 1 + floor(log<sub>2</sub>(`self`)).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::SignificantBits;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.significant_bits(), 0);
    /// assert_eq!(Natural::from(100u32).significant_bits(), 7);
    /// ```
    fn significant_bits(self) -> u64 {
        match *self {
            Natural(Small(small)) => small.significant_bits(),
            Natural(Large(ref limbs)) => limbs_significant_bits(limbs),
        }
    }
}
