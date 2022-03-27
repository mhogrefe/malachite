use malachite_base::num::arithmetic::traits::{CeilingLogBase2, CheckedLogBase2, FloorLogBase2};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_q_test_util::bench::bucketers::rational_bit_bucketer;
use malachite_q_test_util::generators::rational_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_floor_log_base_2);
    register_demo!(runner, demo_rational_ceiling_log_base_2);
    register_demo!(runner, demo_rational_checked_log_base_2);

    register_bench!(runner, benchmark_rational_floor_log_base_2);
    register_bench!(runner, benchmark_rational_ceiling_log_base_2);
    register_bench!(runner, benchmark_rational_checked_log_base_2);
}

fn demo_rational_floor_log_base_2(gm: GenMode, config: GenConfig, limit: usize) {
    for n in rational_gen_var_2().get(gm, &config).take(limit) {
        println!("floor_log_base_2({}) = {}", n, n.floor_log_base_2());
    }
}

fn demo_rational_ceiling_log_base_2(gm: GenMode, config: GenConfig, limit: usize) {
    for n in rational_gen_var_2().get(gm, &config).take(limit) {
        println!("ceiling_log_base_2({}) = {}", n, n.ceiling_log_base_2());
    }
}

fn demo_rational_checked_log_base_2(gm: GenMode, config: GenConfig, limit: usize) {
    for n in rational_gen_var_2().get(gm, &config).take(limit) {
        println!("checked_log_base_2({}) = {:?}", n, n.checked_log_base_2());
    }
}

fn benchmark_rational_floor_log_base_2(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.floor_log_base_2()",
        BenchmarkType::Single,
        rational_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(n.floor_log_base_2()))],
    );
}

fn benchmark_rational_ceiling_log_base_2(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.ceiling_log_base_2()",
        BenchmarkType::Single,
        rational_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(n.ceiling_log_base_2()))],
    );
}

fn benchmark_rational_checked_log_base_2(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.floor_log_base_2()",
        BenchmarkType::Single,
        rational_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(n.checked_log_base_2()))],
    );
}
