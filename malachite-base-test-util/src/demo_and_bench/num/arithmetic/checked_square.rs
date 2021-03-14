use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_gen, unsigned_gen};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_checked_square_unsigned);
    register_signed_demos!(runner, demo_checked_square_signed);
    register_unsigned_benches!(runner, benchmark_checked_square_unsigned);
    register_signed_benches!(runner, benchmark_checked_square_signed);
}

fn demo_checked_square_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for x in unsigned_gen::<T>().get(gm, &config).take(limit) {
        println!("{}.checked_square() = {:?}", x, x.checked_square());
    }
}

fn demo_checked_square_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in signed_gen::<T>().get(gm, &config).take(limit) {
        println!("({}).checked_square() = {:?}", x, x.checked_square());
    }
}

fn benchmark_checked_square_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_square()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |x| no_out!(x.checked_square()))],
    );
}

fn benchmark_checked_square_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_square()", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |x| no_out!(x.checked_square()))],
    );
}
