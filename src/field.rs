use std::fmt;

pub trait Field:
    Sized + Eq + Clone + Copy + fmt::Debug + fmt::Display + rand::Rand
{
    fn zero() -> Self;
    fn one() -> Self;
    fn add(&self, other: &Self) -> Self;
    fn subtract(&self, other: &Self) -> Self;
    fn multiply(&self, other: &Self) -> Self;

    // additive inverse
    fn negative(&self) -> Self;

    // multiplicative inverse
    fn inverse(&self) -> Option<Self>;
    fn to_pow(&self, pow: u32) -> Self;
    fn mul_by_scalar(&self, scalar: u32) -> Self;
}
