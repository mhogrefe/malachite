use common::GenerationMode;
use inputs::base::unsigneds;
use malachite_base::num::{PrimitiveUnsigned, SplitInHalf};
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

fn demo_unsigned_upper_half<T: 'static + PrimitiveUnsigned + SplitInHalf>(
    gm: GenerationMode,
    limit: usize,
) where
    T::Half: PrimitiveUnsigned,
{
    for u in unsigneds::<T>(gm).take(limit) {
        println!("{}.upper_half() = {}", u, u.upper_half());
    }
}

fn benchmark_unsigned_upper_half<T: 'static + PrimitiveUnsigned + SplitInHalf>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::Half: PrimitiveUnsigned,
{
    println!("benchmarking {} {}.upper_half()", gm.name(), T::NAME,);
    benchmark_1(BenchmarkOptions1 {
        xs: unsigneds(gm),
        function_f: &mut (|u: T| u.upper_half()),
        x_cons: &(|&u| u),
        x_param: &(|&u| u.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: &format!("{}.upper_half()", T::NAME,),
        x_axis_label: "u.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

macro_rules! unsigned {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_upper_half::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_upper_half::<$t>(gm, limit, file_name);
        }
    }
}

unsigned!(u16, demo_u16_upper_half, benchmark_u16_upper_half);
unsigned!(u32, demo_u32_upper_half, benchmark_u32_upper_half);
unsigned!(u64, demo_u64_upper_half, benchmark_u64_upper_half);
