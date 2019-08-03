use crate::field::Field;

const P: u32 = 1613;

#[derive(Debug, Clone, Copy)]
pub struct G1613(u32);

impl G1613 {
    pub fn new(v: u32) -> Self {
        G1613(v % P)
    }
}

impl Field for G1613 {


    fn one() -> Self {
        G1613(1)
    }

    fn zero() -> Self {
        G1613(0)
    }

    fn add(&self, other: &Self) -> Self {
        G1613((self.0 + other.0) % P)
    }

    fn subtract(&self, other: &Self) -> Self {
        let a = self.0;
        let b = other.0;
        if a > b {
            G1613(a-b)
        } else {
            G1613(a + other.negative().0)
        }
    }

    fn multiply(&self, other: &Self) -> Self {
        G1613((self.0 * other.0) % P)
    }

    fn negative(&self) -> Self {
        if self.0 == 0 {
            G1613(0)
        } else {
            G1613(P - self.0)
        }
    }

    fn inverse(&self) -> Option<Self> {
        if self.0 == 0 {
            None
        } else {
            let p_int = P as i32;
            let (_, _, y) = extended_euclid(p_int, self.0 as i32);
            let modp = ((y % p_int + p_int) % p_int) as u32;
            Some(G1613(modp))
        }
    }

    fn to_pow(&self, pow: u32) -> Self {
        let mut p = pow;
        let mut base = self.0;
        let mut res = 1;
        while p > 0 {
            if p % 2 == 1 {
                res = (res * base) % P
            }
            p = p >> 1;
            base = (base*base) % P
        }
        G1613(res)
    }

    fn mul_by_scalar(&self, scalar: u32) -> Self {
        G1613((self.0 * scalar) % P)
    }
}

use std::fmt;

impl fmt::Display for G1613 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


use rand::Rng;

impl rand::Rand for G1613 {
    fn rand<R: Rng>(rng: &mut R) -> G1613 {
        let u = rng.gen_range::<u32>(0, 1613);
        G1613(u)
    }
}

impl PartialEq for G1613 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for G1613 {

}

fn extended_euclid(a: i32, b: i32) -> (i32, i32, i32) {
    fn f(a: i32, b: i32, s: i32, s_: i32, t: i32, t_: i32) -> (i32, i32, i32) {
        let quotient = a / b;
        let rest = a - quotient*b;
        match rest {
            0 => (b, s_, t_),
            _ => f(b, rest, s_, s-quotient*s_, t_, t-quotient*t_)
        }
    }
    f(a, b, 1, 0, 0, 1)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclid() {
        let (gcd, x, y) = extended_euclid(1914, 899);
        assert_eq!(gcd, 29);
        assert_eq!(x, 8);
        assert_eq!(y, -17);

        let (_, x, y) = extended_euclid(123211, 1432);
        assert_eq!(x, 267);
        assert_eq!(y, -22973);
    }

    #[test]
    fn test_to_pow() {
        let g = G1613(325);
        assert_eq!(g.to_pow(177).0, 84);
        let g = G1613(1115);
        assert_eq!(g.to_pow(127).0, 497);
    }

    #[test]
    fn test_inverse() {
        let g1_ = G1613(190);
        let g2_ = G1613(820);
        assert_eq!(g1_.inverse().unwrap().0, 1214);
        assert_eq!(g2_.inverse().unwrap().0, 478);
    }

    const g1: G1613 = G1613(192);
    const g2: G1613 = G1613(851);
    const g3: G1613 = G1613(1600);
    const g4: G1613 = G1613(1227);

    #[test]
    fn test_add() {
        assert_eq!(g2.add(&g3).0, (851+1600) % P);
        assert_eq!(g3.add(&g4).0, (1600+1227) % P);
        assert_eq!(g1.add(&g2).0, (192+851));
    }

    #[test]
    fn test_subtract() {
        assert_eq!(g3.subtract(&g2).0, 1600-851);
        assert_eq!(g2.subtract(&g3).0, 864)
    }

    #[test]
    fn test_multiply() {
        assert_eq!(g3.multiply(&g4).0, 179);
        assert_eq!(g1.multiply(&g2).0, 479)
    }
}
