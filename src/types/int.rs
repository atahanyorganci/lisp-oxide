use std::{
    any::Any,
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

use super::MalType;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct MalInt {
    value: i64,
}

impl From<i64> for MalInt {
    fn from(value: i64) -> Self {
        MalInt { value }
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
