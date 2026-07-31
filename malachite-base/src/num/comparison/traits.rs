// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};

/// Determines equality between the absolute values of two numbers.
pub trait EqAbs<Rhs: ?Sized = Self> {
    /// Compares the absolute values of two numbers for equality, taking both by reference.
    fn eq_abs(&self, other: &Rhs) -> bool;

    /// Compares the absolute values of two numbers for inequality, taking both by reference.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of [`eq_abs`](Self::eq_abs).
    #[inline]
    fn ne_abs(&self, other: &Rhs) -> bool {
        !self.eq_abs(other)
    }
}

/// Determines equality between the absolute values of two numbers, where some pairs of numbers may
/// not be comparable.
pub trait PartialOrdAbs<Rhs: ?Sized = Self> {
    /// Compares the absolute values of two numbers, taking both by reference.
    ///
    /// If the two values are not comparable, `None` is returned.
    fn partial_cmp_abs(&self, other: &Rhs) -> Option<Ordering>;

    /// Determines whether the absolute value of one number is less than the absolute value of
    /// another.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of
    /// [`partial_cmp_abs`](Self::partial_cmp_abs).
    #[inline]
    fn lt_abs(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp_abs(other), Some(Less))
    }

    /// Determines whether the absolute value of one number is less than or equal to the absolute
    /// value of another.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of
    /// [`partial_cmp_abs`](Self::partial_cmp_abs).
    #[inline]
    fn le_abs(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp_abs(other), Some(Less | Equal))
    }

    /// Determines whether the absolute value of one number is greater than the absolute value of
    /// another.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of
    /// [`partial_cmp_abs`](Self::partial_cmp_abs).
    #[inline]
    fn gt_abs(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp_abs(other), Some(Greater))
    }

    /// Determines whether the absolute value of one number is greater than or equal to the absolute
    /// value of another.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of
    /// [`partial_cmp_abs`](Self::partial_cmp_abs).
    #[inline]
    fn ge_abs(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp_abs(other), Some(Greater | Equal))
    }
}

/// Compares a number with twice another number, without computing the doubled value.
///
/// If the two values are not comparable, `None` is returned.
pub trait PartialOrdDouble<Rhs: ?Sized = Self> {
    /// Compares a number with twice another number, taking both by reference.
    fn partial_cmp_double(&self, other: &Rhs) -> Option<Ordering>;
}

/// Compares the absolute value of a number with twice the absolute value of another number, without
/// computing the doubled value.
///
/// If the two values are not comparable, `None` is returned.
pub trait PartialOrdAbsDouble<Rhs: ?Sized = Self> {
    /// Compares the absolute value of a number with twice the absolute value of another number,
    /// taking both by reference.
    fn partial_cmp_abs_double(&self, other: &Rhs) -> Option<Ordering>;
}

/// Compares a number with twice another number, without computing the doubled value.
///
/// This is the shape of a round-to-nearest decision, where a remainder is weighed against half a
/// divisor, so it is worth doing without the allocation or the overflow that doubling would cost.
pub trait OrdDouble<Rhs: ?Sized = Self>: PartialOrdDouble<Rhs> {
    /// Compares a number with twice another number, taking both by reference.
    fn cmp_double(&self, other: &Rhs) -> Ordering;
}

/// Compares the absolute value of a number with twice the absolute value of another number, without
/// computing the doubled value.
///
/// This is the shape of a round-to-nearest decision, where a remainder is weighed against half a
/// divisor, so it is worth doing without the allocation or the overflow that doubling would cost.
pub trait OrdAbsDouble<Rhs: ?Sized = Self>: PartialOrdAbsDouble<Rhs> {
    /// Compares the absolute value of a number with twice the absolute value of another number,
    /// taking both by reference.
    fn cmp_abs_double(&self, other: &Rhs) -> Ordering;
}

/// Compares the absolute values of two numbers.
pub trait OrdAbs: Eq + PartialOrdAbs<Self> {
    fn cmp_abs(&self, other: &Self) -> Ordering;
}
