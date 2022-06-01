#![feature(trait_alias)]
use crate::types::{Matrix2x2, Matrix4x4, C};

pub trait QuantumStateTraits {
    fn check_qubit_number(&self, qubits: Vec<&usize>);
    fn reset_all(&mut self);
    fn zero(&mut self);
    fn measure(&mut self, target: &usize);
    fn measure_all(&mut self);
    fn single_qubit_gate(&mut self, target: &usize, u: &Matrix2x2);
    fn single_qubit_kraus(&mut self, target: &usize, u: &Matrix2x2);
    fn two_qubit_gate(&mut self, target: &usize, control: &usize, u: &Matrix4x4);
}


