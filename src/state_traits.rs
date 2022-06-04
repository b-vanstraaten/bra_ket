use crate::types::{Matrix2x2, Matrix4x4, RVector};
use crate::types::Real;

/// The traits every quantum state must possess to permit it being acted upon by a quantum program.
/// In our case both the StateVector and DensityMatrix implement these traits.
pub trait StateTraits {
    /// Checks the qubit number to make sure it is compatible with the quantum program.
    fn check_qubit_number(&self, qubits: Vec<&usize>);
    /// Reinitialises all qubits in their ground state.
    fn reinitialise_all(&mut self);
    /// Sets the every element of the state vector or density matrix to 0. + 0.i. (mainly a convenience function)
    fn zero(&mut self);
    /// Measures the target qubit, collapsing its quantum state.
    fn measure(&mut self, target: &usize);
    /// Measures all the qubits, collapsing the quantum state completely.
    fn measure_all(&mut self);
    /// Performs a single qubit unitary gate on the target qubit.
    fn single_qubit_gate(&mut self, target: &usize, u: &Matrix2x2);
    /// Performs a single qubit kraus operation on the target qubit.
    fn single_qubit_kraus(&mut self, target: &usize, u: &Matrix2x2);
    /// Performs a two qubit gate on the target and control qubits.
    fn two_qubit_gate(&mut self, target: &usize, control: &usize, u: &Matrix4x4);
    /// calculates the probability of measuring |0> in the target qubit
    fn get_probability(&self, target: &usize) -> Real;
    fn get_expectation(&self, target: &usize) -> Real;
}


