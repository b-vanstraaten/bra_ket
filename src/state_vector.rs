use crate::state_traits::QuantumStateTraits;
use crate::types::*;
use nalgebra::ComplexField;
use rayon::prelude::*;
use std::mem::size_of_val;

use log::debug;

use crate::helper_functions::*;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

/// A state vector describing a pure quantum state.
#[derive(Debug, Clone)]
pub struct StateVector {
    pub number_of_qubits: usize,
    pub state_vector: CVector,
    pub classical_register: ClassicalRegister,
    state_vector_pointer: StateVectorPointer<Complex>,
}

impl From<CVector> for StateVector {
    fn from(mut state_vector: CVector) -> Self {
        let shape = state_vector.shape();
        let number_of_qubits = log2(shape.0 as usize);

        let state_vector_pointer = StateVectorPointer::new(&mut state_vector[0]);
        let classical_register = vec![None; number_of_qubits];

        StateVector {
            number_of_qubits,
            state_vector,
            state_vector_pointer,
            classical_register,
        }
    }
}

impl QuantumStateTraits for StateVector {

    fn check_qubit_number(&self, qubits: Vec<&usize>) {
        let required_number_of_qubits = match qubits.last() {
            Some(n) => n.to_owned(),
            None => &0 // no gates in gate_list
        };

        let number_of_qubits = &self.number_of_qubits;
        assert!(required_number_of_qubits < number_of_qubits,
                "fewer qubits in the state vector than required by program {} < {}",
                required_number_of_qubits, number_of_qubits
        )
    }

    fn reinitialise_all(&mut self) {
        self.zero();
        self.state_vector[0] = Complex::new(1., 0.);
        self.classical_register = vec![None; self.number_of_qubits]
    }

    fn zero(&mut self) {
        unsafe {
            (0..1 << self.number_of_qubits)
                .into_par_iter()
                .for_each(|n: usize| {
                    self.write(n, Complex::new(0., 0.));
                })
        }
    }

    fn measure(&mut self, target: &usize) {
        debug!("state vector before: \n{}", self.state_vector);
        let swap = |x| swap_pair(x, target);

        // calculating the probabilities
        unsafe {
            let (p0, p1): (Real, Real) = (0..1 << self.number_of_qubits)
                .into_par_iter()
                .step_by(2)
                .map(|i| {
                    let p0 = self.read(swap(i)).modulus_squared();
                    let p1 = self
                        .state_vector_pointer
                        .read(swap(i + 1))
                        .modulus_squared();
                    (p0, p1)
                })
                .reduce(|| (0., 0.), |(a0, a1), (b0, b1)| (a0 + b0, a1 + b1));

            // sampling randomly from the qubit probability distribution
            let mut rng = thread_rng();
            let probabilities = [p0, p1];
            let dist = WeightedIndex::new(&probabilities).unwrap();
            let qubit_state = dist.sample(&mut rng);

            // updating the classical register
            self.classical_register[target.to_owned()] = Some(qubit_state == 1);

            // updating the state vector
            let p_sqrt = probabilities[qubit_state].sqrt();
            (0..1 << self.number_of_qubits)
                .into_par_iter()
                .step_by(2)
                .for_each(|n: usize| {
                    // getting indexes of the |..0..> and |..1..> states
                    let i0 = swap(n);
                    let i1 = swap(n + 1);
                    match qubit_state {
                        0 => { // qubit in state |0>
                            let s0 = self.read(i1);
                            self.write(i0, s0 / p_sqrt);
                            self.write(i1, Complex::new(0., 0.))
                        }
                        _ => { // qubit in state |1>
                            let s1 = self.read(i1);
                            self.write(i0, Complex::new(0., 0.));
                            self.write(i1, s1 / p_sqrt);
                        }
                    }
                })
        }
    }

