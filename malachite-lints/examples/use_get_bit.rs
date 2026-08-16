use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::BitAccess;

const HIGH_BIT: u64 = 1 << 63;

fn main() {
    let x = std::hint::black_box(0x1234_5678_9abc_def0u64);
    let sh = std::hint::black_box(7u64);
    // Testing one bit by masking with a power of 2: flagged.
    let _ = x & u64::power_of_2(sh) == 0;
    let _ = x & u64::power_of_2(sh) != 0;
    let _ = 0 != x & u64::power_of_2(3);
    // Comparing against a high-bit constant: flagged.
    let _ = x & HIGH_BIT != 0;
    let _ = x & HIGH_BIT == 0;
    // The value of the mask used as a number: fine.
    let _ = (x & u64::power_of_2(sh)) + 1;
    // The suggested forms: fine.
    let _ = x.get_bit(sh);
    let _ = x.get_highest_bit();
}
