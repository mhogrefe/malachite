// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the num-bigint library.
//
//      Copyright The Rust Project Developers
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::{BigInt, BigUint, Sign};
use alloc::vec::Vec;
use core::{cmp, fmt, mem};
use serde::de::{Error, SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// `cautious` is based on the function of the same name in `serde`, but specialized to `u32`:
// https://github.com/dtolnay/serde/blob/399ef081ecc36d2f165ff1f6debdcbf6a1dc7efb/serde/src/private/size_hint.rs#L11-L22
fn cautious(hint: Option<usize>) -> usize {
    const MAX_PREALLOC_BYTES: usize = 1024 * 1024;

    cmp::min(
        hint.unwrap_or(0),
        MAX_PREALLOC_BYTES / mem::size_of::<u32>(),
    )
}

impl Serialize for BigUint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Note: do not change the serialization format: it matches num-bigint's, a sequence of
        // base-2^32 digits, least significant first, with no trailing zeros.
        serializer.collect_seq(self.iter_u32_digits())
    }
}

impl<'de> Deserialize<'de> for BigUint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(U32Visitor)
    }
}

struct U32Visitor;

impl<'de> Visitor<'de> for U32Visitor {
    type Value = BigUint;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a sequence of unsigned 32-bit numbers")
    }

    fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        let len = cautious(seq.size_hint());
        let mut data = Vec::with_capacity(len);

        while let Some(value) = seq.next_element::<u32>()? {
            data.push(value);
        }

        Ok(BigUint::from_slice(&data))
    }
}

impl Serialize for Sign {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Note: do not change the serialization format: it matches num-bigint's.
        match *self {
            Self::Minus => (-1i8).serialize(serializer),
            Self::NoSign => 0i8.serialize(serializer),
            Self::Plus => 1i8.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Sign {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let sign = i8::deserialize(deserializer)?;
        match sign {
            -1 => Ok(Self::Minus),
            0 => Ok(Self::NoSign),
            1 => Ok(Self::Plus),
            _ => Err(D::Error::invalid_value(
                Unexpected::Signed(sign.into()),
                &"a sign of -1, 0, or 1",
            )),
        }
    }
}

impl Serialize for BigInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Note: do not change the serialization format: it matches num-bigint's.
        (self.sign(), self.magnitude()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BigInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (sign, data) = Deserialize::deserialize(deserializer)?;
        Ok(Self::from_biguint(sign, data))
    }
}
