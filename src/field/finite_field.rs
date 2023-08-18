use std::fmt;
use crate::ring::integer_ring::*;

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

impl FiniteField<'_> {
    fn new(order: IntegerRingElement) -> FiniteField {
        // TODO: 位数が素数冪の有限体
        if !order.is_prime() { panic!("The order of FiniteField must be prime."); }
        FiniteField { order: order }
    }
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
        println!("{:?}", Fp);
        println!("{}", Fp);
    }

    #[test]
    #[should_panic]
    fn test_panic_composite_number_order() {
        FiniteField::new(ZZ.lift(10));
    }
}
