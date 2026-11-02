// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    arithmetic::register(runner);
    comparison::register(runner);
    conversion::register(runner);
    exponential::register(runner);
    factorization::register(runner);
    float::register(runner);
    logic::register(runner);
    modular::register(runner);
    number_theory::register(runner);
}

mod arithmetic;
mod comparison;
mod conversion;
mod exponential;
mod factorization;
mod float;
mod logic;
mod modular;
mod number_theory;
