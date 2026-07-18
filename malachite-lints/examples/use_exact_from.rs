use malachite_base::num::conversion::traits::ExactFrom;

fn f(x: i64, y: u128) -> (usize, u64, i64, u64, char) {
    // `try_from(...).unwrap()` into an integer type: flagged.
    let a = usize::try_from(x).unwrap();
    // A wider source is still the pattern: flagged.
    let b = u64::try_from(y).unwrap();
    // Already using `exact_from`: fine.
    let c = i64::exact_from(x);
    // `try_from` without `.unwrap()`: fine.
    let d = u64::try_from(y).unwrap_or(0);
    // A non-integer target (`exact_from` need not apply): fine.
    let e = char::try_from(97u32).unwrap();
    (a, b, c, d, e)
}

fn main() {
    let _ = f(std::hint::black_box(5), std::hint::black_box(10));
}
