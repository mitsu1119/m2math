use crate::util::set::Set;

pub trait Element {
    type Parent: Set;
    fn parent(&self) -> Self::Parent;
}
