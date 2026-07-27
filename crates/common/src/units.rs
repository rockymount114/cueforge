//! Strongly typed SI units to prevent unit confusion in physics calculation.

use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Length in meters (m).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Meters(pub f64);

/// Time in seconds (s).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Seconds(pub f64);

/// Mass in kilograms (kg).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kilograms(pub f64);

/// Speed or velocity magnitude in meters per second (m/s).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MetersPerSecond(pub f64);

/// Angular velocity magnitude in radians per second (rad/s).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RadiansPerSecond(pub f64);

/// Force in newtons (N).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Newtons(pub f64);

/// Angle in radians (rad).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Radians(pub f64);

macro_rules! impl_unit_ops {
    ($Unit:ident) => {
        impl fmt::Display for $Unit {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Add for $Unit {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                $Unit(self.0 + rhs.0)
            }
        }

        impl AddAssign for $Unit {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl Sub for $Unit {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                $Unit(self.0 - rhs.0)
            }
        }

        impl SubAssign for $Unit {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl Mul<f64> for $Unit {
            type Output = Self;
            fn mul(self, rhs: f64) -> Self {
                $Unit(self.0 * rhs)
            }
        }

        impl Mul<$Unit> for f64 {
            type Output = $Unit;
            fn mul(self, rhs: $Unit) -> $Unit {
                $Unit(self * rhs.0)
            }
        }

        impl MulAssign<f64> for $Unit {
            fn mul_assign(&mut self, rhs: f64) {
                self.0 *= rhs;
            }
        }

        impl Div<f64> for $Unit {
            type Output = Self;
            fn div(self, rhs: f64) -> Self {
                $Unit(self.0 / rhs)
            }
        }

        impl DivAssign<f64> for $Unit {
            fn div_assign(&mut self, rhs: f64) {
                self.0 /= rhs;
            }
        }

        impl Div for $Unit {
            type Output = f64;
            fn div(self, rhs: Self) -> f64 {
                self.0 / rhs.0
            }
        }

        impl Neg for $Unit {
            type Output = Self;
            fn neg(self) -> Self {
                $Unit(-self.0)
            }
        }
    };
}

impl_unit_ops!(Meters);
impl_unit_ops!(Seconds);
impl_unit_ops!(Kilograms);
impl_unit_ops!(MetersPerSecond);
impl_unit_ops!(RadiansPerSecond);
impl_unit_ops!(Newtons);
impl_unit_ops!(Radians);
