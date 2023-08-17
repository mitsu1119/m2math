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

// ----------------------------------------------------------------
// Factory of Finite field
// ----------------------------------------------------------------
pub struct FiniteFieldFactory;

impl FnOnce<(IntegerRingElement,)> for FiniteFieldFactory {
    type Output = FiniteField;
    extern "rust-call" fn call_once(self, (order,): (IntegerRingElement,)) -> Self::Output {
        FiniteField::new(order)
    }
}

impl FnMut<(IntegerRingElement,)> for FiniteFieldFactory {
    extern "rust-call" fn call_mut(&mut self, (order,): (IntegerRingElement,)) -> Self::Output {
        FiniteField::new(order)
    }
}

impl Fn<(IntegerRingElement,)> for FiniteFieldFactory {
    extern "rust-call" fn call(&self, (order,): (IntegerRingElement,)) -> Self::Output {
        FiniteField::new(order)
    }
}

pub const GF: FiniteFieldFactory = FiniteFieldFactory;

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
