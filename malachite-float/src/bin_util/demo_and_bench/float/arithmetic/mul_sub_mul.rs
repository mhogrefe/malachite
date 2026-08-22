// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::MulSubMul;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloat;
use malachite_float::test_util::bench::bucketers::{
    pair_2_sextuple_1_2_3_4_float_max_complexity_bucketer,
    sextuple_1_2_3_4_float_float_float_rational_max_complexity_bucketer,
    sextuple_1_2_3_4_float_max_complexity_bucketer,
};
use malachite_float::test_util::float::arithmetic::mul_sub_mul::{
    mul_sub_mul_prec_round_naive, mul_sub_mul_rational_prec_round_naive, rug_mul_sub_mul_prec_round,
};
use malachite_float::test_util::generators::{
    float_float_float_float_rounding_mode_quintuple_gen_var_2,
    float_float_float_float_unsigned_quintuple_gen_var_1,
    float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3,
    float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3_rm,
    float_float_float_rational_quadruple_gen,
    float_float_float_rational_rounding_mode_quintuple_gen_var_2,
    float_float_float_rational_unsigned_quintuple_gen_var_1,
    float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3, float_quadruple_gen,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_mul_sub_mul_prec_round);
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_val_val_val_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_prec_round_val_val_val_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_val_val_ref_val);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_prec_round_val_val_ref_val_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_val_val_ref_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_prec_round_val_val_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_val_ref_val_val);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_prec_round_val_ref_val_val_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_val_ref_val_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_prec_round_val_ref_val_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_val_ref_ref_val);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_prec_round_val_ref_ref_val_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_val_ref_ref_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_prec_round_val_ref_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_ref_ref_ref_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_prec_round_ref_ref_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_assign);
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_assign_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_assign_val_val_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_prec_round_assign_val_val_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_assign_val_ref_val);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_prec_round_assign_val_ref_val_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_assign_val_ref_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_prec_round_assign_val_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_assign_ref_val_val);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_prec_round_assign_ref_val_val_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_assign_ref_val_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_prec_round_assign_ref_val_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_assign_ref_ref_val);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_prec_round_assign_ref_ref_val_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_prec_round_assign_ref_ref_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_prec_round_assign_ref_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_prec);
    register_demo!(runner, demo_float_mul_sub_mul_prec_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_val_val_val_ref);
    register_demo!(runner, demo_float_mul_sub_mul_prec_val_val_val_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_val_val_ref_val);
    register_demo!(runner, demo_float_mul_sub_mul_prec_val_val_ref_val_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_val_val_ref_ref);
    register_demo!(runner, demo_float_mul_sub_mul_prec_val_val_ref_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_val_ref_val_val);
    register_demo!(runner, demo_float_mul_sub_mul_prec_val_ref_val_val_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_val_ref_val_ref);
    register_demo!(runner, demo_float_mul_sub_mul_prec_val_ref_val_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_val_ref_ref_val);
    register_demo!(runner, demo_float_mul_sub_mul_prec_val_ref_ref_val_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_val_ref_ref_ref);
    register_demo!(runner, demo_float_mul_sub_mul_prec_val_ref_ref_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_ref_ref_ref_ref);
    register_demo!(runner, demo_float_mul_sub_mul_prec_ref_ref_ref_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign_val_val_ref);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign_val_val_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign_val_ref_val);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign_val_ref_val_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign_val_ref_ref);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign_val_ref_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign_ref_val_val);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign_ref_val_val_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign_ref_val_ref);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign_ref_val_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign_ref_ref_val);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign_ref_ref_val_debug);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign_ref_ref_ref);
    register_demo!(runner, demo_float_mul_sub_mul_prec_assign_ref_ref_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_round);
    register_demo!(runner, demo_float_mul_sub_mul_round_debug);
    register_demo!(runner, demo_float_mul_sub_mul_round_val_val_val_ref);
    register_demo!(runner, demo_float_mul_sub_mul_round_val_val_val_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_round_val_val_ref_val);
    register_demo!(runner, demo_float_mul_sub_mul_round_val_val_ref_val_debug);
    register_demo!(runner, demo_float_mul_sub_mul_round_val_val_ref_ref);
    register_demo!(runner, demo_float_mul_sub_mul_round_val_val_ref_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_round_val_ref_val_val);
    register_demo!(runner, demo_float_mul_sub_mul_round_val_ref_val_val_debug);
    register_demo!(runner, demo_float_mul_sub_mul_round_val_ref_val_ref);
    register_demo!(runner, demo_float_mul_sub_mul_round_val_ref_val_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_round_val_ref_ref_val);
    register_demo!(runner, demo_float_mul_sub_mul_round_val_ref_ref_val_debug);
    register_demo!(runner, demo_float_mul_sub_mul_round_val_ref_ref_ref);
    register_demo!(runner, demo_float_mul_sub_mul_round_val_ref_ref_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_round_ref_ref_ref_ref);
    register_demo!(runner, demo_float_mul_sub_mul_round_ref_ref_ref_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_round_assign);
    register_demo!(runner, demo_float_mul_sub_mul_round_assign_debug);
    register_demo!(runner, demo_float_mul_sub_mul_round_assign_val_val_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_round_assign_val_val_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_round_assign_val_ref_val);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_round_assign_val_ref_val_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_round_assign_val_ref_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_round_assign_val_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_round_assign_ref_val_val);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_round_assign_ref_val_val_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_round_assign_ref_val_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_round_assign_ref_val_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_round_assign_ref_ref_val);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_round_assign_ref_ref_val_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_round_assign_ref_ref_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_round_assign_ref_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul);
    register_demo!(runner, demo_float_mul_sub_mul_debug);
    register_demo!(runner, demo_float_mul_sub_mul_val_val_val_ref);
    register_demo!(runner, demo_float_mul_sub_mul_val_val_val_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_val_val_ref_val);
    register_demo!(runner, demo_float_mul_sub_mul_val_val_ref_val_debug);
    register_demo!(runner, demo_float_mul_sub_mul_val_val_ref_ref);
    register_demo!(runner, demo_float_mul_sub_mul_val_val_ref_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_val_ref_val_val);
    register_demo!(runner, demo_float_mul_sub_mul_val_ref_val_val_debug);
    register_demo!(runner, demo_float_mul_sub_mul_val_ref_val_ref);
    register_demo!(runner, demo_float_mul_sub_mul_val_ref_val_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_val_ref_ref_val);
    register_demo!(runner, demo_float_mul_sub_mul_val_ref_ref_val_debug);
    register_demo!(runner, demo_float_mul_sub_mul_val_ref_ref_ref);
    register_demo!(runner, demo_float_mul_sub_mul_val_ref_ref_ref_debug);
    register_demo!(runner, demo_float_mul_sub_mul_ref_ref_ref_ref);
    register_demo!(runner, demo_float_mul_sub_mul_ref_ref_ref_ref_debug);

    register_bench!(
        runner,
        benchmark_float_mul_sub_mul_prec_round_evaluation_strategy
    );
    register_bench!(runner, benchmark_float_mul_sub_mul_prec_round_algorithms);
    register_bench!(
        runner,
        benchmark_float_mul_sub_mul_prec_round_library_comparison
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_prec_round);
    register_demo!(runner, demo_float_mul_sub_mul_rational_prec_round_debug);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_val_val_val_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_val_val_val_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_val_val_ref_val
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_val_val_ref_val_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_val_val_ref_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_val_val_ref_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_val_ref_val_val
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_val_ref_val_val_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_val_ref_val_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_val_ref_val_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_val_ref_ref_val
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_val_ref_ref_val_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_val_ref_ref_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_val_ref_ref_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_ref_ref_ref_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_ref_ref_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_prec_round_assign);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_assign_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_assign_val_val_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_assign_val_val_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_assign_val_ref_val
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_assign_val_ref_val_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_assign_val_ref_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_assign_val_ref_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_assign_ref_val_val
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_assign_ref_val_val_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_assign_ref_val_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_assign_ref_val_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_assign_ref_ref_val
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_assign_ref_ref_val_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_assign_ref_ref_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_round_assign_ref_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_prec);
    register_demo!(runner, demo_float_mul_sub_mul_rational_prec_debug);
    register_demo!(runner, demo_float_mul_sub_mul_rational_prec_val_val_val_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_val_val_val_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_prec_val_val_ref_val);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_val_val_ref_val_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_prec_val_val_ref_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_val_val_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_prec_val_ref_val_val);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_val_ref_val_val_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_prec_val_ref_val_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_val_ref_val_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_prec_val_ref_ref_val);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_val_ref_ref_val_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_prec_val_ref_ref_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_val_ref_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_prec_ref_ref_ref_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_ref_ref_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_prec_assign);
    register_demo!(runner, demo_float_mul_sub_mul_rational_prec_assign_debug);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_assign_val_val_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_assign_val_val_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_assign_val_ref_val
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_assign_val_ref_val_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_assign_val_ref_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_assign_val_ref_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_assign_ref_val_val
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_assign_ref_val_val_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_assign_ref_val_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_assign_ref_val_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_assign_ref_ref_val
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_assign_ref_ref_val_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_assign_ref_ref_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_prec_assign_ref_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_round);
    register_demo!(runner, demo_float_mul_sub_mul_rational_round_debug);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_val_val_val_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_val_val_val_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_val_val_ref_val
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_val_val_ref_val_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_val_val_ref_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_val_val_ref_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_val_ref_val_val
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_val_ref_val_val_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_val_ref_val_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_val_ref_val_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_val_ref_ref_val
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_val_ref_ref_val_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_val_ref_ref_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_val_ref_ref_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_ref_ref_ref_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_ref_ref_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_round_assign);
    register_demo!(runner, demo_float_mul_sub_mul_rational_round_assign_debug);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_assign_val_val_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_assign_val_val_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_assign_val_ref_val
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_assign_val_ref_val_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_assign_val_ref_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_assign_val_ref_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_assign_ref_val_val
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_assign_ref_val_val_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_assign_ref_val_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_assign_ref_val_ref_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_assign_ref_ref_val
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_assign_ref_ref_val_debug
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_assign_ref_ref_ref
    );
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_round_assign_ref_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational);
    register_demo!(runner, demo_float_mul_sub_mul_rational_debug);
    register_demo!(runner, demo_float_mul_sub_mul_rational_val_val_val_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_val_val_val_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_val_val_ref_val);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_val_val_ref_val_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_val_val_ref_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_val_val_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_val_ref_val_val);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_val_ref_val_val_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_val_ref_val_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_val_ref_val_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_val_ref_ref_val);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_val_ref_ref_val_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_val_ref_ref_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_val_ref_ref_ref_debug
    );
    register_demo!(runner, demo_float_mul_sub_mul_rational_ref_ref_ref_ref);
    register_demo!(
        runner,
        demo_float_mul_sub_mul_rational_ref_ref_ref_ref_debug
    );
    register_bench!(
        runner,
        benchmark_float_mul_sub_mul_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_mul_sub_mul_rational_prec_round_algorithms
    );
}

