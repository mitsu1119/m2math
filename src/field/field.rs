use std::ops::*;
use crate::ring::ring::Ring;

pub trait Field: Ring + Div {
}
