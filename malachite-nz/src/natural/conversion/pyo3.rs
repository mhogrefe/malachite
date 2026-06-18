// Copyright © 2026 Mikhail Hogrefe
//
// PyO3 integration contributed by Antonio Mamić.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#![cfg(feature = "enable_pyo3")]

//!
//! This is useful for converting Python integers when they may not fit in Rust's built-in integer
//! types.
//!
//! To use this enable the `enable_pyo3` feature.
//!
//! ## Examples
//!
//! Using [`Natural`](crate::natural::Natural) to correctly increment an arbitrary precision
//! integer. This is not possible with Rust's native integers if the Python integer is too large,
//! in which case it will fail its conversion and raise `OverflowError`.
//! ```rust
//! use malachite_base::num::basic::traits::One;
//! use malachite_nz::natural::Natural;
//! use pyo3::prelude::*;
//! use pyo3::types::PyModule;
//!
//! #[pyfunction]
//! fn add_one(n: Natural) -> Natural {
//!     // negative n would raise ValueError here
//!     n + Natural::ONE
//! }
//!
//! #[pymodule]
//! fn my_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
//!     m.add_function(wrap_pyfunction!(add_one, m)?)?;
//!     Ok(())
//! }
//! ```
//!
//! Python code:
//! ```python
//! from my_module import add_one
//!
//! n = 1 << 1337
//! value = add_one(n)
//!
//! assert n + 1 == value
//! ```

use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::convert::Infallible;
use malachite_base::num::basic::traits::Zero;
#[cfg(any(not(Py_3_13), Py_LIMITED_API))]
use pyo3::intern;
#[allow(unused_imports)]
use pyo3::{
    Borrowed, Bound, FromPyObject, IntoPyObject, IntoPyObjectExt, Py, PyErr, PyResult, Python,
    exceptions::PyValueError, ffi, types::*,
};

#[cfg_attr(docsrs, doc(cfg(feature = "enable_pyo3")))]
impl<'py> FromPyObject<'_, 'py> for Natural {
    type Error = PyErr;

    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> Result<Self, PyErr> {
        // get the Python interpreter
        let py = ob.py();

        // get the PyInt object, calling `__index__` if necessary
        let num_owned: Bound<'_, PyInt>;
        let num = if let Ok(long) = ob.cast::<PyInt>() {
            long
        } else {
            num_owned =
                unsafe { Bound::from_owned_ptr_or_err(py, ffi::PyNumber_Index(ob.as_ptr()))? }
                    .cast_into()?;
            num_owned.as_borrowed()
        };

        // check if the number is negative, and if so, raise ValueError
        if num.lt(0)? {
            return Err(PyErr::new::<PyValueError, _>(
                "expected non-negative integer",
            ));
        }

        Ok(Self::from_owned_limbs_asc(int_to_limbs(&num, false)?))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "enable_pyo3")))]
impl<'py> IntoPyObject<'py> for Natural {
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "enable_pyo3")))]
impl<'py> IntoPyObject<'py> for &Natural {
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        if self == &Natural::ZERO {
            return 0i32.into_pyobject(py);
        }

        let bytes = limbs_to_bytes(self.limbs(), self.limb_count());

        #[cfg(all(not(Py_LIMITED_API), Py_3_13))]
        unsafe {
            let flags =
                ffi::Py_ASNATIVEBYTES_LITTLE_ENDIAN | ffi::Py_ASNATIVEBYTES_UNSIGNED_BUFFER;
            let obj = ffi::PyLong_FromNativeBytes(bytes.as_ptr().cast(), bytes.len(), flags);
            Ok(Bound::from_owned_ptr(py, obj).cast_into_unchecked())
        }

        #[cfg(all(not(Py_LIMITED_API), not(Py_3_13)))]
        unsafe {
            let obj = ffi::_PyLong_FromByteArray(
                bytes.as_ptr().cast(),
                bytes.len(),
                1,            // little endian
                false.into(), // unsigned
            );
            Ok(Bound::from_owned_ptr(py, obj).cast_into_unchecked())
        }

