use common::GenerationMode;
use malachite_base::num::PrimitiveUnsigned;
use inputs::base::positive_unsigneds;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

fn demo_unsigned_floor_log_two<T: 'static + PrimitiveUnsigned>(gm: GenerationMode, limit: usize) {
    for n in positive_unsigneds::<T>(gm).take(limit) {
        println!("{}.floor_log_two() = {}", n, n.floor_log_two());
    }
}

fn demo_unsigned_ceiling_log_two<T: 'static + PrimitiveUnsigned>(gm: GenerationMode, limit: usize) {
    for n in positive_unsigneds::<T>(gm).take(limit) {
        println!("{}.ceiling_log_two() = {}", n, n.ceiling_log_two());
    }
}

fn benchmark_unsigned_floor_log_two<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} {}.floor_log_two()", gm.name(), T::NAME);
    benchmark_1(BenchmarkOptions1 {
        xs: positive_unsigneds(gm),
        function_f: &mut (|n: T| n.floor_log_two()),
        x_cons: &(|&n| n),
        x_param: &(|&n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: &format!("{}.floor_log_two()", T::NAME),
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

fn benchmark_unsigned_ceiling_log_two<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} {}.ceiling_log_two()", gm.name(), T::NAME);
    benchmark_1(BenchmarkOptions1 {
        xs: positive_unsigneds(gm),
        function_f: &mut (|n: T| n.ceiling_log_two()),
        x_cons: &(|&n| n),
        x_param: &(|&n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: &format!("{}.ceiling_log_two()", T::NAME),
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

macro_rules! unsigned {
    (
        $t: ident,
        $demo_name_floor: ident,
        $demo_name_ceiling: ident,
        $bench_name_floor: ident,
        $bench_name_ceiling: ident
    ) => {
        pub fn $demo_name_floor(gm: GenerationMode, limit: usize) {
            demo_unsigned_floor_log_two::<$t>(gm, limit);
        }

        pub fn $demo_name_ceiling(gm: GenerationMode, limit: usize) {
            demo_unsigned_ceiling_log_two::<$t>(gm, limit);
        }

        pub fn $bench_name_floor(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_floor_log_two::<$t>(gm, limit, file_name);
        }

        pub fn $bench_name_ceiling(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_ceiling_log_two::<$t>(gm, limit, file_name);
        }
    }
}

unsigned!(
    u8,
    demo_u8_floor_log_two,
    demo_u8_ceiling_log_two,
    benchmark_u8_floor_log_two,
    benchmark_u8_ceiling_log_two
);
unsigned!(
    u16,
    demo_u16_floor_log_two,
    demo_u16_ceiling_log_two,
    benchmark_u16_floor_log_two,
    benchmark_u16_ceiling_log_two
);
unsigned!(
    u32,
    demo_u32_floor_log_two,
    demo_u32_ceiling_log_two,
    benchmark_u32_floor_log_two,
    benchmark_u32_ceiling_log_two
);
unsigned!(
    u64,
    demo_u64_floor_log_two,
    demo_u64_ceiling_log_two,
    benchmark_u64_floor_log_two,
    benchmark_u64_ceiling_log_two
);
