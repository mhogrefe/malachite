// Copyright © 2026 Mikhail Hogrefe
//
// PyO3 integration contributed by Antonio Mamić.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

fn main() {
    #[cfg(feature = "enable_pyo3")]
    pyo3_build_config::use_pyo3_cfgs();
    fft_avx2_cfg();
}

// Sets `fft_avx2` when the FFT multiplication needs a second copy of its kernels, compiled for AVX2
// and FMA and selected at run time: on x86, with `std` (which provides the CPU feature detection),
// and only when the whole crate is not already compiled with those features.
fn fft_avx2_cfg() {
    println!("cargo::rustc-check-cfg=cfg(fft_avx2)");
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let features = std::env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
    let has = |feature: &str| features.split(',').any(|f| f == feature);
    if (arch == "x86" || arch == "x86_64")
        && std::env::var_os("CARGO_FEATURE_STD").is_some()
        && !(has("avx2") && has("fma"))
    {
        println!("cargo::rustc-cfg=fft_avx2");
    }
}
