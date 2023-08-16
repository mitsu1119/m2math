#![feature(unboxed_closures)]
#![feature(fn_traits)]

use std::fmt;
use std::ops::*;
use rug::Integer;

#[derive(Debug, Clone)]
struct IntegerRingElement {
    val: Integer,
}

impl<T> Add<T> for IntegerRingElement where Integer: From<T> {
    type Output = IntegerRingElement;
    fn add(self, _rhs: T) -> IntegerRingElement {
        IntegerRingElement { val: self.val + Integer::from(_rhs) }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ZZ = IntegerRing;
        let x = ZZ(2);
        println!("{}", ZZ);
        println!("{:?}", x.clone() + 2);
        println!("{:?}", x.clone() + 2);
    }
}
