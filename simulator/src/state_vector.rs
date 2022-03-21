use crate::state::{Measure, MeasureAll, Reset, SingleQubitGate, SingleQubitKraus, TwoQubitGate};
use crate::types::*;
use rayon::prelude::*;

use itertools::iproduct;
use log::debug;
use crate::index_swapping::*;

#[derive(Debug)]
pub struct StateVector {
    pub number_of_qubits: usize,
    pub state_vector: CVector,
    pub state_vector_pointer: StateVectorPointer<C>,
    pub classical_register: ClassicalRegister,
}

impl Reset for StateVector {
    fn reset(&mut self) {
        todo!();
    }
}

impl Measure for StateVector {
    fn measure(&mut self, target: &usize) {
        todo!();
    }
}

impl MeasureAll for StateVector {
    fn measure_all(&mut self) {
        todo!();
    }
}

impl SingleQubitGate for StateVector {
    fn single_qubit_gate(&mut self, target: &usize, u: &Matrix2x2) {
        todo!();
    }
}

impl SingleQubitKraus for StateVector {
    fn single_qubit_kraus(&mut self, target: &usize, u: &Matrix2x2) {
        panic!("Kraus operators cannot be performed on state vectors");
    }
}

impl TwoQubitGate for StateVector {
    fn two_qubit_gate(&mut self, target: &usize, control: &usize, u: &Matrix4x4) {
        todo!()
        }
}