use common::NoSpecialGenerationMode;
use inputs::base::chars_not_min;
use malachite_base::chars::char_to_contiguous_range;
use malachite_base::misc::Walkable;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_char_decrement(gm: NoSpecialGenerationMode, limit: usize) {
    for mut c in chars_not_min(gm).take(limit) {
        let c_old = c;
        c.decrement();
        println!("c := {:?}; c.decrement(); c = {:?}", c_old, c);
    }
}

pub fn benchmark_char_decrement(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} char.decrement()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: chars_not_min(gm),
        function_f: &mut (|mut c: char| c.decrement()),
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
