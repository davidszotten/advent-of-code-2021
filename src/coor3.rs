use anyhow::{Context, Error, Result};
use std::fmt;
use std::ops::{Add, AddAssign, Mul, Sub};
use std::str::FromStr;

#[derive(PartialEq, Eq, Default, Clone, Copy, Hash)]
pub struct Coor3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Coor3 {
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Coor3 { x, y, z }
    }
}
impl fmt::Debug for Coor3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl FromStr for Coor3 {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let (x, yz) = s.split_once(',').context("No 1st comma")?;
        let (y, z) = yz.split_once(',').context("No 2nd comma")?;
        Ok(Coor3::new(x.parse()?, y.parse()?, z.parse()?))
    }
}

impl From<(i64, i64, i64)> for Coor3 {
    fn from(tup: (i64, i64, i64)) -> Self {
        let (x, y, z) = tup;
        Coor3 { x, y, z }
    }
}

impl Add for Coor3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coor3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Coor3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Coor3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + other * -1
    }
}

impl Mul<i64> for Coor3 {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Coor3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl PartialOrd for Coor3 {
    fn partial_cmp(&self, other: &Coor3) -> Option<std::cmp::Ordering> {
        (self.x, self.y, self.x).partial_cmp(&(other.x, other.y, other.z))
    }
}

impl Ord for Coor3 {
    fn cmp(&self, other: &Coor3) -> std::cmp::Ordering {
        (self.x, self.y, self.x).cmp(&(other.x, other.y, other.z))
    }
}
