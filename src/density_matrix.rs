use std::mem::size_of_val;

use itertools::iproduct;
use log::debug;
use nalgebra::ComplexField;
use rayon::prelude::*;

use crate::state_traits::StateTraits;

use crate::helper_functions::*;
use crate::types::*;
use crate::StateVector;
use std::fmt;

/// A density matrix describing an in general mixed quantum state.
#[derive(Clone, Debug)]
pub struct DensityMatrix {
    pub number_of_qubits: usize,
    pub density_matrix: CMatrix,
    density_matrix_pointer: DensityMatrixPointer<Complex>,
}

impl fmt::Display for DensityMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "density matrix:{}", self.density_matrix)
    }
}

impl StateTraits for DensityMatrix {
    fn check_qubit_number(&self, qubits: Vec<&usize>) {
        let required_number_of_qubits = qubits.last().unwrap();
        let number_of_qubits = &self.number_of_qubits;
        assert!(
            required_number_of_qubits < &number_of_qubits,
            "fewer qubits in the density matrix than required by program {} < {}",
            required_number_of_qubits,
            number_of_qubits
        )
    }

    fn reinitialise_all(&mut self) {
        self.zero();
        self.density_matrix[(0, 0)] = Complex::new(1., 0.)
    }

    fn zero(&mut self) {
        (0..1 << &self.number_of_qubits)
            .into_par_iter()
            .for_each(|n: usize| {
                (0..1 << &self.number_of_qubits).for_each(|m: usize| unsafe {
                    self.write((n, m), Complex::new(0., 0.))
                })
            });
    }

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
                            self.write((swap(i + n), swap(j + m)), Complex::new(0., 0.))
                        }
                    });
            });
        debug!("density matrix after:\n{}", self.density_matrix);
    }

    fn measure_all(&mut self) {
        (0..1 << self.number_of_qubits)
            .into_par_iter()
            .for_each(|n: usize| {
                (0..1 << self.number_of_qubits).for_each(|m: usize| unsafe {
                    if n != m {
                        self.write((n, m), Complex::new(0., 0.))
                    }
                })
            });
    }

    fn single_qubit_gate(&mut self, target: &usize, u: &Matrix2x2) {
        debug!("density matrix before:\n{}", self.density_matrix);
        let swap = |x| swap_pair(x, target);

        unsafe {
            (0..1 << &self.number_of_qubits)
                .into_par_iter()
                .step_by(2)
                .for_each(|n: usize| {
                    let mut rho = Matrix2x2::zeros();
                    (0..1 << &self.number_of_qubits)
                        .step_by(2)
                        .for_each(|m: usize| {
                            iproduct!(0..2, 0..2).for_each(|(i, j)| {
                                rho[(i, j)] =
                                    self.read((swap(i + n), swap(j + m)))
                            });

                            rho = u * rho * u.adjoint();
                            iproduct!(0..2, 0..2).for_each(|(i, j)| {
                                self
                                    .write((swap(i + n), swap(j + m)), rho[(i, j)])
                            });
                        })
                });
        }
    }

    fn single_qubit_kraus(&mut self, _target: &usize, _u: &Matrix2x2) {
        todo!("not implemented yet");
    }

    fn two_qubit_gate(&mut self, target: &usize, control: &usize, u: &Matrix4x4) {
        debug!("density matrix before:\n{}", self.density_matrix);
        let swap = |x| swap_two_pairs(x, target, control);

        (0..1 << &self.number_of_qubits)
            .into_par_iter()
            .step_by(4)
            .for_each(|n: usize| unsafe {
                let mut rho = Matrix4x4::zeros();
                (0..1 << &self.number_of_qubits)
                    .step_by(4)
                    .for_each(|m: usize| {
                        iproduct!(0..4, 0..4).for_each(|(i, j)| {
                            rho[(i, j)] =
                                self.read((swap(i + n), swap(j + m)))
                        });

                        rho = u * rho * u.adjoint();

                        iproduct!(0..4, 0..4).for_each(|(i, j)| {
                            self.write((swap(i + n), swap(j + m)), rho[(i, j)])
                        });
                    })
            });
        debug!("density matrix after:\n{}", self.density_matrix);
    }

    fn get_probability(&self, _target: &usize) -> Real {
        todo!()
    }

    fn get_expectation(&self, _target: &usize) -> Real {
        todo!()
    }
}

