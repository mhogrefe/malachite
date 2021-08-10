use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::triple_max_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_triple_gen, unsigned_triple_gen_var_19};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_checked_sub_mul_unsigned);
    register_signed_demos!(runner, demo_checked_sub_mul_signed);

    register_unsigned_benches!(runner, benchmark_checked_sub_mul_unsigned);
    register_signed_benches!(runner, benchmark_checked_sub_mul_signed);
}

fn demo_checked_sub_mul_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (x, y, z) in unsigned_triple_gen_var_19::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}.checked_sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            x.checked_sub_mul(y, z)
        );
    }
}

fn demo_checked_sub_mul_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, z) in signed_triple_gen::<T>().get(gm, &config).take(limit) {
        println!(
            "({}).checked_sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            x.checked_sub_mul(y, z)
        );
    }
}

fn benchmark_checked_sub_mul_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_sub_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_19::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| {
            no_out!(x.checked_sub_mul(y, z))
        })],
    );
}

fn benchmark_checked_sub_mul_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_sub_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_triple_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| {
            no_out!(x.checked_sub_mul(y, z))
        })],
    );
}
