fn f(n: usize) -> u64 {
    // three same-type buffers back to back: flagged.
    let mut a = vec![0u64; n << 1];
    let mut b = vec![0u64; n];
    let mut c = vec![0u64; n];
    a[0] = 1;
    b[0] = 2;
    c[0] = 3;
    // different element types: fine.
    let d = vec![0u32; n];
    let e = vec![0u64; n];
    // separated by another statement: fine.
    let x = u64::from(d[0]) + e[0];
    let y = vec![0u64; n];
    a[0] + b[0] + c[0] + x + y[0]
}

fn main() {
    let _ = f(std::hint::black_box(5));
}
