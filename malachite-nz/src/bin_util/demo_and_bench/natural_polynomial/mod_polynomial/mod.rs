// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    mod_compose_mod_polynomial::register(runner);
    mod_compose_mod_polynomial_precomputed::register(runner);
    mod_compose_mod_polynomial_precomputed_powers::register(runner);
    mod_mul_mod_polynomial::register(runner);
    mod_mul_mod_polynomial_precomputed::register(runner);
    mod_pow_mod_polynomial::register(runner);
    mod_pow_mod_polynomial_precomputed::register(runner);
    mod_power_of_2_compose_mod_polynomial::register(runner);
    mod_power_of_2_compose_mod_polynomial_precomputed::register(runner);
    mod_power_of_2_compose_mod_polynomial_precomputed_powers::register(runner);
    mod_power_of_2_mul_mod_polynomial::register(runner);
    mod_power_of_2_mul_mod_polynomial_precomputed::register(runner);
    mod_power_of_2_pow_mod_polynomial::register(runner);
    mod_power_of_2_pow_mod_polynomial_precomputed::register(runner);
}

mod mod_compose_mod_polynomial;
mod mod_compose_mod_polynomial_precomputed;
mod mod_compose_mod_polynomial_precomputed_powers;
mod mod_mul_mod_polynomial;
mod mod_mul_mod_polynomial_precomputed;
mod mod_pow_mod_polynomial;
mod mod_pow_mod_polynomial_precomputed;
mod mod_power_of_2_compose_mod_polynomial;
mod mod_power_of_2_compose_mod_polynomial_precomputed;
mod mod_power_of_2_compose_mod_polynomial_precomputed_powers;
mod mod_power_of_2_mul_mod_polynomial;
mod mod_power_of_2_mul_mod_polynomial_precomputed;
mod mod_power_of_2_pow_mod_polynomial;
mod mod_power_of_2_pow_mod_polynomial_precomputed;