    fn measure_all(&mut self) {

        // calculating the probability of measuring each state
        let probabilities: Vec<Real> = (0..1 << self.number_of_qubits)
            .into_par_iter()
            .map(|n: usize| unsafe { self.read(n).modulus_squared() })
            .collect();

        // sampling from the probability distribution
        let dist = WeightedIndex::new(&probabilities).unwrap();
        let mut rng = thread_rng();
        let s = dist.sample(&mut rng);

        // updating the classical register
        for n in 0..self.number_of_qubits {
            // extracting the bit value at position n in s
            let qubit_state = (s & (1 << n)) >> n;
            self.classical_register[n] = Some(qubit_state == 1)
        }

        // updating the state_vector
        self.zero();
        self.state_vector[s] = Complex::new(1., 0.);
    }

    fn single_qubit_gate(&mut self, target: &usize, u: &Matrix2x2) {
        let swap = |x| swap_pair(x, target);
        unsafe {
            (0..1 << self.number_of_qubits)
                .into_par_iter()
                .step_by(2)
                .for_each(|n: usize| {
                    let i0 = swap(n);
                    let i1 = swap(n + 1);

                    let s0 = self.read(i0);
                    let s1 = self.read(i1);

                    self.write(i0, u[(0, 0)] * s0 + u[(0, 1)] * s1);
                    self.write(i1, u[(1, 0)] * s0 + u[(1, 1)] * s1);
                })
        }
    }

    fn single_qubit_kraus(&mut self, _target: &usize, _u: &Matrix2x2) {
        panic!("Kraus operators cannot be performed on state vectors");
    }

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

                    let s0 = self.read(i0);
                    let s1 = self.read(i1);
                    let s2 = self.read(i2);
                    let s3 = self.read(i3);

                    for (i, index) in [i0, i1, i2, i3].iter().enumerate() {
                        self.write(
                            index.to_owned(),
                            u[(i, 0)] * s0 + u[(i, 1)] * s1 + u[(i, 2)] * s2 + u[(i, 3)] * s3,
                        );
                    }
                })
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



impl StateVector {
    pub fn new(number_of_qubits: usize) -> StateVector {
        let hilbert_dim = 1 << number_of_qubits;
        let mut state_vector = {
            // calculating the hilbert dim
            // printing the size of the density matrix to be created
            {
                let state_vector_footprint = (hilbert_dim << 2) * size_of_val(&Complex::new(0., 0.));
                let bytes_to_gigabyte: usize = 2 << 33;
                debug!(
                    "Allocating density matrix of size: {:.4} Gb",
                    (state_vector_footprint as f32) / (bytes_to_gigabyte as f32)
                );
            }

            // creating the density matrix
            let mut state_vector = CVector::from_element(hilbert_dim, Complex::new(0., 0.));
            // setting the (0, 0) element to 1 to represent initialisation in the |000...> state
            state_vector[0] = Complex::new(1., 0.);
            state_vector
        };
        let state_vector_pointer = StateVectorPointer::new(&mut state_vector[0]);

        let classical_register = vec![None; number_of_qubits];

        StateVector {
            number_of_qubits,
            state_vector,
            state_vector_pointer,
            classical_register,
        }
    }

    pub fn measured_qubit_state(&self, target: usize) -> bool {
        match self.classical_register[target] {
            Some(qubit_state) => qubit_state,
            None => panic!("qubit {} not measured yet", target)
        }
    }

    pub fn measured_overall_state(&self) -> Int {
        let mut overall_state = 0;
        for qubit in 0..self.number_of_qubits {
            let qubit_state = self.measured_qubit_state(qubit);
            overall_state += (1 & (qubit_state as Int)) << qubit;
        }
        overall_state
    }

    pub fn reset_classical_register(&mut self) {
        self.classical_register = vec![None; self.number_of_qubits];
    }

    pub unsafe fn write(&self, index: usize, value: Complex) {
        self.state_vector_pointer.write(index, value);
    }

    pub unsafe fn read(&self, index: usize) -> Complex {
        self.state_vector_pointer.read(index)
    }

}
