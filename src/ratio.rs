use std::ops::{Div, Mul};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ratio {
    pub numer: i32,
    pub denom: i32,
}

impl Mul<Ratio> for Ratio {
    type Output = Ratio;

    fn mul(self, rhs: Ratio) -> Self::Output {
        Self::new(self.numer * rhs.numer, self.denom * rhs.denom)
    }
}

impl Div<Ratio> for Ratio {
    type Output = Ratio;

    fn div(self, rhs: Ratio) -> Self::Output {
        Self::new(self.numer * rhs.denom, rhs.numer * self.denom)
    }
}

impl From<&Ratio> for f32 {
    fn from(value: &Ratio) -> Self {
        value.numer as f32 / value.denom as f32
    }
}

impl Ratio {
    pub fn new(numer: i32, denom: i32) -> Self {
        let (numer, denom) = reduce(numer, denom);
        Self { numer, denom }
    }

    pub fn normalize(&self) -> Self {
        let f: f32 = self.into();

        match f {
            n if n < 1. => Self::new(self.numer * 2, self.denom).normalize(),
            n if n >= 2. => Self::new(self.numer, self.denom * 2).normalize(),
            _ => Self::new(self.numer, self.denom),
        }
    }

    pub fn complement(&self) -> Self {
        (Self::new(2, 1) / *self).normalize()
    }

    pub fn pow(&self, exp: i32) -> Self {
        match exp {
            n if n == 0 => Self::new(1, 1),
            n if n < 0 => self.complement().pow(-exp),
            _ => Self::new(self.numer.pow(exp as u32), self.denom.pow(exp as u32)),
        }
    }
}

fn reduce(a: i32, b: i32) -> (i32, i32) {
    let g = gcd(a, b);
    (a / g, b / g)
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while a % b > 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ratio() {
        let r = Ratio::new(3, 2);

        assert_eq!(r.numer, 3);
        assert_eq!(r.denom, 2);
    }

    #[test]
    fn reduces_ratio() {
        let r = Ratio::new(3, 6);

        assert_eq!(r.numer, 1);
        assert_eq!(r.denom, 2);
    }

    #[test]
    fn normalize() {
        let r = Ratio::new(1, 2);
        assert_eq!(r.normalize(), Ratio::new(1, 1));
    }

    #[test]
    fn multiply() {
        let r1 = Ratio::new(3, 2);
        let r2 = Ratio::new(9, 8);

        assert_eq!(r1 * r2, Ratio::new(27, 16));
    }

    #[test]
    fn divide() {
        let r1 = Ratio::new(3, 2);
        let r2 = Ratio::new(9, 8);

        assert_eq!(r1 / r2, Ratio::new(4, 3));
        assert_eq!(r2 / r1, Ratio::new(3, 4));
    }

    #[test]
    fn complement() {
        let r1 = Ratio::new(3, 2);

        assert_eq!(r1.complement(), Ratio::new(4, 3))
    }

    #[test]
    fn pow() {
        let r = Ratio::new(3, 2);

        assert_eq!(r.pow(0), Ratio::new(1, 1));
        assert_eq!(r.pow(2), Ratio::new(9, 4));
        assert_eq!(r.pow(-2), Ratio::new(16, 9));
    }
}