        #[cfg(Py_LIMITED_API)]
        unsafe {
            let bytes_obj = PyBytes::new(py, &bytes);
            py.get_type::<PyInt>()
                .call_method("from_bytes", (bytes_obj, "little"), None)
                .expect("int.from_bytes() failed during into_pyobject()")
                .cast_into_unchecked()
        }
    }
}

/// Convert 32-bit limbs (little endian) used by malachite to bytes (little endian)
#[cfg(feature = "32_bit_limbs")]
#[inline]
fn limbs_to_bytes(limbs: impl Iterator<Item = u32>, limb_count: u64) -> Vec<u8> {
    let mut bytes = Vec::with_capacity((limb_count << 3) as usize);

    for limb in limbs {
        for byte in limb.to_le_bytes() {
            bytes.push(byte);
        }
    }

    bytes
}

/// Convert 64-bit limbs (little endian) used by malachite to bytes (little endian)
#[cfg(not(feature = "32_bit_limbs"))]
#[inline]
fn limbs_to_bytes(limbs: impl Iterator<Item = u64>, limb_count: u64) -> Vec<u8> {
    let mut bytes = Vec::with_capacity((limb_count << 3) as usize);

    for limb in limbs {
        for byte in limb.to_le_bytes() {
            bytes.push(byte);
        }
    }

    bytes
}

/// Converts a Python integer to a vector of little-endian limbs. If `is_signed` is true, the
/// integer is treated as signed and the limbs are its two's complement representation.
///
/// This is the fast path for Python 3.13 and later, which exposes the stable `PyLong_AsNativeBytes`
/// API. It writes directly into a limb-aligned buffer (with sign or zero extension to fill the
/// final limb).
#[cfg(all(not(Py_LIMITED_API), Py_3_13))]
#[inline]
fn int_to_limbs(long: &Bound<PyInt>, is_signed: bool) -> PyResult<Vec<Limb>> {
    let py = long.py();
    let mut flags = ffi::Py_ASNATIVEBYTES_LITTLE_ENDIAN;
    if !is_signed {
        flags |= ffi::Py_ASNATIVEBYTES_UNSIGNED_BUFFER | ffi::Py_ASNATIVEBYTES_REJECT_NEGATIVE;
    }

    // Passing a null buffer asks for the number of bytes needed.
    let n_bytes =
        unsafe { ffi::PyLong_AsNativeBytes(long.as_ptr().cast(), core::ptr::null_mut(), 0, flags) };
    let n_bytes: usize = n_bytes.try_into().map_err(|_| PyErr::fetch(py))?;
    if n_bytes == 0 {
        return Ok(Vec::new());
    }

    let n_limbs = n_bytes.div_ceil(size_of::<Limb>());
    let mut buffer = Vec::<Limb>::with_capacity(n_limbs);
    unsafe {
        let written = ffi::PyLong_AsNativeBytes(
            long.as_ptr().cast(),
            buffer.as_mut_ptr().cast(),
            (n_limbs * size_of::<Limb>()).try_into().unwrap(),
            flags,
        );
        if written < 0 {
            return Err(PyErr::fetch(py));
        }
        buffer.set_len(n_limbs);
    };
    buffer.iter_mut().for_each(|limb| *limb = Limb::from_le(*limb));

    Ok(buffer)
}

/// Converts a Python integer to a vector of little-endian limbs, using the (now internal) byte-array
/// API available before Python 3.13. If `is_signed` is true, the integer is treated as signed and
/// the limbs are its two's complement representation.
#[cfg(all(not(Py_LIMITED_API), not(Py_3_13)))]
#[inline]
fn int_to_limbs(long: &Bound<PyInt>, is_signed: bool) -> PyResult<Vec<Limb>> {
    let py = long.py();
    let n_bits = int_n_bits(long)?;
    if n_bits == 0 {
        return Ok(Vec::new());
    }
    // The number of bits needed, plus a sign bit for signed values, rounded up to a whole number of
    // limbs.
    let n_limbs = (n_bits + usize::from(is_signed)).div_ceil(size_of::<Limb>() << 3);
    let mut buffer = Vec::<Limb>::with_capacity(n_limbs);
    unsafe {
        let error_code = ffi::_PyLong_AsByteArray(
            long.as_ptr().cast(),
            buffer.as_mut_ptr().cast(),
            n_limbs * size_of::<Limb>(),
            1,                // little endian
            is_signed.into(), // signed flag
        );
        if error_code == -1 {
            return Err(PyErr::fetch(py));
        }
        buffer.set_len(n_limbs);
    };
    buffer.iter_mut().for_each(|limb| *limb = Limb::from_le(*limb));

    Ok(buffer)
}

