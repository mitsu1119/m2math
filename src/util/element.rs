use crate::util::set::Set;

pub trait Element {
    type Parent;
    fn parent(&self) -> Self::Parent;
}
