use malachite_base::num::arithmetic::traits::{FloorLogBase2, IsPowerOf2};
use malachite_q::Rational;

fn f(x: Rational, y: Rational, n: u64) -> i64 {
    // A `Rational` guarded by `is_power_of_2`, then `floor_log_base_2_abs` on the same value:
    // flagged.
    if x.is_power_of_2() {
        return x.floor_log_base_2_abs();
    }
    // A primitive with `floor_log_base_2`: flagged.
    if n.is_power_of_2() {
        return n.floor_log_base_2() as i64;
    }
    // The floor-log is taken of a different value than the guard: fine.
    if x.is_power_of_2() {
        return y.floor_log_base_2_abs();
    }
    // No floor-log in the guarded body: fine.
    if x.is_power_of_2() {
        return 0;
    }
    -1
}

fn main() {
    let _ = f(
        Rational::from(std::hint::black_box(8u32)),
        Rational::from(std::hint::black_box(4u32)),
        std::hint::black_box(16),
    );
}
