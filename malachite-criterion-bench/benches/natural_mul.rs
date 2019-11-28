use criterion::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural::random::random_natural_with_bits::random_natural_with_bits;
use rand::{SeedableRng, StdRng};
use num::BigUint;
use rug;
use std::str::FromStr;

pub fn natural_to_biguint(n: &Natural) -> BigUint {
    BigUint::from_str(n.to_string().as_ref()).unwrap()
}

pub fn natural_to_rug_integer(n: &Natural) -> rug::Integer {
    rug::Integer::from_str(n.to_string().as_ref()).unwrap()
}

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Natural * Natural");
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for &i in [1u64, 10, 100, 1000, 10000, 100000, 1000000].iter() {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let x = random_natural_with_bits(&mut rng, i);
        let y = random_natural_with_bits(&mut rng, i);
        let x_num = natural_to_biguint(&x);
        let y_num = natural_to_biguint(&y);
        let x_rug = natural_to_rug_integer(&x);
        let y_rug = natural_to_rug_integer(&y);
        group.bench_function(BenchmarkId::new("malachite", i),
            |b| b.iter_with_setup(|| (x.clone(), y.clone()), |(x, y)| x * y));
        group.bench_function(BenchmarkId::new("num", i),
            |b| b.iter_with_setup(|| (x_num.clone(), y_num.clone()), |(x, y)| x * y));
        group.bench_function(BenchmarkId::new("rug", i),
            |b| b.iter_with_setup(|| (x_rug.clone(), y_rug.clone()), |(x, y)| x * y));
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