fn demo_float_mul_sub_mul_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec_round({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone()
                .mul_sub_mul_prec_round(b.clone(), c.clone(), d.clone(), prec, rm)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_prec_round(b.clone(), c.clone(), d.clone(), prec, rm);
        println!(
            "({:#x}).mul_sub_mul_prec_round({:#x}, {:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_val_val_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec_round_val_val_val_ref({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone()
                .mul_sub_mul_prec_round_val_val_val_ref(b.clone(), c.clone(), &d, prec, rm)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_val_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res =
            a.clone()
                .mul_sub_mul_prec_round_val_val_val_ref(b.clone(), c.clone(), &d, prec, rm);
        println!(
            "({:#x}).mul_sub_mul_prec_round_val_val_val_ref({:#x}, {:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_val_val_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec_round_val_val_ref_val({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone()
                .mul_sub_mul_prec_round_val_val_ref_val(b.clone(), &c, d.clone(), prec, rm)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_val_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res =
            a.clone()
                .mul_sub_mul_prec_round_val_val_ref_val(b.clone(), &c, d.clone(), prec, rm);
        println!(
            "({:#x}).mul_sub_mul_prec_round_val_val_ref_val({:#x}, {:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_val_val_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec_round_val_val_ref_ref({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone()
                .mul_sub_mul_prec_round_val_val_ref_ref(b.clone(), &c, &d, prec, rm)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_val_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_prec_round_val_val_ref_ref(b.clone(), &c, &d, prec, rm);
        println!(
            "({:#x}).mul_sub_mul_prec_round_val_val_ref_ref({:#x}, {:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_val_ref_val_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec_round_val_ref_val_val({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone()
                .mul_sub_mul_prec_round_val_ref_val_val(&b, c.clone(), d.clone(), prec, rm)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_val_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res =
            a.clone()
                .mul_sub_mul_prec_round_val_ref_val_val(&b, c.clone(), d.clone(), prec, rm);
        println!(
            "({:#x}).mul_sub_mul_prec_round_val_ref_val_val({:#x}, {:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_val_ref_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec_round_val_ref_val_ref({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone()
                .mul_sub_mul_prec_round_val_ref_val_ref(&b, c.clone(), &d, prec, rm)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_val_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_prec_round_val_ref_val_ref(&b, c.clone(), &d, prec, rm);
        println!(
            "({:#x}).mul_sub_mul_prec_round_val_ref_val_ref({:#x}, {:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_val_ref_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec_round_val_ref_ref_val({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone()
                .mul_sub_mul_prec_round_val_ref_ref_val(&b, &c, d.clone(), prec, rm)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_val_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_prec_round_val_ref_ref_val(&b, &c, d.clone(), prec, rm);
        println!(
            "({:#x}).mul_sub_mul_prec_round_val_ref_ref_val({:#x}, {:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_val_ref_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec_round_val_ref_ref_ref({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone()
                .mul_sub_mul_prec_round_val_ref_ref_ref(&b, &c, &d, prec, rm)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_val_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_prec_round_val_ref_ref_ref(&b, &c, &d, prec, rm);
        println!(
            "({:#x}).mul_sub_mul_prec_round_val_ref_ref_ref({:#x}, {:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_ref_ref_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "(&{}).mul_sub_mul_prec_round_ref_ref_ref_ref({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.mul_sub_mul_prec_round_ref_ref_ref_ref(&b, &c, &d, prec, rm)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_ref_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res = a.mul_sub_mul_prec_round_ref_ref_ref_ref(&b, &c, &d, prec, rm);
        println!(
            "(&{:#x}).mul_sub_mul_prec_round_ref_ref_ref_ref({:#x}, {:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_round_assign(b.clone(), c.clone(), d.clone(), prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_round_assign({b}, {c}, {d}, {prec}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_round_assign(b.clone(), c.clone(), d.clone(), prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_round_assign({:#x}, {:#x}, {:#x}, {}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign_val_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_round_assign_val_val_ref(b.clone(), c.clone(), &d, prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_round_assign_val_val_ref({b}, {c}, {d}, {prec}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_round_assign_val_val_ref(b.clone(), c.clone(), &d, prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_round_assign_val_val_ref({:#x}, {:#x}, {:#x}, {}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign_val_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_round_assign_val_ref_val(b.clone(), &c, d.clone(), prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_round_assign_val_ref_val({b}, {c}, {d}, {prec}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_round_assign_val_ref_val(b.clone(), &c, d.clone(), prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_round_assign_val_ref_val({:#x}, {:#x}, {:#x}, {}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign_val_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_round_assign_val_ref_ref(b.clone(), &c, &d, prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_round_assign_val_ref_ref({b}, {c}, {d}, {prec}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_round_assign_val_ref_ref(b.clone(), &c, &d, prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_round_assign_val_ref_ref({:#x}, {:#x}, {:#x}, {}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign_ref_val_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_round_assign_ref_val_val(&b, c.clone(), d.clone(), prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_round_assign_ref_val_val({b}, {c}, {d}, {prec}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_round_assign_ref_val_val(&b, c.clone(), d.clone(), prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_round_assign_ref_val_val({:#x}, {:#x}, {:#x}, {}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign_ref_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_round_assign_ref_val_ref(&b, c.clone(), &d, prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_round_assign_ref_val_ref({b}, {c}, {d}, {prec}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_round_assign_ref_val_ref(&b, c.clone(), &d, prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_round_assign_ref_val_ref({:#x}, {:#x}, {:#x}, {}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign_ref_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_round_assign_ref_ref_val(&b, &c, d.clone(), prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_round_assign_ref_ref_val({b}, {c}, {d}, {prec}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_round_assign_ref_ref_val(&b, &c, d.clone(), prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_round_assign_ref_ref_val({:#x}, {:#x}, {:#x}, {}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign_ref_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_round_assign_ref_ref_ref(&b, &c, &d, prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_round_assign_ref_ref_ref({b}, {c}, {d}, {prec}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_round_assign_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_round_assign_ref_ref_ref(&b, &c, &d, prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_round_assign_ref_ref_ref({:#x}, {:#x}, {:#x}, {}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone()
                .mul_sub_mul_prec(b.clone(), c.clone(), d.clone(), prec)
        );
    }
}

fn demo_float_mul_sub_mul_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_prec(b.clone(), c.clone(), d.clone(), prec);
        println!(
            "({:#x}).mul_sub_mul_prec({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_val_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec_val_val_val_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone()
                .mul_sub_mul_prec_val_val_val_ref(b.clone(), c.clone(), &d, prec)
        );
    }
}

fn demo_float_mul_sub_mul_prec_val_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_prec_val_val_val_ref(b.clone(), c.clone(), &d, prec);
        println!(
            "({:#x}).mul_sub_mul_prec_val_val_val_ref({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_val_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec_val_val_ref_val({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone()
                .mul_sub_mul_prec_val_val_ref_val(b.clone(), &c, d.clone(), prec)
        );
    }
}

fn demo_float_mul_sub_mul_prec_val_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_prec_val_val_ref_val(b.clone(), &c, d.clone(), prec);
        println!(
            "({:#x}).mul_sub_mul_prec_val_val_ref_val({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_val_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec_val_val_ref_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone()
                .mul_sub_mul_prec_val_val_ref_ref(b.clone(), &c, &d, prec)
        );
    }
}

fn demo_float_mul_sub_mul_prec_val_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_prec_val_val_ref_ref(b.clone(), &c, &d, prec);
        println!(
            "({:#x}).mul_sub_mul_prec_val_val_ref_ref({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_val_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec_val_ref_val_val({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone()
                .mul_sub_mul_prec_val_ref_val_val(&b, c.clone(), d.clone(), prec)
        );
    }
}

fn demo_float_mul_sub_mul_prec_val_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_prec_val_ref_val_val(&b, c.clone(), d.clone(), prec);
        println!(
            "({:#x}).mul_sub_mul_prec_val_ref_val_val({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_val_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec_val_ref_val_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone()
                .mul_sub_mul_prec_val_ref_val_ref(&b, c.clone(), &d, prec)
        );
    }
}

