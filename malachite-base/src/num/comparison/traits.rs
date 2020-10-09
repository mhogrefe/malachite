use std::cmp::Ordering;

/// This trait defines a comparison between the absolute values of `self` and a value of another
/// type, where some pairs of elements may not be comparable.
pub trait PartialOrdAbs<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    /// Compare the absolute values of `self` and `other`, taking both by reference. If the two
    /// values are not comparable, `None` is returned.
    fn partial_cmp_abs(&self, other: &Rhs) -> Option<Ordering>;

    /// Determines whether |`self`| < |`other`|.
    ///
    /// Time: worst case O(f(n)), where f(n) is the worst-case time complexity of
    ///     `Self::partial_cmp_abs`.
    ///
    /// Additional memory: worst case O(f(n)), where f(n) is the worst-case additional-memory
    ///     complexity of `Self::partial_cmp_abs`.
    ///
    #[inline]
    fn lt_abs(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp_abs(other), Some(Ordering::Less))
    }

    /// Determines whether |`self`| <= |`other`|.
    ///
    /// Time: worst case O(f(n)), where f(n) is the worst-case time complexity of
    ///     `Self::partial_cmp_abs`.
    ///
    /// Additional memory: worst case O(f(n)), where f(n) is the worst-case additional-memory
    ///     complexity of `Self::partial_cmp_abs`.
    ///
    #[inline]
    fn le_abs(&self, other: &Rhs) -> bool {
        matches!(
            self.partial_cmp_abs(other),
            Some(Ordering::Less) | Some(Ordering::Equal)
        )
    }

    /// Determines whether |`self`| > |`other`|.
    ///
    /// Time: worst case O(f(n)), where f(n) is the worst-case time complexity of
    ///     `Self::partial_cmp_abs`.
    ///
    /// Additional memory: worst case O(f(n)), where f(n) is the worst-case additional-memory
    ///     complexity of `Self::partial_cmp_abs`.
    ///
    #[inline]
    fn gt_abs(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp_abs(other), Some(Ordering::Greater))
    }

    /// Determines whether |`self`| >= |`other`|.
    ///
    /// Time: worst case O(f(n)), where f(n) is the worst-case time complexity of
    ///     `Self::partial_cmp_abs`.
    ///
    /// Additional memory: worst case O(f(n)), where f(n) is the worst-case additional-memory
    ///     complexity of `Self::partial_cmp_abs`.
    ///
    #[inline]
    fn ge_abs(&self, other: &Rhs) -> bool {
        matches!(
            self.partial_cmp_abs(other),
            Some(Ordering::Greater) | Some(Ordering::Equal)
        )
    }
}

/// This trait defines a comparison between the absolute values of `self` and a value of another
/// type.
pub trait OrdAbs: Eq + PartialOrdAbs<Self> {
    /// Compare the absolute values of `self` and `other`, taking both by reference.
    fn cmp_abs(&self, other: &Self) -> Ordering;
}
