use crate::state::{Measure, MeasureAll, Reset, SingleQubitGate, SingleQubitKraus, TwoQubitGate};
use crate::types::*;
use rayon::prelude::*;
use std::mem::size_of_val;
use nalgebra::ComplexField;

use itertools::iproduct;
use log::debug;
use crate::index_swapping::*;

use crate::helper_functions::log2;

#[derive(Debug)]
pub struct StateVector {
    pub number_of_qubits: usize,
    pub state_vector: CVector,
    pub state_vector_pointer: StateVectorPointer<C>,
    pub classical_register: ClassicalRegister,
}

impl Reset for StateVector {
    fn reset(&mut self) {
        todo!("not implemented yet");
    }
}

impl Measure for StateVector {
    fn measure(&mut self, target: &usize) {
        todo!("not implemented yet");
    }
}

impl MeasureAll for StateVector {
    fn measure_all(&mut self) {
        todo!("not implemented yet");
    }
}

impl SingleQubitGate for StateVector {
    fn single_qubit_gate(&mut self, target: &usize, u: &Matrix2x2) {
        todo!("not implemented yet");
    }
}

impl SingleQubitKraus for StateVector {
    fn single_qubit_kraus(&mut self, target: &usize, u: &Matrix2x2) {
        panic!("Kraus operators cannot be performed on state vectors");
    }
}

impl TwoQubitGate for StateVector {
    fn two_qubit_gate(&mut self, target: &usize, control: &usize, u: &Matrix4x4) {
        todo!("not implemented yet");
        }
}

impl StateVector {
    pub fn new(number_of_qubits: usize) -> StateVector {
        let hilbert_dim = 1 << number_of_qubits;
        let mut state_vector = {
            // calculating the hilbert dim
            // printing the size of the density matrix to be created
            {
                let state_vector_footprint = (hilbert_dim << 2) * size_of_val(&C::new(0., 0.));
                let bytes_to_gigabyte: usize = 2 << 33;
                debug!(
                    "Allocating density matrix of size: {:.4} Gb",
                    (state_vector_footprint as f32) / (bytes_to_gigabyte as f32)
                );
            }

            // creating the density matrix
            let mut state_vector = CVector::from_element(hilbert_dim, C::new(0., 0.));
            // setting the (0, 0) element to 1 to represent initialisation in the |000...> state
            state_vector[0] = C::new(1., 0.);
            state_vector
        };
        let state_vector_pointer = StateVectorPointer::new(
            &mut state_vector[0],
            hilbert_dim,
        );

        let classical_register = ClassicalRegister::zeros(number_of_qubits);

        StateVector {
            number_of_qubits,
            state_vector,
            state_vector_pointer,
            classical_register,
        }
    }

    pub fn new_from_state_vector(mut state_vector: CVector) -> StateVector {
        let shape = state_vector.shape();
        let number_of_qubits = log2(shape.0 as usize);

        let state_vector_pointer = StateVectorPointer::new(&mut state_vector[0], shape.1);
        let classical_register = ClassicalRegister::zeros(number_of_qubits);

        StateVector {
            number_of_qubits,
            state_vector,
            state_vector_pointer,
            classical_register,
        }
    }

}

pub fn assert_approximately_equal_vector(state: &StateVector, other_state: &StateVector) {
    if !approx_eq(&state, &other_state) {
        println!("state: \n{}", state.state_vector);
        println!("other state: \n{}", other_state.state_vector);
        panic!("states are different");
    }
}

fn approx_eq(state: &StateVector, other_state: &StateVector) -> bool {
    let mut result = false;
    if state.number_of_qubits == other_state.number_of_qubits {
        if state.state_vector.shape() == other_state.state_vector.shape() {
            let difference = &state.state_vector - &other_state.state_vector;
            if difference.iter().all(|d| d.abs() < COMPARISON_PRECISION) {
                result = true;
            }
        } else {
            panic!(
                "vector are different sizes: {:#?} =/= {:#?}",
                state.state_vector.shape(),
                other_state.state_vector.shape()
            )
        }
    } else {
        panic!(
            "vectors represent different numbers of qubits: {} =/= {}",
            state.number_of_qubits, other_state.number_of_qubits
        )
    }
    result
}