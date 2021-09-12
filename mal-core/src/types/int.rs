use std::{
    any::Any,
    convert::TryInto,
    fmt::{Debug, Display},
    num::TryFromIntError,
    ops::{Add, Div, Mul, Sub},
};

use super::MalType;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct MalInt {
    value: i64,
}

impl<T> From<T> for MalInt
where
    T: Into<i64>,
{
    fn from(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl TryInto<u64> for MalInt {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<u64, Self::Error> {
        self.value.try_into()
    }
}
impl TryInto<usize> for MalInt {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<usize, Self::Error> {
        self.value.try_into()
    }
}

impl Debug for MalInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for MalInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl MalType for MalInt {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn equal(&self, rhs: &dyn MalType) -> bool {
        match rhs.as_type::<Self>() {
            Ok(int) => self.value == int.value,
            Err(_) => false,
        }
    }
}

impl Add for &MalInt {
    type Output = MalInt;

    fn add(self, rhs: Self) -> Self::Output {
        (self.value + rhs.value).into()
    }
}

impl Sub for &MalInt {
    type Output = MalInt;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.value - rhs.value).into()
    }
}

impl Mul for &MalInt {
    type Output = MalInt;

    fn mul(self, rhs: Self) -> Self::Output {
        (self.value * rhs.value).into()
    }
}

impl Div for &MalInt {
    type Output = MalInt;

    fn div(self, rhs: Self) -> Self::Output {
        (self.value / rhs.value).into()
    }
}
