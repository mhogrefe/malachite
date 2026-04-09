use malachite_base::num::{
    arithmetic::traits::DivMod,
    basic::traits::{One, Zero},
};

use crate::integer::Integer;

pub trait DivEuclidean: Sized {
    fn div_rem_euclid(self, rhs: Self) -> (Self, Self);

    fn div_rem_euclid_ref(self, rhs: &Self) -> (Self, Self);
}

impl DivEuclidean for Integer {
    fn div_rem_euclid(self, rhs: Self) -> (Self, Self) {
        assert_ne!(rhs, Self::ZERO, "division by zero");

        let (mut q, mut r) = self.div_mod(&rhs);

        if r < Self::ZERO {
            let abs_rhs = &rhs.abs;

            r += Self::from(abs_rhs);

            if rhs > Self::ZERO {
                q -= Self::ONE;
            } else {
                q += Self::ONE;
            }
        }

        (q, r)
    }
    fn div_rem_euclid_ref(self, rhs: &Self) -> (Self, Self) {
        assert_ne!(*rhs, Self::ZERO, "division by zero");

        let (mut q, mut r) = self.div_mod(rhs);

        if r < Self::ZERO {
            let abs_rhs = &rhs.abs;

            r += Self::from(abs_rhs);

            if *rhs > Self::ZERO {
                q -= Self::ONE;
            } else {
                q += Self::ONE;
            }
        }

        (q, r)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::panic)]
#[allow(clippy::expect_used)]
mod test {
    use crate::integer::{Integer, arithmetic::div_euclid::DivEuclidean};

    #[test]
    fn t0() {
        {
            let q_init = 50;
            let r_init: i32 = -23;

            let q_mala = Integer::from(q_init);

            let r_mala = Integer::from(r_init);

            let ret = q_mala.div_rem_euclid(r_mala);

            assert_eq!(ret.0, Integer::from(-2));

            assert_eq!(ret.1, Integer::from(4));
        }

        {
            let q_init = -50;
            let r_init: i32 = -23;

            let q_mala = Integer::from(q_init);

            let r_mala = Integer::from(r_init);

            let ret = q_mala.div_rem_euclid(r_mala);

            assert_eq!(ret.0, Integer::from(3));

            assert_eq!(ret.1, Integer::from(19));
        }
    }
}
