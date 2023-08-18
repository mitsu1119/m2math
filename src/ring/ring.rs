use std::ops::*;

pub trait Ring: Add + Sub + Mul + AddAssign + SubAssign + MulAssign + PartialEq + Sized {
}
