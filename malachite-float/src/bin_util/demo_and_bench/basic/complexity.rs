use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::float_complexity_bucketer;
use malachite_float::test_util::generators::float_gen;
use malachite_float::ComparableFloatRef;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_complexity);
    register_demo!(runner, demo_float_complexity_debug);
    register_demo!(runner, demo_float_significant_bits);
    register_demo!(runner, demo_float_significant_bits_debug);

    register_bench!(runner, benchmark_float_complexity);
    register_bench!(runner, benchmark_float_significant_bits);
}

fn demo_float_complexity(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("complexity({}) = {}", x, x.complexity());
    }
}

fn demo_float_complexity_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "complexity({:#x}) = {}",
            ComparableFloatRef(&x),
            x.complexity()
        );
    }
}

fn demo_float_significant_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("significant_bits({}) = {}", x, x.significant_bits());
    }
}

fn demo_float_significant_bits_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "significant_bits({:#x}) = {}",
            ComparableFloatRef(&x),
            x.significant_bits()
        );
    }
}

fn benchmark_float_complexity(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.complexity()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.complexity()))],
    );
}

fn benchmark_float_significant_bits(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.significant_bits()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.significant_bits()))],
    );
}
