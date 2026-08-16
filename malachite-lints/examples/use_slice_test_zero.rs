use malachite_base::slices::slice_test_zero;

fn main() {
    let xs = std::hint::black_box(vec![0u64, 1, 2]);
    // Scanning for a nonzero element: flagged.
    let _ = xs.iter().any(|&x| x != 0);
    let _ = xs[..2].iter().any(|x| *x != 0);
    // Testing that all elements are zero: flagged.
    let _ = xs.iter().all(|&x| x == 0);
    // A different predicate: fine.
    let _ = xs.iter().any(|&x| x != 1);
    // The suggested form: fine.
    let _ = !slice_test_zero(&xs);
}
