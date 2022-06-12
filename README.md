
A program to create the 3 qubit Greenberger–Horne–Zeilinger state represented as both a state 
vector and density matrix. 

```rust
use bra_ket::*;
fn main() {
   
    // creating the quantum program 
    let mut program = Program::new();
    program.h(0); 
    program.cnot(0, 1); 
    program.cnot(0, 2);
    
    // drawing the program 
    program.draw();

    // creating a three qubit state vector / density matrix to hold the quantum state
    let mut state_vector = StateVector::new(3);
    let mut density_matrix = DensityMatrix::new(3)
    
    // running the program to evolve the state vector from its 
    //initial state of |000> to (1 / SQRT_2) *(|000> + |111>)
    program.run(&mut state_vector);
    program.run(&mut density_matrix);

    // printing the final state vector / density matrix
    println!("state vector \n{}", state_vector);
    println!("density matrix \n{}", density_matrix);
}

```