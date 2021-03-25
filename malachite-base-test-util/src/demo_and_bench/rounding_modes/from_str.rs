use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::bench::bucketers::string_len_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{string_gen, string_gen_var_2};
use malachite_base_test_util::runner::Runner;
use std::str::FromStr;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rounding_mode_from_str);
    register_demo!(runner, demo_rounding_mode_from_str_targeted);
    register_bench!(runner, benchmark_rounding_mode_from_str);
}

fn demo_rounding_mode_from_str(gm: GenMode, config: GenConfig, limit: usize) {
    for s in string_gen().get(gm, &config).take(limit) {
        println!(
            "RoundingMode::from_str({:?}) = {:?}",
            s,
            RoundingMode::from_str(&s)
        );
    }
}

fn demo_rounding_mode_from_str_targeted(gm: GenMode, config: GenConfig, limit: usize) {
    for s in string_gen_var_2().get(gm, &config).take(limit) {
        println!(
            "RoundingMode::from_str({:?}) = {:?}",
            s,
            RoundingMode::from_str(&s)
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_rounding_mode_from_str(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "from_str(&str)",
        BenchmarkType::Single,
        string_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [("Malachite", &mut |s| no_out!(RoundingMode::from_str(&s)))],
    );
}
