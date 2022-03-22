use crate::types::{Matrix2x2, Matrix4x4, C};

pub trait Reset {
    fn reset(&mut self);
}

pub trait Measure {
    fn measure(&mut self, target: &usize);
}

pub trait MeasureAll {
    fn measure_all(&mut self);
}

pub trait SingleQubitGate {
    fn single_qubit_gate(&mut self, target: &usize, u: &Matrix2x2);
}

pub trait SingleQubitKraus {
    fn single_qubit_kraus(&mut self, target: &usize, u: &Matrix2x2);
}

pub trait TwoQubitGate {
    fn two_qubit_gate(&mut self, target: &usize, control: &usize, u: &Matrix4x4);
}

pub trait Collapse {
    fn collapse(&mut self, target: &usize, collapsed_state: &usize, state_sum: C);
}
