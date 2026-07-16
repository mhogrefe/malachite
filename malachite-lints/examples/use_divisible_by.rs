use malachite_nz::natural::Natural;

fn main() {
    let n = std::hint::black_box(12u64);
    let b = std::hint::black_box(3u64);
    // Divisibility of a primitive via `% b == 0` / `!= 0`: flagged.
    let _ = n % b == 0;
    let _ = n % 3 != 0;
    // Divisibility of a bignum: flagged.
    let x = const { Natural::const_from(100) };
    let three = const { Natural::const_from(3) };
    let _ = &x % &three == 0u32;
    // A bare remainder, not a divisibility test: fine.
    let _ = std::hint::black_box(n % b);
}
