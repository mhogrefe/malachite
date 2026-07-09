use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

fn f(nat: &Natural, negative: bool) -> (Integer, Integer) {
    // A magnitude built then conditionally negated in place: flagged.
    let mut a = Integer::from(nat);
    if negative {
        a.neg_assign();
    }
    // Already using the sign-and-abs constructor: fine.
    let c = Integer::from_sign_and_abs_ref(!negative, nat);
    (a, c)
}

fn main() {
    let n = Natural::from(std::hint::black_box(5u32));
    let _ = f(&n, std::hint::black_box(true));
}
