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
        // TODO: finite field that the order is a prime power
        if !order.is_prime() { panic!("The order of a finite field must be a prime"); }
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

    #[test]
    #[should_panic]
    fn test_panic_finite_field_composite_number_order() {
        GF(ZZ.from_str("12345678913280321980321804372894327894327894327899"));
    }

    #[test]
    #[should_panic]
    fn test_panic_finite_field_prime_power_order() {
        // q = 2647^10
        GF(ZZ.from_str("16886491005078742190744687163972049"));
    }
}
