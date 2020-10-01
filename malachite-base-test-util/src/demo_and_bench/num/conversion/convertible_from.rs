use malachite_base::named::Named;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;

use malachite_base_test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_gen, unsigned_gen};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_int_unsigned_demos!(runner, demo_primitive_int_convertible_from_unsigned);
    register_primitive_int_signed_demos!(runner, demo_primitive_int_convertible_from_signed);
    register_primitive_int_unsigned_benches!(
        runner,
        benchmark_primitive_int_convertible_from_unsigned
    );
    register_primitive_int_signed_benches!(runner, benchmark_primitive_int_convertible_from_signed);
}

fn demo_primitive_int_convertible_from_unsigned<
    T: ConvertibleFrom<U> + Named,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<U>().get(gm, &config).take(limit) {
        println!(
            "{} is {}convertible to a {}",
            u,
            if T::convertible_from(u) { "" } else { "not " },
            U::NAME,
        );
    }
}

fn demo_primitive_int_convertible_from_signed<T: ConvertibleFrom<U> + Named, U: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for i in signed_gen::<U>().get(gm, &config).take(limit) {
        println!(
            "{} is {}convertible to a {}",
            i,
            if T::convertible_from(i) { "" } else { "not " },
            U::NAME,
        );
    }
}

fn benchmark_primitive_int_convertible_from_unsigned<
    T: ConvertibleFrom<U> + Named,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.convertible_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_gen::<U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut (|n| no_out!(T::convertible_from(n))))],
    );
}

fn benchmark_primitive_int_convertible_from_signed<
    T: ConvertibleFrom<U> + Named,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.convertible_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_gen::<U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut (|n| no_out!(T::convertible_from(n))))],
    );
}
