use std::cmp::Ordering;

/// This trait defines equality between the absolute values of `self` and a value of another type.
pub trait EqAbs<Rhs: ?Sized = Self> {
    /// Compares the absolute values of `self` and `other` for equality, taking both by reference.
    fn eq_abs(&self, other: &Rhs) -> bool;

    /// Determines whether $|x| \neq |y|$.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `eq_abs`.
    #[inline]
    fn ne_abs(&self, other: &Rhs) -> bool {
        !self.eq_abs(other)
    }
}

/// This trait defines a comparison between the absolute values of `self` and a value of another
/// type, where some pairs of elements may not be comparable.
pub trait PartialOrdAbs<Rhs: ?Sized = Self> {
    /// Compares the absolute values of `self` and `other`, taking both by reference.
    ///
    /// If the two values are not comparable, `None` is returned.
    fn partial_cmp_abs(&self, other: &Rhs) -> Option<Ordering>;

    /// Determines whether $|x| < |y|$.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `partial_cmp_abs`.
    #[inline]
    fn lt_abs(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp_abs(other), Some(Ordering::Less))
    }

    /// Determines whether $|x| \leq |y|$.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `partial_cmp_abs`.
    #[inline]
    fn le_abs(&self, other: &Rhs) -> bool {
        matches!(
            self.partial_cmp_abs(other),
            Some(Ordering::Less) | Some(Ordering::Equal)
        )
    }

    /// Determines whether $|x| > |y|$.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `partial_cmp_abs`.
    #[inline]
    fn gt_abs(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp_abs(other), Some(Ordering::Greater))
    }

    /// Determines whether $|x| \geq |y|$.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `partial_cmp_abs`.
    #[inline]
    fn ge_abs(&self, other: &Rhs) -> bool {
        matches!(
            self.partial_cmp_abs(other),
            Some(Ordering::Greater) | Some(Ordering::Equal)
        )
    }
}

/// This trait defines a comparison between the absolute values of `self` and a value of the same
/// type.
pub trait OrdAbs: Eq + PartialOrdAbs<Self> {
    /// Compare the absolute values of `self` and `other`, taking both by reference.
    fn cmp_abs(&self, other: &Self) -> Ordering;
}
