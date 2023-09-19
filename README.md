# Rust Quantum Simulator
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/b-vanstraaten/bra_ket/tests)](https://github.com/b-vanstraaten/bra_ket/actions)


![Rust logo](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

This is a Rust-based quantum simulator that provides an identical interface for state vector and density matrix simulations. It is fully parallelized using the Rayon library. This project was developed as a learning exercise in Rust programming and qubit simulation.

## Features

- Simulate quantum circuits using state vector or density matrix representations.
- Support for common quantum gates and operations.
- Parallelized simulation for improved performance on multi-core processors.
- User-friendly interface for defining and running quantum circuits.

## Installation

To use this quantum simulator, you need to have Rust and Cargo (Rust's package manager) installed on your system. If you don't have Rust installed, you can download it from the official website: [Rust Installation](https://www.rust-lang.org/learn/get-started).
then add it as dependancy in your `Cargo.toml`:

```toml
[dependencies]
bra_ket = "0.1.3"
```
Make sure to replace `"0.1.2"` with the actual version number from the release you want to use.

Then, in your Rust code:
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
    let mut density_matrix = DensityMatrix::new(3);

    // running the program to evolve the state vector from its 
    //initial state of |000> to (1 / SQRT_2) *(|000> + |111>)
    program.run(&mut state_vector);
    program.run(&mut density_matrix);

    // printing the final state vector / density matrix
    println!("state vector \n{}", state_vector);
    println!("density matrix \n{}", density_matrix);
}
```
This code produces the GHZ state (1 / SQRT_2) *(|000> + |111>) in both state vector and density matrix representations.

## Documentation

For detailed documentation and examples, please refer to the [official documentation](https://docs.rs/bra_ket/0.1.3/bra_ket/).

## Contributing

This project was developed as a learning exercise, but contributions are welcome! If you want to contribute, please:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure they pass the existing tests.
4. Add new tests if necessary.
5. Create a pull request to submit your changes.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/b-vanstraaten/bra_ket/blob/main/License.txt) file for details.

## Acknowledgments

- Thanks to the Rust community for creating an excellent language and ecosystem.
- The inspiration for this project comes from the field of quantum computing and quantum circuit simulation.

Happy Quantum Computing! 🌌🔬🧬
