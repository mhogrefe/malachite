use malachite_base::num::arithmetic::traits::{ModPowerOf2Shr, ModPowerOf2ShrAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::triple_3_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_signed_unsigned_triple_gen_var_1;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_signed_demos!(runner, demo_mod_power_of_2_shr);
    register_unsigned_signed_demos!(runner, demo_mod_power_of_2_shr_assign);

    register_unsigned_signed_benches!(runner, benchmark_mod_power_of_2_shr);
    register_unsigned_signed_benches!(runner, benchmark_mod_power_of_2_shr_assign);
}

fn demo_mod_power_of_2_shr<
    T: ModPowerOf2Shr<U, Output = T> + PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, u, pow) in unsigned_signed_unsigned_triple_gen_var_1::<T, U>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}.mod_power_of_2_shr({}, {}) = {}",
            n,
            u,
            pow,
            n.mod_power_of_2_shr(u, pow)
        );
    }
}

fn demo_mod_power_of_2_shr_assign<
    T: ModPowerOf2ShrAssign<U> + PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut n, u, pow) in unsigned_signed_unsigned_triple_gen_var_1::<T, U>()
        .get(gm, &config)
        .take(limit)
    {
        let old_n = n;
        n.mod_power_of_2_shr_assign(u, pow);
        println!(
            "x := {}; x.mod_power_of_2_shr_assign({}, {}); x = {}",
            old_n, u, pow, n
        );
    }
}

fn benchmark_mod_power_of_2_shr<
    T: ModPowerOf2Shr<U, Output = T> + PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_shr({}, u64)", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_signed_unsigned_triple_gen_var_1::<T, U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(x, y, pow)| {
            no_out!(x.mod_power_of_2_shr(y, pow))
        })],
    );
}

fn benchmark_mod_power_of_2_shr_assign<
    T: ModPowerOf2ShrAssign<U> + PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_shr_assign({}, u64)", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_signed_unsigned_triple_gen_var_1::<T, U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(mut x, y, pow)| {
            x.mod_power_of_2_shr_assign(y, pow)
        })],
    );
}