fn demo_float_mul_sub_mul_prec_val_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_prec_val_ref_val_ref(&b, c.clone(), &d, prec);
        println!(
            "({:#x}).mul_sub_mul_prec_val_ref_val_ref({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_val_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec_val_ref_ref_val({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone()
                .mul_sub_mul_prec_val_ref_ref_val(&b, &c, d.clone(), prec)
        );
    }
}

fn demo_float_mul_sub_mul_prec_val_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_prec_val_ref_ref_val(&b, &c, d.clone(), prec);
        println!(
            "({:#x}).mul_sub_mul_prec_val_ref_ref_val({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_val_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_prec_val_ref_ref_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone().mul_sub_mul_prec_val_ref_ref_ref(&b, &c, &d, prec)
        );
    }
}

fn demo_float_mul_sub_mul_prec_val_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = a.clone().mul_sub_mul_prec_val_ref_ref_ref(&b, &c, &d, prec);
        println!(
            "({:#x}).mul_sub_mul_prec_val_ref_ref_ref({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_ref_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).mul_sub_mul_prec_ref_ref_ref_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.mul_sub_mul_prec_ref_ref_ref_ref(&b, &c, &d, prec)
        );
    }
}

fn demo_float_mul_sub_mul_prec_ref_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = a.mul_sub_mul_prec_ref_ref_ref_ref(&b, &c, &d, prec);
        println!(
            "(&{:#x}).mul_sub_mul_prec_ref_ref_ref_ref({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_assign(b.clone(), c.clone(), d.clone(), prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_assign({b}, {c}, {d}, {prec}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_assign(b.clone(), c.clone(), d.clone(), prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_assign({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_assign_val_val_ref(b.clone(), c.clone(), &d, prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_assign_val_val_ref({b}, {c}, {d}, {prec}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_assign_val_val_ref(b.clone(), c.clone(), &d, prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_assign_val_val_ref({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_assign_val_ref_val(b.clone(), &c, d.clone(), prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_assign_val_ref_val({b}, {c}, {d}, {prec}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_assign_val_ref_val(b.clone(), &c, d.clone(), prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_assign_val_ref_val({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_assign_val_ref_ref(b.clone(), &c, &d, prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_assign_val_ref_ref({b}, {c}, {d}, {prec}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_assign_val_ref_ref(b.clone(), &c, &d, prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_assign_val_ref_ref({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_assign_ref_val_val(&b, c.clone(), d.clone(), prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_assign_ref_val_val({b}, {c}, {d}, {prec}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_assign_ref_val_val(&b, c.clone(), d.clone(), prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_assign_ref_val_val({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_assign_ref_val_ref(&b, c.clone(), &d, prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_assign_ref_val_ref({b}, {c}, {d}, {prec}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_assign_ref_val_ref(&b, c.clone(), &d, prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_assign_ref_val_ref({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_assign_ref_ref_val(&b, &c, d.clone(), prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_assign_ref_ref_val({b}, {c}, {d}, {prec}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_assign_ref_ref_val(&b, &c, d.clone(), prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_assign_ref_ref_val({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_prec_assign_ref_ref_ref(&b, &c, &d, prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_prec_assign_ref_ref_ref({b}, {c}, {d}, {prec}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_prec_assign_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_float_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_prec_assign_ref_ref_ref(&b, &c, &d, prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_prec_assign_ref_ref_ref({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_round({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone()
                .mul_sub_mul_round(b.clone(), c.clone(), d.clone(), rm)
        );
    }
}

