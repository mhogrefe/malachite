use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use malachite_base::num::floats::PrimitiveFloat;

use num::float::nice_float::FloatType::{
    NegativeFinite, NegativeInfinity, NegativeZero, PositiveFinite, PositiveInfinity, PositiveZero,
};

pub trait FmtRyuString: Copy {
    fn fmt_ryu_string(self, f: &mut Formatter<'_>) -> fmt::Result;
}

macro_rules! impl_to_ryu_string {
    ($f: ident) => {
        impl FmtRyuString for $f {
            #[inline]
            fn fmt_ryu_string(self, f: &mut Formatter<'_>) -> fmt::Result {
                let mut buffer = ryu::Buffer::new();
                let printed = buffer.format_finite(self);
                // Convert e.g. "1e100" to "1.0e100".
                // `printed` is ASCII, so we can manipulate bytes rather than chars.
                let mut e_index = None;
                let mut found_dot = false;
                for (i, &b) in printed.as_bytes().iter().enumerate() {
                    match b {
                        b'.' => {
                            found_dot = true;
                            break;
                        }
                        b'e' => {
                            e_index = Some(i);
                            break;
                        }
                        _ => {}
                    }
                }
                if !found_dot {
                    if let Some(e_index) = e_index {
                        let mut out_bytes = vec![0; printed.len() + 2];
                        let (in_bytes_lo, in_bytes_hi) = printed.as_bytes().split_at(e_index);
                        let (out_bytes_lo, out_bytes_hi) = out_bytes.split_at_mut(e_index);
                        out_bytes_lo.copy_from_slice(in_bytes_lo);
                        out_bytes_hi[0] = b'.';
                        out_bytes_hi[1] = b'0';
                        out_bytes_hi[2..].copy_from_slice(in_bytes_hi);
                        return f.write_str(&String::from_utf8(out_bytes).unwrap());
                    } else {
                        panic!("Unexpected Ryu string: {}", printed);
                    }
                }
                f.write_str(printed)
            }
        }
    };
}
impl_to_ryu_string!(f32);
impl_to_ryu_string!(f64);

#[derive(Clone, Copy)]
pub struct NiceFloat<T: PrimitiveFloat + FmtRyuString>(pub T);

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum FloatType {
    NegativeInfinity,
    NegativeFinite,
    NegativeZero,
    NaN,
    PositiveZero,
    PositiveFinite,
    PositiveInfinity,
}

impl<T: PrimitiveFloat + FmtRyuString> NiceFloat<T> {
    #[inline]
    pub fn unwrap(self) -> T {
        self.0
    }

    fn float_type(self) -> FloatType {
        if self.0.is_nan() {
            FloatType::NaN
        } else if self.0.is_sign_positive() {
            if self.0 == T::ZERO {
                PositiveZero
            } else if self.0.is_finite() {
                PositiveFinite
            } else {
                PositiveInfinity
            }
        } else if self.0 == T::ZERO {
            NegativeZero
        } else if self.0.is_finite() {
            NegativeFinite
        } else {
            NegativeInfinity
        }
    }
}

impl<T: PrimitiveFloat + FmtRyuString> PartialEq<NiceFloat<T>> for NiceFloat<T> {
    fn eq(&self, other: &NiceFloat<T>) -> bool {
        self.float_type() == other.float_type() && (self.0.is_nan() || self.0 == other.0)
    }
}

impl<T: PrimitiveFloat + FmtRyuString> Eq for NiceFloat<T> {}

impl<T: PrimitiveFloat + FmtRyuString> Ord for NiceFloat<T> {
    fn cmp(&self, other: &NiceFloat<T>) -> Ordering {
        let self_type = self.float_type();
        let other_type = other.float_type();
        self_type.cmp(&other_type).then_with(|| {
            if self_type == PositiveFinite || self_type == NegativeFinite {
                self.0.partial_cmp(&other.0).unwrap()
            } else {
                Ordering::Equal
            }
        })
    }
}

impl<T: PrimitiveFloat + FmtRyuString> PartialOrd<NiceFloat<T>> for NiceFloat<T> {
    fn partial_cmp(&self, other: &NiceFloat<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: PrimitiveFloat + FmtRyuString> Display for NiceFloat<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0.is_nan() {
            f.write_str("NaN")
        } else if self.0.is_infinite() {
            if self.0.is_sign_positive() {
                f.write_str("Infinity")
            } else {
                f.write_str("-Infinity")
            }
        } else {
            self.0.fmt_ryu_string(f)
        }
    }
}

impl<T: PrimitiveFloat + FmtRyuString> Debug for NiceFloat<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl<T: PrimitiveFloat + FmtRyuString> FromStr for NiceFloat<T> {
    type Err = <T as FromStr>::Err;

    #[inline]
    fn from_str(src: &str) -> Result<NiceFloat<T>, <T as FromStr>::Err> {
        match src {
            "NaN" => Ok(T::NAN),
            "Infinity" => Ok(T::POSITIVE_INFINITY),
            "-Infinity" => Ok(T::NEGATIVE_INFINITY),
            src => T::from_str(src),
        }
        .map(NiceFloat)
    }
}
