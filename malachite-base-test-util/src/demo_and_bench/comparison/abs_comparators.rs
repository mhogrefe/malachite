use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

use malachite_base_test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_pair_gen, unsigned_pair_gen};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_lt_abs);
    register_unsigned_demos!(runner, demo_unsigned_gt_abs);
    register_unsigned_demos!(runner, demo_unsigned_le_abs);
    register_unsigned_demos!(runner, demo_unsigned_ge_abs);
    register_signed_demos!(runner, demo_signed_lt_abs);
    register_signed_demos!(runner, demo_signed_gt_abs);
    register_signed_demos!(runner, demo_signed_le_abs);
    register_signed_demos!(runner, demo_signed_ge_abs);
    register_unsigned_benches!(runner, benchmark_unsigned_lt_abs);
    register_unsigned_benches!(runner, benchmark_unsigned_gt_abs);
    register_unsigned_benches!(runner, benchmark_unsigned_le_abs);
    register_unsigned_benches!(runner, benchmark_unsigned_ge_abs);
    register_signed_benches!(runner, benchmark_signed_lt_abs);
    register_signed_benches!(runner, benchmark_signed_gt_abs);
    register_signed_benches!(runner, benchmark_signed_le_abs);
    register_signed_benches!(runner, benchmark_signed_ge_abs);
}

fn demo_unsigned_lt_abs<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen::<T>().get(gm, &config).take(limit) {
        println!("{}.lt_abs(&{}) = {}", x, y, x.lt_abs(&y));
    }
}

fn demo_unsigned_gt_abs<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen::<T>().get(gm, &config).take(limit) {
        println!("{}.gt_abs(&{}) = {}", x, y, x.gt_abs(&y));
    }
}

fn demo_unsigned_le_abs<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen::<T>().get(gm, &config).take(limit) {
        println!("{}.le_abs(&{}) = {}", x, y, x.le_abs(&y));
    }
}

fn demo_unsigned_ge_abs<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen::<T>().get(gm, &config).take(limit) {
        println!("{}.ge_abs(&{}) = {}", x, y, x.ge_abs(&y));
    }
}

fn demo_signed_lt_abs<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen::<T>().get(gm, &config).take(limit) {
        println!("({}).lt_abs(&{}) = {}", x, y, x.lt_abs(&y));
    }
}

fn demo_signed_gt_abs<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen::<T>().get(gm, &config).take(limit) {
        println!("({}).gt_abs(&{}) = {}", x, y, x.gt_abs(&y));
    }
}

fn demo_signed_le_abs<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen::<T>().get(gm, &config).take(limit) {
        println!("({}).le_abs(&{}) = {}", x, y, x.le_abs(&y));
    }
}

fn demo_signed_ge_abs<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen::<T>().get(gm, &config).take(limit) {
        println!("({}).ge_abs(&{}) = {}", x, y, x.ge_abs(&y));
    }
}

fn benchmark_unsigned_lt_abs<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.lt_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.lt_abs(&y))))],
    );
}

fn benchmark_unsigned_gt_abs<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.gt_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.gt_abs(&y))))],
    );
}

fn benchmark_unsigned_le_abs<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.le_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.le_abs(&y))))],
    );
}

fn benchmark_unsigned_ge_abs<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ge_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.ge_abs(&y))))],
    );
}

fn benchmark_signed_lt_abs<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.lt_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.lt_abs(&y))))],
    );
}

fn benchmark_signed_gt_abs<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.gt_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.gt_abs(&y))))],
    );
}

fn benchmark_signed_le_abs<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.le_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.le_abs(&y))))],
    );
}

fn benchmark_signed_ge_abs<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ge_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.ge_abs(&y))))],
    );
}