fn demo_float_mul_sub_mul_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_round(b.clone(), c.clone(), d.clone(), rm);
        println!(
            "({:#x}).mul_sub_mul_round({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_round_val_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_round_val_val_val_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone()
                .mul_sub_mul_round_val_val_val_ref(b.clone(), c.clone(), &d, rm)
        );
    }
}

fn demo_float_mul_sub_mul_round_val_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_round_val_val_val_ref(b.clone(), c.clone(), &d, rm);
        println!(
            "({:#x}).mul_sub_mul_round_val_val_val_ref({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_round_val_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_round_val_val_ref_val({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone()
                .mul_sub_mul_round_val_val_ref_val(b.clone(), &c, d.clone(), rm)
        );
    }
}

fn demo_float_mul_sub_mul_round_val_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_round_val_val_ref_val(b.clone(), &c, d.clone(), rm);
        println!(
            "({:#x}).mul_sub_mul_round_val_val_ref_val({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_round_val_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_round_val_val_ref_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone()
                .mul_sub_mul_round_val_val_ref_ref(b.clone(), &c, &d, rm)
        );
    }
}

fn demo_float_mul_sub_mul_round_val_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_round_val_val_ref_ref(b.clone(), &c, &d, rm);
        println!(
            "({:#x}).mul_sub_mul_round_val_val_ref_ref({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_round_val_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_round_val_ref_val_val({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone()
                .mul_sub_mul_round_val_ref_val_val(&b, c.clone(), d.clone(), rm)
        );
    }
}

fn demo_float_mul_sub_mul_round_val_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_round_val_ref_val_val(&b, c.clone(), d.clone(), rm);
        println!(
            "({:#x}).mul_sub_mul_round_val_ref_val_val({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_round_val_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_round_val_ref_val_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone()
                .mul_sub_mul_round_val_ref_val_ref(&b, c.clone(), &d, rm)
        );
    }
}

fn demo_float_mul_sub_mul_round_val_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_round_val_ref_val_ref(&b, c.clone(), &d, rm);
        println!(
            "({:#x}).mul_sub_mul_round_val_ref_val_ref({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_round_val_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_round_val_ref_ref_val({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone()
                .mul_sub_mul_round_val_ref_ref_val(&b, &c, d.clone(), rm)
        );
    }
}

fn demo_float_mul_sub_mul_round_val_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_round_val_ref_ref_val(&b, &c, d.clone(), rm);
        println!(
            "({:#x}).mul_sub_mul_round_val_ref_ref_val({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_round_val_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_round_val_ref_ref_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone().mul_sub_mul_round_val_ref_ref_ref(&b, &c, &d, rm)
        );
    }
}

fn demo_float_mul_sub_mul_round_val_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = a.clone().mul_sub_mul_round_val_ref_ref_ref(&b, &c, &d, rm);
        println!(
            "({:#x}).mul_sub_mul_round_val_ref_ref_ref({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_round_ref_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).mul_sub_mul_round_ref_ref_ref_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.mul_sub_mul_round_ref_ref_ref_ref(&b, &c, &d, rm)
        );
    }
}

fn demo_float_mul_sub_mul_round_ref_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = a.mul_sub_mul_round_ref_ref_ref_ref(&b, &c, &d, rm);
        println!(
            "(&{:#x}).mul_sub_mul_round_ref_ref_ref_ref({:#x}, {:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_round_assign(b.clone(), c.clone(), d.clone(), rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_round_assign({b}, {c}, {d}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_round_assign(b.clone(), c.clone(), d.clone(), rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_round_assign({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_round_assign_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_round_assign_val_val_ref(b.clone(), c.clone(), &d, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_round_assign_val_val_ref({b}, {c}, {d}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_round_assign_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_round_assign_val_val_ref(b.clone(), c.clone(), &d, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_round_assign_val_val_ref({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_round_assign_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_round_assign_val_ref_val(b.clone(), &c, d.clone(), rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_round_assign_val_ref_val({b}, {c}, {d}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_round_assign_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_round_assign_val_ref_val(b.clone(), &c, d.clone(), rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_round_assign_val_ref_val({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_round_assign_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_round_assign_val_ref_ref(b.clone(), &c, &d, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_round_assign_val_ref_ref({b}, {c}, {d}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_round_assign_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_round_assign_val_ref_ref(b.clone(), &c, &d, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_round_assign_val_ref_ref({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_round_assign_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_round_assign_ref_val_val(&b, c.clone(), d.clone(), rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_round_assign_ref_val_val({b}, {c}, {d}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_round_assign_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_round_assign_ref_val_val(&b, c.clone(), d.clone(), rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_round_assign_ref_val_val({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_round_assign_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_round_assign_ref_val_ref(&b, c.clone(), &d, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_round_assign_ref_val_ref({b}, {c}, {d}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_round_assign_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_round_assign_ref_val_ref(&b, c.clone(), &d, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_round_assign_ref_val_ref({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_round_assign_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_round_assign_ref_ref_val(&b, &c, d.clone(), rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_round_assign_ref_ref_val({b}, {c}, {d}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_round_assign_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_round_assign_ref_ref_val(&b, &c, d.clone(), rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_round_assign_ref_ref_val({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_round_assign_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_round_assign_ref_ref_ref(&b, &c, &d, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_round_assign_ref_ref_ref({b}, {c}, {d}, {rm}); a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_round_assign_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_float_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_round_assign_ref_ref_ref(&b, &c, &d, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_round_assign_ref_ref_ref({:#x}, {:#x}, {:#x}, {}); a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(b.clone(), c.clone(), d.clone())
        );
    }
}

