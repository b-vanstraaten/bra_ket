#![macro_use]
use crate::types::Complex;

/// A Convenience macro for complex number construction.
#[macro_export]
macro_rules! c {
    ($re:expr, $im:expr) => {
        Complex::new($re, $im)
    };
}

#[macro_export]
macro_rules! expi {
    ($angle:expr) => {
        Complex::new($angle.cos(), $angle.sin())
    };
}

pub(crate) use {c, expi};