use common::GenerationMode;
use inputs::base::unsigneds;
use malachite_base::num::{PrimitiveUnsigned, SplitInHalf};
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

fn demo_unsigned_lower_half<T: 'static + PrimitiveUnsigned + SplitInHalf>(
    gm: GenerationMode,
    limit: usize,
) where
    T::Half: PrimitiveUnsigned,
{
    for u in unsigneds::<T>(gm).take(limit) {
        println!("{}.lower_half() = {}", u, u.lower_half());
    }
}

fn benchmark_unsigned_lower_half<T: 'static + PrimitiveUnsigned + SplitInHalf>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::Half: PrimitiveUnsigned,
{
    println!("benchmarking {} {}.lower_half()", gm.name(), T::NAME,);
    benchmark_1(BenchmarkOptions1 {
        xs: unsigneds(gm),
        function_f: &mut (|u: T| u.lower_half()),
        x_cons: &(|&u| u),
        x_param: &(|&u| u.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: &format!("{}.lower_half()", T::NAME,),
        x_axis_label: "u.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

macro_rules! unsigned {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_lower_half::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_lower_half::<$t>(gm, limit, file_name);
        }
    }
}

unsigned!(u16, demo_u16_lower_half, benchmark_u16_lower_half);
unsigned!(u32, demo_u32_lower_half, benchmark_u32_lower_half);
unsigned!(u64, demo_u64_lower_half, benchmark_u64_lower_half);
