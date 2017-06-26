use common::gmp_natural_to_native;
use malachite_native::natural as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::bools::exhaustive_bools;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, log_pairs, random_triples};
use std::cmp::max;

pub fn demo_exhaustive_natural_assign_bit(limit: usize) {
    for ((mut n, index), bit) in
        log_pairs(exhaustive_pairs(exhaustive_naturals(), exhaustive_u::<u64>()),
                  exhaustive_bools())
                .take(limit) {
        let n_old = n.clone();
        n.assign_bit(index, bit);
        println!("x := {}; x.assign_bit({}, {}); x = {}",
                 n_old,
                 index,
                 bit,
                 n);
    }
}

pub fn demo_random_natural_assign_bit(limit: usize) {
    for (mut n, index, bit) in
        random_triples(&EXAMPLE_SEED,
                       &(|seed| random_naturals(seed, 32)),
                       &(|seed| natural_u32s_geometric(seed, 32).map(|i| i as u64)),
                       &(|seed| random_x(seed)))
                .take(limit) {
        let n_old = n.clone();
        n.assign_bit(index, bit);
        println!("x := {}; x.assign_bit({}, {}); x = {}",
                 n_old,
                 index,
                 bit,
                 n);
    }
}

pub fn benchmark_exhaustive_natural_assign_bit(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.assign_bit(u64)");
    benchmark_2(BenchmarkOptions2 {
                    xs: log_pairs(exhaustive_pairs(exhaustive_naturals(), exhaustive_u::<u64>()),
                                  exhaustive_bools()),
                    function_f: &(|((mut n, index), bit)| n.assign_bit(index, bit)),
                    function_g: &(|((mut n, index), bit): ((native::Natural, u64), bool)| {
                                      n.assign_bit(index, bit)
                                  }),
                    x_to_y: &(|&((ref n, index), bit)| ((gmp_natural_to_native(n), index), bit)),
                    x_param: &(|&((ref n, index), _)| max(n.significant_bits(), index) as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "Natural.assign\\\\_bit(u64, bool)",
                    x_axis_label: "max(n.significant\\\\_bits(), index)",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_natural_assign_bit(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.assign_bit(u64, bool)");
    benchmark_2(BenchmarkOptions2 {
                    xs: random_triples(&EXAMPLE_SEED,
                                       &(|seed| random_naturals(seed, scale)),
                                       &(|seed| {
                                             natural_u32s_geometric(seed, scale).map(|i| i as u64)
                                         }),
                                       &(|seed| random_x(seed))),
                    function_f: &(|(mut n, index, bit)| n.assign_bit(index, bit)),
                    function_g: &(|(mut n, index, bit): (native::Natural, u64, bool)| {
                                      n.assign_bit(index, bit)
                                  }),
                    x_to_y: &(|&(ref n, index, bit)| (gmp_natural_to_native(n), index, bit)),
                    x_param: &(|&(ref n, index, _)| max(n.significant_bits(), index) as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "Natural.assign\\\\_bit(u64, bool)",
                    x_axis_label: "max(n.significant\\\\_bits(), index)",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
