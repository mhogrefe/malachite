use malachite_base::num::basic::traits::{One, Two};
use malachite_float::Float;

fn consume(_: Float) {}

fn main() {
    let p = 10u64;
    let value = Float::TWO;
    // Fresh mut binding, one in-place assign, then moved once: flagged.
    let mut t = Float::ONE.mul_prec(Float::TWO, p).0;
    t.add_prec_assign_ref(&value, p);
    consume(t);
    // Used more than once after the assign: fine (not a simple chain).
    let mut u = Float::ONE.mul_prec(Float::TWO, p).0;
    u.add_prec_assign_ref(&value, p);
    let _ = &u;
    consume(u);
}
