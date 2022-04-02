use malachite_base::strings::string_unique;
use malachite_base::test_util::bench::bucketers::string_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{string_gen, string_gen_var_1};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_string_unique);
    register_demo!(runner, demo_string_unique_ascii);
    register_bench!(runner, benchmark_string_unique);
}

fn demo_string_unique(gm: GenMode, config: GenConfig, limit: usize) {
    for s in string_gen().get(gm, &config).take(limit) {
        println!("string_unique({:?}) = {:?}", s, string_unique(&s));
    }
}

fn demo_string_unique_ascii(gm: GenMode, config: GenConfig, limit: usize) {
    for s in string_gen_var_1().get(gm, &config).take(limit) {
        println!("string_unique({:?}) = {:?}", s, string_unique(&s));
    }
}

fn benchmark_string_unique(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "string_unique(&str)",
        BenchmarkType::Single,
        string_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [("Malachite", &mut |s| no_out!(string_unique(&s)))],
    );
}
