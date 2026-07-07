fn f(x: i32, y: u8, z: i64) -> (bool, bool, bool, bool, bool) {
    // Widening then comparing with a representable literal: flagged.
    let a = i64::from(x) <= 32;
    // Any comparison operator, either operand order: flagged.
    let b = 100 > u64::from(y);
    let c = i64::from(x) == -7;
    // Literal not representable in the source type (i32): the conversion is not redundant.
    let d = i64::from(x) < 3_000_000_000i64;
    // Comparing with a non-literal: not covered.
    let e = i64::from(x) <= z;
    (a, b, c, d, e)
}

fn main() {
    let (a, b, c, d, e) = f(5, 3, 40);
    let _ = (a, b, c, d, e);
}
