use common::GenerationMode;
use inputs::base::chars_var_1;
use malachite_base::chars::char_to_contiguous_range;
use malachite_base::num::Walkable;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_char_decrement(gm: GenerationMode, limit: usize) {
    for mut c in chars_var_1(gm).take(limit) {
        let c_old = c;
        c.decrement();
        println!("c := {:?}; x.decrement(); c = {:?}", c_old, c);
    }
}

pub fn benchmark_char_decrement(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} char.decrement()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: chars_var_1(gm),
        function_f: &(|mut c: char| c.decrement()),
        x_cons: &(|&c| c),
        x_param: &(|&c| char_to_contiguous_range(c) as usize),
        limit,
        f_name: "malachite",
        title: "char.decrement()",
        x_axis_label: "char_to_contiguous_range(char)",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
