use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::base::pairs_of_unsigneds;
use malachite_base::misc::Named;
use malachite_base::num::{JoinHalves, PrimitiveUnsigned, SignificantBits};
use std::cmp::max;

fn demo_unsigned_join_halves<T: 'static + JoinHalves + PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
) where
    T::Half: PrimitiveUnsigned,
{
    for (x, y) in pairs_of_unsigneds::<T::Half>(gm).take(limit) {
        println!(
            "{}::join_halves({}, {}) = {}",
            T::NAME,
            x,
            y,
            T::join_halves(x, y)
        );
    }
}

fn benchmark_unsigned_join_halves<T: 'static + JoinHalves + PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::Half: PrimitiveUnsigned,
{
    m_run_benchmark(
        &format!(
            "{}::join_halves({}, {})",
            T::NAME,
            T::Half::NAME,
            T::Half::NAME
        ),
        BenchmarkType::Single,
        pairs_of_unsigneds::<T::Half>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(T::join_halves(x, y))))],
    );
}

macro_rules! unsigned {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_join_halves::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_join_halves::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u16, demo_u16_join_halves, benchmark_u16_join_halves);
unsigned!(u32, demo_u32_join_halves, benchmark_u32_join_halves);
unsigned!(u64, demo_u64_join_halves, benchmark_u64_join_halves);
