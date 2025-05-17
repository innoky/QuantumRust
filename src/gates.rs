use crate::complex::Complex;

pub type Gate = [[Complex; 2]; 2];

pub fn identity() -> Gate {
    [
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]
}

pub fn x_gate() -> Gate {
    [
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    ]
}

pub fn z_gate() -> Gate {
    [
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0)],
    ]
}

pub fn h_gate() -> Gate {
    let s = 1.0 / 2.0f64.sqrt();
    [
        [Complex::new(s, 0.0), Complex::new(s, 0.0)],
        [Complex::new(s, 0.0), Complex::new(-s, 0.0)],
    ]
}


pub fn y_gate() -> Gate {
    [
        [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
        [Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)],
    ]
}
