// Copyright © 2025 Mikhail Hogrefe
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
//! Using [`Integer`](crate::integer::Integer) to correctly increment an arbitrary precision
//! integer. This is not possible with Rust's native integers if the Python integer is too large,
//! in which case it will fail its conversion and raise `OverflowError`.
//! ```rust
//! use malachite::Integer;
//! use malachite::num::basic::traits::One;
//! use pyo3::prelude::*;
//!
//! #[pyfunction]
//! fn add_one(n: Integer) -> Integer {
//!     n + Integer::ONE
//! }
//!
//! #[pymodule]
//! fn my_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
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

use crate::integer::Integer;
use alloc::vec::Vec;
use core::convert::Infallible;
use malachite_base::num::basic::traits::Zero;
#[cfg(Py_LIMITED_API)]
use pyo3::intern;
#[allow(unused_imports)]
use pyo3::{
    Bound, FromPyObject, IntoPyObject, IntoPyObjectExt, Py, PyErr, PyObject, PyResult, Python, ffi,
    types::*,
};

#[cfg_attr(docsrs, doc(cfg(feature = "enable_pyo3")))]
impl<'source> FromPyObject<'source> for Integer {
    fn extract_bound(ob: &Bound<'source, PyAny>) -> PyResult<Integer> {
        // get the Python interpreter
        let py = ob.py();

        // get PyInt object
        let num_owned: Py<PyInt>;
        let num = if let Ok(long) = ob.downcast::<PyInt>() {
            long
        } else {
            num_owned = unsafe { Py::from_owned_ptr_or_err(py, ffi::PyNumber_Index(ob.as_ptr()))? };
            num_owned.bind(py)
        };

        // check if number is zero, and if so, return zero
        let n_bits = int_n_bits(num)?;
        if n_bits == 0 {
            return Ok(Integer::ZERO);
        }

        // the number of bytes needed to store the integer
        let mut n_bytes = (n_bits + 7 + 1) >> 3; // +1 for the sign bit

        #[cfg(feature = "32_bit_limbs")]
        {
            // convert the number of bytes to a multiple of 4, because of 32-bit limbs
            n_bytes = ((n_bytes + 7) >> 2) << 2;
        }
        #[cfg(not(feature = "32_bit_limbs"))]
        {
            // convert the number of bytes to a multiple of 8, because of 64-bit limbs
            n_bytes = ((n_bytes + 7) >> 3) << 3;
        }

        #[cfg(not(Py_LIMITED_API))]
        {
            let limbs = int_to_limbs(num, n_bytes, true)?;
            Ok(Integer::from_owned_twos_complement_limbs_asc(limbs))
        }
        #[cfg(all(Py_LIMITED_API, feature = "32_bit_limbs"))]
        {
            let py_bytes = int_to_py_bytes(num, n_bytes, true)?;
            let bytes = py_bytes.as_bytes();
            let n_limbs_32 = n_bytes >> 2; // the number of 32-bit limbs needed to store the integer
            let mut limbs_32 = Vec::with_capacity(n_limbs_32);
            for i in (0..n_bytes).step_by(4) {
                limbs_32.push(u32::from_le_bytes(bytes[i..(i + 4)].try_into().unwrap()));
            }
            Ok(Integer::from_owned_twos_complement_limbs_asc(limbs_32))
        }
        #[cfg(all(Py_LIMITED_API, not(feature = "32_bit_limbs")))]
        {
            let bytes = int_to_py_bytes(num, n_bytes, true)?.as_bytes();
            let n_limbs_64 = n_bytes >> 3; // the number of 64-bit limbs needed to store the integer
            let mut limbs_64 = Vec::with_capacity(n_limbs_64);
            for i in (0..n_bytes).step_by(8) {
                limbs_64.push(u64::from_le_bytes(bytes[i..(i + 8)].try_into().unwrap()));
            }
            Ok(Integer::from_owned_twos_complement_limbs_asc(limbs_64))
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "enable_pyo3")))]
impl<'py> IntoPyObject<'py> for Integer {
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "enable_pyo3")))]
impl<'py> IntoPyObject<'py> for &Integer {
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        if self == &Integer::ZERO {
            return Ok(0i32.into_pyobject(py).unwrap());
        }

        let bytes = limbs_to_bytes(
            self.twos_complement_limbs(),
            self.twos_complement_limb_count(),
        );

