use crate::bench::bucketers::{
    natural_bit_bucketer, pair_2_natural_bit_bucketer, triple_3_natural_bit_bucketer,
};
use malachite_base::num::arithmetic::traits::{
    CeilingSqrt, CeilingSqrtAssign, CheckedSqrt, FloorSqrt, FloorSqrtAssign, SqrtRem, SqrtRemAssign,
};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    large_type_gen_var_2, unsigned_pair_gen_var_31, unsigned_vec_pair_gen_var_4,
};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::arithmetic::sqrt::{
    _ceiling_sqrt_binary, _checked_sqrt_binary, _floor_sqrt_binary, _sqrt_rem_binary,
};
use malachite_nz::natural::arithmetic::sqrt::{
    _limbs_sqrt_helper, _limbs_sqrt_rem_helper, _limbs_sqrt_rem_helper_scratch_len,
    _sqrt_rem_2_newton,
};
use malachite_nz_test_util::generators::{natural_gen, natural_gen_nrm, natural_gen_rm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_sqrt_rem_2_newton);
    register_demo!(runner, demo_limbs_sqrt_rem_helper);
    register_demo!(runner, demo_limbs_sqrt_helper);
    register_demo!(runner, demo_natural_floor_sqrt);
    register_demo!(runner, demo_natural_floor_sqrt_ref);
    register_demo!(runner, demo_natural_floor_sqrt_assign);
    register_demo!(runner, demo_natural_ceiling_sqrt);
    register_demo!(runner, demo_natural_ceiling_sqrt_ref);
    register_demo!(runner, demo_natural_ceiling_sqrt_assign);
    register_demo!(runner, demo_natural_checked_sqrt);
    register_demo!(runner, demo_natural_checked_sqrt_ref);
    register_demo!(runner, demo_natural_sqrt_rem);
    register_demo!(runner, demo_natural_sqrt_rem_ref);
    register_demo!(runner, demo_natural_sqrt_rem_assign);
    register_bench!(runner, benchmark_natural_floor_sqrt_evaluation_strategy);
    register_bench!(runner, benchmark_natural_floor_sqrt_algorithms);
    register_bench!(runner, benchmark_natural_floor_sqrt_library_comparison);
    register_bench!(runner, benchmark_natural_floor_sqrt_assign);
    register_bench!(runner, benchmark_natural_ceiling_sqrt_evaluation_strategy);
    register_bench!(runner, benchmark_natural_ceiling_sqrt_algorithms);
    register_bench!(runner, benchmark_natural_ceiling_sqrt_assign);
    register_bench!(runner, benchmark_natural_checked_sqrt_evaluation_strategy);
    register_bench!(runner, benchmark_natural_checked_sqrt_algorithms);
    register_bench!(runner, benchmark_natural_sqrt_rem_evaluation_strategy);
    register_bench!(runner, benchmark_natural_sqrt_rem_algorithms);
    register_bench!(runner, benchmark_natural_sqrt_rem_library_comparison);
    register_bench!(runner, benchmark_natural_sqrt_rem_assign);
}

fn demo_sqrt_rem_2_newton(gm: GenMode, config: GenConfig, limit: usize) {
    for (h_hi, h_lo) in unsigned_pair_gen_var_31().get(gm, &config).take(limit) {
        println!(
            "sqrt_rem_2_newton({}, {}) = {:?}",
            h_hi,
            h_lo,
            _sqrt_rem_2_newton(h_hi, h_lo)
        );
    }
}

fn demo_limbs_sqrt_rem_helper(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut out, mut xs) in unsigned_vec_pair_gen_var_4().get(gm, &config).take(limit) {
        let mut scratch = vec![0; _limbs_sqrt_rem_helper_scratch_len(out.len())];
        let old_out = out.clone();
        let old_xs = xs.clone();
        let r_hi = _limbs_sqrt_rem_helper(&mut out, &mut xs, 0, &mut scratch);
        println!(
            "out := {:?}, xs := {:?}; \
            _limbs_sqrt_rem_helper(&mut out, &mut xs, 0, &mut scratch) = {}; \
            out = {:?}, xs = {:?}",
            old_out, old_xs, r_hi, out, xs
        );
    }
}

fn demo_limbs_sqrt_helper(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut out, xs, shift, odd) in large_type_gen_var_2().get(gm, &config).take(limit) {
        let old_out = out.clone();
        let r = _limbs_sqrt_helper(&mut out, &xs, shift, odd);
        println!(
            "out := {:?}, _limbs_sqrt_helper(&mut out, {:?}, {}, {}) = {}; out = {:?}",
            old_out, xs, shift, odd, r, out
        );
    }
}

fn demo_natural_floor_sqrt(gm: GenMode, config: GenConfig, limit: usize) {
    for x in natural_gen().get(gm, &config).take(limit) {
        println!("{}.floor_sqrt() = {}", x, x.clone().floor_sqrt());
    }
}

fn demo_natural_floor_sqrt_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for x in natural_gen().get(gm, &config).take(limit) {
        println!("(&{}).floor_sqrt() = {}", x, (&x).floor_sqrt());
    }
}

fn demo_natural_floor_sqrt_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for mut x in natural_gen().get(gm, &config).take(limit) {
        let old_x = x.clone();
        x.floor_sqrt_assign();
        println!("x := {}; x.floor_sqrt_assign(); x = {}", old_x, x);
    }
}

