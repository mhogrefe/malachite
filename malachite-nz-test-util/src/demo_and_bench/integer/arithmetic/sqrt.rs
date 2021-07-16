use crate::bench::bucketers::{
    integer_bit_bucketer, pair_2_integer_bit_bucketer, triple_3_integer_bit_bucketer,
};
use malachite_base::num::arithmetic::traits::{
    CeilingSqrt, CeilingSqrtAssign, CheckedSqrt, FloorSqrt, FloorSqrtAssign, SqrtRem, SqrtRemAssign,
};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz_test_util::generators::{
    integer_gen_var_4, integer_gen_var_4_nrm, integer_gen_var_4_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_floor_sqrt);
    register_demo!(runner, demo_integer_floor_sqrt_ref);
    register_demo!(runner, demo_integer_floor_sqrt_assign);
    register_demo!(runner, demo_integer_ceiling_sqrt);
    register_demo!(runner, demo_integer_ceiling_sqrt_ref);
    register_demo!(runner, demo_integer_ceiling_sqrt_assign);
    register_demo!(runner, demo_integer_checked_sqrt);
    register_demo!(runner, demo_integer_checked_sqrt_ref);
    register_demo!(runner, demo_integer_sqrt_rem);
    register_demo!(runner, demo_integer_sqrt_rem_ref);
    register_demo!(runner, demo_integer_sqrt_rem_assign);
    register_bench!(runner, benchmark_integer_floor_sqrt_evaluation_strategy);
    register_bench!(runner, benchmark_integer_floor_sqrt_library_comparison);
    register_bench!(runner, benchmark_integer_floor_sqrt_assign);
    register_bench!(runner, benchmark_integer_ceiling_sqrt_evaluation_strategy);
    register_bench!(runner, benchmark_integer_ceiling_sqrt_assign);
    register_bench!(runner, benchmark_integer_checked_sqrt_evaluation_strategy);
    register_bench!(runner, benchmark_integer_sqrt_rem_evaluation_strategy);
    register_bench!(runner, benchmark_integer_sqrt_rem_library_comparison);
    register_bench!(runner, benchmark_integer_sqrt_rem_assign);
}

fn demo_integer_floor_sqrt(gm: GenMode, config: GenConfig, limit: usize) {
    for x in integer_gen_var_4().get(gm, &config).take(limit) {
        println!("{}.floor_sqrt() = {}", x, x.clone().floor_sqrt());
    }
}

fn demo_integer_floor_sqrt_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for x in integer_gen_var_4().get(gm, &config).take(limit) {
        println!("(&{}).floor_sqrt() = {}", x, (&x).floor_sqrt());
    }
}

fn demo_integer_floor_sqrt_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for mut x in integer_gen_var_4().get(gm, &config).take(limit) {
        let old_x = x.clone();
        x.floor_sqrt_assign();
        println!("x := {}; x.floor_sqrt_assign(); x = {}", old_x, x);
    }
}

fn demo_integer_ceiling_sqrt(gm: GenMode, config: GenConfig, limit: usize) {
    for x in integer_gen_var_4().get(gm, &config).take(limit) {
        println!("{}.ceiling_sqrt() = {}", x, x.clone().ceiling_sqrt());
    }
}

fn demo_integer_ceiling_sqrt_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for x in integer_gen_var_4().get(gm, &config).take(limit) {
        println!("(&{}).ceiling_sqrt() = {}", x, (&x).ceiling_sqrt());
    }
}

fn demo_integer_ceiling_sqrt_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for mut x in integer_gen_var_4().get(gm, &config).take(limit) {
        let old_x = x.clone();
        x.ceiling_sqrt_assign();
        println!("x := {}; x.ceiling_sqrt_assign(); x = {}", old_x, x);
    }
}

fn demo_integer_checked_sqrt(gm: GenMode, config: GenConfig, limit: usize) {
    for x in integer_gen_var_4().get(gm, &config).take(limit) {
        println!("{}.checked_sqrt() = {:?}", x, x.clone().checked_sqrt());
    }
}

