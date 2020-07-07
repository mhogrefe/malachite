use num::random::random_unsigneds_less_than::RandomUnsignedsLessThan;

#[derive(Clone, Debug)]
pub struct RandomValuesFromSlice<'a, T> {
    pub(crate) xs: &'a [T],
    pub(crate) indices: RandomUnsignedsLessThan<usize>,
}

impl<'a, T> Iterator for RandomValuesFromSlice<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        Some(&self.xs[self.indices.next().unwrap()])
    }
}