        #[cfg(not(Py_LIMITED_API))]
        unsafe {
            let obj = ffi::_PyLong_FromByteArray(
                bytes.as_ptr().cast(),
                bytes.len(),
                1,           // little endian
                true.into(), // signed
            );
            Ok(Bound::from_owned_ptr(py, obj).downcast_into_unchecked::<PyInt>())
        }

        #[cfg(Py_LIMITED_API)]
        {
            let bytes_obj = PyBytes::new(py, &bytes);
            let kwargs = PyDict::new(py);
            kwargs.set_item(intern!(py, "signed"), true).unwrap();
            let result: Bound<'py, PyAny> = py
                .get_type::<PyInt>()
                .call_method("from_bytes", (bytes_obj, "little"), Some(&kwargs))
                .expect("int.from_bytes() failed during into_pyobject()");
            Ok(result)
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

/// Converts a Python integer to a vector of 32-bit limbs (little endian). Takes number of bytes to
/// convert to. Multiple of 4. If `is_signed` is true, the integer is treated as signed, and two's
/// complement is returned.
#[cfg(all(not(Py_LIMITED_API), feature = "32_bit_limbs"))]
#[inline]
fn int_to_limbs(long: &Bound<PyInt>, n_bytes: usize, is_signed: bool) -> PyResult<Vec<u32>> {
    let mut buffer = Vec::with_capacity(n_bytes);
    unsafe {
        let error_code = ffi::_PyLong_AsByteArray(
            long.as_ptr().cast(),           // ptr to PyLong object
            buffer.as_mut_ptr() as *mut u8, // ptr to first byte of buffer
            n_bytes << 2,                   // 4 bytes per u32
            1,                              // little endian
            is_signed.into(),               // signed flag
        );
        if error_code == -1 {
            return Err(PyErr::fetch(long.py()));
        }
        buffer.set_len(n_bytes); // set buffer length to the number of bytes
    };
    buffer
        .iter_mut()
        .for_each(|chunk| *chunk = u32::from_le(*chunk));

    Ok(buffer)
}

/// Converts a Python integer to a vector of 64-bit limbs (little endian). Takes number of bytes to
/// convert to. Multiple of 8. If `is_signed` is true, the integer is treated as signed, and two's
/// complement is returned.
#[cfg(all(not(Py_LIMITED_API), not(feature = "32_bit_limbs")))]
#[inline]
fn int_to_limbs(long: &Bound<PyInt>, n_bytes: usize, is_signed: bool) -> PyResult<Vec<u64>> {
    let mut buffer = Vec::with_capacity(n_bytes);
    unsafe {
        let error_code = ffi::_PyLong_AsByteArray(
            long.as_ptr().cast(),           // ptr to PyLong object
            buffer.as_mut_ptr() as *mut u8, // ptr to first byte of buffer
            n_bytes << 3,                   // 8 bytes per u64
            1,                              // little endian
            is_signed.into(),               // signed flag
        );
        if error_code == -1 {
            return Err(PyErr::fetch(long.py()));
        }
        buffer.set_len(n_bytes); // set buffer length to the number of bytes
    };
    buffer
        .iter_mut()
        .for_each(|chunk| *chunk = u64::from_le(*chunk));

    Ok(buffer)
}

/// Converts a Python integer to a Python bytes object. Bytes are in little endian order. Takes
/// number of bytes to convert to (can be calculated from the number of bits in the integer). If
/// `is_signed` is true, the integer is treated as signed, and two's complement is returned.
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
    Ok(bytes.downcast_into()?)
}

/// Returns the number of bits in the absolute value of the given integer. The number of bits
/// returned is the smallest number of bits that can represent the integer, not the multiple of 8
/// (bytes) that it would take up in memory.
#[inline]
fn int_n_bits(long: &Bound<PyInt>) -> PyResult<usize> {
    let py = long.py();

    #[cfg(not(Py_LIMITED_API))]
    {
        // fast path
        let n_bits = unsafe { ffi::_PyLong_NumBits(long.as_ptr()) };
        if n_bits == (-1isize as usize) {
            return Err(PyErr::fetch(py));
        }
        Ok(n_bits)
    }

    #[cfg(Py_LIMITED_API)]
    {
        // slow path
        long.call_method0(intern!(py, "bit_length"))
            .and_then(|ob| ob.extract())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Prepare Python
    fn prepare_python() {
        pyo3::prepare_freethreaded_python();
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
        let mut f0 = 1i32.into_pyobject(py).unwrap();
        let mut f1 = 1i32.into_pyobject(py).unwrap();
        std::iter::from_fn(move || {
            let f2 = f0
                .call_method1("__add__", (&f1,))
                .unwrap()
                .downcast_into::<PyInt>()
                .unwrap();
            Some(std::mem::replace(&mut f0, std::mem::replace(&mut f1, f2)).unbind())
        })
    }

    /// Generate test python class
    fn python_index_class(py: Python<'_>) -> Bound<'_, PyModule> {
        let index_code = std::ffi::CStr::from_bytes_with_nul(
            concat!(
                r#"
                class C:
                    def __init__(self, x):
                        self.x = x
                    def __index__(self):
                        return self.x
                "#,
                "\0"
            )
            .as_bytes(),
        )
        .unwrap();
        let filename = c"index.py";
        let modulename = c"index";
        PyModule::from_code(py, index_code, filename, modulename).unwrap()
    }

    /// - Test conversion to and from Integer
    /// - Tests the first 2000 numbers in the fibonacci sequence and their negations
    #[test]
    fn convert_integer() {
        prepare_python();
        Python::with_gil(|py| {
            // check the first 2000 numbers in the fibonacci sequence
            for (py_result, rs_result) in python_fib(py).zip(rust_fib::<Integer>()).take(2000) {
                // Python -> Rust
                assert_eq!(py_result.extract::<Integer>(py).unwrap(), rs_result);
                // Rust -> Python
                assert!(
                    py_result
                        .bind(py)
                        .as_any()
                        .eq(rs_result.clone().into_pyobject(py).unwrap())
                        .unwrap()
                );

                // negate
                let rs_result = rs_result * Integer::from(-1);
                let py_result = py_result.call_method0(py, "__neg__").unwrap();

                // Python -> Rust
                assert_eq!(py_result.extract::<Integer>(py).unwrap(), rs_result);
                // Rust -> Python
                assert!(py_result.bind(py).eq(&rs_result).unwrap());
            }
        });
    }

    /// Test Python class conversion
    #[test]
    fn convert_index_class() {
        prepare_python();
        Python::with_gil(|py| {
            let index = python_index_class(py);
            let locals = PyDict::new(py);
            locals.set_item("index", &index).unwrap();
            let expr = c"index.C(10)";
            let ob = py.eval(expr, None, Some(&locals)).unwrap();
            let integer: Integer = <Integer as FromPyObject>::extract_bound(&ob).unwrap();

            assert_eq!(integer, Integer::from(10));

            let expr2 = c"index.C(-10)";
            let ob2 = py.eval(expr2, None, Some(&locals)).unwrap();
            let integer2: Integer = <Integer as FromPyObject>::extract_bound(&ob2).unwrap();

            assert_eq!(integer2, Integer::from(-10));
        });
    }

    /// Test conversion to and from zero
    #[test]
    fn handle_zero() {
        prepare_python();
        Python::with_gil(|py| {
            // Python -> Rust
            let zero_integer: Integer = 0u8.into_pyobject(py).unwrap().extract().unwrap();
            assert_eq!(zero_integer, Integer::from(0));

            // Rust -> Python
            let zero_integer = zero_integer.into_pyobject(py).unwrap();
            assert!(
                zero_integer
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
        Python::with_gil(|py| {
            macro_rules! test {
                ($T:ty, $value:expr, $py:expr) => {
                    let value = $value;
                    println!("{}: {}", stringify!($T), value);
                    let python_value = value.clone().into_pyobject(py).unwrap();
                    let roundtrip_value =
                        <$T as FromPyObject>::extract_bound(&python_value).unwrap();
                    assert_eq!(value, roundtrip_value);
                };
            }

            for i in 0..=256usize {
                // test a lot of values to help catch other bugs too
                test!(Integer, Integer::from(i), py);
                test!(Integer, -Integer::from(i), py);
                test!(Integer, Integer::from(1) << i, py);
                test!(Integer, -Integer::from(1) << i, py);
                test!(Integer, (Integer::from(1) << i) + Integer::from(1u32), py);
                test!(Integer, (-Integer::from(1) << i) + Integer::from(1u32), py);
                test!(Integer, (Integer::from(1) << i) - Integer::from(1u32), py);
                test!(Integer, (-Integer::from(1) << i) - Integer::from(1u32), py);
            }
        });
    }
}
