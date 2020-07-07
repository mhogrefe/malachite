use num::random::random_unsigneds_less_than::RandomUnsignedsLessThan;

#[derive(Clone, Debug)]
pub struct RandomValuesFromVec<T: Clone> {
    pub(crate) xs: Vec<T>,
    pub(crate) indices: RandomUnsignedsLessThan<usize>,
}

impl<T: Clone> Iterator for RandomValuesFromVec<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        Some(self.xs[self.indices.next().unwrap()].clone())
    }
}
