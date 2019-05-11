pub fn reshape_2_1_to_3<A: 'static, B: 'static, C: 'static>(
    it: Box<Iterator<Item = ((A, B), C)>>,
) -> Box<Iterator<Item = (A, B, C)>> {
    Box::new(it.map(|((a, b), c)| (a, b, c)))
}

pub fn reshape_1_2_to_3<A: 'static, B: 'static, C: 'static>(
    it: Box<Iterator<Item = (A, (B, C))>>,
) -> Box<Iterator<Item = (A, B, C)>> {
    Box::new(it.map(|(a, (b, c))| (a, b, c)))
}

pub fn reshape_3_1_to_4<A: 'static, B: 'static, C: 'static, D: 'static>(
    it: Box<Iterator<Item = ((A, B, C), D)>>,
) -> Box<Iterator<Item = (A, B, C, D)>> {
    Box::new(it.map(|((a, b, c), d)| (a, b, c, d)))
}

pub fn reshape_2_2_to_4<A: 'static, B: 'static, C: 'static, D: 'static>(
    it: Box<Iterator<Item = ((A, B), (C, D))>>,
) -> Box<Iterator<Item = (A, B, C, D)>> {
    Box::new(it.map(|((a, b), (c, d))| (a, b, c, d)))
}

pub fn permute_2_1<A: 'static, B: 'static>(
    it: Box<Iterator<Item = (A, B)>>,
) -> Box<Iterator<Item = (B, A)>> {
    Box::new(it.map(|(a, b)| (b, a)))
}

pub fn permute_1_3_2<A: 'static, B: 'static, C: 'static>(
    it: Box<Iterator<Item = (A, B, C)>>,
) -> Box<Iterator<Item = (A, C, B)>> {
    Box::new(it.map(|(a, b, c)| (a, c, b)))
}

pub fn permute_1_2_4_3<A: 'static, B: 'static, C: 'static, D: 'static>(
    it: Box<Iterator<Item = (A, B, C, D)>>,
) -> Box<Iterator<Item = (A, B, D, C)>> {
    Box::new(it.map(|(a, b, c, d)| (a, b, d, c)))
}
