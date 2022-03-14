use std::mem::size_of_val;

use log::{debug, info, warn};
use nalgebra::ComplexField;

use crate::types::*;

fn create_density_matrix(number_of_qubits: usize) -> DensityMatrix {
    // calculating the hilbert dim
    let hilbert_dim = (1 << number_of_qubits) as usize;
    // printing the size of the density matrix to be created
    {
        let density_matrix_footprint = (hilbert_dim << 2) * size_of_val(&C::new(0., 0.));
        let bytes_to_gigabyte: usize = 2 << 33;
        debug!(
            "Allocating density matrix of size: {:.4} Gb",
            (density_matrix_footprint as f32) / (bytes_to_gigabyte as f32)
        );
    }

    // creating the density matrix
    let mut rho = DensityMatrix::from_element(hilbert_dim, hilbert_dim, C::new(0., 0.));
    // setting the (0, 0) element to 1 to represent initialisation in the |000...> state
    rho[(0, 0)] = C::new(1., 0.);
    return rho;
}

/// A struct to contain the state the quantum experiment. The system is comprised of a
/// quantum register and a classical register. The classical register is described by 'number_of_bits' bits.
/// Whilest the quantum register, of 'number_of_qubits' qubits, must be described by a density matrix.
#[derive(Debug)]
pub struct State {
    pub number_of_qubits: Qubit,
    pub density_matrix: DensityMatrix,
}

impl State {
    pub fn new(number_of_qubits: Qubit) -> State {
        let mut density_matrix = create_density_matrix(number_of_qubits);
        State {
            number_of_qubits,
            density_matrix,
        }
    }

    pub fn reset(&mut self) {
        self.density_matrix = &self.density_matrix * C::new(0., 0.);
        self.density_matrix[(0, 0)] = C::new(1., 0.);
    }

    pub fn is_pure(&self) -> bool {
        let trace = (&self.density_matrix * &self.density_matrix).trace();
        trace.re > (1. - COMPARISON_PRECISION)
    }
}

pub fn assert_approximately_equal(required_state: State, state: State) {
    if !approx_eq(&required_state, &state) {
        println!("final_state: \n{}", state.density_matrix);
        panic!("matrices are different");
    }
}

fn approx_eq(state: &State, other_state: &State) -> bool {
    let mut result = false;
    if state.number_of_qubits == other_state.number_of_qubits {
        if state.density_matrix.shape() == other_state.density_matrix.shape() {
            let difference = &state.density_matrix - &other_state.density_matrix;
            if difference.iter().all(|d| d.abs() < COMPARISON_PRECISION) {
                result = true;
            }
        } else {
            panic!("matrices are different sizes")
        }
    }
    result
}
