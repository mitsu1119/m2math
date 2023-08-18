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
}


/*
impl IntegerRing {
    pub fn from_str_radix(&self, n_str: &str, radix: i32) -> IntegerRingElement {
        match Integer::from_str_radix(n_str, radix) {
            Ok(v) => IntegerRingElement { val: v, parent: *self },
            Err(e) => {
                panic!("Error from_str_radix: {}", e);
            }
        }
    }

    pub fn from_str(&self, n_str: &str) -> IntegerRingElement {
        self.from_str_radix(n_str, 10)
    }
}
*/

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
        println!("{:?}", x);
    }
}
