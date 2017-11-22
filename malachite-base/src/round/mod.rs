use std::ops::Neg;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RoundingMode {
    Down,
    Up,
    Floor,
    Ceiling,
    Nearest,
    Exact,
}

impl Neg for RoundingMode {
    type Output = RoundingMode;

    fn neg(self) -> RoundingMode {
        match self {
            RoundingMode::Floor => RoundingMode::Ceiling,
            RoundingMode::Ceiling => RoundingMode::Floor,
            rm => rm,
        }
    }
}