/// Converts a Python integer to a vector of little-endian limbs, using only the limited (stable
/// ABI) API. If `is_signed` is true, the integer is treated as signed and the limbs are its two's
/// complement representation.
#[cfg(Py_LIMITED_API)]
#[inline]
fn int_to_limbs(long: &Bound<PyInt>, is_signed: bool) -> PyResult<Vec<Limb>> {
    let n_bits = int_n_bits(long)?;
    if n_bits == 0 {
        return Ok(Vec::new());
    }
    let n_limbs = (n_bits + usize::from(is_signed)).div_ceil(size_of::<Limb>() << 3);
    let py_bytes = int_to_py_bytes(long, n_limbs * size_of::<Limb>(), is_signed)?;
    Ok(py_bytes
        .as_bytes()
        .chunks_exact(size_of::<Limb>())
        .map(|chunk| Limb::from_le_bytes(chunk.try_into().unwrap()))
        .collect())
}

/// Converts a Python integer to a Python bytes object. Bytes are in little endian order. Takes the
/// number of bytes to convert to. If `is_signed` is true, the integer is treated as signed, and
/// two's complement is returned.
#[cfg(Py_LIMITED_API)]
#[inline]
fn int_to_py_bytes<'py>(
    long: &Bound<'py, PyInt>,
    n_bytes: usize,
    is_signed: bool,
) -> PyResult<Bound<'py, PyBytes>> {
    // get the Python interpreter
    let py = long.py();

    // setup kwargs for to_bytes (only if signed)
    let kwargs_dict = PyDict::new(py);
    let kwargs = if is_signed {
        kwargs_dict.set_item(intern!(py, "signed"), true)?;
        Some(&kwargs_dict)
    } else {
        None
    };

    // call to_bytes
    let bytes = long.call_method(
        intern!(py, "to_bytes"),
        (n_bytes, intern!(py, "little")),
        kwargs,
    )?;

    // downcast to PyBytes
    Ok(bytes.cast_into()?)
}

