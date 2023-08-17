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
pub struct IntegerRingElement {
    val: Integer,
    parent: IntegerRing
}

impl fmt::Display for IntegerRingElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl IntegerRingElement {
    pub fn is_prime(&self) -> bool {
        match self.val.is_probably_prime(100) {
            IsPrime::No => false,
            _ => true
        }
    }
}

impl Element for IntegerRingElement {
    type Parent = IntegerRing;
    fn parent(&self) -> Self::Parent {
        self.parent.clone()
    }
}

// ----------------------------------------------------------------
// Arithmetic Operators
// ----------------------------------------------------------------
impl Add for IntegerRingElement {
    type Output = Self;
    fn add(self, item: Self) -> Self::Output {
        IntegerRingElement { val: self.val + item.val, parent: self.parent }
    }
}

impl Sub for IntegerRingElement {
    type Output = Self;
    fn sub(self, item: Self) -> Self::Output {
        IntegerRingElement { val: self.val - item.val, parent: self.parent }
    }
}

impl Mul for IntegerRingElement {
    type Output = Self;
    fn mul(self, item: Self) -> Self::Output {
        IntegerRingElement { val: self.val * item.val, parent: self.parent }
    }
}

impl Rem for IntegerRingElement {
    type Output = Self;
    fn rem(self, item: Self) -> Self::Output {
        IntegerRingElement { val: self.val % item.val, parent: self.parent }
    }
}

impl AddAssign for IntegerRingElement {
    fn add_assign(&mut self, other: Self) {
        *self = Self { val: &self.val + other.val, parent: self.parent };
    }
}

impl SubAssign for IntegerRingElement {
    fn sub_assign(&mut self, other: Self) {
        *self = Self { val: &self.val - other.val, parent: self.parent };
    }
}

impl MulAssign for IntegerRingElement {
    fn mul_assign(&mut self, other: Self) {
        *self = Self { val: &self.val * other.val, parent: self.parent };
    }
}

impl RemAssign for IntegerRingElement {
    fn rem_assign(&mut self, other: Self) {
        *self = Self { val: &self.val & other.val, parent: self.parent };
    }
}

// ----------------------------------------------------------------
// Comparison Operators
// ----------------------------------------------------------------
impl PartialOrd for IntegerRingElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IntegerRingElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

// ----------------------------------------------------------------
// Set of Integer Ring
// ----------------------------------------------------------------
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct IntegerRing;

impl Set for IntegerRing {
    type Child = IntegerRingElement;
}

impl fmt::Display for IntegerRing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Integer Ring")
    }
}

impl<T> FnOnce<(T,)> for IntegerRing where Integer: From<T> {
    type Output = IntegerRingElement;
    extern "rust-call" fn call_once(self, (val,): (T,)) -> Self::Output {
        IntegerRingElement { val: Integer::from(val), parent: self }
    }
}

impl<T> FnMut<(T,)> for IntegerRing where Integer: From<T> {
    extern "rust-call" fn call_mut(&mut self, (val,): (T,)) -> Self::Output {
        IntegerRingElement { val: Integer::from(val), parent: *self }
    }
}

impl<T> Fn<(T,)> for IntegerRing where Integer: From<T> {
    extern "rust-call" fn call(&self, (val,): (T,)) -> Self::Output {
        IntegerRingElement { val: Integer::from(val), parent: *self }
    }
}

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

pub const ZZ: IntegerRing = IntegerRing;

// ----------------------------------------------------------------
// Test
// ----------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = ZZ(5);
        println!("ueiuei {}", x.parent());
        assert_eq!(ZZ(2) + ZZ(5), ZZ(7));
        assert_eq!(ZZ(2) - ZZ(5), ZZ(-3));
        assert_eq!(ZZ(5) - ZZ(2), ZZ(3));
        assert_eq!(ZZ(5) * ZZ(2), ZZ(10));
        assert_eq!(ZZ(10) % ZZ(6), ZZ(4));
        assert_eq!(ZZ(-10) % ZZ(6), ZZ(-4));
        assert!(ZZ(5) == ZZ(5));
        assert!(ZZ(5) >= ZZ(5));
        assert!(ZZ(6) >= ZZ(5));
        assert!(ZZ(6) > ZZ(5));
        assert!(ZZ(5) < ZZ(6));
        assert!(ZZ(5) <= ZZ(6));
        assert!(!ZZ(10).is_prime());
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
