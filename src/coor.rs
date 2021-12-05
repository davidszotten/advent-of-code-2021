use anyhow::{Context, Error, Result};
use std::fmt;
use std::ops::{Add, AddAssign, Mul, Sub};
use std::str::FromStr;

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

impl FromStr for Coor {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = s.split_once(',').context("No comma")?;
        Ok(Coor::new(x.parse()?, y.parse()?))
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

impl Sub for Coor {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + other * -1
    }
}

impl Mul<i64> for Coor {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Coor::new(self.x * rhs, self.y * rhs)
    }
}
