// implement the properties of the density matrix, but for the state vector
use std::mem::size_of_val;

use log::debug;
use nalgebra::ComplexField;

use crate::index_swapping::*;
use crate::types::*;
use itertools::iproduct;
use rand::thread_rng;
use rayon::prelude::*;

fn create_state_vector(number_of_qubits: usize) -> StateVector {
    // calculating the hilbert dim
    let hilbert_dim = (1 << number_of_qubits) as usize;
    // printing the size of the density matrix to be created
    {
        let density_matrix_footprint = hilbert_dim * size_of_val(&C::new(0., 0.));
        let bytes_to_gigabyte: usize = 2 << 33;
        debug!(
            "Allocating density matrix of size: {:.4} Gb",
            (density_matrix_footprint as f32) / (bytes_to_gigabyte as f32)
        );
    }

    // creating the density matrix
    let mut rho = StateVector::from_element(hilbert_dim, C::new(0., 0.));
    // setting the (0, 0) element to 1 to represent initialisation in the |000...> state
    rho[0] = C::new(1., 0.);
    return rho;
}

/// A struct to contain the state the quantum experiment. The system is comprised of a
/// quantum register and a classical register. The classical register is described by 'number_of_bits' bits.
/// Whilst the quantum register, of 'number_of_qubits' qubits, must be described by a state vector.
#[derive(Debug)]
pub struct State {
    pub number_of_qubits: Qubit,
    pub state_vector: StateVector,
    pub state_vector_pointer: StateVectorPointer<C>,
}

impl State {
    pub fn new(number_of_qubits: Qubit) -> State {
        let mut state_vector = create_state_vector(number_of_qubits);
        let state_vector_pointer = StateVectorPointer::new(
            &mut state_vector[0],
            1 << number_of_qubits,
        );
        State {
            number_of_qubits,
            state_vector,
            state_vector_pointer,
        }
    }

    pub fn new_from_density_matrix(
        number_of_qubits: Qubit,
        mut state_vector: StateVector,
    ) -> State {
        let state_vector_pointer = StateVectorPointer::new(
            &mut state_vector[0],
            1 << number_of_qubits,
        );
        State {
            number_of_qubits,
            state_vector,
            state_vector_pointer,
        }
    }

    pub fn index_state(n: usize, target: usize) -> usize {
        let mask: usize = (1 << target) - 1;
        let not_mask: usize = !mask;
        return (n & mask) | ((n & not_mask) << 1);
    }

    pub fn collapse(&mut self, target: &Qubit, collapsed_state: usize, state_sum: C) {
        let index_partial = |x| index_state(x, target);

        for i in 0..1<<(self.number_of_qubits-1) {

            let zero_state: usize = index_partial(i);
            let one_state: usize = zero_state | (1 << target);

            match  collapsed_state {
                0 => {
                    self.state_vector[zero_state] = self.state_vector[zero_state] / state_sum;
                    self.state_vector[one_state] = C::new(0.0, 0.0);
                },
                1 => {
                    self.state_vector[one_state] = self.state_vector[one_state] / state_sum;
                    self.state_vector[zero_state] = C::new(0.0, 0.0);
                },
                _ => (),
            }
        }
    }
    pub fn measure(&mut self, target: &Qubit) {
        debug!("state vector before: \n{}", self.state_vector);
        let index_partial = |x| index_state(x, target);

        let mut sum_zero: C = C::new(0.0, 0.0);
        let mut sum_one: C = C::new(0.0,0.0);

        for i in 0..1<<(self.number_of_qubits-1) {
            let zero_state: usize = index_partial(i);
            let one_state: usize = zero_state | (1 << target);

            sum_zero += self.state_vector[zero_state].modulus_squared();
            sum_one += self.state_vector[one_state].modulus_squared();
        }
        assert_eq!(sum_one + sum_zero, 1);

        let mut rng = rand::thread_rng();
        let n: f64 = rng.gen();

        if sum_zero > n {
            let collapsed_state: usize = 0;
            collapse(&mut self, target, collapsed_state, state_sum);
        }
    }

    pub fn single_qubit_gate(&mut self, target: &Qubit, u: &Matrix2x2) {
        debug!("state vector before: \n{}", self.state_vector);
        let index_partial = |x| index_state(x, target);
        for i in 0..1<<(self.number_of_qubits-1) {

            let zero_state: usize = index_partial(i);
            let one_state: usize = zero_state | (1<<target);

            let zero_amplitude: C = self.state_vector[zero_state];
            let one_amplitude: C = self.state_vector[one_state];

            self.state_vector[zero_state] = zero_amplitude * u[(0,0)] + one_amplitude * u[(0,1)];
            self.state_vector[one_state] = zero_amplitude * u[(1, 0)] + one_amplitude * u[(1,1)];
        }
    }

    pub fn two_qubit_gate(&mut self, target: &Qubit, control: &Qubit, u: &Matrix2x2) {
        debug!("density matrix before:\n{}", self.density_matrix);
        let index_partial = |x| index_state(x, target);
        for i in 0..1<<(self.number_of_qubits-1) {

            let zero_state: usize = index_partial(i);
            let one_state: usize = zero_state | (1<<target);

            let control_val: usize = (((1 << control) & zero_state) > 0).into();

            let zero_amplitude: C = self.state_vector[zero_state];
            let one_amplitude: C = self.state_vector[one_state];

            match control_val {
                0 => {
                    self.state_vector[zero_state] = zero_amplitude * u[(0,0)] + one_amplitude * u[(0,1)]
                },
                1 => {
                    self.state_vector[one_state] = zero_amplitude * u[(1, 0)] + one_amplitude * u[(1,1)];
                } ,
                _ => (),
            }
        }
    }

    // pub fn two_qubit_gate(&mut self, target: &Qubit, control: &Qubit, u: &Matrix4x4) {
    //     debug!("density matrix before:\n{}", self.density_matrix);
    //     let swap = |x| swap_two_pairs(x, target, control);
    //     unsafe {
    //         (0..1 << self.number_of_qubits)
    //             .into_par_iter()
    //             .step_by(4)
    //             .for_each(|n: usize| {
    //                 let mut rho = Matrix4x4::zeros();
    //                 (0..1 << self.number_of_qubits)
    //                     .step_by(4)
    //                     .for_each(|m: usize| {
    //                         iproduct!(0..4, 0..4).for_each(|(i, j)| {
    //                             rho[(i, j)] =
    //                                 self.density_matrix_pointer.read((swap(i + n), swap(j + m)))
    //                         });
    //
    //                         rho = u * rho * u.adjoint();
    //                         iproduct!(0..4, 0..4).for_each(|(i, j)| {
    //                             self.density_matrix_pointer
    //                                 .write((swap(i + n), swap(j + m)), rho[(i, j)])
    //                         });
    //                     })
    //             });
    //     }
    //
    //     debug!("density matrix after:\n{}", self.density_matrix);
    // }
    //
    pub fn reset(&mut self) {
        self.state_vector = &self.state_vector * C::new(0., 0.);
        self.state_vector[0] = C::new(1., 0.);
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
