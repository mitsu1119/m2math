use crate::util::set::Set;

pub trait Element {
    type Parent: for<'a> Set<'a>;
    fn parent(&self) -> &Self::Parent;
}
