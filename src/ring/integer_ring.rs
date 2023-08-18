use std::fmt;
use std::ops::*;
use rug::Integer;
use rug::integer::IsPrime;
use std::cmp::Ordering;
use crate::util::element::Element;
use crate::util::set::Set;
use crate::ring::ring::Ring;

// ----------------------------------------------------------------
// element of Integer Ring
// ----------------------------------------------------------------
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntegerRingElement<'a> {
    val: Integer,
    parent: &'a IntegerRing
}

impl<'a> Ring for IntegerRingElement<'a> {
}

impl fmt::Display for IntegerRingElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<'a> Element for IntegerRingElement<'a> {
    type Parent = IntegerRing;
    fn parent(&self) -> &'a Self::Parent {
        self.parent
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
impl Add for IntegerRingElement<'_> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let mut res = self;
        res += other;
        res
    }
}

impl Sub for IntegerRingElement<'_> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        let mut res = self;
        res -= other;
        res
    }
}

impl Mul for IntegerRingElement<'_> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let mut res = self;
        res *= other;
        res
    }
}

impl Rem for IntegerRingElement<'_> {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        let mut res = self;
        res %= other;
        res
    }
}

impl AddAssign for IntegerRingElement<'_> {
    fn add_assign(&mut self, other: Self) {
        self.val += other.val;
    }
}

impl SubAssign for IntegerRingElement<'_> {
    fn sub_assign(&mut self, other: Self) {
        self.val -= other.val;
    }
}

impl MulAssign for IntegerRingElement<'_> {
    fn mul_assign(&mut self, other: Self) {
        self.val *= other.val;
    }
}

impl RemAssign for IntegerRingElement<'_> {
    fn rem_assign(&mut self, other: Self) {
        self.val %= other.val;
    }
}

// ----------------------------------------------------------------
// Comparison Operators
// ----------------------------------------------------------------
impl PartialOrd for IntegerRingElement<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IntegerRingElement<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

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

impl<'a> Set<'a> for IntegerRing {
    type Child = IntegerRingElement<'a>;
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
        let mut x = ZZ.lift(5);
        println!("{:?}", x);

        x += ZZ.lift(3);
        assert_eq!(x, ZZ.lift(8));

        let y = x + ZZ.lift(2);
        assert_eq!(y, ZZ.lift(10));

        assert_eq!(ZZ.lift(2) + ZZ.lift(5), ZZ.lift(7));
        assert_eq!(ZZ.lift(2) - ZZ.lift(5), ZZ.lift(-3));
        assert_eq!(ZZ.lift(5) - ZZ.lift(2), ZZ.lift(3));
        assert_eq!(ZZ.lift(5) * ZZ.lift(2), ZZ.lift(10));
        assert_eq!(ZZ.lift(10) % ZZ.lift(6), ZZ.lift(4));
        assert_eq!(ZZ.lift(-10) % ZZ.lift(6), ZZ.lift(-4));
        assert_eq!(ZZ.from_str("-10") % ZZ.lift(6), ZZ.lift(-4));
        assert_eq!(*ZZ.lift(5).parent(), ZZ);
        assert_eq!(ZZ.lift(5).parent(), &ZZ);
        assert!(ZZ.lift(5) == ZZ.lift(5));
        assert!(ZZ.lift(5) >= ZZ.lift(5));
        assert!(ZZ.lift(6) >= ZZ.lift(5));
        assert!(ZZ.lift(6) > ZZ.lift(5));
        assert!(ZZ.lift(5) < ZZ.lift(6));
        assert!(ZZ.lift(5) <= ZZ.lift(6));

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