fn demo_float_mul_sub_mul_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        let res = a.clone().mul_sub_mul(b.clone(), c.clone(), d.clone());
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {:#x}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            res
        );
    }
}

fn demo_float_mul_sub_mul_val_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(b.clone(), c.clone(), &d)
        );
    }
}

fn demo_float_mul_sub_mul_val_val_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        let res = a.clone().mul_sub_mul(b.clone(), c.clone(), &d);
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {:#x}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            res
        );
    }
}

fn demo_float_mul_sub_mul_val_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(b.clone(), &c, d.clone())
        );
    }
}

fn demo_float_mul_sub_mul_val_val_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        let res = a.clone().mul_sub_mul(b.clone(), &c, d.clone());
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {:#x}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            res
        );
    }
}

fn demo_float_mul_sub_mul_val_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(b.clone(), &c, &d)
        );
    }
}

fn demo_float_mul_sub_mul_val_val_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        let res = a.clone().mul_sub_mul(b.clone(), &c, &d);
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {:#x}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            res
        );
    }
}

fn demo_float_mul_sub_mul_val_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(&b, c.clone(), d.clone())
        );
    }
}

fn demo_float_mul_sub_mul_val_ref_val_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        let res = a.clone().mul_sub_mul(&b, c.clone(), d.clone());
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {:#x}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            res
        );
    }
}

fn demo_float_mul_sub_mul_val_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(&b, c.clone(), &d)
        );
    }
}

fn demo_float_mul_sub_mul_val_ref_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        let res = a.clone().mul_sub_mul(&b, c.clone(), &d);
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {:#x}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            res
        );
    }
}

fn demo_float_mul_sub_mul_val_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(&b, &c, d.clone())
        );
    }
}

fn demo_float_mul_sub_mul_val_ref_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        let res = a.clone().mul_sub_mul(&b, &c, d.clone());
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {:#x}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            res
        );
    }
}

fn demo_float_mul_sub_mul_val_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(&b, &c, &d)
        );
    }
}

fn demo_float_mul_sub_mul_val_ref_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        let res = a.clone().mul_sub_mul(&b, &c, &d);
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {:#x}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            res
        );
    }
}

fn demo_float_mul_sub_mul_ref_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        println!(
            "(&{}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            (&a).mul_sub_mul(&b, &c, &d)
        );
    }
}

fn demo_float_mul_sub_mul_ref_ref_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_quadruple_gen().get(gm, config).take(limit) {
        let res = (&a).mul_sub_mul(&b, &c, &d);
        println!(
            "(&{:#x}).mul_sub_mul({:#x}, {:#x}, {:#x}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            ComparableFloat(d),
            res
        );
    }
}

