use common::GenerationMode;
use inputs::base::chars_var_2;
use malachite_base::chars::char_to_contiguous_range;
use malachite_base::misc::Walkable;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_char_increment(gm: GenerationMode, limit: usize) {
    for mut c in chars_var_2(gm).take(limit) {
        let c_old = c;
        c.increment();
        println!("c := {:?}; x.increment(); c = {:?}", c_old, c);
    }
}

pub fn benchmark_char_increment(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} char.increment()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: chars_var_2(gm),
        function_f: &(|mut c: char| c.increment()),
        x_cons: &(|&c| c),
        x_param: &(|&c| char_to_contiguous_range(c) as usize),
        limit,
        f_name: "malachite",
        title: "char.increment()",
        x_axis_label: "char_to_contiguous_range(char)",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
