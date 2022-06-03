#![macro_use]
use crate::types::Complex;

/// A convenience macro for complex number construction.
#[macro_export]
macro_rules! c {
    ($re:expr, $im:expr) => {
        Complex::new($re, $im)
    };
}
/// A convenience macro to calculate the complex exponential of i times the argument
#[macro_export]
macro_rules! expi {
    ($angle:expr) => {
        Complex::new($angle.cos(), $angle.sin())
    };
}

pub(crate) use {c, expi};