fn benchmark_float_mul_sub_mul_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.mul_sub_mul_prec_round(Float, Float, Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &sextuple_1_2_3_4_float_max_complexity_bucketer("a", "b", "c", "d"),
        &mut [
            (
                "Float.mul_sub_mul_prec_round(Float, Float, Float, u64, RoundingMode)",
                &mut |(a, b, c, d, prec, rm)| {
                    no_out!(a.mul_sub_mul_prec_round(b, c, d, prec, rm));
                },
            ),
            (
                "(&Float).mul_sub_mul_prec_round_ref_ref_ref_ref(&Float, &Float, &Float, u64, \
                RoundingMode)",
                &mut |(a, b, c, d, prec, rm)| {
                    no_out!(a.mul_sub_mul_prec_round_ref_ref_ref_ref(&b, &c, &d, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_float_mul_sub_mul_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.mul_sub_mul_prec_round(Float, Float, Float, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &sextuple_1_2_3_4_float_max_complexity_bucketer("a", "b", "c", "d"),
        &mut [
            ("default", &mut |(a, b, c, d, prec, rm)| {
                no_out!(a.mul_sub_mul_prec_round(b, c, d, prec, rm));
            }),
            ("naive", &mut |(a, b, c, d, prec, rm)| {
                no_out!(mul_sub_mul_prec_round_naive(&a, &b, &c, &d, prec, rm));
            }),
        ],
    );
}

fn benchmark_float_mul_sub_mul_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.mul_sub_mul_prec_round(Float, Float, Float, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_sextuple_1_2_3_4_float_max_complexity_bucketer("a", "b", "c", "d"),
        &mut [
            ("Malachite", &mut |(_, (a, b, c, d, prec, rm))| {
                no_out!(a.mul_sub_mul_prec_round_ref_ref_ref_ref(&b, &c, &d, prec, rm));
            }),
            ("rug", &mut |((a, b, c, d, prec, rm), _)| {
                no_out!(rug_mul_sub_mul_prec_round(&a, &b, &c, &d, prec, rm));
            }),
        ],
    );
}

fn demo_float_mul_sub_mul_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec_round({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone()
                .mul_sub_mul_rational_prec_round(b.clone(), c.clone(), d.clone(), prec, rm)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res =
            a.clone()
                .mul_sub_mul_rational_prec_round(b.clone(), c.clone(), d.clone(), prec, rm);
        println!(
            "({:#x}).mul_sub_mul_rational_prec_round({:#x}, {:#x}, {}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_val_val_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec_round_val_val_val_ref({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone().mul_sub_mul_rational_prec_round_val_val_val_ref(
                b.clone(),
                c.clone(),
                &d,
                prec,
                rm
            )
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_val_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res = a.clone().mul_sub_mul_rational_prec_round_val_val_val_ref(
            b.clone(),
            c.clone(),
            &d,
            prec,
            rm,
        );
        println!(
            "({:#x}).mul_sub_mul_rational_prec_round_val_val_val_ref({:#x}, {:#x}, {}, {}, {}) = \
             {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_val_val_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec_round_val_val_ref_val({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone().mul_sub_mul_rational_prec_round_val_val_ref_val(
                b.clone(),
                &c,
                d.clone(),
                prec,
                rm
            )
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_val_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res = a.clone().mul_sub_mul_rational_prec_round_val_val_ref_val(
            b.clone(),
            &c,
            d.clone(),
            prec,
            rm,
        );
        println!(
            "({:#x}).mul_sub_mul_rational_prec_round_val_val_ref_val({:#x}, {:#x}, {}, {}, {}) = \
             {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_val_val_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec_round_val_val_ref_ref({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone()
                .mul_sub_mul_rational_prec_round_val_val_ref_ref(b.clone(), &c, &d, prec, rm)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_val_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res =
            a.clone()
                .mul_sub_mul_rational_prec_round_val_val_ref_ref(b.clone(), &c, &d, prec, rm);
        println!(
            "({:#x}).mul_sub_mul_rational_prec_round_val_val_ref_ref({:#x}, {:#x}, {}, {}, {}) = \
             {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_val_ref_val_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec_round_val_ref_val_val({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone().mul_sub_mul_rational_prec_round_val_ref_val_val(
                &b,
                c.clone(),
                d.clone(),
                prec,
                rm
            )
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_val_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res = a.clone().mul_sub_mul_rational_prec_round_val_ref_val_val(
            &b,
            c.clone(),
            d.clone(),
            prec,
            rm,
        );
        println!(
            "({:#x}).mul_sub_mul_rational_prec_round_val_ref_val_val({:#x}, {:#x}, {}, {}, {}) = \
             {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_val_ref_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec_round_val_ref_val_ref({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone()
                .mul_sub_mul_rational_prec_round_val_ref_val_ref(&b, c.clone(), &d, prec, rm)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_val_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res =
            a.clone()
                .mul_sub_mul_rational_prec_round_val_ref_val_ref(&b, c.clone(), &d, prec, rm);
        println!(
            "({:#x}).mul_sub_mul_rational_prec_round_val_ref_val_ref({:#x}, {:#x}, {}, {}, {}) = \
             {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_val_ref_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec_round_val_ref_ref_val({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone()
                .mul_sub_mul_rational_prec_round_val_ref_ref_val(&b, &c, d.clone(), prec, rm)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_val_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res =
            a.clone()
                .mul_sub_mul_rational_prec_round_val_ref_ref_val(&b, &c, d.clone(), prec, rm);
        println!(
            "({:#x}).mul_sub_mul_rational_prec_round_val_ref_ref_val({:#x}, {:#x}, {}, {}, {}) = \
             {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_val_ref_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec_round_val_ref_ref_ref({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.clone()
                .mul_sub_mul_rational_prec_round_val_ref_ref_ref(&b, &c, &d, prec, rm)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_val_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_rational_prec_round_val_ref_ref_ref(&b, &c, &d, prec, rm);
        println!(
            "({:#x}).mul_sub_mul_rational_prec_round_val_ref_ref_ref({:#x}, {:#x}, {}, {}, {}) = \
             {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_ref_ref_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "(&{}).mul_sub_mul_rational_prec_round_ref_ref_ref_ref({}, {}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            rm,
            a.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&b, &c, &d, prec, rm)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_ref_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let res = a.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&b, &c, &d, prec, rm);
        println!(
            "(&{:#x}).mul_sub_mul_rational_prec_round_ref_ref_ref_ref({:#x}, {:#x}, {}, {}, {}) = \
             {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_round_assign(b.clone(), c.clone(), d.clone(), prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_round_assign({b}, {c}, {d}, {prec}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_round_assign(b.clone(), c.clone(), d.clone(), prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_round_assign({:#x}, {:#x}, {}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign_val_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_round_assign_val_val_ref(b.clone(), c.clone(), &d, prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_round_assign_val_val_ref({b}, {c}, {d}, {prec}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_round_assign_val_val_ref(b.clone(), c.clone(), &d, prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_round_assign_val_val_ref({:#x}, {:#x}, {}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign_val_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_round_assign_val_ref_val(b.clone(), &c, d.clone(), prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_round_assign_val_ref_val({b}, {c}, {d}, {prec}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_round_assign_val_ref_val(b.clone(), &c, d.clone(), prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_round_assign_val_ref_val({:#x}, {:#x}, {}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign_val_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_round_assign_val_ref_ref(b.clone(), &c, &d, prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_round_assign_val_ref_ref({b}, {c}, {d}, {prec}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_round_assign_val_ref_ref(b.clone(), &c, &d, prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_round_assign_val_ref_ref({:#x}, {:#x}, {}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign_ref_val_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_round_assign_ref_val_val(&b, c.clone(), d.clone(), prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_round_assign_ref_val_val({b}, {c}, {d}, {prec}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_round_assign_ref_val_val(&b, c.clone(), d.clone(), prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_round_assign_ref_val_val({:#x}, {:#x}, {}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign_ref_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_round_assign_ref_val_ref(&b, c.clone(), &d, prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_round_assign_ref_val_ref({b}, {c}, {d}, {prec}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_round_assign_ref_val_ref(&b, c.clone(), &d, prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_round_assign_ref_val_ref({:#x}, {:#x}, {}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign_ref_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_round_assign_ref_ref_val(&b, &c, d.clone(), prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_round_assign_ref_ref_val({b}, {c}, {d}, {prec}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_round_assign_ref_ref_val(&b, &c, d.clone(), prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_round_assign_ref_ref_val({:#x}, {:#x}, {}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign_ref_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_round_assign_ref_ref_ref(&b, &c, &d, prec, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_round_assign_ref_ref_ref({b}, {c}, {d}, {prec}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_round_assign_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec, rm) in
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_round_assign_ref_ref_ref(&b, &c, &d, prec, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_round_assign_ref_ref_ref({:#x}, {:#x}, {}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone()
                .mul_sub_mul_rational_prec(b.clone(), c.clone(), d.clone(), prec)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_rational_prec(b.clone(), c.clone(), d.clone(), prec);
        println!(
            "({:#x}).mul_sub_mul_rational_prec({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_val_val_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec_val_val_val_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone()
                .mul_sub_mul_rational_prec_val_val_val_ref(b.clone(), c.clone(), &d, prec)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_val_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res =
            a.clone()
                .mul_sub_mul_rational_prec_val_val_val_ref(b.clone(), c.clone(), &d, prec);
        println!(
            "({:#x}).mul_sub_mul_rational_prec_val_val_val_ref({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_val_val_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec_val_val_ref_val({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone()
                .mul_sub_mul_rational_prec_val_val_ref_val(b.clone(), &c, d.clone(), prec)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_val_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res =
            a.clone()
                .mul_sub_mul_rational_prec_val_val_ref_val(b.clone(), &c, d.clone(), prec);
        println!(
            "({:#x}).mul_sub_mul_rational_prec_val_val_ref_val({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_val_val_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec_val_val_ref_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone()
                .mul_sub_mul_rational_prec_val_val_ref_ref(b.clone(), &c, &d, prec)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_val_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_rational_prec_val_val_ref_ref(b.clone(), &c, &d, prec);
        println!(
            "({:#x}).mul_sub_mul_rational_prec_val_val_ref_ref({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_val_ref_val_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec_val_ref_val_val({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone()
                .mul_sub_mul_rational_prec_val_ref_val_val(&b, c.clone(), d.clone(), prec)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_val_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res =
            a.clone()
                .mul_sub_mul_rational_prec_val_ref_val_val(&b, c.clone(), d.clone(), prec);
        println!(
            "({:#x}).mul_sub_mul_rational_prec_val_ref_val_val({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_val_ref_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec_val_ref_val_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone()
                .mul_sub_mul_rational_prec_val_ref_val_ref(&b, c.clone(), &d, prec)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_val_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_rational_prec_val_ref_val_ref(&b, c.clone(), &d, prec);
        println!(
            "({:#x}).mul_sub_mul_rational_prec_val_ref_val_ref({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_val_ref_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec_val_ref_ref_val({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone()
                .mul_sub_mul_rational_prec_val_ref_ref_val(&b, &c, d.clone(), prec)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_val_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_rational_prec_val_ref_ref_val(&b, &c, d.clone(), prec);
        println!(
            "({:#x}).mul_sub_mul_rational_prec_val_ref_ref_val({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_val_ref_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_prec_val_ref_ref_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.clone()
                .mul_sub_mul_rational_prec_val_ref_ref_ref(&b, &c, &d, prec)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_val_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_rational_prec_val_ref_ref_ref(&b, &c, &d, prec);
        println!(
            "({:#x}).mul_sub_mul_rational_prec_val_ref_ref_ref({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_ref_ref_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).mul_sub_mul_rational_prec_ref_ref_ref_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            prec,
            a.mul_sub_mul_rational_prec_ref_ref_ref_ref(&b, &c, &d, prec)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_ref_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = a.mul_sub_mul_rational_prec_ref_ref_ref_ref(&b, &c, &d, prec);
        println!(
            "(&{:#x}).mul_sub_mul_rational_prec_ref_ref_ref_ref({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_assign(b.clone(), c.clone(), d.clone(), prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_assign({b}, {c}, {d}, {prec}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_assign(b.clone(), c.clone(), d.clone(), prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_assign({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign_val_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_assign_val_val_ref(b.clone(), c.clone(), &d, prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_assign_val_val_ref({b}, {c}, {d}, {prec}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_assign_val_val_ref(b.clone(), c.clone(), &d, prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_assign_val_val_ref({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign_val_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_assign_val_ref_val(b.clone(), &c, d.clone(), prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_assign_val_ref_val({b}, {c}, {d}, {prec}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_assign_val_ref_val(b.clone(), &c, d.clone(), prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_assign_val_ref_val({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign_val_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_assign_val_ref_ref(b.clone(), &c, &d, prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_assign_val_ref_ref({b}, {c}, {d}, {prec}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_assign_val_ref_ref(b.clone(), &c, &d, prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_assign_val_ref_ref({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign_ref_val_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_assign_ref_val_val(&b, c.clone(), d.clone(), prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_assign_ref_val_val({b}, {c}, {d}, {prec}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_assign_ref_val_val(&b, c.clone(), d.clone(), prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_assign_ref_val_val({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign_ref_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_assign_ref_val_ref(&b, c.clone(), &d, prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_assign_ref_val_ref({b}, {c}, {d}, {prec}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_assign_ref_val_ref(&b, c.clone(), &d, prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_assign_ref_val_ref({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign_ref_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_assign_ref_ref_val(&b, &c, d.clone(), prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_assign_ref_ref_val({b}, {c}, {d}, {prec}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_assign_ref_ref_val(&b, &c, d.clone(), prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_assign_ref_ref_val({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign_ref_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_prec_assign_ref_ref_ref(&b, &c, &d, prec);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_prec_assign_ref_ref_ref({b}, {c}, {d}, {prec}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_prec_assign_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, prec) in float_float_float_rational_unsigned_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_prec_assign_ref_ref_ref(&b, &c, &d, prec);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_prec_assign_ref_ref_ref({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            prec,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_round({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone()
                .mul_sub_mul_rational_round(b.clone(), c.clone(), d.clone(), rm)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_rational_round(b.clone(), c.clone(), d.clone(), rm);
        println!(
            "({:#x}).mul_sub_mul_rational_round({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_val_val_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_round_val_val_val_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone()
                .mul_sub_mul_rational_round_val_val_val_ref(b.clone(), c.clone(), &d, rm)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_val_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res =
            a.clone()
                .mul_sub_mul_rational_round_val_val_val_ref(b.clone(), c.clone(), &d, rm);
        println!(
            "({:#x}).mul_sub_mul_rational_round_val_val_val_ref({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_val_val_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_round_val_val_ref_val({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone()
                .mul_sub_mul_rational_round_val_val_ref_val(b.clone(), &c, d.clone(), rm)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_val_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res =
            a.clone()
                .mul_sub_mul_rational_round_val_val_ref_val(b.clone(), &c, d.clone(), rm);
        println!(
            "({:#x}).mul_sub_mul_rational_round_val_val_ref_val({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_val_val_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_round_val_val_ref_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone()
                .mul_sub_mul_rational_round_val_val_ref_ref(b.clone(), &c, &d, rm)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_val_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_rational_round_val_val_ref_ref(b.clone(), &c, &d, rm);
        println!(
            "({:#x}).mul_sub_mul_rational_round_val_val_ref_ref({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_val_ref_val_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_round_val_ref_val_val({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone()
                .mul_sub_mul_rational_round_val_ref_val_val(&b, c.clone(), d.clone(), rm)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_val_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res =
            a.clone()
                .mul_sub_mul_rational_round_val_ref_val_val(&b, c.clone(), d.clone(), rm);
        println!(
            "({:#x}).mul_sub_mul_rational_round_val_ref_val_val({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_val_ref_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_round_val_ref_val_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone()
                .mul_sub_mul_rational_round_val_ref_val_ref(&b, c.clone(), &d, rm)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_val_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_rational_round_val_ref_val_ref(&b, c.clone(), &d, rm);
        println!(
            "({:#x}).mul_sub_mul_rational_round_val_ref_val_ref({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_val_ref_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_round_val_ref_ref_val({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone()
                .mul_sub_mul_rational_round_val_ref_ref_val(&b, &c, d.clone(), rm)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_val_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_rational_round_val_ref_ref_val(&b, &c, d.clone(), rm);
        println!(
            "({:#x}).mul_sub_mul_rational_round_val_ref_ref_val({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_val_ref_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul_rational_round_val_ref_ref_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.clone()
                .mul_sub_mul_rational_round_val_ref_ref_ref(&b, &c, &d, rm)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_val_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = a
            .clone()
            .mul_sub_mul_rational_round_val_ref_ref_ref(&b, &c, &d, rm);
        println!(
            "({:#x}).mul_sub_mul_rational_round_val_ref_ref_ref({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_ref_ref_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).mul_sub_mul_rational_round_ref_ref_ref_ref({}, {}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            rm,
            a.mul_sub_mul_rational_round_ref_ref_ref_ref(&b, &c, &d, rm)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_ref_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = a.mul_sub_mul_rational_round_ref_ref_ref_ref(&b, &c, &d, rm);
        println!(
            "(&{:#x}).mul_sub_mul_rational_round_ref_ref_ref_ref({:#x}, {:#x}, {}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_round_assign(b.clone(), c.clone(), d.clone(), rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_round_assign({b}, {c}, {d}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_round_assign(b.clone(), c.clone(), d.clone(), rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_round_assign({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign_val_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_round_assign_val_val_ref(b.clone(), c.clone(), &d, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_round_assign_val_val_ref({b}, {c}, {d}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_round_assign_val_val_ref(b.clone(), c.clone(), &d, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_round_assign_val_val_ref({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign_val_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_round_assign_val_ref_val(b.clone(), &c, d.clone(), rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_round_assign_val_ref_val({b}, {c}, {d}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_round_assign_val_ref_val(b.clone(), &c, d.clone(), rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_round_assign_val_ref_val({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign_val_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_round_assign_val_ref_ref(b.clone(), &c, &d, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_round_assign_val_ref_ref({b}, {c}, {d}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_round_assign_val_ref_ref(b.clone(), &c, &d, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_round_assign_val_ref_ref({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign_ref_val_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_round_assign_ref_val_val(&b, c.clone(), d.clone(), rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_round_assign_ref_val_val({b}, {c}, {d}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_round_assign_ref_val_val(&b, c.clone(), d.clone(), rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_round_assign_ref_val_val({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign_ref_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_round_assign_ref_val_ref(&b, c.clone(), &d, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_round_assign_ref_val_ref({b}, {c}, {d}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_round_assign_ref_val_ref(&b, c.clone(), &d, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_round_assign_ref_val_ref({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign_ref_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_round_assign_ref_ref_val(&b, &c, d.clone(), rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_round_assign_ref_ref_val({b}, {c}, {d}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_round_assign_ref_ref_val(&b, &c, d.clone(), rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_round_assign_ref_ref_val({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign_ref_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = a.clone();
        a.mul_sub_mul_rational_round_assign_ref_ref_ref(&b, &c, &d, rm);
        println!(
            "a := {a_old}; \
             a.mul_sub_mul_rational_round_assign_ref_ref_ref({b}, {c}, {d}, {rm}); \
             a = {a}"
        );
    }
}

fn demo_float_mul_sub_mul_rational_round_assign_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d, rm) in float_float_float_rational_rounding_mode_quintuple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut a = a;
        let a_old = ComparableFloat(a.clone());
        a.mul_sub_mul_rational_round_assign_ref_ref_ref(&b, &c, &d, rm);
        println!(
            "a := {:#x}; \
             a.mul_sub_mul_rational_round_assign_ref_ref_ref({:#x}, {:#x}, {}, {}); \
             a = {:#x}",
            a_old,
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            rm,
            ComparableFloat(a)
        );
    }
}

fn demo_float_mul_sub_mul_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(b.clone(), c.clone(), d.clone())
        );
    }
}

fn demo_float_mul_sub_mul_rational_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = a.clone().mul_sub_mul(b.clone(), c.clone(), d.clone());
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_val_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(b.clone(), c.clone(), &d)
        );
    }
}

fn demo_float_mul_sub_mul_rational_val_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = a.clone().mul_sub_mul(b.clone(), c.clone(), &d);
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_val_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(b.clone(), &c, d.clone())
        );
    }
}

fn demo_float_mul_sub_mul_rational_val_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = a.clone().mul_sub_mul(b.clone(), &c, d.clone());
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_val_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(b.clone(), &c, &d)
        );
    }
}

fn demo_float_mul_sub_mul_rational_val_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = a.clone().mul_sub_mul(b.clone(), &c, &d);
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_val_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(&b, c.clone(), d.clone())
        );
    }
}

fn demo_float_mul_sub_mul_rational_val_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = a.clone().mul_sub_mul(&b, c.clone(), d.clone());
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_val_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(&b, c.clone(), &d)
        );
    }
}

fn demo_float_mul_sub_mul_rational_val_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = a.clone().mul_sub_mul(&b, c.clone(), &d);
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_val_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(&b, &c, d.clone())
        );
    }
}

fn demo_float_mul_sub_mul_rational_val_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = a.clone().mul_sub_mul(&b, &c, d.clone());
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_val_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            a.clone().mul_sub_mul(&b, &c, &d)
        );
    }
}

fn demo_float_mul_sub_mul_rational_val_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = a.clone().mul_sub_mul(&b, &c, &d);
        println!(
            "({:#x}).mul_sub_mul({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            res
        );
    }
}

fn demo_float_mul_sub_mul_rational_ref_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).mul_sub_mul({}, {}, {}) = {:?}",
            a,
            b,
            c,
            d,
            (&a).mul_sub_mul(&b, &c, &d)
        );
    }
}

