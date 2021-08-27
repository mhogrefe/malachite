use malachite_base::num::arithmetic::root::{
    _ceiling_root_binary, _checked_root_binary, _floor_root_binary, _root_rem_binary,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::pair_1_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    signed_unsigned_pair_gen_var_18, unsigned_pair_gen_var_32,
};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_floor_root_unsigned);
    register_signed_demos!(runner, demo_floor_root_signed);
    register_unsigned_demos!(runner, demo_floor_root_assign_unsigned);
    register_signed_demos!(runner, demo_floor_root_assign_signed);
    register_unsigned_demos!(runner, demo_ceiling_root_unsigned);
    register_signed_demos!(runner, demo_ceiling_root_signed);
    register_unsigned_demos!(runner, demo_ceiling_root_assign_unsigned);
    register_signed_demos!(runner, demo_ceiling_root_assign_signed);
    register_unsigned_demos!(runner, demo_checked_root_unsigned);
    register_signed_demos!(runner, demo_checked_root_signed);
    register_unsigned_demos!(runner, demo_root_rem);
    register_unsigned_demos!(runner, demo_root_assign_rem);

    register_unsigned_benches!(runner, benchmark_floor_root_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_floor_root_signed);
    register_unsigned_benches!(runner, benchmark_floor_root_assign_unsigned);
    register_signed_benches!(runner, benchmark_floor_root_assign_signed);
    register_unsigned_benches!(runner, benchmark_ceiling_root_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_ceiling_root_signed);
    register_unsigned_benches!(runner, benchmark_ceiling_root_assign_unsigned);
    register_signed_benches!(runner, benchmark_ceiling_root_assign_signed);
    register_unsigned_benches!(runner, benchmark_checked_root_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_checked_root_signed);
    register_unsigned_benches!(runner, benchmark_root_rem_algorithms);
    register_unsigned_benches!(runner, benchmark_root_assign_rem);
}

fn demo_floor_root_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, exp) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        println!("floor_root({}, {}) = {}", n, exp, n.floor_root(exp));
    }
}

fn demo_floor_root_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, exp) in signed_unsigned_pair_gen_var_18::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        println!("floor_root({}, {}) = {}", n, exp, n.floor_root(exp));
    }
}

fn demo_floor_root_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut n, exp) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        let old_n = n;
        n.floor_root_assign(exp);
        println!("n := {}; n.floor_root_assign({}); n = {}", old_n, exp, n);
    }
}

fn demo_floor_root_assign_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut n, exp) in signed_unsigned_pair_gen_var_18::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        let old_n = n;
        n.floor_root_assign(exp);
        println!("n := {}; n.floor_root_assign({}); n = {}", old_n, exp, n);
    }
}

fn demo_ceiling_root_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, exp) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        println!("ceiling_root({}, {}) = {}", n, exp, n.ceiling_root(exp));
    }
}

fn demo_ceiling_root_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, exp) in signed_unsigned_pair_gen_var_18::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        println!("ceiling_root({}, {}) = {}", n, exp, n.ceiling_root(exp));
    }
}

fn demo_ceiling_root_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut n, exp) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        let old_n = n;
        n.floor_root_assign(exp);
        println!("n := {}; n.ceiling_root_assign({}); n = {}", old_n, exp, n);
    }
}

fn demo_ceiling_root_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut n, exp) in signed_unsigned_pair_gen_var_18::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        let old_n = n;
        n.floor_root_assign(exp);
        println!("n := {}; n.ceiling_root_assign({}); n = {}", old_n, exp, n);
    }
}

fn demo_checked_root_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, exp) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        println!("checked_root({}, {}) = {:?}", n, exp, n.checked_root(exp));
    }
}

fn demo_checked_root_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, exp) in signed_unsigned_pair_gen_var_18::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        println!("checked_root({}, {}) = {:?}", n, exp, n.checked_root(exp));
    }
}

fn demo_root_rem<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, exp) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        println!("root_rem({}, {}) = {:?}", n, exp, n.root_rem(exp));
    }
}

fn demo_root_assign_rem<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut n, exp) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        let old_n = n;
        let rem = n.root_assign_rem(exp);
        println!(
            "n := {}; n.root_assign_rem({}) = {}; n = {}",
            old_n, exp, rem, n
        );
    }
}

fn benchmark_floor_root_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_root(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_32::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, exp)| no_out!(n.floor_root(exp))),
            ("binary", &mut |(n, exp)| {
                no_out!(_floor_root_binary(n, exp))
            }),
        ],
    );
}

fn benchmark_floor_root_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_root(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_18::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(n, exp)| no_out!(n.floor_root(exp)))],
    );
}

fn benchmark_floor_root_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_root_assign(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_32::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut n, exp)| n.floor_root_assign(exp))],
    );
}

fn benchmark_floor_root_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_root_assign(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_18::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut n, exp)| n.floor_root_assign(exp))],
    );
}

fn benchmark_ceiling_root_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_root(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_32::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, exp)| no_out!(n.ceiling_root(exp))),
            ("binary", &mut |(n, exp)| {
                no_out!(_ceiling_root_binary(n, exp))
            }),
        ],
    );
}

fn benchmark_ceiling_root_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_root(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_18::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(n, exp)| no_out!(n.ceiling_root(exp)))],
    );
}

fn benchmark_ceiling_root_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_root_assign(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_32::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut n, exp)| n.ceiling_root_assign(exp))],
    );
}

fn benchmark_ceiling_root_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_root_assign(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_18::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut n, exp)| n.ceiling_root_assign(exp))],
    );
}

fn benchmark_checked_root_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_root(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_32::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, exp)| no_out!(n.checked_root(exp))),
            ("binary", &mut |(n, exp)| {
                no_out!(_checked_root_binary(n, exp))
            }),
        ],
    );
}

fn benchmark_checked_root_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_root(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_18::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(n, exp)| no_out!(n.checked_root(exp)))],
    );
}

fn benchmark_root_rem_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.root_rem(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_32::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, exp)| no_out!(n.root_rem(exp))),
            ("binary", &mut |(n, exp)| no_out!(_root_rem_binary(n, exp))),
        ],
    );
}

fn benchmark_root_assign_rem<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.root_assign_rem(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_32::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut n, exp)| {
            no_out!(n.root_assign_rem(exp))
        })],
    );
}
