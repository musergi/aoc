pub trait Gcd: Sized {
    fn gcd(self, rhs: Self) -> Option<Self>;
}

impl Gcd for u64 {
    fn gcd(mut self, mut rhs: Self) -> Option<Self> {
        if self != 0 && rhs != 0 {
            while rhs != 0 {
                if rhs < self {
                    std::mem::swap(&mut rhs, &mut self);
                }
                rhs %= self;
            }
            Some(self)
        } else {
            None
        }
    }
}

impl Gcd for usize {
    fn gcd(mut self, mut rhs: Self) -> Option<Self> {
        if self != 0 && rhs != 0 {
            while rhs != 0 {
                if rhs < self {
                    std::mem::swap(&mut rhs, &mut self);
                }
                rhs %= self;
            }
            Some(self)
        } else {
            None
        }
    }
}
