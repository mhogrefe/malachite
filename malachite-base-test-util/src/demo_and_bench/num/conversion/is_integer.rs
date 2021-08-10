use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base_test_util::bench::bucketers::{
    primitive_float_bucketer, signed_bit_bucketer, unsigned_bit_bucketer,
};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{primitive_float_gen, signed_gen, unsigned_gen};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_is_integer_unsigned);
    register_signed_demos!(runner, demo_is_integer_signed);
    register_primitive_float_demos!(runner, demo_is_integer_primitive_float);
    register_unsigned_benches!(runner, benchmark_is_integer_unsigned);
    register_signed_benches!(runner, benchmark_is_integer_signed);
    register_primitive_float_benches!(runner, benchmark_is_integer_primitive_float);
}

fn demo_is_integer_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for x in primitive_float_gen::<T>().get(gm, &config).take(limit) {
        if x.is_integer() {
            println!("{} is an integer", NiceFloat(x));
        } else {
            println!("{} is not an integer", NiceFloat(x));
        }
    }
}

fn demo_is_integer_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in unsigned_gen::<T>().get(gm, &config).take(limit) {
        if x.is_integer() {
            println!("{} is an integer", x);
        } else {
            println!("{} is not an integer", x);
        }
    }
}

fn demo_is_integer_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in signed_gen::<T>().get(gm, &config).take(limit) {
        if x.is_integer() {
            println!("{} is an integer", x);
        } else {
            println!("{} is not an integer", x);
        }
    }
}

fn benchmark_is_integer_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.is_integer()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.is_integer()))],
    );
}

fn benchmark_is_integer_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.is_integer()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |x| no_out!(x.is_integer()))],
    );
}

fn benchmark_is_integer_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.is_integer()", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |x| no_out!(x.is_integer()))],
    );
}
