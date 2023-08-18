use std::fmt;
use std::ops::*;
use rug::Integer;
use rug::integer::IsPrime;
use std::cmp::Ordering;
use crate::util::element::Element;
use crate::util::set::Set;

// ----------------------------------------------------------------
// element of Integer Ring
// ----------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerRingElement<'a> {
    val: Integer,
    parent: &'a IntegerRing
}

impl fmt::Display for IntegerRingElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl IntegerRingElement<'_> {
    pub fn is_prime(&self) -> bool {
        match self.val.is_probably_prime(100) {
            IsPrime::No => false,
            _ => true
        }
    }
}

// ----------------------------------------------------------------
// Arithmetic Operators
// ----------------------------------------------------------------

// ----------------------------------------------------------------
// Set of Integer Ring
// ----------------------------------------------------------------
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct IntegerRing;

impl fmt::Display for IntegerRing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Integer Ring")
    }
}

impl<'a> IntegerRing {
    fn lift<T>(&'a self, val: T) -> IntegerRingElement where Integer: From<T> {
        IntegerRingElement {  val: Integer::from(val), parent: self }
    }

    fn from_str_radix(&'a self, n_str: &str, radix: i32) -> IntegerRingElement {
        match Integer::from_str_radix(n_str, radix) {
            Ok(v) => IntegerRingElement { val: v, parent: self },
            Err(e) => {
                panic!("Error from_str_radix: {}", e);
            }
        }
    }

    fn from_str(&'a self, n_str: &str) -> IntegerRingElement {
        self.from_str_radix(n_str, 10)
    }
}

pub const ZZ: IntegerRing = IntegerRing;

// ----------------------------------------------------------------
// Test
// ----------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = ZZ.lift(5);
        println!("{:?}", x);
        assert!(!ZZ.from_str("63881801352479295820993181514863074496724272670065922597057931030213707690709").is_prime());
        assert!(ZZ.from_str("12513184754843391318297613509216180345616625665202041567883117414712490368118894860874737593058204662888282042487382831063055422314262328170908247278561361").is_prime());
    }

    #[test]
    #[should_panic]
    fn test_panic_radix_from_str() {
        ZZ.from_str("12345678913280321980321804372894327894327894327899f");
    }
    
    #[test]
    #[should_panic]
    fn test_panic_radix_from_str_radix() {
        ZZ.from_str_radix("12345678913280321980321804372894327894327894327899", 8);
    }
}
