use crate::traits::{
    Measure, MeasureAll, ResetAll, SingleQubitGate, SingleQubitKraus, TwoQubitGate, Zero,
};
use crate::types::*;
use nalgebra::ComplexField;
use rayon::prelude::*;
use std::mem::size_of_val;

use itertools::iproduct;
use log::debug;

use crate::helper_functions::*;

use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::seq::index::sample;

#[derive(Debug)]
pub struct StateVector {
    pub number_of_qubits: usize,
    pub state_vector: CVector,
    pub state_vector_pointer: StateVectorPointer<C>,
    pub classical_register: ClassicalRegister,
}

impl From<CVector> for StateVector {
    fn from(mut state_vector: CVector) -> Self {
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


impl PartialEq for StateVector {
    fn eq(&self, other: &Self) -> bool {
        let mut result = false;
        if self.number_of_qubits == other.number_of_qubits {
            if self.state_vector.shape() == other.state_vector.shape() {
                let difference = &self.state_vector - &other.state_vector;
                if difference.iter().all(|d| d.abs() < COMPARISON_PRECISION) {
                    result = true;
                }
            }
        }
        result
    }
}

impl Zero for StateVector {
    fn zero(&mut self) {
        unsafe {
            (0..1 << self.number_of_qubits)
                .into_par_iter()
                .for_each(|n: usize| {
                    self.state_vector_pointer.write(n, C::new(0., 0.));
                })
        }
    }
}

impl ResetAll for StateVector {
    fn reset_all(&mut self) {
        self.zero();
        self.state_vector[0] = C::new(1., 0.);
    }
}

impl Measure for StateVector {
    fn measure(&mut self, target: &usize) {
        debug!("state vector before: \n{}", self.state_vector);
        let swap = |x| swap_pair(x, target);
        unsafe {
            let (mut p0, mut p1): (R, R) = (0..1 << self.number_of_qubits)
                .into_par_iter()
                .step_by(2)
                .map(|i| {
                    let p0 = self.state_vector_pointer.read(swap(i)).modulus_squared();
                    let p1 = self
                        .state_vector_pointer
                        .read(swap(i + 1))
                        .modulus_squared();
                    (p0, p1)
                })
                .reduce(|| (0., 0.), |(a0, a1), (b0, b1)| (a0 + b0, a1 + b1));

            p0 = p0 / (p0 + p1);
            p1 = p1 / (p0 + p1);

            let mut rng = thread_rng();
            let probabilities = [p0, p1];
            let dist = WeightedIndex::new(&probabilities).unwrap();
            let s = dist.sample(&mut rng);
            let p_sqrt = probabilities[s].sqrt();

            (0..1 << self.number_of_qubits)
                .into_par_iter()
                .step_by(2)
                .for_each(|n: usize| {
                    let i0 = swap(n);
                    let i1 = swap(n + 1);
                    match s {
                        0 => {
                            let s0 = self.state_vector_pointer.read(i1);
                            self.state_vector_pointer.write(i0, s0 / p_sqrt);
                            self.state_vector_pointer.write(i1, C::new(0., 0.))
                        }
                        _ => {
                            let s1 = self.state_vector_pointer.read(i1);
                            self.state_vector_pointer.write(i0, C::new(0., 0.));
                            self.state_vector_pointer.write(i1, s1 / p_sqrt);
                        }
                    }
                })
        }
    }
}

impl MeasureAll for StateVector {
    fn measure_all(&mut self) {
        let probabilities: Vec<R> = (0..1 << self.number_of_qubits)
            .into_par_iter()
            .map(|n: usize| unsafe { self.state_vector_pointer.read(n).modulus_squared() })
            .collect();

        let dist = WeightedIndex::new(&probabilities).unwrap();
        let mut rng = thread_rng();
        let s = dist.sample(&mut rng);

        self.zero();
        self.state_vector[s] = C::new(1., 0.);
    }
}

impl SingleQubitGate for StateVector {
    fn single_qubit_gate(&mut self, target: &usize, u: &Matrix2x2) {
        let swap = |x| swap_pair(x, target);
        unsafe {
            (0..1 << self.number_of_qubits)
                .into_par_iter()
                .step_by(2)
                .for_each(|n: usize| {
                    let i0 = swap(n);
                    let i1 = swap(n + 1);

                    let s0 = self.state_vector_pointer.read(i0);
                    let s1 = self.state_vector_pointer.read(i1);

                    self.state_vector_pointer
                        .write(i0, u[(0, 0)] * s0 + u[(0, 1)] * s1);
                    self.state_vector_pointer
                        .write(i1, u[(1, 0)] * s0 + u[(1, 1)] * s1);
                })
        }
    }
}

impl SingleQubitKraus for StateVector {
    fn single_qubit_kraus(&mut self, target: &usize, u: &Matrix2x2) {
        panic!("Kraus operators cannot be performed on state vectors");
    }
}

impl TwoQubitGate for StateVector {
    fn two_qubit_gate(&mut self, target: &usize, control: &usize, u: &Matrix4x4) {
        debug!("State vector before:\n{}", self.state_vector);
        let swap = |x| swap_two_pairs(x, target, control);

        unsafe {
            (0..1 << self.number_of_qubits)
                .into_par_iter()
                .step_by(4)
                .for_each(|n: usize| {
                    let i0 = swap(n);
                    let i1 = swap(n + 1);
                    let i2 = swap(n + 2);
                    let i3 = swap(n + 3);

                    let s0 = self.state_vector_pointer.read(i0);
                    let s1 = self.state_vector_pointer.read(i1);
                    let s2 = self.state_vector_pointer.read(i2);
                    let s3 = self.state_vector_pointer.read(i3);

                    for (i, index) in [i0, i1, i2, i3].iter().enumerate() {
                        self.state_vector_pointer.write(
                            *index,
                            u[(i, 0)] * s0 + u[(i, 1)] * s1 + u[(i, 2)] * s2 + u[(i, 3)] * s3,
                        );
                    }
                })
        }
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
        let state_vector_pointer = StateVectorPointer::new(&mut state_vector[0], hilbert_dim);

        let classical_register = ClassicalRegister::zeros(number_of_qubits);

        StateVector {
            number_of_qubits,
            state_vector,
            state_vector_pointer,
            classical_register,
        }
    }
}
