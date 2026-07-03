use malachite_base::rounding_modes::RoundingMode::{self, *};

struct MyFloat;

impl MyFloat {
    fn exp_prec_round(&self, prec: u64, rm: RoundingMode) -> u64 {
        prec.wrapping_add(rm as u64)
    }

    fn exp_prec(&self, prec: u64) -> u64 {
        // The defining delegation: fine.
        self.exp_prec_round(prec, Nearest)
    }

    fn ln_prec_round(&self, prec: u64, rm: RoundingMode) -> u64 {
        prec.wrapping_mul(rm as u64)
    }
}

trait Exp {
    fn exp(&self) -> u64;
}

impl Exp for MyFloat {
    fn exp(&self) -> u64 {
        // Trait impls delegate via the explicit form by convention: fine.
        self.exp_prec_round(53, Nearest)
    }
}

fn main() {
    let x = MyFloat;
    // The `exp_prec` shorthand exists: flagged, in both call forms.
    let _ = x.exp_prec_round(10, Nearest);
    let _ = MyFloat::exp_prec_round(&x, 10, Nearest);
    // A rounding mode other than `Nearest`: fine.
    let _ = x.exp_prec_round(10, Floor);
    // No `ln_prec` shorthand exists: fine.
    let _ = x.ln_prec_round(10, Nearest);
}
