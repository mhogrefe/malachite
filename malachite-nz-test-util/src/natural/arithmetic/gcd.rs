use malachite_base::slices::slice_test_zero;
use malachite_nz::natural::arithmetic::gcd::half_gcd::{HalfGcdMatrix, HalfGcdMatrix1};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

pub fn half_gcd_matrix_create(s: usize, n: usize, data: Vec<Limb>) -> HalfGcdMatrix {
    assert!(n <= s);
    assert_eq!(data.len(), s << 2);
    HalfGcdMatrix {
        data,
        s,
        two_s: s << 1,
        three_s: s * 3,
        n,
    }
}

pub fn half_gcd_matrix_to_naturals(m: &HalfGcdMatrix) -> (Natural, Natural, Natural, Natural) {
    let n = m.n;
    (
        Natural::from_limbs_asc(&m.get(0, 0)[..n]),
        Natural::from_limbs_asc(&m.get(0, 1)[..n]),
        Natural::from_limbs_asc(&m.get(1, 0)[..n]),
        Natural::from_limbs_asc(&m.get(1, 1)[..n]),
    )
}

pub fn half_gcd_matrix_1_to_naturals(m_1: &HalfGcdMatrix1) -> (Natural, Natural, Natural, Natural) {
    (
        Natural::from(m_1.data[0][0]),
        Natural::from(m_1.data[0][1]),
        Natural::from(m_1.data[1][0]),
        Natural::from(m_1.data[1][1]),
    )
}

pub fn half_gcd_matrix_create_string(m: &HalfGcdMatrix) -> String {
    format!("half_gcd_matrix_create({}, {}, vec!{:?})", m.s, m.n, m.data)
}

pub fn half_gcd_matrix_all_elements_nonzero(m: &HalfGcdMatrix) -> bool {
    for i in 0..2 {
        for j in 0..2 {
            if slice_test_zero(m.get(i, j)) {
                return false;
            }
        }
    }
    true
}
