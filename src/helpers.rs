use crate::{Program, Qubit};
use crate::{gates::Gate};

pub fn draw_circuit(
    program: &Program
) {
    let mut v: Vec<String> = vec!["Qubit ".to_owned(); program.state.number_of_qubits];
    for n in 0..program.state.number_of_qubits {
        v[n].push_str(format!("{} : ---", n).as_str())
    }

    for gate in &program.gates {
        for n in 0..program.state.number_of_qubits{
            let m: String = plot_gate(gate, n) + "---";
            v[n].push_str(&m);
        }
    }

    for n in 0..program.state.number_of_qubits {
        v[n].push_str("||")
    }

    println!("_____________________________");
    println!("****** Quantum Circuit ******");
    println!("{}", aadvark.to_owned());
    println!("_____________________________");
    println!("*****************************");
    println!("");
    for line in v {
        println!("{:#?}", line);
    }
    println!("");
    println!("*****************************");
    println!("_____________________________");

}
//
fn plot_gate( gate: &Gate, qubit_index: Qubit) -> String {
    match gate {
        Gate::Measure(qubit) => {
            let m: String = format!("{:-<7}","M");
            return_string(qubit_index, *qubit, m)
            },

        Gate::X(qubit) => {
            let m: String = format!("{:-<7}","X");
            return_string(qubit_index, *qubit, m)
        },
        Gate::Y(qubit) => {
            let m: String = format!("{:-<7}","Y");
            return_string(qubit_index, *qubit, m)
        },
        Gate::Z(qubit) => {
            let m: String = format!("{:-<7}","Z");
            return_string(qubit_index, *qubit, m)
        },

        Gate::RX(qubit, angle) => {
            let m: String = format!("RX({:.*})", 1, angle);
            return_string(qubit_index, *qubit, m)
        },
        Gate::RY(qubit, angle) => {
            let m: String = format!("RY({:.*})", 1, angle);
            return_string(qubit_index, *qubit, m)
        },
        Gate::RZ(qubit, angle) => {
            let m: String = format!("RZ({:.*})", 1, angle);
            return_string(qubit_index, *qubit, m)
        },
        Gate::H(qubit) => {
            let m: String = format!("{:-<7}","H");
            return_string(qubit_index, *qubit, m)
        },
        Gate::R(qubit, omega, theta, phi) => {
            let m: String = format!("R.{:.*},{:.*},{:.*}", 0, omega, 0, theta, 0, phi);
            return_string(qubit_index, *qubit, m)
        },
        Gate::ArbitarySingle(qubit, u) => {
            let m: String = format!("{:-<7}","ArbU");
            return_string(qubit_index, *qubit, m)
        },
        Gate::CNOT(control, target) => {
            let m: String = "CNOT".to_owned();
            return_two_gate_string(qubit_index, *control, *target, m)
        },
        Gate::SISWAP(control, target) => {
            let m: String = "SSwap".to_owned();
            return_two_gate_string(qubit_index, *control, *target, m)
        },
        Gate::ArbitaryTwo(control, target, u) => {
            let m: String = "ArbU".to_owned();
            return_two_gate_string(qubit_index, *control, *target, m)
        },

        _ => {
            let default: String = "Other__".to_owned();
            default
        }
    }
}

fn return_string(indexed_qubit: Qubit, gate_qubit: Qubit, message: String) -> String {
    if indexed_qubit == gate_qubit {
        return message
    }
    let default: String = format!("{:-<7}","-");
    return  default
}

fn return_two_gate_string(indexed_qubit: Qubit, control: Qubit, target: Qubit, message: String) -> String {
    if indexed_qubit == control {
        let m: String = format!("{:->5}_C", message);
        return m
        } else if  indexed_qubit == target {
        let m: String = format!("{:->5}_T", message);
        return m
        }
    let default: String = format!("{:-<7}","-");
    return  default
}

pub const aadvark: &str = r#"
         (`.  : /               __..----..__
          `.`.| |:          _,-':::''' '  `:`-._
            `.:/||       _,':::::'         `::::`-.
              \\`|    _,':::::::'     `:.     `':::`.
               ;` `-''  `::::::.                  `::\
            ,-'      .::'  `:::::.         `::..    `:\
          ,' /_) -.            `::.           `:.     |
        ,'.:     `    `:.        `:.     .::.          \
   __,-'   ___,..-''-.  `:.        `.   /::::.         |
  |):'_,--'           `.    `::..       |::::::.      ::\
   `-'                 |`--.:_::::|_____|::::::::.__  ::|
                       |   _/|::::|      |::::::|::/|  :|
                       /:./  |:::/        |__:::):/  |  :\
                     ,'::'  /:::|        ,'::::/_/    `. ``-.
                    ''''   (//|/|      ,';':,-'         `-._  `'--..__
                                                             `''---::::"#;