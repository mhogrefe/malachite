// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_howell_form::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

/// The Howell form of `a` modulo `m`, computed from the definition by enumerating the row span, as
/// for an `UnsignedMatrix<u64>`. `m` must fit in a `u64`, and the computation is only practical for
/// small matrices and moduli.
pub fn natural_matrix_mod_howell_form_naive(a: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
    let a = UnsignedMatrix::<u64>::try_from(a).unwrap();
    NaturalMatrix::from(unsigned_matrix_mod_howell_form_naive(
        &a,
        u64::exact_from(m),
    ))
}
