fn main() {
    #[cfg(feature = "enable_pyo3")]
    pyo3_build_config::use_pyo3_cfgs();
}