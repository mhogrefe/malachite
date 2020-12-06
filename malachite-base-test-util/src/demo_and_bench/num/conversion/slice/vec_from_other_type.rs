use std::fmt::Debug;

use malachite_base::named::Named;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::VecFromOtherType;

use malachite_base_test_util::bench::bucketers::unsigned_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_gen;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_unsigned_vec_from_other_type);
    register_unsigned_unsigned_benches!(runner, benchmark_unsigned_vec_from_other_type);
}

fn demo_unsigned_vec_from_other_type<
    T: Debug + VecFromOtherType<U> + Named,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<U>().get(gm, &config).take(limit) {
        println!(
            "{}::vec_from_other_type({}) = {:?}",
            T::NAME,
            u,
            T::vec_from_other_type(u)
        );
    }
}

fn benchmark_unsigned_vec_from_other_type<T: VecFromOtherType<U> + Named, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.vec_from_other_type({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_gen::<U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::vec_from_other_type(n)))],
    );
}
