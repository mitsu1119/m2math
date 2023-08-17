use std::fmt;
use std::ops::*;
use rug::Integer;
use std::cmp::Ordering;

// ----------------------------------------------------------------
// element of Integer Ring
// ----------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
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
    type Output = Self;
    fn add(self, item: Self) -> Self::Output {
        IntegerRingElement { val: self.val + item.val }
    }
}

impl Sub for IntegerRingElement {
    type Output = Self;
    fn sub(self, item: Self) -> Self::Output {
        IntegerRingElement { val: self.val - item.val }
    }
}

impl Mul for IntegerRingElement {
    type Output = Self;
    fn mul(self, item: Self) -> Self::Output {
        IntegerRingElement { val: self.val * item.val }
    }
}

impl AddAssign for IntegerRingElement {
    fn add_assign(&mut self, other: Self) {
        *self = Self { val: &self.val + other.val };
    }
}

impl SubAssign for IntegerRingElement {
    fn sub_assign(&mut self, other: Self) {
        *self = Self { val: &self.val - other.val };
    }
}

impl MulAssign for IntegerRingElement {
    fn mul_assign(&mut self, other: Self) {
        *self = Self { val: &self.val * other.val };
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

impl IntegerRing {
    fn from_str_radix(&self, n_str: &str, radix: i32) -> IntegerRingElement {
        match Integer::from_str_radix(n_str, radix) {
            Ok(v) => IntegerRingElement { val: v },
            Err(e) => {
                panic!("Error from_str_radix: {}", e);
            }
        }
    }

    fn from_str(&self, n_str: &str) -> IntegerRingElement {
        self.from_str_radix(n_str, 10)
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
        assert!(ZZ(5) == ZZ(5));
        assert!(ZZ(5) >= ZZ(5));
        assert!(ZZ(6) >= ZZ(5));
        assert!(ZZ(6) > ZZ(5));
        assert!(ZZ(5) < ZZ(6));
        assert!(ZZ(5) <= ZZ(6));
    }

    #[test]
    #[should_panic]
    fn test_panic_radix_from_str() {
        let ZZ = IntegerRing;
        ZZ.from_str("12345678913280321980321804372894327894327894327899f");
    }
    
    #[test]
    #[should_panic]
    fn test_panic_radix_from_str_radix() {
        let ZZ = IntegerRing;
        ZZ.from_str_radix("12345678913280321980321804372894327894327894327899", 8);
    }
}
