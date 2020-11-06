use malachite_base::chars::crement::{decrement_char, increment_char};

use malachite_base_test_util::bench::bucketers::char_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{char_gen_var_1, char_gen_var_2};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_increment_char);
    register_demo!(runner, demo_decrement_char);
    register_bench!(runner, benchmark_increment_char);
    register_bench!(runner, benchmark_decrement_char);
}

fn demo_increment_char(gm: GenMode, config: GenConfig, limit: usize) {
    for mut c in char_gen_var_1().get(gm, &config).take(limit) {
        let c_old = c;
        increment_char(&mut c);
        println!("c := {:?}; increment_char(&mut c); c = {:?}", c_old, c);
    }
}

fn demo_decrement_char(gm: GenMode, config: GenConfig, limit: usize) {
    for mut c in char_gen_var_2().get(gm, &config).take(limit) {
        let c_old = c;
        increment_char(&mut c);
        println!("c := {:?}; decrement_char(&mut c); c = {:?}", c_old, c);
    }
}

fn benchmark_increment_char(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "increment_char(&mut char)",
        BenchmarkType::Single,
        char_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &char_bucketer(),
        &mut [("Malachite", &mut (|mut c| increment_char(&mut c)))],
    );
}

fn benchmark_decrement_char(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "decrement_char(&mut char)",
        BenchmarkType::Single,
        char_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &char_bucketer(),
        &mut [("Malachite", &mut (|mut c| decrement_char(&mut c)))],
    );
}
