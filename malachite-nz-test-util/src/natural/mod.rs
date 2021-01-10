use malachite_base_test_util::stats::moments::CheckedToF64;
use malachite_nz::natural::conversion::floating_point_from_natural::gt_max_finite_f64;
use malachite_nz::natural::Natural;

pub struct NaturalCheckedToF64Wrapper(pub Natural);

impl CheckedToF64 for NaturalCheckedToF64Wrapper {
    #[inline]
    fn checked_to_f64(&self) -> f64 {
        assert!(!gt_max_finite_f64(&self.0));
        f64::from(&self.0)
    }
}

pub mod arithmetic;
pub mod comparison;
pub mod logic;
