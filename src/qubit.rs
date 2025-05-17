
use crate::complex::Complex;

#[derive(Copy, Clone, Debug)]
pub struct Qubit {
    pub alpha: Complex,
    pub beta: Complex,
}

impl Qubit {
    pub fn try_new(alpha: Complex, beta: Complex) -> Result<Self, String> {
        let norm = alpha.abs_squared() + beta.abs_squared();

        if (norm - 1.0).abs() > 1e-10 {
            return Err("Qubit not normalized".to_string());
        }

        Ok(Self { alpha, beta })
    }

    pub fn measure_probabilities(&self) -> (f64, f64) {
        (self.alpha.abs_squared(), self.beta.abs_squared())
    }

    pub fn apply(&mut self, gate: [[Complex; 2]; 2]) {
        let a = self.alpha;
        let b = self.beta;

        self.alpha = gate[0][0] * a + gate[0][1] * b;
        self.beta  = gate[1][0] * a + gate[1][1] * b;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TwoQubit {
    pub control: Qubit,
    pub target: Qubit,
}

impl TwoQubit {
    pub fn new(control: Qubit, target: Qubit) -> Self {
        Self { control, target }
    }

    pub fn apply_cnot(&mut self) {
        let (_, p1) = self.control.measure_probabilities();
        if p1 > 0.5 {
            let x = crate::gates::x_gate();
            self.target.apply(x);
        }
    }
}
