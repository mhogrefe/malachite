use common::gmp_integer_to_native;
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};

pub fn demo_exhaustive_integer_mod_power_of_2_assign(limit: usize) {
    for (mut n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        n.mod_power_of_2_assign(u);
        println!("x := {}; x.mod_power_of_2_assign({}); x = {}", n_old, u, n);
    }
}

pub fn demo_random_integer_mod_power_of_2_assign(limit: usize) {
    for (mut n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        n.mod_power_of_2_assign(u);
        println!("x := {}; x.mod_power_of_2_assign({}); x = {}", n_old, u, n);
    }
}

pub fn demo_exhaustive_integer_mod_power_of_2(limit: usize) {
    for (n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        println!("{}.mod_power_of_2({}) = {}", n_old, u, n.mod_power_of_2(u));
    }
}

pub fn demo_random_integer_mod_power_of_2(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{}.mod_power_of_2({}) = {}", n_old, u, n.mod_power_of_2(u));
    }
}

pub fn demo_exhaustive_integer_mod_power_of_2_ref(limit: usize) {
    for (n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        println!(
            "{}.mod_power_of_2_ref({}) = {}",
            n,
            u,
            n.mod_power_of_2_ref(u)
        );
    }
}

pub fn demo_random_integer_mod_power_of_2_ref(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        println!(
            "{}.mod_power_of_2_ref({}) = {}",
            n,
            u,
            n.mod_power_of_2_ref(u)
        );
    }
}

pub fn demo_exhaustive_integer_rem_power_of_2_assign(limit: usize) {
    for (mut n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        n.rem_power_of_2_assign(u);
        println!("x := {}; x.rem_power_of_2_assign({}); x = {}", n_old, u, n);
    }
}

pub fn demo_random_integer_rem_power_of_2_assign(limit: usize) {
    for (mut n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        n.rem_power_of_2_assign(u);
        println!("x := {}; x.rem_power_of_2_assign({}); x = {}", n_old, u, n);
    }
}

pub fn demo_exhaustive_integer_rem_power_of_2(limit: usize) {
    for (n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        println!("{}.rem_power_of_2({}) = {}", n_old, u, n.rem_power_of_2(u));
    }
}

pub fn demo_random_integer_rem_power_of_2(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{}.rem_power_of_2({}) = {}", n_old, u, n.rem_power_of_2(u));
    }
}

pub fn demo_exhaustive_integer_rem_power_of_2_ref(limit: usize) {
    for (n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        println!(
            "{}.rem_power_of_2_ref({}) = {}",
            n,
            u,
            n.rem_power_of_2_ref(u)
        );
    }
}

pub fn demo_random_integer_rem_power_of_2_ref(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        println!(
            "{}.rem_power_of_2_ref({}) = {}",
            n,
            u,
            n.rem_power_of_2_ref(u)
        );
    }
}

pub fn demo_exhaustive_integer_ceiling_mod_power_of_2_assign(limit: usize) {
    for (mut n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        n.ceiling_mod_power_of_2_assign(u);
        println!(
            "x := {}; x.ceiling_mod_power_of_2_assign({}); x = {}",
            n_old,
            u,
            n
        );
    }
}

pub fn demo_random_integer_ceiling_mod_power_of_2_assign(limit: usize) {
    for (mut n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        n.ceiling_mod_power_of_2_assign(u);
        println!(
            "x := {}; x.ceiling_mod_power_of_2_assign({}); x = {}",
            n_old,
            u,
            n
        );
    }
}

pub fn demo_exhaustive_integer_ceiling_mod_power_of_2(limit: usize) {
    for (n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_mod_power_of_2({}) = {}",
            n_old,
            u,
            n.ceiling_mod_power_of_2(u)
        );
    }
}

pub fn demo_random_integer_ceiling_mod_power_of_2(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.ceiling_mod_power_of_2({}) = {}",
            n_old,
            u,
            n.ceiling_mod_power_of_2(u)
        );
    }
}

pub fn demo_exhaustive_integer_ceiling_mod_power_of_2_ref(limit: usize) {
    for (n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        println!(
            "{}.ceiling_mod_power_of_2_ref({}) = {}",
            n,
            u,
            n.ceiling_mod_power_of_2_ref(u)
        );
    }
}

pub fn demo_random_integer_ceiling_mod_power_of_2_ref(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        println!(
            "{}.ceiling_mod_power_of_2_ref({}) = {}",
            n,
            u,
            n.ceiling_mod_power_of_2_ref(u)
        );
    }
}

