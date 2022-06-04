#![macro_use]
/// A convenience macro for complex number construction.
#[macro_export]
macro_rules! c {
    ($re:expr, $im:expr) => {
        Complex::new($re, $im)
    };
}
pub(crate) use {c};

/// A convenience macro to calculate the complex exponential of the argument
#[macro_export]
macro_rules! exp {
    ($x:expr) => {
        x.exp()
    };
}

/// A convenience macro to calculate the complex exponential of i times the argument
#[macro_export]
macro_rules! expi {
    ($angle:expr) => {
        Complex::new($angle.cos(), $angle.sin())
    };
}
