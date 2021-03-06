use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::triple_max_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_triple_gen_var_2, unsigned_triple_gen_var_2};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_sub_mul_unsigned);
    register_signed_demos!(runner, demo_sub_mul_signed);
    register_unsigned_demos!(runner, demo_sub_mul_assign_unsigned);
    register_signed_demos!(runner, demo_sub_mul_assign_signed);

    register_unsigned_benches!(runner, benchmark_sub_mul_unsigned);
    register_signed_benches!(runner, benchmark_sub_mul_signed);
    register_unsigned_benches!(runner, benchmark_sub_mul_assign_unsigned);
    register_signed_benches!(runner, benchmark_sub_mul_assign_signed);
}

fn demo_sub_mul_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, z) in unsigned_triple_gen_var_2::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!("{}.sub_mul({}, {}) = {}", x, y, z, x.sub_mul(y, z));
    }
}

fn demo_sub_mul_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, z) in signed_triple_gen_var_2::<T>().get(gm, &config).take(limit) {
        println!("({}).sub_mul({}, {}) = {}", x, y, z, x.sub_mul(y, z));
    }
}

fn demo_sub_mul_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut x, y, z) in unsigned_triple_gen_var_2::<T>()
        .get(gm, &config)
        .take(limit)
    {
        let old_x = x;
        x.sub_mul_assign(y, z);
        println!("x := {}; x.sub_mul_assign({}, {}); x = {}", old_x, y, z, x);
    }
}

fn demo_sub_mul_assign_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, y, z) in signed_triple_gen_var_2::<T>().get(gm, &config).take(limit) {
        let old_x = x;
        x.sub_mul_assign(y, z);
        println!("x := {}; x.sub_mul_assign({}, {}); x = {}", old_x, y, z, x);
    }
}

fn benchmark_sub_mul_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sub_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_2::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| no_out!(x.sub_mul(y, z)))],
    );
}

fn benchmark_sub_mul_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sub_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_triple_gen_var_2::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| no_out!(x.sub_mul(y, z)))],
    );
}

fn benchmark_sub_mul_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sub_mul_assign({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_2::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(mut x, y, z)| x.sub_mul_assign(y, z))],
    );
}

fn benchmark_sub_mul_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sub_mul_assign({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_triple_gen_var_2::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(mut x, y, z)| x.sub_mul_assign(y, z))],
    );
}
