use common::GenerationMode;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};

type It = Iterator<Item = (Integer, u32)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(log_pairs(exhaustive_integers(), exhaustive_u::<u32>()))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| natural_u32s_geometric(seed, scale)),
    ))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_integer_mod_power_of_2_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        n.mod_power_of_2_assign(u);
        println!("x := {}; x.mod_power_of_2_assign({}); x = {}", n_old, u, n);
    }
}

pub fn demo_integer_mod_power_of_2(gm: GenerationMode, limit: usize) {
    for (n, u) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod_power_of_2({}) = {}", n_old, u, n.mod_power_of_2(u));
    }
}

pub fn demo_integer_mod_power_of_2_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in select_inputs(gm).take(limit) {
        println!(
            "{}.mod_power_of_2_ref({}) = {}",
            n,
            u,
            n.mod_power_of_2_ref(u)
        );
    }
}

pub fn demo_integer_rem_power_of_2_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        n.rem_power_of_2_assign(u);
        println!("x := {}; x.rem_power_of_2_assign({}); x = {}", n_old, u, n);
    }
}

pub fn demo_integer_rem_power_of_2(gm: GenerationMode, limit: usize) {
    for (n, u) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.rem_power_of_2({}) = {}", n_old, u, n.rem_power_of_2(u));
    }
}

pub fn demo_integer_rem_power_of_2_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in select_inputs(gm).take(limit) {
        println!(
            "{}.rem_power_of_2_ref({}) = {}",
            n,
            u,
            n.rem_power_of_2_ref(u)
        );
    }
}

pub fn demo_integer_ceiling_mod_power_of_2_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        n.ceiling_mod_power_of_2_assign(u);
        println!(
            "x := {}; x.ceiling_mod_power_of_2_assign({}); x = {}",
            n_old, u, n
        );
    }
}

pub fn demo_integer_ceiling_mod_power_of_2(gm: GenerationMode, limit: usize) {
    for (n, u) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_mod_power_of_2({}) = {}",
            n_old,
            u,
            n.ceiling_mod_power_of_2(u)
        );
    }
}

pub fn demo_integer_ceiling_mod_power_of_2_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in select_inputs(gm).take(limit) {
        println!(
            "{}.ceiling_mod_power_of_2_ref({}) = {}",
            n,
            u,
            n.ceiling_mod_power_of_2_ref(u)
        );
    }
}

pub fn benchmark_integer_mod_power_of_2_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.mod_power_of_2_assign(u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(mut n, u): (Integer, u32)| n.mod_power_of_2_assign(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.mod\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_mod_power_of_2(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.mod_power_of_2(u32)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(n, u): (Integer, u32)| n.mod_power_of_2(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.mod\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_mod_power_of_2_ref(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.mod_power_of_2_ref(u32)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(n, u): (Integer, u32)| n.mod_power_of_2_ref(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.mod\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_rem_power_of_2_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.rem_power_of_2_assign(u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(mut n, u): (Integer, u32)| n.rem_power_of_2_assign(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.rem\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_rem_power_of_2(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.rem_power_of_2(u32)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(n, u): (Integer, u32)| n.rem_power_of_2(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.rem\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_rem_power_of_2_ref(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.rem_power_of_2_ref(u32)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(n, u): (Integer, u32)| n.rem_power_of_2_ref(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.rem\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_ceiling_mod_power_of_2_assign(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.ceiling_mod_power_of_2_assign(u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(mut n, u): (Integer, u32)| n.ceiling_mod_power_of_2_assign(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.ceiling\\\\_mod\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_ceiling_mod_power_of_2(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.ceiling_mod_power_of_2(u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(n, u): (Integer, u32)| n.ceiling_mod_power_of_2(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.ceiling\\\\_mod\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_ceiling_mod_power_of_2_ref(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.ceiling_mod_power_of_2_ref(u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(n, u): (Integer, u32)| n.ceiling_mod_power_of_2_ref(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.ceiling\\\\_mod\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
