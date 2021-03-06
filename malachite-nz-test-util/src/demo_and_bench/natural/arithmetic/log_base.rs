use crate::bench::bucketers::{natural_bit_bucketer, natural_bit_ratio_bucketer};
use malachite_base::num::arithmetic::traits::{CeilingLogBase, CheckedLogBase, FloorLogBase};
use malachite_base::num::float::NiceFloat;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz_test_util::generators::{natural_gen_var_2, natural_pair_gen_var_3};
use malachite_nz_test_util::natural::arithmetic::log_base::{
    _ceiling_log_base_by_squaring, _ceiling_log_base_naive, _checked_log_base_by_squaring,
    _checked_log_base_naive, _floor_log_base_by_squaring, _floor_log_base_naive,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_log_approx);
    register_demo!(runner, demo_natural_floor_log_base);
    register_demo!(runner, demo_natural_ceiling_log_base);
    register_demo!(runner, demo_natural_checked_log_base);
    register_bench!(runner, benchmark_approx_log);
    register_bench!(runner, benchmark_natural_floor_log_base_algorithms);
    register_bench!(runner, benchmark_natural_ceiling_log_base_algorithms);
    register_bench!(runner, benchmark_natural_checked_log_base_algorithms);
}

fn demo_natural_log_approx(gm: GenMode, config: GenConfig, limit: usize) {
    for n in natural_gen_var_2().get(gm, &config).take(limit) {
        println!("log({}) ≈ {}", n, NiceFloat(n.approx_log()));
    }
}

fn demo_natural_floor_log_base(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, base) in natural_pair_gen_var_3().get(gm, &config).take(limit) {
        println!(
            "floor_log_base({}, {}) = {}",
            n,
            base,
            n.floor_log_base(&base)
        );
    }
}

fn demo_natural_ceiling_log_base(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, base) in natural_pair_gen_var_3().get(gm, &config).take(limit) {
        println!(
            "ceiling_log_base({}, {}) = {}",
            n,
            base,
            n.ceiling_log_base(&base)
        );
    }
}

fn demo_natural_checked_log_base(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, base) in natural_pair_gen_var_3().get(gm, &config).take(limit) {
        println!(
            "checked_log_base({}, {}) = {:?}",
            n,
            base,
            n.checked_log_base(&base)
        );
    }
}

fn benchmark_approx_log(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "(&Natural).approx_log()",
        BenchmarkType::Single,
        natural_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("default", &mut |n| no_out!(n.approx_log()))],
    );
}

fn benchmark_natural_floor_log_base_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&Natural).floor_log_base(&Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_3().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_ratio_bucketer("n", "base"),
        &mut [
            ("default", &mut |(n, base)| no_out!(n.floor_log_base(&base))),
            ("naive", &mut |(n, base)| {
                no_out!(_floor_log_base_naive(&n, &base))
            }),
            ("by squaring", &mut |(n, base)| {
                no_out!(_floor_log_base_by_squaring(&n, &base))
            }),
        ],
    );
}

fn benchmark_natural_ceiling_log_base_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&Natural).ceiling_log_base(&Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_3().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_ratio_bucketer("n", "base"),
        &mut [
            ("default", &mut |(n, base)| {
                no_out!(n.ceiling_log_base(&base))
            }),
            ("naive", &mut |(n, base)| {
                no_out!(_ceiling_log_base_naive(&n, &base))
            }),
            ("by squaring", &mut |(n, base)| {
                no_out!(_ceiling_log_base_by_squaring(&n, &base))
            }),
        ],
    );
}

fn benchmark_natural_checked_log_base_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&Natural).checked_log_base(&Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_3().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_ratio_bucketer("n", "base"),
        &mut [
            ("default", &mut |(n, base)| {
                no_out!(n.checked_log_base(&base))
            }),
            ("naive", &mut |(n, base)| {
                no_out!(_checked_log_base_naive(&n, &base))
            }),
            ("by squaring", &mut |(n, base)| {
                no_out!(_checked_log_base_by_squaring(&n, &base))
            }),
        ],
    );
}
