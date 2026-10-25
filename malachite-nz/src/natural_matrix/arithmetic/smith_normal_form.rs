// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::num::arithmetic::traits::{SmithNormalForm, SmithNormalFormAssign};

// The Smith normal form of `a`, whose diagonal elements are nonnegative.
fn snf_helper(a: IntegerMatrix) -> NaturalMatrix {
    NaturalMatrix::try_from(a.smith_normal_form()).unwrap()
}

impl SmithNormalForm for NaturalMatrix {
    type Output = Self;

    /// Computes the Smith normal form of a [`NaturalMatrix`], taking it by value.
    ///
    /// The Smith normal form is described in
    /// [`smith_normal_form`](malachite_base::num::arithmetic::traits::SmithNormalForm) for an
    /// [`IntegerMatrix`]; its elements are nonnegative, so it is a [`NaturalMatrix`].
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `smith_normal_form` for an [`IntegerMatrix`] with the same
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SmithNormalForm;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.smith_normal_form().to_string(), "[1, 0; 0, 2]");
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 0, 3]").unwrap();
    /// assert_eq!(a.smith_normal_form().to_string(), "[1, 0; 0, 6]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(
    ///     a.smith_normal_form().to_string(),
    ///     "[1, 0, 0; 0, 3, 0; 0, 0, 0]"
    /// );
    /// ```
    #[inline]
    fn smith_normal_form(self) -> Self {
        snf_helper(IntegerMatrix::from(self))
    }
}

impl SmithNormalForm for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the Smith normal form of a [`NaturalMatrix`], taking it by reference.
    ///
    /// The Smith normal form is described in
    /// [`smith_normal_form`](malachite_base::num::arithmetic::traits::SmithNormalForm) for an
    /// [`IntegerMatrix`]; its elements are nonnegative, so it is a [`NaturalMatrix`].
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `smith_normal_form` for an [`IntegerMatrix`] with the same
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SmithNormalForm;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).smith_normal_form().to_string(), "[1, 0; 0, 2]");
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 0, 3]").unwrap();
    /// assert_eq!((&a).smith_normal_form().to_string(), "[1, 0; 0, 6]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(
    ///     (&a).smith_normal_form().to_string(),
    ///     "[1, 0, 0; 0, 3, 0; 0, 0, 0]"
    /// );
    /// ```
    #[inline]
    fn smith_normal_form(self) -> NaturalMatrix {
        snf_helper(IntegerMatrix::from(self))
    }
}

impl SmithNormalFormAssign for NaturalMatrix {
    /// Replaces a [`NaturalMatrix`] with its Smith normal form.
    ///
    /// The Smith normal form is described in
    /// [`smith_normal_form`](malachite_base::num::arithmetic::traits::SmithNormalForm) for an
    /// [`IntegerMatrix`]; its elements are nonnegative.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `smith_normal_form` for an [`IntegerMatrix`] with the same
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SmithNormalFormAssign;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.smith_normal_form_assign();
    /// assert_eq!(a.to_string(), "[1, 0; 0, 2]");
    ///
    /// let mut a = NaturalMatrix::from_str("[2, 0; 0, 3]").unwrap();
    /// a.smith_normal_form_assign();
    /// assert_eq!(a.to_string(), "[1, 0; 0, 6]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// a.smith_normal_form_assign();
    /// assert_eq!(a.to_string(), "[1, 0, 0; 0, 3, 0; 0, 0, 0]");
    /// ```
    #[inline]
    fn smith_normal_form_assign(&mut self) {
        *self = snf_helper(IntegerMatrix::from(&*self));
    }
}
