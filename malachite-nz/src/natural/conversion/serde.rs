use crate::natural::{Natural, SerdeNatural};
use malachite_base::num::conversion::traits::FromStringBase;
use std::convert::TryFrom;

impl From<Natural> for SerdeNatural {
    #[inline]
    fn from(x: Natural) -> SerdeNatural {
        SerdeNatural(format!("{x:#x}"))
    }
}

impl TryFrom<SerdeNatural> for Natural {
    type Error = String;

    #[inline]
    fn try_from(s: SerdeNatural) -> Result<Natural, String> {
        if s.0.starts_with("0x") {
            Natural::from_string_base(16, &s.0[2..])
                .ok_or_else(|| format!("Unrecognized digits in {}", s.0))
        } else {
            Err(format!("String '{}' does not start with '0x'", s.0))
        }
    }
}
