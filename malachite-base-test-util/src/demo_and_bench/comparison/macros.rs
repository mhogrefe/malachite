use malachite_base_test_util::bench::bucketers::{
    pair_max_bit_bucketer, triple_max_bit_bucketer, unsigned_bit_bucketer,
};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{unsigned_gen, unsigned_pair_gen, unsigned_triple_gen};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_max_1);
    register_demo!(runner, demo_max_2);
    register_demo!(runner, demo_max_3);
    register_demo!(runner, demo_min_1);
    register_demo!(runner, demo_min_2);
    register_demo!(runner, demo_min_3);
    register_bench!(runner, benchmark_max_1);
    register_bench!(runner, benchmark_max_2);
    register_bench!(runner, benchmark_max_3);
    register_bench!(runner, benchmark_min_1);
    register_bench!(runner, benchmark_min_2);
    register_bench!(runner, benchmark_min_3);
}

fn demo_max_1(gm: GenMode, config: GenConfig, limit: usize) {
    for x in unsigned_gen::<u8>().get(gm, &config).take(limit) {
        println!("max!({}) = {}", x, max!(x));
    }
}

fn demo_max_2(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen::<u8>().get(gm, &config).take(limit) {
        println!("max!({}, {}) = {}", x, y, max!(x, y));
    }
}

fn demo_max_3(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, z) in unsigned_triple_gen::<u8>().get(gm, &config).take(limit) {
        println!("max!({}, {}, {}) = {}", x, y, z, max!(x, y, z));
    }
}

fn demo_min_1(gm: GenMode, config: GenConfig, limit: usize) {
    for x in unsigned_gen::<u8>().get(gm, &config).take(limit) {
        println!("min!({}) = {}", x, min!(x));
    }
}

fn demo_min_2(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen::<u8>().get(gm, &config).take(limit) {
        println!("min!({}, {}) = {}", x, y, min!(x, y));
    }
}

fn demo_min_3(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, z) in unsigned_triple_gen::<u8>().get(gm, &config).take(limit) {
        println!("min!({}, {}, {}) = {}", x, y, z, min!(x, y, z));
    }
}

fn benchmark_max_1(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "max!(T)",
        BenchmarkType::Single,
        unsigned_gen::<u8>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |x| no_out!(max!(x)))],
    );
}

fn benchmark_max_2(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "max!(T, T)",
        BenchmarkType::Single,
        unsigned_pair_gen::<u8>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(max!(x, y)))],
    );
}

fn benchmark_max_3(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "max!(T, T, T)",
        BenchmarkType::Single,
        unsigned_triple_gen::<u8>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| no_out!(max!(x, y, z)))],
    );
}

fn benchmark_min_1(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "min!(T)",
        BenchmarkType::Single,
        unsigned_gen::<u8>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |x| no_out!(min!(x)))],
    );
}

fn benchmark_min_2(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "min!(T, T)",
        BenchmarkType::Single,
        unsigned_pair_gen::<u8>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(min!(x, y)))],
    );
}

fn benchmark_min_3(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "min!(T, T, T)",
        BenchmarkType::Single,
        unsigned_triple_gen::<u8>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| no_out!(min!(x, y, z)))],
    );
}
