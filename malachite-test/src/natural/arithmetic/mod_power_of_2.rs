use common::gmp_natural_to_native;
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};

pub fn demo_exhaustive_natural_mod_power_of_2_assign(limit: usize) {
    for (mut n, u) in log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        n.mod_power_of_2_assign(u);
        println!("x := {}; x.mod_power_of_2_assign({}); x = {}", n_old, u, n);
    }
}

pub fn demo_random_natural_mod_power_of_2_assign(limit: usize) {
    for (mut n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        n.mod_power_of_2_assign(u);
        println!("x := {}; x.mod_power_of_2_assign({}); x = {}", n_old, u, n);
    }
}

pub fn demo_exhaustive_natural_mod_power_of_2(limit: usize) {
    for (n, u) in log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        println!("{}.mod_power_of_2({}) = {}", n_old, u, n.mod_power_of_2(u));
    }
}

pub fn demo_random_natural_mod_power_of_2(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{}.mod_power_of_2({}) = {}", n_old, u, n.mod_power_of_2(u));
    }
}

pub fn demo_exhaustive_natural_mod_power_of_2_ref(limit: usize) {
    for (n, u) in log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(limit) {
        println!(
            "{}.mod_power_of_2_ref({}) = {}",
            n,
            u,
            n.mod_power_of_2_ref(u)
        );
    }
}

pub fn demo_random_natural_mod_power_of_2_ref(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
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

pub fn demo_exhaustive_natural_complement_mod_power_of_2_assign(limit: usize) {
    for (mut n, u) in log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        n.complement_mod_power_of_2_assign(u);
        println!(
            "x := {}; x.complement_mod_power_of_2_assign({}); x = {}",
            n_old,
            u,
            n
        );
    }
}

pub fn demo_random_natural_complement_mod_power_of_2_assign(limit: usize) {
    for (mut n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        n.complement_mod_power_of_2_assign(u);
        println!(
            "x := {}; x.complement_mod_power_of_2_assign({}); x = {}",
            n_old,
            u,
            n
        );
    }
}

pub fn demo_exhaustive_natural_complement_mod_power_of_2(limit: usize) {
    for (n, u) in log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.complement_mod_power_of_2({}) = {}",
            n_old,
            u,
            n.complement_mod_power_of_2(u)
        );
    }
}

pub fn demo_random_natural_complement_mod_power_of_2(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.complement_mod_power_of_2({}) = {}",
            n_old,
            u,
            n.complement_mod_power_of_2(u)
        );
    }
}

pub fn demo_exhaustive_natural_complement_mod_power_of_2_ref(limit: usize) {
    for (n, u) in log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(limit) {
        println!(
            "{}.complement_mod_power_of_2_ref({}) = {}",
            n,
            u,
            n.complement_mod_power_of_2_ref(u)
        );
    }
}

pub fn demo_random_natural_complement_mod_power_of_2_ref(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        println!(
            "{}.complement_mod_power_of_2_ref({}) = {}",
            n,
            u,
            n.complement_mod_power_of_2_ref(u)
        );
    }
}

pub fn benchmark_exhaustive_natural_mod_power_of_2_assign(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.mod_power_of_2_assign(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
        function_f: &(|(mut n, u): (gmp::Natural, u32)| n.mod_power_of_2_assign(u)),
        function_g: &(|(mut n, u): (native::Natural, u32)| n.mod_power_of_2_assign(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.mod\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_mod_power_of_2_assign(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.mod_power_of_2_assign(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(mut n, u): (gmp::Natural, u32)| n.mod_power_of_2_assign(u)),
        function_g: &(|(mut n, u): (native::Natural, u32)| n.mod_power_of_2_assign(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.mod\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_mod_power_of_2(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.mod_power_of_2(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
        function_f: &(|(n, u): (gmp::Natural, u32)| n.mod_power_of_2(u)),
        function_g: &(|(n, u): (native::Natural, u32)| n.mod_power_of_2(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.mod\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_mod_power_of_2(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.mod_power_of_2(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u): (gmp::Natural, u32)| n.mod_power_of_2(u)),
        function_g: &(|(n, u): (native::Natural, u32)| n.mod_power_of_2(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.mod\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_mod_power_of_2_ref(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.mod_power_of_2_ref(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
        function_f: &(|(n, u): (gmp::Natural, u32)| n.mod_power_of_2_ref(u)),
        function_g: &(|(n, u): (native::Natural, u32)| n.mod_power_of_2_ref(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.mod\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_mod_power_of_2_ref(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.mod_power_of_2_ref(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u): (gmp::Natural, u32)| n.mod_power_of_2_ref(u)),
        function_g: &(|(n, u): (native::Natural, u32)| n.mod_power_of_2_ref(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.mod\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_complement_mod_power_of_2_assign(
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking exhaustive Natural.complement_mod_power_of_2_assign(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
        function_f: &(|(mut n, u): (gmp::Natural, u32)| n.complement_mod_power_of_2_assign(u)),
        function_g: &(|(mut n, u): (native::Natural, u32)| n.complement_mod_power_of_2_assign(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.complement\\\\_mod\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_complement_mod_power_of_2_assign(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.complement_mod_power_of_2_assign(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(mut n, u): (gmp::Natural, u32)| n.complement_mod_power_of_2_assign(u)),
        function_g: &(|(mut n, u): (native::Natural, u32)| n.complement_mod_power_of_2_assign(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.complement\\\\_mod\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_complement_mod_power_of_2(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.complement_mod_power_of_2(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
        function_f: &(|(n, u): (gmp::Natural, u32)| n.complement_mod_power_of_2(u)),
        function_g: &(|(n, u): (native::Natural, u32)| n.complement_mod_power_of_2(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.complement\\\\_mod\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_complement_mod_power_of_2(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.complement_mod_power_of_2(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u): (gmp::Natural, u32)| n.complement_mod_power_of_2(u)),
        function_g: &(|(n, u): (native::Natural, u32)| n.complement_mod_power_of_2(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.complement\\\\_mod\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_complement_mod_power_of_2_ref(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.complement_mod_power_of_2_ref(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
        function_f: &(|(n, u): (gmp::Natural, u32)| n.complement_mod_power_of_2_ref(u)),
        function_g: &(|(n, u): (native::Natural, u32)| n.complement_mod_power_of_2_ref(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.complement\\\\_mod\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_complement_mod_power_of_2_ref(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.complement_mod_power_of_2_ref(u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u): (gmp::Natural, u32)| n.complement_mod_power_of_2_ref(u)),
        function_g: &(|(n, u): (native::Natural, u32)| n.complement_mod_power_of_2_ref(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.complement\\\\_mod\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