fn demo_natural_ceiling_sqrt(gm: GenMode, config: GenConfig, limit: usize) {
    for x in natural_gen().get(gm, &config).take(limit) {
        println!("{}.ceiling_sqrt() = {}", x, x.clone().ceiling_sqrt());
    }
}

fn demo_natural_ceiling_sqrt_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for x in natural_gen().get(gm, &config).take(limit) {
        println!("(&{}).ceiling_sqrt() = {}", x, (&x).ceiling_sqrt());
    }
}

fn demo_natural_ceiling_sqrt_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for mut x in natural_gen().get(gm, &config).take(limit) {
        let old_x = x.clone();
        x.ceiling_sqrt_assign();
        println!("x := {}; x.ceiling_sqrt_assign(); x = {}", old_x, x);
    }
}

fn demo_natural_checked_sqrt(gm: GenMode, config: GenConfig, limit: usize) {
    for x in natural_gen().get(gm, &config).take(limit) {
        println!("{}.checked_sqrt() = {:?}", x, x.clone().checked_sqrt());
    }
}

fn demo_natural_checked_sqrt_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for x in natural_gen().get(gm, &config).take(limit) {
        println!("(&{}).checked_sqrt() = {:?}", x, (&x).checked_sqrt());
    }
}

fn demo_natural_sqrt_rem(gm: GenMode, config: GenConfig, limit: usize) {
    for x in natural_gen().get(gm, &config).take(limit) {
        println!("{}.sqrt_rem() = {:?}", x, x.clone().sqrt_rem());
    }
}

fn demo_natural_sqrt_rem_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for x in natural_gen().get(gm, &config).take(limit) {
        println!("(&{}).sqrt_rem() = {:?}", x, (&x).sqrt_rem());
    }
}

fn demo_natural_sqrt_rem_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for mut x in natural_gen().get(gm, &config).take(limit) {
        let old_x = x.clone();
        let rem = x.sqrt_rem_assign();
        println!("x := {}; x.sqrt_rem_assign() = {}; x = {}", old_x, rem, x);
    }
}

fn benchmark_natural_floor_sqrt_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_sqrt()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("Natural.floor_sqrt()", &mut |x| no_out!(x.floor_sqrt())),
            ("(&Natural).floor_sqrt()", &mut |x| {
                no_out!((&x).floor_sqrt())
            }),
        ],
    );
}

fn benchmark_natural_floor_sqrt_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_sqrt()",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.floor_sqrt())),
            ("binary", &mut |x| no_out!(_floor_sqrt_binary(&x))),
        ],
    );
}

fn benchmark_natural_floor_sqrt_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_sqrt()",
        BenchmarkType::LibraryComparison,
        natural_gen_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("x"),
        &mut [
            ("num", &mut |(x, _, _)| no_out!(x.sqrt())),
            ("rug", &mut |(_, x, _)| no_out!(x.sqrt())),
            ("Malachite", &mut |(_, _, x)| no_out!(x.floor_sqrt())),
        ],
    );
}

fn benchmark_natural_floor_sqrt_assign(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_sqrt_assign()",
        BenchmarkType::Single,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.floor_sqrt_assign())],
    );
}

fn benchmark_natural_ceiling_sqrt_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_sqrt()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("Natural.ceiling_sqrt()", &mut |x| no_out!(x.ceiling_sqrt())),
            ("(&Natural).ceiling_sqrt()", &mut |x| {
                no_out!((&x).ceiling_sqrt())
            }),
        ],
    );
}

fn benchmark_natural_ceiling_sqrt_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_sqrt()",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.ceiling_sqrt())),
            ("binary", &mut |x| no_out!(_ceiling_sqrt_binary(&x))),
        ],
    );
}

fn benchmark_natural_ceiling_sqrt_assign(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_sqrt_assign()",
        BenchmarkType::Single,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.ceiling_sqrt_assign())],
    );
}

fn benchmark_natural_checked_sqrt_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_sqrt()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("Natural.checked_sqrt()", &mut |x| no_out!(x.checked_sqrt())),
            ("(&Natural).checked_sqrt()", &mut |x| {
                no_out!((&x).checked_sqrt())
            }),
        ],
    );
}

fn benchmark_natural_checked_sqrt_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_sqrt()",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.checked_sqrt())),
            ("binary", &mut |x| no_out!(_checked_sqrt_binary(&x))),
        ],
    );
}

fn benchmark_natural_sqrt_rem_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sqrt_rem()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("Natural.sqrt_rem()", &mut |x| no_out!(x.sqrt_rem())),
            ("(&Natural).sqrt_rem()", &mut |x| no_out!((&x).sqrt_rem())),
        ],
    );
}

fn benchmark_natural_sqrt_rem_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sqrt_rem()",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.sqrt_rem())),
            ("binary", &mut |x| no_out!(_sqrt_rem_binary(&x))),
        ],
    );
}

fn benchmark_natural_sqrt_rem_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sqrt_rem()",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("x"),
        &mut [
            (
                "rug",
                &mut |(x, _)| no_out!(x.sqrt_rem(rug::Integer::new())),
            ),
            ("Malachite", &mut |(_, x)| no_out!(x.sqrt_rem())),
        ],
    );
}

fn benchmark_natural_sqrt_rem_assign(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sqrt_rem_assign()",
        BenchmarkType::Single,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut x| no_out!(x.sqrt_rem_assign()))],
    );
}
