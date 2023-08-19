pub trait Set<'a> {
    type Child;
}

pub trait Lifter<'a, T>: Set<'a> {
    fn lift(&'a self, val: T) -> Self::Child;
}
