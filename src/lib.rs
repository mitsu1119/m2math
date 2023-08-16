#![feature(unboxed_closures)]
#![feature(fn_traits)]

use std::fmt;
use std::ops::*;
use rug::Integer;

// ----------------------------------------------------------------
// element of Integer Ring
// ----------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
struct IntegerRingElement {
    val: Integer,
}

impl fmt::Display for IntegerRingElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

// ----------------------------------------------------------------
// Arithmetic Operators
// ----------------------------------------------------------------
impl Add for IntegerRingElement {
    type Output = IntegerRingElement;
    fn add(self, item: IntegerRingElement) -> Self::Output {
        IntegerRingElement { val: self.val + item.val }
    }
}

impl Sub for IntegerRingElement {
    type Output = IntegerRingElement;
    fn sub(self, item: IntegerRingElement) -> Self::Output {
        IntegerRingElement { val: self.val - item.val }
    }
}

impl Mul for IntegerRingElement {
    type Output = IntegerRingElement;
    fn mul(self, item: IntegerRingElement) -> Self::Output {
        IntegerRingElement { val: self.val * item.val }
    }
}

// ----------------------------------------------------------------
// Set of Integer Ring
// ----------------------------------------------------------------
#[derive(Debug)]
struct IntegerRing;

impl fmt::Display for IntegerRing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Integer Ring")
    }
}

impl<T> FnOnce<(T,)> for IntegerRing where Integer: From<T> {
    type Output = IntegerRingElement;
    extern "rust-call" fn call_once(self, (val,): (T,)) -> Self::Output {
        IntegerRingElement { val: Integer::from(val) }
    }
}

impl<T> FnMut<(T,)> for IntegerRing where Integer: From<T> {
    extern "rust-call" fn call_mut(&mut self, (val,): (T,)) -> Self::Output {
        IntegerRingElement { val: Integer::from(val) }
    }
}

impl<T> Fn<(T,)> for IntegerRing where Integer: From<T> {
    extern "rust-call" fn call(&self, (val,): (T,)) -> Self::Output {
        IntegerRingElement { val: Integer::from(val) }
    }
}

// ----------------------------------------------------------------
// Test
// ----------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ZZ = IntegerRing;
        assert_eq!(ZZ(2) + ZZ(5), ZZ(7));
        assert_eq!(ZZ(2) - ZZ(5), ZZ(-3));
        assert_eq!(ZZ(5) - ZZ(2), ZZ(3));
        assert_eq!(ZZ(5) * ZZ(2), ZZ(10));
    }
}
