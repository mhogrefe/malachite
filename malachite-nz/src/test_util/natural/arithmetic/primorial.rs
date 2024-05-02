// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::Primes;

pub fn primorial_naive(n: u64) -> Natural {
    Natural::primes_less_than_or_equal_to(&Natural::from(n)).product()
}

pub fn product_of_first_n_primes_naive(n: u64) -> Natural {
    Natural::primes().take(usize::exact_from(n)).product()
}
