use malachite_base::num::arithmetic::sqrt::{
    _ceiling_sqrt_binary, _checked_sqrt_binary, _floor_sqrt_binary, _sqrt_rem_binary,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::unsigned_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_gen;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_floor_sqrt);
    register_unsigned_demos!(runner, demo_floor_sqrt_assign);
    register_unsigned_demos!(runner, demo_ceiling_sqrt);
    register_unsigned_demos!(runner, demo_ceiling_sqrt_assign);
    register_unsigned_demos!(runner, demo_checked_sqrt);
    register_unsigned_demos!(runner, demo_sqrt_rem);
    register_unsigned_demos!(runner, demo_sqrt_rem_assign);
    register_unsigned_benches!(runner, benchmark_floor_sqrt_algorithms);
    register_unsigned_benches!(runner, benchmark_floor_sqrt_assign);
    register_unsigned_benches!(runner, benchmark_ceiling_sqrt_algorithms);
    register_unsigned_benches!(runner, benchmark_ceiling_sqrt_assign);
    register_unsigned_benches!(runner, benchmark_checked_sqrt_algorithms);
    register_unsigned_benches!(runner, benchmark_sqrt_rem_algorithms);
    register_unsigned_benches!(runner, benchmark_sqrt_rem_assign);
}

fn demo_floor_sqrt<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for n in unsigned_gen::<T>().get(gm, &config).take(limit) {
        println!("floor_sqrt({}) = {}", n, n.floor_sqrt());
    }
}

fn demo_floor_sqrt_assign<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for mut n in unsigned_gen::<T>().get(gm, &config).take(limit) {
        let old_n = n;
        n.floor_sqrt_assign();
        println!("n := {}; n.floor_sqrt_assign(); n = {}", old_n, n);
    }
}

fn demo_ceiling_sqrt<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for n in unsigned_gen::<T>().get(gm, &config).take(limit) {
        println!("ceiling_sqrt({}) = {}", n, n.ceiling_sqrt());
    }
}

fn demo_ceiling_sqrt_assign<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for mut n in unsigned_gen::<T>().get(gm, &config).take(limit) {
        let old_n = n;
        n.ceiling_sqrt_assign();
        println!("n := {}; n.ceiling_sqrt_assign(); n = {}", old_n, n);
    }
}

fn demo_checked_sqrt<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for n in unsigned_gen::<T>().get(gm, &config).take(limit) {
        println!("checked_sqrt({}) = {:?}", n, n.checked_sqrt());
    }
}

fn demo_sqrt_rem<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for n in unsigned_gen::<T>().get(gm, &config).take(limit) {
        let (sqrt, rem) = n.sqrt_rem();
        println!("{} = {} ^ 2 + {}", n, sqrt, rem);
    }
}

fn demo_sqrt_rem_assign<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for mut n in unsigned_gen::<T>().get(gm, &config).take(limit) {
        let old_n = n;
        let rem = n.sqrt_rem_assign();
        println!("n := {}; n.sqrt_assign() = {}; n = {}", old_n, rem, n);
    }
}

fn benchmark_floor_sqrt_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_sqrt()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(n.floor_sqrt())),
            ("binary", &mut |n| no_out!(_floor_sqrt_binary(n))),
        ],
    );
}

fn benchmark_floor_sqrt_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_sqrt_assign()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |mut n| n.floor_sqrt_assign())],
    );
}

fn benchmark_ceiling_sqrt_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_sqrt()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(n.ceiling_sqrt())),
            ("binary", &mut |n| no_out!(_ceiling_sqrt_binary(n))),
        ],
    );
}

fn benchmark_ceiling_sqrt_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_sqrt_assign()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |mut n| n.ceiling_sqrt_assign())],
    );
}

fn benchmark_checked_sqrt_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_sqrt()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(n.checked_sqrt())),
            ("binary", &mut |n| no_out!(_checked_sqrt_binary(n))),
        ],
    );
}

fn benchmark_sqrt_rem_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sqrt_rem()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(n.sqrt_rem())),
            ("binary", &mut |n| no_out!(_sqrt_rem_binary(n))),
        ],
    );
}

fn benchmark_sqrt_rem_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sqrt_rem_assign()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |mut n| no_out!(n.sqrt_rem_assign()))],
    );
}