fn demo_float_mul_sub_mul_rational_ref_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, c, d) in float_float_float_rational_quadruple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = (&a).mul_sub_mul(&b, &c, &d);
        println!(
            "(&{:#x}).mul_sub_mul({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(a),
            ComparableFloat(b),
            ComparableFloat(c),
            d,
            res
        );
    }
}

fn benchmark_float_mul_sub_mul_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.mul_sub_mul_rational_prec_round(Float, Float, Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &sextuple_1_2_3_4_float_float_float_rational_max_complexity_bucketer("a", "b", "c", "d"),
        &mut [
            (
                "Float.mul_sub_mul_rational_prec_round(Float, Float, Rational, u64, RoundingMode)",
                &mut |(a, b, c, d, prec, rm)| {
                    no_out!(a.mul_sub_mul_rational_prec_round(b, c, d, prec, rm));
                },
            ),
            (
                "(&Float).mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&Float, &Float, \
                &Rational, u64, RoundingMode)",
                &mut |(a, b, c, d, prec, rm)| {
                    let r = a.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&b, &c, &d, prec, rm);
                    no_out!(r);
                },
            ),
        ],
    );
}

fn benchmark_float_mul_sub_mul_rational_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.mul_sub_mul_rational_prec_round(Float, Float, Rational, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &sextuple_1_2_3_4_float_float_float_rational_max_complexity_bucketer("a", "b", "c", "d"),
        &mut [
            ("default", &mut |(a, b, c, d, prec, rm)| {
                no_out!(a.mul_sub_mul_rational_prec_round(b, c, d, prec, rm));
            }),
            ("naive", &mut |(a, b, c, d, prec, rm)| {
                no_out!(mul_sub_mul_rational_prec_round_naive(
                    &a, &b, &c, &d, prec, rm
                ));
            }),
        ],
    );
}