impl PartialEq for DensityMatrix {
    fn eq(&self, other: &Self) -> bool {
        let mut result = false;
        if self.number_of_qubits == other.number_of_qubits {
            if self.density_matrix.shape() == other.density_matrix.shape() {
                let difference = &self.density_matrix - &other.density_matrix;
                if difference.iter().all(|d| d.abs() < COMPARISON_PRECISION) {
                    result = true;
                }
            }
        }
        result
    }
}

impl From<CMatrix> for DensityMatrix {
    /// Create a density matrix from a complex matrix
    fn from(mut density_matrix: CMatrix) -> Self {
        let shape = density_matrix.shape();
        assert_eq!(
            shape.0, shape.1,
            "density matrix not square {} =/= {}",
            shape.0, shape.1
        );
        let number_of_qubits = log2(shape.0 as usize);

        let density_matrix_pointer = DensityMatrixPointer::new(
            &mut density_matrix[(0, 0)], shape);

        DensityMatrix {
            number_of_qubits,
            density_matrix,
            density_matrix_pointer,
        }

    }
}

impl From<StateVector> for DensityMatrix {
    fn from(state_vector: StateVector) -> Self {
        let density_matrix = DensityMatrix::new(state_vector.number_of_qubits);
        (0..1 << &state_vector.number_of_qubits)
            .into_par_iter()
            .for_each(|n: usize| {
                (0..1 << &state_vector.number_of_qubits).for_each(|m: usize| unsafe {
                    let s_n = state_vector.read(n);
                    let s_m = state_vector.read(m);
                    density_matrix
                        .density_matrix_pointer
                        .write((n, m), s_n * s_m.conj())
                })
            });
        density_matrix
    }
}

impl DensityMatrix {
    pub fn new(number_of_qubits: usize) -> DensityMatrix {
        let mut density_matrix = {
            // calculating the hilbert dim
            let hilbert_dim = 1 << number_of_qubits;
            // printing the size of the density matrix to be created
            {
                let density_matrix_footprint = (hilbert_dim << 2) * size_of_val(&Complex::new(0., 0.));
                let bytes_to_gigabyte: usize = 2 << 33;
                debug!(
                    "Allocating density matrix of size: {:.4} Gb",
                    (density_matrix_footprint as f32) / (bytes_to_gigabyte as f32)
                );
            }

            // creating the density matrix
            let mut rho = CMatrix::from_element(hilbert_dim, hilbert_dim, Complex::new(0., 0.));
            // setting the (0, 0) element to 1 to represent initialisation in the |000...> state
            rho[(0, 0)] = Complex::new(1., 0.);
            rho
        };
        let density_matrix_pointer = DensityMatrixPointer::new(
            &mut density_matrix[(0, 0)],
            (1 << &number_of_qubits, 1 << &number_of_qubits),
        );

        DensityMatrix {
            number_of_qubits,
            density_matrix,
            density_matrix_pointer,
        }
    }

    pub fn is_pure(&self) -> bool {
        let trace = (&self.density_matrix * &self.density_matrix).trace();
        trace.re > (1. - COMPARISON_PRECISION)
    }

    pub unsafe fn write(&self, indices: (usize, usize), value: Complex) {
        self.density_matrix_pointer.write(indices, value)
    }

    pub unsafe fn read(&self, indices: (usize, usize)) -> Complex {
        self.density_matrix_pointer.read(indices)
    }
}
