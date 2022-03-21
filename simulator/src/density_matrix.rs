use std::mem::size_of_val;

use itertools::iproduct;
use log::debug;
use nalgebra::ComplexField;
use rayon::prelude::*;

use crate::helper_functions::log2;
use crate::index_swapping::*;
use crate::state::{Measure, MeasureAll, Reset, SingleQubitGate, SingleQubitKraus, TwoQubitGate};
use crate::types::*;

/// A struct to contain the state the quantum experiment. The system is comprised of a
/// quantum register and a classical register. The classical register is described by 'number_of_bits' bits.
/// Whilest the quantum register, of 'number_of_qubits' qubits, must be described by a density matrix.
#[derive(Debug)]
pub struct DensityMatrix {
    pub number_of_qubits: usize,
    pub density_matrix: CMatrix,
    pub density_matrix_pointer: DensityMatrixPointer<C>,
    pub classical_register: ClassicalRegister,
}

impl Reset for DensityMatrix {
    fn reset(&mut self) {
        (0..1 << self.number_of_qubits)
            .into_par_iter()
            .for_each(|n: usize| {
                (0..1 << self.number_of_qubits).for_each(|m: usize| unsafe {
                    self.density_matrix_pointer.write((n, m), C::new(0., 0.))
                })
            });
    }
}

impl Measure for DensityMatrix {
    fn measure(&mut self, target: &usize) {
        let swap = |x| swap_pair(x, target);
        (0..1 << self.number_of_qubits)
            .into_par_iter()
            .step_by(2)
            .for_each(|n: usize| unsafe {
                (0..1 << self.number_of_qubits)
                    .step_by(2)
                    .for_each(|m: usize| {
                        for (i, j) in [(0, 1), (1, 0)] {
                            self.density_matrix_pointer
                                .write((swap(i + n), swap(j + m)), C::new(0., 0.))
                        }
                    });
            });
        debug!("density matrix after:\n{}", self.density_matrix);
    }
}

impl MeasureAll for DensityMatrix {
    fn measure_all(&mut self) {
        (0..1 << self.number_of_qubits)
            .into_par_iter()
            .for_each(|n: usize| {
                (0..1 << self.number_of_qubits).for_each(|m: usize| unsafe {
                    if n != m {
                        self.density_matrix_pointer.write((n, m), C::new(0., 0.))
                    }
                })
            });
    }
}

impl SingleQubitGate for DensityMatrix {
    fn single_qubit_gate(&mut self, target: &usize, u: &Matrix2x2) {
        debug!("density matrix before:\n{}", self.density_matrix);
        let swap = |x| swap_pair(x, target);

        unsafe {
            (0..1 << self.number_of_qubits)
                .into_par_iter()
                .step_by(2)
                .for_each(|n: usize| {
                    let mut rho = Matrix2x2::zeros();
                    (0..1 << self.number_of_qubits)
                        .step_by(2)
                        .for_each(|m: usize| {
                            iproduct!(0..2, 0..2).for_each(|(i, j)| {
                                rho[(i, j)] =
                                    self.density_matrix_pointer.read((swap(i + n), swap(j + m)))
                            });

                            rho = u * rho * u.adjoint();
                            iproduct!(0..2, 0..2).for_each(|(i, j)| {
                                self.density_matrix_pointer
                                    .write((swap(i + n), swap(j + m)), rho[(i, j)])
                            });
                        })
                });
        }
    }
}

impl SingleQubitKraus for DensityMatrix {
    fn single_qubit_kraus(&mut self, target: &usize, u: &Matrix2x2) {
        todo!("not implemented yet");
    }
}

impl TwoQubitGate for DensityMatrix {
    fn two_qubit_gate(&mut self, target: &usize, control: &usize, u: &Matrix4x4) {
        debug!("density matrix before:\n{}", self.density_matrix);
        let swap = |x| swap_two_pairs(x, target, control);

        (0..1 << self.number_of_qubits)
            .into_par_iter()
            .step_by(4)
            .for_each(|n: usize| unsafe {
                let mut rho = Matrix4x4::zeros();
                (0..1 << self.number_of_qubits)
                    .step_by(4)
                    .for_each(|m: usize| {
                        iproduct!(0..4, 0..4).for_each(|(i, j)| {
                            rho[(i, j)] =
                                self.density_matrix_pointer.read((swap(i + n), swap(j + m)))
                        });

                        rho = u * rho * u.adjoint();

                        iproduct!(0..4, 0..4).for_each(|(i, j)| {
                            self.density_matrix_pointer
                                .write((swap(i + n), swap(j + m)), rho[(i, j)])
                        });
                    })
            });
        debug!("density matrix after:\n{}", self.density_matrix);
    }
}

impl DensityMatrix {
    pub fn new(number_of_qubits: usize) -> DensityMatrix {
        let mut density_matrix = {
            // calculating the hilbert dim
            let hilbert_dim = 1 << number_of_qubits;
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
            let mut rho = CMatrix::from_element(hilbert_dim, hilbert_dim, C::new(0., 0.));
            // setting the (0, 0) element to 1 to represent initialisation in the |000...> state
            rho[(0, 0)] = C::new(1., 0.);
            rho
        };
        let density_matrix_pointer = DensityMatrixPointer::new(
            &mut density_matrix[(0, 0)],
            (1 << number_of_qubits, 1 << number_of_qubits),
        );

        let classical_register = ClassicalRegister::zeros(number_of_qubits);

        DensityMatrix {
            number_of_qubits,
            density_matrix,
            density_matrix_pointer,
            classical_register,
        }
    }

    pub fn new_from_density_matrix(mut density_matrix: CMatrix) -> DensityMatrix {
        let shape = density_matrix.shape();
        assert_eq!(shape.0, shape.1, "density matrix not square {} =/= {}", shape.0, shape.1);
        let number_of_qubits = log2(shape.0 as usize);

        let density_matrix_pointer = DensityMatrixPointer::new(&mut density_matrix[(0, 0)], shape);
        let classical_register = ClassicalRegister::zeros(number_of_qubits);

        DensityMatrix {
            number_of_qubits,
            density_matrix,
            density_matrix_pointer,
            classical_register,
        }
    }

    pub fn is_pure(&self) -> bool {
        let trace = (&self.density_matrix * &self.density_matrix).trace();
        trace.re > (1. - COMPARISON_PRECISION)
    }
}

pub fn assert_approximately_equal_density(state: &DensityMatrix, other_state: &DensityMatrix) {
    if !approx_eq(&state, &other_state) {
        println!("state: \n{}", state.density_matrix);
        println!("other state: \n{}", other_state.density_matrix);
        panic!("matrices are different");
    }
}

fn approx_eq(state: &DensityMatrix, other_state: &DensityMatrix) -> bool {
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
            "states represent different numbers of qubits: {} =/= {}",
            state.number_of_qubits, other_state.number_of_qubits
        )
    }
    result
}
