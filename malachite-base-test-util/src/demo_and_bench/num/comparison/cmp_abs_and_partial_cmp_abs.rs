use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

use malachite_base_test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_pair_gen, unsigned_pair_gen};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_cmp_abs);
    register_unsigned_demos!(runner, demo_unsigned_partial_cmp_abs);
    register_signed_demos!(runner, demo_signed_cmp_abs);
    register_signed_demos!(runner, demo_signed_partial_cmp_abs);
    register_unsigned_benches!(runner, benchmark_unsigned_cmp_abs);
    register_unsigned_benches!(runner, benchmark_unsigned_partial_cmp_abs);
    register_signed_benches!(runner, benchmark_signed_cmp_abs);
    register_signed_benches!(runner, benchmark_signed_partial_cmp_abs);
}

fn demo_unsigned_cmp_abs<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen::<T>().get(gm, &config).take(limit) {
        println!("{}.cmp_abs(&{}) = {:?}", x, y, x.cmp_abs(&y));
    }
}

fn demo_unsigned_partial_cmp_abs<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (x, y) in unsigned_pair_gen::<T>().get(gm, &config).take(limit) {
        println!(
            "{}.partial_cmp_abs(&{}) = {:?}",
            x,
            y,
            x.partial_cmp_abs(&y)
        );
    }
}

fn demo_signed_cmp_abs<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen::<T>().get(gm, &config).take(limit) {
        println!("({}).cmp_abs(&{}) = {:?}", x, y, x.cmp_abs(&y));
    }
}

fn demo_signed_partial_cmp_abs<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen::<T>().get(gm, &config).take(limit) {
        println!(
            "({}).partial_cmp_abs(&{}) = {:?}",
            x,
            y,
            x.partial_cmp_abs(&y)
        );
    }
}

fn benchmark_unsigned_cmp_abs<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.cmp_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.cmp_abs(&y)))],
    );
}

fn benchmark_unsigned_partial_cmp_abs<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.partial_cmp_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.partial_cmp_abs(&y)))],
    );
}

fn benchmark_signed_cmp_abs<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.cmp_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.cmp_abs(&y)))],
    );
}

fn benchmark_signed_partial_cmp_abs<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.partial_cmp_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.partial_cmp_abs(&y)))],
    );
}
