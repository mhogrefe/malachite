fn f(a: u32, b: i32) -> (u32, u32, u32, i32, u32, u32, u32, u32) {
    // `x * pow2`: flagged, use `<<`.
    let m1 = a * 8;
    // the power-of-two literal may be on either side of `*`: flagged.
    let m2 = 16 * a;
    // `x / pow2` on an unsigned type: flagged, use `>>`.
    let d1 = a / 4;
    // `x / pow2` on a signed type: truncation differs from the floor, so `shr_round`: flagged.
    let d2 = b / 8;
    // `*=` by a power of two: flagged, use `<<=`.
    let mut ma = a;
    ma *= 2;
    // `/=` by a power of two: flagged, use `>>=`.
    let mut da = a;
    da /= 32;
    // not a power of two: fine.
    let n1 = a * 3;
    // already a shift: fine.
    let n2 = a << 1;
    (m1, m2, d1, d2, ma, da, n1, n2)
}

fn main() {
    let _ = f(std::hint::black_box(100), std::hint::black_box(-100));
}
