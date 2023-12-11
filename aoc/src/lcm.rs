use crate::gcd::Gcd;

pub trait Lcm: Sized {
    fn lcm(self, rhs: Self) -> Option<Self>;
}

impl<T> Lcm for T
where
    T: Gcd + std::ops::Mul<Output = T> + std::ops::Div<Output = T> + Copy,
{
    fn lcm(self, rhs: Self) -> Option<Self> {
        Some(self / self.gcd(rhs)? * rhs)
    }
}
