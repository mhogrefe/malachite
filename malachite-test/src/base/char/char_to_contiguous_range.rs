use common::NoSpecialGenerationMode;
use inputs::base::chars;
use malachite_base::chars::char_to_contiguous_range;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_char_to_contiguous_range(gm: NoSpecialGenerationMode, limit: usize) {
    for c in chars(gm).take(limit) {
        println!(
            "char_to_contiguous_range({:?}) = {}",
            c,
            char_to_contiguous_range(c)
        );
    }
}

pub fn benchmark_char_to_contiguous_range(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} char_to_contiguous_range(char)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: chars(gm),
        function_f: &mut (|c| char_to_contiguous_range(c)),
        x_cons: &(|&c| c),
        x_param: &(|&c| char_to_contiguous_range(c) as usize),
        limit,
        f_name: "malachite",
        title: "char_to_contiguous_range(char)",
        x_axis_label: "char_to_contiguous_range(char)",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
