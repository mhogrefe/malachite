use std::fmt::Display;

use malachite_base::named::Named;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::SaturatingFrom;

use malachite_base_test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_gen, unsigned_gen};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_int_unsigned_demos!(runner, demo_primitive_int_saturating_from_unsigned);
    register_primitive_int_signed_demos!(runner, demo_primitive_int_saturating_from_signed);
    register_primitive_int_unsigned_benches!(
        runner,
        benchmark_primitive_int_saturating_from_unsigned
    );
    register_primitive_int_signed_benches!(runner, benchmark_primitive_int_saturating_from_signed);
}

fn demo_primitive_int_saturating_from_unsigned<
    T: Display + SaturatingFrom<U> + Named,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<U>().get(gm, &config).take(limit) {
        println!(
            "{}::saturating_from({}) = {}",
            T::NAME,
            u,
            T::saturating_from(u)
        );
    }
}

fn demo_primitive_int_saturating_from_signed<
    T: Display + SaturatingFrom<U> + Named,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for i in signed_gen::<U>().get(gm, &config).take(limit) {
        println!(
            "{}::saturating_from({}) = {}",
            T::NAME,
            i,
            T::saturating_from(i)
        );
    }
}

fn benchmark_primitive_int_saturating_from_unsigned<
    T: SaturatingFrom<U> + Named,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.saturating_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_gen::<U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::saturating_from(n)))],
    );
}

fn benchmark_primitive_int_saturating_from_signed<
    T: SaturatingFrom<U> + Named,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.saturating_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_gen::<U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::saturating_from(n)))],
    );
}
