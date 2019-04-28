use conversion::WrappingFrom;
use num::traits::{
    DivisibleByPowerOfTwo, EqModPowerOfTwo, ModPowerOfTwo, NegAssign, NotAssign, Parity, ShrRound,
};
use round::RoundingMode;

impl NegAssign for isize {
    fn neg_assign(&mut self) {
        *self = -*self
    }
}

impl NotAssign for isize {
    fn not_assign(&mut self) {
        *self = !*self
    }
}

//TODO fix code duplication
impl Parity for usize {
    #[inline]
    fn even(self) -> bool {
        (self & 1) == 0
    }

    #[inline]
    fn odd(self) -> bool {
        (self & 1) != 0
    }
}

impl ModPowerOfTwo for usize {
    type Output = usize;

    #[inline]
    fn mod_power_of_two(self, pow: u64) -> usize {
        if self == 0 || pow >= u64::from(0usize.trailing_zeros()) {
            self
        } else {
            self & ((1 << pow) - 1)
        }
    }
}

impl DivisibleByPowerOfTwo for usize {
    #[inline]
    fn divisible_by_power_of_two(self, pow: u64) -> bool {
        self.mod_power_of_two(pow) == 0
    }
}

impl EqModPowerOfTwo<usize> for usize {
    #[inline]
    fn eq_mod_power_of_two(self, other: usize, pow: u64) -> bool {
        (self ^ other).divisible_by_power_of_two(pow)
    }
}

impl Parity for isize {
    #[inline]
    fn even(self) -> bool {
        (self & 1) == 0
    }

    #[inline]
    fn odd(self) -> bool {
        (self & 1) != 0
    }
}

impl ShrRound<u64> for usize {
    type Output = usize;

    fn shr_round(self, other: u64, rm: RoundingMode) -> usize {
        if other == 0 || self == 0 {
            return self;
        }
        let width = u64::wrapping_from(0usize.trailing_zeros());
        match rm {
            RoundingMode::Down | RoundingMode::Floor if other >= width => 0,
            RoundingMode::Down | RoundingMode::Floor => self >> other,
            RoundingMode::Up | RoundingMode::Ceiling if other >= width => 1,
            RoundingMode::Up | RoundingMode::Ceiling => {
                let shifted = self >> other;
                if shifted << other == self {
                    shifted
                } else {
                    shifted + 1
                }
            }
            RoundingMode::Nearest
                if other == width && self > (1 << (0usize.trailing_zeros() - 1)) =>
            {
                1
            }
            RoundingMode::Nearest if other >= width => 0,
            RoundingMode::Nearest => {
                let mostly_shifted = self >> (other - 1);
                if mostly_shifted.even() {
                    // round down
                    mostly_shifted >> 1
                } else if mostly_shifted << (other - 1) != self {
                    // round up
                    (mostly_shifted >> 1) + 1
                } else {
                    // result is half-integer; round to even
                    let shifted = mostly_shifted >> 1;
                    if shifted.even() {
                        shifted
                    } else {
                        shifted + 1
                    }
                }
            }
            RoundingMode::Exact if other >= width => {
                panic!("Right shift is not exact: {} >> {}", self, other);
            }
            RoundingMode::Exact => {
                let shifted = self >> other;
                if shifted << other != self {
                    panic!("Right shift is not exact: {} >> {}", self, other);
                }
                shifted
            }
        }
    }
}

pub mod conversion;
pub mod floats;
#[macro_use]
pub mod integers;
#[macro_use]
pub mod unsigneds;
pub mod signeds;
pub mod traits;
