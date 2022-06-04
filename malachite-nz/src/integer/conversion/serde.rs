use integer::{Integer, SerdeInteger};
use malachite_base::num::conversion::traits::FromStringBase;
use natural::Natural;
use std::convert::TryFrom;

impl From<Integer> for SerdeInteger {
    #[inline]
    fn from(x: Integer) -> SerdeInteger {
        SerdeInteger(format!("{:#x}", x))
    }
}

impl TryFrom<SerdeInteger> for Integer {
    type Error = String;

    #[inline]
    fn try_from(s: SerdeInteger) -> Result<Integer, String> {
        if s.0.starts_with('-') {
            if s.0.starts_with("-0x") {
                Ok(Integer::from_sign_and_abs(
                    false,
                    Natural::from_string_base(16, &s.0[3..])
                        .ok_or_else(|| format!("Unrecognized digits in {}", s.0))?,
                ))
            } else {
                Err(format!(
                    "String '{}' starts with '-' but not with '-0x'",
                    s.0
                ))
            }
        } else if s.0.starts_with("0x") {
            Ok(Integer::from(
                Natural::from_string_base(16, &s.0[2..])
                    .ok_or_else(|| format!("Unrecognized digits in {}", s.0))?,
            ))
        } else {
            Err(format!(
                "String '{}' does not start with '0x' or '-0x'",
                s.0
            ))
        }
    }
}
