use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{primitive_float_bucketer, signed_bit_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen_var_1};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_signed_demos!(runner, demo_neg_assign_signed);
    register_primitive_float_demos!(runner, demo_neg_assign_primitive_float);

    register_signed_benches!(runner, benchmark_neg_assign_signed);
    register_primitive_float_benches!(runner, benchmark_neg_assign_primitive_float);
}

fn demo_neg_assign_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for mut i in signed_gen_var_1::<T>().get(gm, &config).take(limit) {
        let old_i = i;
        i.neg_assign();
        println!("i := {}; i.neg_assign(); i = {}", old_i, i);
    }
}

fn demo_neg_assign_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for mut x in primitive_float_gen::<T>().get(gm, &config).take(limit) {
        let old_x = x;
        x.neg_assign();
        println!(
            "x := {}; x.neg_assign(); x = {}",
            NiceFloat(old_x),
            NiceFloat(x)
        );
    }
}

fn benchmark_neg_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.neg_assign()", T::NAME),
        BenchmarkType::Single,
        signed_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |mut i| i.neg_assign())],
    );
}

fn benchmark_neg_assign_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.neg_assign()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.neg_assign())],
    );
}
