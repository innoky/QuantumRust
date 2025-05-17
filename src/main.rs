
mod complex;
mod qubit;
mod gates;

use complex::Complex;
use qubit::{Qubit, TwoQubit};
use gates::{x_gate, y_gate, z_gate};

fn main() {
    let alpha = Complex::new(0.0, 0.0);
    let beta  = Complex::new(1.0, 0.0);
    let ctrl = Qubit::try_new(alpha, beta).expect("Invalid control qubit");

    let target = Qubit::try_new(Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)).unwrap();
    let mut pair = TwoQubit::new(ctrl, target);

    println!("Before CNOT:");
    println!("Control: {:?}", pair.control.measure_probabilities());
    println!("Target: {:?}", pair.target.measure_probabilities());

    pair.apply_cnot();

    println!("After CNOT:");
    println!("Control: {:?}", pair.control.measure_probabilities());
    println!("Target: {:?}", pair.target.measure_probabilities());
}