/// Returns the number of bits in the absolute value of the given integer. The number of bits
/// returned is the smallest number of bits that can represent the integer, not the multiple of 8
/// (bytes) that it would take up in memory.
///
/// On Python 3.13 and later (with access to the non-limited API) the number of bits is not needed,
/// because [`int_to_limbs`] queries `PyLong_AsNativeBytes` for the buffer size directly.
#[cfg(any(not(Py_3_13), Py_LIMITED_API))]
#[inline]
fn int_n_bits(long: &Bound<PyInt>) -> PyResult<usize> {
    let py = long.py();
    long.call_method0(intern!(py, "bit_length"))
        .and_then(|l| l.extract::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Prepare Python
    fn prepare_python() {
        Python::initialize();
    }

    /// Fibonacci sequence iterator (Rust)
    fn rust_fib<T>() -> impl Iterator<Item = T>
    where
        T: From<u8>,
        for<'a> &'a T: std::ops::Add<Output = T>,
    {
        let mut f0: T = T::from(1);
        let mut f1: T = T::from(1);
        std::iter::from_fn(move || {
            let f2 = &f0 + &f1;
            Some(std::mem::replace(&mut f0, std::mem::replace(&mut f1, f2)))
        })
    }

    /// Fibonacci sequence iterator (Python)
    fn python_fib(py: Python<'_>) -> impl Iterator<Item = Py<PyInt>> {
        let mut f0 = 1u32.into_pyobject(py).unwrap();
        let mut f1 = 1u32.into_pyobject(py).unwrap();
        std::iter::from_fn(move || {
            let f2 = f0
                .call_method1("__add__", (&f1,))
                .unwrap()
                .cast_into::<PyInt>()
                .unwrap();
            Some(std::mem::replace(&mut f0, std::mem::replace(&mut f1, f2)).unbind())
        })
    }

    /// Generate test python class
    fn python_index_class(py: Python<'_>) -> Bound<'_, PyModule> {
        let index_code = c"
class C:
    def __init__(self, x):
        self.x = x
    def __index__(self):
        return self.x
";
        let filename = c"index.py";
        let modulename = c"index";
        PyModule::from_code(py, index_code, filename, modulename).unwrap()
    }

    /// - Test conversion to and from Natural
    /// - Tests the first 2000 numbers in the fibonacci sequence
    #[test]
    fn convert_natural() {
        prepare_python();
        Python::attach(|py| {
            // check the first 2000 numbers in the fibonacci sequence
            for (py_result, rs_result) in python_fib(py).zip(rust_fib::<Natural>()).take(2000) {
                // Python -> Rust
                assert_eq!(py_result.extract::<Natural>(py).unwrap(), rs_result);
                // Rust -> Python
                assert!(
                    py_result
                        .bind(py)
                        .as_any()
                        .eq(rs_result.into_pyobject(py).unwrap())
                        .unwrap()
                );
            }
        });
    }

    /// Test Python class conversion
    #[test]
    fn convert_index_class() {
        prepare_python();
        Python::attach(|py| {
            let index = python_index_class(py);
            let locals = PyDict::new(py);
            locals.set_item("index", &index).unwrap();
            let expr = c"index.C(10)";
            let ob = py.eval(expr, None, Some(&locals)).unwrap();
            let natural: Natural = ob.extract().unwrap();

            assert_eq!(natural, Natural::from(10_u8));
        });
    }

    /// Test conversion to and from zero
    #[test]
    fn handle_zero() {
        prepare_python();
        Python::attach(|py| {
            // Python -> Rust
            let zero_natural: Natural = 0u32.into_pyobject(py).unwrap().extract().unwrap();
            assert_eq!(zero_natural, Natural::from(0_u8));

            // Rust -> Python
            let zero_natural = zero_natural.into_pyobject(py).unwrap();
            assert!(
                zero_natural
                    .as_any()
                    .eq(0u8.into_py_any(py).unwrap())
                    .unwrap()
            );
        });
    }

    /// Test for possible overflows
    #[test]
    fn check_overflow() {
        prepare_python();
        Python::attach(|py| {
            macro_rules! test {
                ($T:ty, $value:expr, $py:expr) => {
                    let value = $value;
                    println!("{}: {}", stringify!($T), value);
                    let python_value = value.clone().into_pyobject(py).unwrap();
                    let roundtrip_value = python_value.extract::<$T>().unwrap();
                    assert_eq!(value, roundtrip_value);
                };
            }

            for i in 0..=256usize {
                // test a lot of values to help catch other bugs too
                test!(Natural, Natural::from(i), py);
                test!(Natural, Natural::from(1u32) << i, py);
                test!(
                    Natural,
                    (Natural::from(1u32) << i) + Natural::from(1u32),
                    py
                );
                test!(
                    Natural,
                    (Natural::from(1u32) << i) - Natural::from(1u32),
                    py
                );
            }
        });
    }

    /// Test error when converting negative integer to Natural
    #[test]
    fn negative_natural() {
        prepare_python();
        Python::attach(|py| {
            let zero = 0u32.into_pyobject(py).unwrap();
            let minus_one = (-1i32).into_pyobject(py).unwrap();
            assert_eq!(zero.extract::<Natural>().unwrap(), Natural::ZERO);
            assert!(
                minus_one
                    .extract::<Natural>()
                    .unwrap_err()
                    .get_type(py)
                    .is(&PyType::new::<PyValueError>(py))
            );
        });
    }
}
