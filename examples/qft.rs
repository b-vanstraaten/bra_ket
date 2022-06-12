use bra_ket::*;
use std::time::{Instant};
use gnuplot::{Figure, Caption, Color, AxesCommon};

/// function to time the call of function f
fn time_and_average<T: StateTraits>(program: Program, state: &mut T) -> (usize, f32, Option<f32>) {
    let start_time = Instant::now();
    let mut times_ns:Vec<u128> = Vec::new();
    while start_time.elapsed().as_millis() < 1000 && times_ns.len() < 1000 {
        let t0 = Instant::now();
        program.run(state);
        times_ns.push(t0.elapsed().as_nanos())
    }

    let times_s: Vec<f32> = times_ns.iter().map(|t| *t as f32 * 1e-9).collect();

    let number_of_averages = times_s.len();
    let sum: f32 = times_s.iter().sum();
    let squared_sum: f32 = times_s.iter().map(|x| x * x).sum();
    let mean = sum / number_of_averages as f32;
    let squared_mean = squared_sum as f32 / number_of_averages as f32;

    let std = match number_of_averages {
        1 => None,
        _ => {
            let std = (squared_mean - mean * mean).sqrt() / (number_of_averages as f32).sqrt();
            Some(std)
        }
    };

    (number_of_averages, mean, std)
}

fn main() {
    //vectors to hold the data relating to timing the evolution of state vectors of various sizes
    let n_state_vector = Vec::from_iter(1..25);
    let mut means_state_vector: Vec<f32> = Vec::with_capacity(n_state_vector.len());
    let mut stds_state_vector: Vec<Option<f32>> = Vec::with_capacity(n_state_vector.len());

    println!("\nState Vector");
    for n in &n_state_vector {
        // creating a the quantum fourier transform program
        let program = qft(*n);
        let mut state = StateVector::new(*n);

        let (number_of_averages, mean, std) = time_and_average(program, &mut state);
        means_state_vector.push(mean);
        stds_state_vector.push(std);

        let message = match std {
            Some(std) => format!("The qft of {} qubits took {} +/- {:?}s (averged over {} runs)",
                                 n, mean, std, number_of_averages),
            None => format!("The qft of {} qubits took {}s (averaged over 1 run)",
                            n, mean)
        };
        println!("{}", message)
    };

    //vectors to hold the data relating to timing the evolution of density matrix of various sizes
    let n_density_matrix = Vec::from_iter(1..13);
    let mut means_density_matrix: Vec<f32> = Vec::with_capacity(n_state_vector.len());
    let mut stds_density_matrix: Vec<Option<f32>> = Vec::with_capacity(n_state_vector.len());

    println!("\nDensity Matrix");
    for n in &n_density_matrix {
        // creating the quantum fourier transform program
        let program = qft(*n);
        let mut state = DensityMatrix::new(*n);

        let (number_of_averages, mean, std) = time_and_average(program, &mut state);
        means_density_matrix.push(mean);
        stds_density_matrix.push(std);

        let message = match std {
            Some(std) => format!("The qft of {} qubits took {} +/- {:?}s (averged over {} runs)",
                                 n, mean, std, number_of_averages),
            None => format!("The qft of {} qubits took {}s (averaged over 1 run)",
                            n, mean)
        };
        println!("{}", message)
    };

    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Time scaling of Quantum Fourier Transform", &[])
        .set_x_label("number of qubits", &[])
        .set_y_label("time (s)", &[])
        .set_y_log(Some(2.))
        .lines(
            &n_state_vector,
            &means_state_vector,
            &[Caption("State Vector"), Color("red")])
        .lines(
            &n_density_matrix,
            &means_density_matrix,
            &[Caption("Density Matrix"), Color("blue")]);
    fg.show().expect("TODO: panic message");

}