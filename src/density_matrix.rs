use std::mem::size_of_val;

use log::debug;
use nalgebra::ComplexField;

use crate::index_swapping::*;
use crate::types::*;
use itertools::iproduct;


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
        let density_matrix = create_density_matrix(number_of_qubits);
        State {
            number_of_qubits,
            density_matrix,
        }
    }

    pub fn measure(&mut self, target: &Qubit) {
        let swap_qubits = |x| swap_pair(x, target);

        for (i_offset, j_offset) in iproduct!(
            (0..1 << self.number_of_qubits).step_by(2),
            (0..1 << self.number_of_qubits).step_by(2)
        ) {
            for (i, j) in [(0, 1), (1, 0)] {
                self.density_matrix[(swap_qubits(i + i_offset), swap_qubits(j + j_offset))] =
                    C::new(0., 0.)
            }
        }
        debug!("density matrix after:\n{}", self.density_matrix);
    }

    pub fn single_qubit_gate(&mut self, target: &Qubit, u: &Matrix2x2) {
        debug!("density matrix before:\n{}", self.density_matrix);
        let swap = |x| swap_pair(x, target);

        let mut rho = Matrix2x2::zeros();

        iproduct!(
            (0..1 << self.number_of_qubits).step_by(2),
            (0..1 << self.number_of_qubits).step_by(2)
        ).for_each(|(i_offset, j_offset)| {
            iproduct!(0..2, 0..2).for_each(|(i, j)| {
                rho[(i, j)] = self.density_matrix[(swap(i + i_offset), swap(j + j_offset))]
            });
            rho = u * rho * u.adjoint();
            iproduct!(0..2, 0..2).for_each(|(i, j)| {
                self.density_matrix[(swap(i + i_offset), swap(j + j_offset))] = rho[(i, j)]
            });
        });
        debug!("density matrix after:\n{}", self.density_matrix);
    }

    pub fn two_qubit_gate(&mut self, target: &Qubit, control: &Qubit, u: &Matrix4x4) {
        debug!("density matrix before:\n{}", self.density_matrix);
        let swap = |x| swap_two_pairs(x, target, control);
        let mut rho = Matrix4x4::zeros();
        for (i_offset, j_offset) in iproduct!(
            (0..1 << self.number_of_qubits).step_by(4),
            (0..1 << self.number_of_qubits).step_by(4)
        ) {
            iproduct!(0..4, 0..4).for_each(|(i, j)| {
                rho[(i, j)] = self.density_matrix[(swap(i + i_offset), swap(j + j_offset))]
            });
            rho = u * rho * u.adjoint();
            iproduct!(0..4, 0..4).for_each(|(i, j)| {
                self.density_matrix[(swap(i + i_offset), swap(j + j_offset))] = rho[(i, j)]
            });
        }
        debug!("density matrix after:\n{}", self.density_matrix);
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

pub fn assert_approximately_equal(state: State, other_state: State) {
    if !approx_eq(&state, &other_state) {
        println!("state: \n{}", state.density_matrix);
        println!("other state: \n{}", other_state.density_matrix);
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
            panic!(
                "matrices are different sizes: {:#?} =/= {:#?}",
                state.density_matrix.shape(),
                other_state.density_matrix.shape()
            )
        }
    } else {
        panic!(
            "states reprsent different numbers of qubits: {} =/= {}",
            state.number_of_qubits, other_state.number_of_qubits
        )
    }
    result
}
