use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use core::cmp::Ordering;

impl PartialOrdAbs<Natural> for Integer {
    /// Compares the absolute values of an [`Integer`] and a [`Natural`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Integer::from(123).gt_abs(&Natural::from(122u32)));
    /// assert!(Integer::from(123).ge_abs(&Natural::from(122u32)));
    /// assert!(Integer::from(123).lt_abs(&Natural::from(124u32)));
    /// assert!(Integer::from(123).le_abs(&Natural::from(124u32)));
    /// assert!(Integer::from(-124).gt_abs(&Natural::from(123u32)));
    /// assert!(Integer::from(-124).ge_abs(&Natural::from(123u32)));
    /// ```
    fn partial_cmp_abs(&self, other: &Natural) -> Option<Ordering> {
        self.abs.partial_cmp(other)
    }
}

impl PartialOrdAbs<Integer> for Natural {
    /// Compares the absolute values of a [`Natural`] and an [`Integer`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::comparison::traits::PartialOrdAbs;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(123u32).gt_abs(&Integer::from(122)));
    /// assert!(Natural::from(123u32).ge_abs(&Integer::from(122)));
    /// assert!(Natural::from(123u32).lt_abs(&Integer::from(124)));
    /// assert!(Natural::from(123u32).le_abs(&Integer::from(124)));
    /// assert!(Natural::from(123u32).lt_abs(&Integer::from(-124)));
    /// assert!(Natural::from(123u32).le_abs(&Integer::from(-124)));
    /// ```
    fn partial_cmp_abs(&self, other: &Integer) -> Option<Ordering> {
        self.partial_cmp(&other.abs)
    }
}
