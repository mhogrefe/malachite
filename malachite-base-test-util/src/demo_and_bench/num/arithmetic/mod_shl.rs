use malachite_base::num::arithmetic::traits::{ModShl, ModShlAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::triple_2_3_product_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    unsigned_signed_unsigned_triple_gen_var_2, unsigned_triple_gen_var_18,
};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_mod_shl_unsigned_unsigned);
    register_unsigned_signed_demos!(runner, demo_mod_shl_unsigned_signed);
    register_unsigned_unsigned_demos!(runner, demo_mod_shl_assign_unsigned_unsigned);
    register_unsigned_signed_demos!(runner, demo_mod_shl_assign_unsigned_signed);

    register_unsigned_unsigned_benches!(runner, benchmark_mod_shl_unsigned_unsigned);
    register_unsigned_signed_benches!(runner, benchmark_mod_shl_unsigned_signed);
    register_unsigned_unsigned_benches!(runner, benchmark_mod_shl_assign_unsigned_unsigned);
    register_unsigned_signed_benches!(runner, benchmark_mod_shl_assign_unsigned_signed);
}

fn demo_mod_shl_unsigned_unsigned<
    T: ModShl<U, Output = T> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (x, u, m) in unsigned_triple_gen_var_18::<T, U>()
        .get(gm, &config)
        .take(limit)
    {
        println!("{} << {} === {} mod {}", x, u, x.mod_shl(u, m), m);
    }
}

fn demo_mod_shl_unsigned_signed<
    T: ModShl<U, Output = T> + PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (x, u, m) in unsigned_signed_unsigned_triple_gen_var_2::<T, U>()
        .get(gm, &config)
        .take(limit)
    {
        println!("{} << {} === {} mod {}", x, u, x.mod_shl(u, m), m);
    }
}

fn demo_mod_shl_assign_unsigned_unsigned<
    T: ModShlAssign<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut x, u, m) in unsigned_triple_gen_var_18::<T, U>()
        .get(gm, &config)
        .take(limit)
    {
        let old_x = x;
        x.mod_shl_assign(u, m);
        println!("x := {}; x.mod_shl_assign({}, {}); x = {}", old_x, u, m, x);
    }
}

fn demo_mod_shl_assign_unsigned_signed<
    T: ModShlAssign<U> + PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut x, u, m) in unsigned_signed_unsigned_triple_gen_var_2::<T, U>()
        .get(gm, &config)
        .take(limit)
    {
        let old_x = x;
        x.mod_shl_assign(u, m);
        println!("x := {}; x.mod_shl_assign({}, {}); x = {}", old_x, u, m, x);
    }
}

fn benchmark_mod_shl_unsigned_unsigned<
    T: ModShl<U, Output = T> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_shl({}, {})", T::NAME, U::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_18::<T, U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_product_bit_bucketer("u", "m"),
        &mut [("Malachite", &mut |(x, pow, m)| no_out!(x.mod_shl(pow, m)))],
    );
}

fn benchmark_mod_shl_unsigned_signed<
    T: ModShl<U, Output = T> + PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_shl({}, {})", T::NAME, U::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_signed_unsigned_triple_gen_var_2::<T, U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_product_bit_bucketer("u", "m"),
        &mut [("Malachite", &mut |(x, u, m)| no_out!(x.mod_shl(u, m)))],
    );
}

fn benchmark_mod_shl_assign_unsigned_unsigned<
    T: ModShlAssign<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_shl_assign({}, u64)", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_18::<T, U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_product_bit_bucketer("u", "m"),
        &mut [("Malachite", &mut |(mut x, u, m)| x.mod_shl_assign(u, m))],
    );
}

fn benchmark_mod_shl_assign_unsigned_signed<
    T: ModShlAssign<U> + PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_shl_assign({}, u64)", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_signed_unsigned_triple_gen_var_2::<T, U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_product_bit_bucketer("u", "m"),
        &mut [("Malachite", &mut |(mut x, u, m)| x.mod_shl_assign(u, m))],
    );
}
