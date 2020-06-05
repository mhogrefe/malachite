use rug::ops::RemRounding;

pub fn rug_neg_mod(x: rug::Integer, y: rug::Integer) -> rug::Integer {
    -x.rem_ceil(y)
}
