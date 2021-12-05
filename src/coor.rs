use std::fmt;
use std::ops::{Add, AddAssign, Mul};

#[derive(PartialEq, Eq, Default, Clone, Copy, Hash)]
pub struct Coor {
    pub x: i64,
    pub y: i64,
}

impl Coor {
    pub const fn new(x: i64, y: i64) -> Self {
        Coor { x, y }
    }
}
impl fmt::Debug for Coor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Coor {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coor::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Coor {
    // type Output = Self;

    fn add_assign(&mut self, other: Self) {
        // Coor::new(self.x + other.x, self.y + other.y)
        *self = *self + other;
    }
}

impl Mul<i64> for Coor {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Coor::new(self.x * rhs, self.y * rhs)
    }
}