fn demo_integer_checked_sqrt_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for x in integer_gen_var_4().get(gm, &config).take(limit) {
        println!("(&{}).checked_sqrt() = {:?}", x, (&x).checked_sqrt());
    }
}

fn demo_integer_sqrt_rem(gm: GenMode, config: GenConfig, limit: usize) {
    for x in integer_gen_var_4().get(gm, &config).take(limit) {
        println!("{}.sqrt_rem() = {:?}", x, x.clone().sqrt_rem());
    }
}

fn demo_integer_sqrt_rem_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for x in integer_gen_var_4().get(gm, &config).take(limit) {
        println!("(&{}).sqrt_rem() = {:?}", x, (&x).sqrt_rem());
    }
}

fn demo_integer_sqrt_rem_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for mut x in integer_gen_var_4().get(gm, &config).take(limit) {
        let old_x = x.clone();
        let rem = x.sqrt_rem_assign();
        println!("x := {}; x.sqrt_rem_assign() = {}; x = {}", old_x, rem, x);
    }
}

fn benchmark_integer_floor_sqrt_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.floor_sqrt()",
        BenchmarkType::EvaluationStrategy,
        integer_gen_var_4().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [
            ("Integer.floor_sqrt()", &mut |x| no_out!(x.floor_sqrt())),
            ("(&Integer).floor_sqrt()", &mut |x| {
                no_out!((&x).floor_sqrt())
            }),
        ],
    );
}

fn benchmark_integer_floor_sqrt_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.floor_sqrt()",
        BenchmarkType::LibraryComparison,
        integer_gen_var_4_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_integer_bit_bucketer("x"),
        &mut [
            ("num", &mut |(x, _, _)| no_out!(x.sqrt())),
            ("rug", &mut |(_, x, _)| no_out!(x.sqrt())),
            ("Malachite", &mut |(_, _, x)| no_out!(x.floor_sqrt())),
        ],
    );
}

fn benchmark_integer_floor_sqrt_assign(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.floor_sqrt_assign()",
        BenchmarkType::Single,
        integer_gen_var_4().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.floor_sqrt_assign())],
    );
}

fn benchmark_integer_ceiling_sqrt_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.ceiling_sqrt()",
        BenchmarkType::EvaluationStrategy,
        integer_gen_var_4().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [
            ("Integer.ceiling_sqrt()", &mut |x| no_out!(x.ceiling_sqrt())),
            ("(&Integer).ceiling_sqrt()", &mut |x| {
                no_out!((&x).ceiling_sqrt())
            }),
        ],
    );
}

fn benchmark_integer_ceiling_sqrt_assign(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.ceiling_sqrt_assign()",
        BenchmarkType::Single,
        integer_gen_var_4().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.ceiling_sqrt_assign())],
    );
}

fn benchmark_integer_checked_sqrt_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.checked_sqrt()",
        BenchmarkType::EvaluationStrategy,
        integer_gen_var_4().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [
            ("Integer.checked_sqrt()", &mut |x| no_out!(x.checked_sqrt())),
            ("(&Integer).checked_sqrt()", &mut |x| {
                no_out!((&x).checked_sqrt())
            }),
        ],
    );
}

fn benchmark_integer_sqrt_rem_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sqrt_rem()",
        BenchmarkType::EvaluationStrategy,
        integer_gen_var_4().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [
            ("Integer.sqrt_rem()", &mut |x| no_out!(x.sqrt_rem())),
            ("(&Integer).sqrt_rem()", &mut |x| no_out!((&x).sqrt_rem())),
        ],
    );
}

fn benchmark_integer_sqrt_rem_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sqrt_rem()",
        BenchmarkType::LibraryComparison,
        integer_gen_var_4_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_bit_bucketer("x"),
        &mut [
            (
                "rug",
                &mut |(x, _)| no_out!(x.sqrt_rem(rug::Integer::new())),
            ),
            ("Malachite", &mut |(_, x)| no_out!(x.sqrt_rem())),
        ],
    );
}

fn benchmark_integer_sqrt_rem_assign(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sqrt_rem_assign()",
        BenchmarkType::Single,
        integer_gen_var_4().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut x| no_out!(x.sqrt_rem_assign()))],
    );
}
