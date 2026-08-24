use malachite_base::num::logic::traits::LowMask;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

const THREE: Natural = Natural::const_from(3);
const FOUR: Natural = Natural::const_from(4);
const NEG_SEVEN: Integer = Integer::const_from_signed(-7);
const N: Natural = Natural::const_from(123);
const I: Integer = Integer::const_from_signed(-123);

// A mask in a `limbs_`-named kernel: not flagged; the mask is the idiom there.
fn limbs_low_bits(x: u64) -> u64 {
    x & 7
}

fn main() {
    let x = 123u64;
    let n = N;
    let i = I;

    // An unsigned primitive masked with a literal one less than a power of 2: flagged.
    let _ = x & 7;
    let _ = 0xffff & x;
    let _ = x & 0x3f == 5;
    let mut y = x;
    y &= 15;
    // A `Natural` masked with a constant defined from such a literal: flagged.
    let _ = &n & &THREE;

    // A mask of 1 is `use_parity`'s domain: not flagged.
    let _ = x & 1;
    // Not one less than a power of 2: not flagged.
    let _ = x & 5;
    // A named constant of primitive type carries its own meaning (`WIDTH_MASK` and friends): not
    // flagged.
    const MASK: u64 = 63;
    let _ = x & MASK;
    // A signed value: not flagged, since `mod_power_of_2` returns the unsigned remainder.
    let s = -123i64;
    let _ = s & 7;
    // An `Integer`: not flagged, since `mod_power_of_2` returns a `Natural`.
    let _ = &i & &NEG_SEVEN;
    // A `Natural` constant that is not a mask: not flagged.
    let _ = &n & &FOUR;
    let _ = limbs_low_bits(x);
    // A mask built by calling low_mask: flagged, in any function.
    let sh = std::hint::black_box(5u64);
    let _ = n & u64::low_mask(sh);
}