pub fn benchmark_exhaustive_integer_mod_power_of_2_assign(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.mod_power_of_2_assign(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(mut n, u): (gmp::Integer, u32)| n.mod_power_of_2_assign(u)),
        function_g: &(|(mut n, u): (native::Integer, u32)| n.mod_power_of_2_assign(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.mod\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_mod_power_of_2_assign(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.mod_power_of_2_assign(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(mut n, u): (gmp::Integer, u32)| n.mod_power_of_2_assign(u)),
        function_g: &(|(mut n, u): (native::Integer, u32)| n.mod_power_of_2_assign(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.mod\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_mod_power_of_2(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.mod_power_of_2(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(n, u): (gmp::Integer, u32)| n.mod_power_of_2(u)),
        function_g: &(|(n, u): (native::Integer, u32)| n.mod_power_of_2(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.mod\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_mod_power_of_2(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.mod_power_of_2(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u): (gmp::Integer, u32)| n.mod_power_of_2(u)),
        function_g: &(|(n, u): (native::Integer, u32)| n.mod_power_of_2(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.mod\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_mod_power_of_2_ref(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.mod_power_of_2_ref(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(n, u): (gmp::Integer, u32)| n.mod_power_of_2_ref(u)),
        function_g: &(|(n, u): (native::Integer, u32)| n.mod_power_of_2_ref(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.mod\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_mod_power_of_2_ref(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.mod_power_of_2_ref(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u): (gmp::Integer, u32)| n.mod_power_of_2_ref(u)),
        function_g: &(|(n, u): (native::Integer, u32)| n.mod_power_of_2_ref(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.mod\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_rem_power_of_2_assign(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.rem_power_of_2_assign(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(mut n, u): (gmp::Integer, u32)| n.rem_power_of_2_assign(u)),
        function_g: &(|(mut n, u): (native::Integer, u32)| n.rem_power_of_2_assign(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.rem\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_rem_power_of_2_assign(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.rem_power_of_2_assign(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(mut n, u): (gmp::Integer, u32)| n.rem_power_of_2_assign(u)),
        function_g: &(|(mut n, u): (native::Integer, u32)| n.rem_power_of_2_assign(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.rem\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_rem_power_of_2(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.rem_power_of_2(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(n, u): (gmp::Integer, u32)| n.rem_power_of_2(u)),
        function_g: &(|(n, u): (native::Integer, u32)| n.rem_power_of_2(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.rem\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_rem_power_of_2(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.rem_power_of_2(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u): (gmp::Integer, u32)| n.rem_power_of_2(u)),
        function_g: &(|(n, u): (native::Integer, u32)| n.rem_power_of_2(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.rem\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_rem_power_of_2_ref(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.rem_power_of_2_ref(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(n, u): (gmp::Integer, u32)| n.rem_power_of_2_ref(u)),
        function_g: &(|(n, u): (native::Integer, u32)| n.rem_power_of_2_ref(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.rem\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_rem_power_of_2_ref(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.rem_power_of_2_ref(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u): (gmp::Integer, u32)| n.rem_power_of_2_ref(u)),
        function_g: &(|(n, u): (native::Integer, u32)| n.rem_power_of_2_ref(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.rem\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}


pub fn benchmark_exhaustive_integer_ceiling_mod_power_of_2_assign(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.ceiling_mod_power_of_2_assign(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(mut n, u): (gmp::Integer, u32)| n.ceiling_mod_power_of_2_assign(u)),
        function_g: &(|(mut n, u): (native::Integer, u32)| n.ceiling_mod_power_of_2_assign(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.ceiling\\\\_mod\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_ceiling_mod_power_of_2_assign(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.ceiling_mod_power_of_2_assign(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(mut n, u): (gmp::Integer, u32)| n.ceiling_mod_power_of_2_assign(u)),
        function_g: &(|(mut n, u): (native::Integer, u32)| n.ceiling_mod_power_of_2_assign(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.ceiling\\\\_mod\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_ceiling_mod_power_of_2(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.ceiling_mod_power_of_2(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(n, u): (gmp::Integer, u32)| n.ceiling_mod_power_of_2(u)),
        function_g: &(|(n, u): (native::Integer, u32)| n.ceiling_mod_power_of_2(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.ceiling\\\\_mod\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_ceiling_mod_power_of_2(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.ceiling_mod_power_of_2(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u): (gmp::Integer, u32)| n.ceiling_mod_power_of_2(u)),
        function_g: &(|(n, u): (native::Integer, u32)| n.ceiling_mod_power_of_2(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.ceiling\\\\_mod\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_ceiling_mod_power_of_2_ref(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.ceiling_mod_power_of_2_ref(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(n, u): (gmp::Integer, u32)| n.ceiling_mod_power_of_2_ref(u)),
        function_g: &(|(n, u): (native::Integer, u32)| n.ceiling_mod_power_of_2_ref(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.ceiling\\\\_mod\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_ceiling_mod_power_of_2_ref(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.ceiling_mod_power_of_2_ref(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u): (gmp::Integer, u32)| n.ceiling_mod_power_of_2_ref(u)),
        function_g: &(|(n, u): (native::Integer, u32)| n.ceiling_mod_power_of_2_ref(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.ceiling\\\\_mod\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
