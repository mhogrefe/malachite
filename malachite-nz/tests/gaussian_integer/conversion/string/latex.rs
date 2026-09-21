// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ImaginaryFrom;
use malachite_base::strings::latex::ToLatex;
use malachite_nz::gaussian_integer::{
    ComparableGaussianInteger, ComparableGaussianIntegerRef, GaussianInteger,
};
use malachite_nz::test_util::generators::gaussian_integer_gen;

#[test]
fn test_gaussian_integer_to_latex() {
    // Every shape `Display` distinguishes, since the fragment is what it gives.
    assert_eq!(GaussianInteger::default().to_latex_string(), "0");
    assert_eq!(GaussianInteger::from(2).to_latex_string(), "2");
    assert_eq!(GaussianInteger::from(-2).to_latex_string(), "-2");
    assert_eq!(GaussianInteger::imaginary_from(1).to_latex_string(), "i");
    assert_eq!(GaussianInteger::imaginary_from(-1).to_latex_string(), "-i");
    assert_eq!(GaussianInteger::imaginary_from(2).to_latex_string(), "2i");
    assert_eq!(GaussianInteger::imaginary_from(-2).to_latex_string(), "-2i");
}

#[test]
fn gaussian_integer_to_latex_properties() {
    gaussian_integer_gen().test_properties(|x| {
        // The fragment is the value as `Display` writes it, which is already how a Gaussian integer
        // is written in mathematics.
        assert_eq!(x.to_latex_string(), x.to_string());
        // The wrappers exist to give an ordering, and do not change what the value is.
        assert_eq!(
            ComparableGaussianInteger(x.clone()).to_latex_string(),
            x.to_latex_string()
        );
        assert_eq!(
            ComparableGaussianIntegerRef(&x).to_latex_string(),
            x.to_latex_string()
        );
    });
}
