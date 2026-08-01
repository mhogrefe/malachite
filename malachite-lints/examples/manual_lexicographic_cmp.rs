struct Pair {
    hi: u64,
    lo: u64,
}

fn cost(x: u64) -> u64 {
    x + 1
}

fn main() {
    let (a1, a0, b1, b0) = (1u64, 2u64, 3u64, 4u64);
    let (a2, b2) = (5u64, 6u64);
    let p = Pair { hi: 1, lo: 2 };
    let q = Pair { hi: 3, lo: 4 };
    let xs = [1u64, 2];
    let ys = [3u64, 4];

    // Two elements, one per ordering operator: flagged.
    let _ = a1 < b1 || a1 == b1 && a0 < b0;
    let _ = a1 < b1 || a1 == b1 && a0 <= b0;
    let _ = a1 > b1 || a1 == b1 && a0 > b0;
    let _ = a1 > b1 || a1 == b1 && a0 >= b0;
    // Explicit parentheses around the second half: flagged the same way.
    let _ = a1 < b1 || (a1 == b1 && a0 < b0);
    // The equality written the other way round: flagged.
    let _ = a1 < b1 || b1 == a1 && a0 < b0;
    // Three elements: flagged once, for the whole chain.
    let _ = a1 < b1 || a1 == b1 && (a0 < b0 || a0 == b0 && a2 < b2);
    // The flat spelling, where each disjunct restates the earlier equalities: flagged once, for
    // the whole three-element chain.
    let _ = a1 > b1 || (a1 == b1 && a0 > b0) || (a1 == b1 && a0 == b0 && a2 > b2);
    // Fields and indexes are place expressions too: flagged.
    let _ = p.hi < q.hi || p.hi == q.hi && p.lo < q.lo;
    let _ = xs[0] < ys[0] || xs[0] == ys[0] && xs[1] < ys[1];

    // The last element uses the strict operator, so this is not `(a1, a0) <= (b1, b0)`: fine.
    let _ = a1 <= b1 || a1 == b1 && a0 < b0;
    // The equality is of a different pair: fine.
    let _ = a1 < b1 || a0 == b0 && a0 < b0;
    // The two halves disagree about direction: fine.
    let _ = a1 < b1 || a1 == b1 && a0 > b0;
    // A plain comparison: fine.
    let _ = a1 < b1;
    // Calls could have side effects, and tuple comparison would evaluate all of them: fine.
    let _ = cost(a1) < cost(b1) || cost(a1) == cost(b1) && cost(a0) < cost(b0);
}
