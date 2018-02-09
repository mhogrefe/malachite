use common::GenerationMode;
use inputs::base::pairs_of_unsigneds;
use malachite_base::misc::Named;
use malachite_base::num::{JoinHalves, PrimitiveUnsigned, SignificantBits};
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};
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
    println!(
        "benchmarking {} {}::join_halves({}, {})",
        gm.name(),
        T::NAME,
        T::Half::NAME,
        T::Half::NAME
    );
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_unsigneds(gm),
        function_f: &(|(x, y): (T::Half, T::Half)| T::join_halves(x, y)),
        x_cons: &(|&p| p),
        x_param: &(|&(x, y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        title: &format!(
            "{}::join_halves({}, {})",
            T::NAME,
            T::Half::NAME,
            T::Half::NAME
        ),
        x_axis_label: "max(x.significant_bits(), y.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

macro_rules! unsigned {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_join_halves::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_join_halves::<$t>(gm, limit, file_name);
        }
    }
}

unsigned!(u16, demo_u16_join_halves, benchmark_u16_join_halves);
unsigned!(u32, demo_u32_join_halves, benchmark_u32_join_halves);
unsigned!(u64, demo_u64_join_halves, benchmark_u64_join_halves);
