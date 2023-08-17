use std::fmt;
use crate::ring::integer_ring::*;

// ----------------------------------------------------------------
// Set of Finite field
// ----------------------------------------------------------------
#[derive(Debug)]
pub struct FiniteField {
    order: IntegerRingElement
}

impl fmt::Display for FiniteField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Finite Field of order {}", self.order)
    }
}

impl FiniteField {
    fn new(order: IntegerRingElement) -> Self {
        Self { order: order }
    }
}

pub fn GF(order: IntegerRingElement) -> FiniteField {
    FiniteField::new(order)
}

// ----------------------------------------------------------------
// Test
// ----------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let Fp = GF(ZZ(5));
        println!("{}", Fp);
    }
}
