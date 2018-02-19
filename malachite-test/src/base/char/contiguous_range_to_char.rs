use common::GenerationMode;
use inputs::base::unsigneds;
use malachite_base::chars::contiguous_range_to_char;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_contiguous_range_to_char(gm: GenerationMode, limit: usize) {
    for i in unsigneds(gm).take(limit) {
        println!(
            "contiguous_range_to_char({}) = {:?}",
            i,
            contiguous_range_to_char(i)
        );
    }
}

pub fn benchmark_contiguous_range_to_char(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} contiguous_range_to_char(char)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: unsigneds(gm),
        function_f: &mut (|i| contiguous_range_to_char(i)),
        x_cons: &(|&i| i),
        x_param: &(|&i| i as usize),
        limit,
        f_name: "malachite",
        title: "contiguous_range_to_char(char)",
        x_axis_label: "i",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
