#![feature(unboxed_closures)]
#![feature(fn_traits)]

use std::fmt;
use std::ops::{FnOnce, FnMut, Fn};
use rug::Integer;

#[derive(Debug)]
struct IntegerRingElement {
    val: Integer,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ZZ = IntegerRing;
        println!("{}", ZZ);
        println!("{:?}", ZZ(3 + 5));
    }
}
