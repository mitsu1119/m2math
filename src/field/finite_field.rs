use std::fmt;
use crate::util::element::Element;
use crate::util::set::Set;
use crate::ring::integer_ring::*;

// ----------------------------------------------------------------
// element of Integer Ring
// ----------------------------------------------------------------
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FiniteFieldElement<'a> {
    val: IntegerRingElement<'a>,
    parent: &'a FiniteField<'a>
}

impl fmt::Display for FiniteFieldElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<'a> Element for FiniteFieldElement<'a> {
    type Parent = FiniteField<'a>;
    fn parent(&self) -> &'a Self::Parent {
        self.parent
    }
}

// ----------------------------------------------------------------
// Set of Finite Field
// ----------------------------------------------------------------
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FiniteField<'a> {
    order: IntegerRingElement<'a>
}

impl fmt::Display for FiniteField<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Finite Field of order {}", self.order)
    }
}

impl<'a> Set<'a> for FiniteField<'_> {
    type Child = FiniteFieldElement<'a>;
}

impl FiniteField<'_> {
    fn new(order: IntegerRingElement) -> FiniteField {
        // TODO: finite field that order is prime power
        if !order.is_prime() { panic!("The order of FiniteField must be prime."); }
        FiniteField { order: order }
    }
}

fn GF(order: IntegerRingElement) -> FiniteField {
    FiniteField::new(order)
}

// ----------------------------------------------------------------
// Test
// ----------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_words() {
        let Fp = FiniteField::new(ZZ.lift(5));
        let FF = GF(ZZ.lift(5));
        println!("{:?}", Fp);
        println!("{}", Fp);
        assert_eq!(Fp, FF);
    }

    #[test]
    #[should_panic]
    fn test_panic_composite_number_order() {
        FiniteField::new(ZZ.lift(10));
    }
}
