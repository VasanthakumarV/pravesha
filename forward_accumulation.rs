use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug)]
struct Dual {
    value: f64,
    derivative: f64,
}

impl Dual {
    fn new(value: f64, derivative: f64) -> Self {
        Dual { value, derivative }
    }
}

impl Add for Dual {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
            derivative: self.derivative + other.derivative,
        }
    }
}

impl Mul for Dual {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            value: self.value * other.value,
            derivative: self.value * other.derivative + other.value * self.derivative,
        }
    }
}

fn ln(dual: Dual) -> Dual {
    Dual {
        value: dual.value.ln(),
        derivative: dual.derivative / dual.value,
    }
}

fn max(dual: Dual, value: f64) -> Dual {
    let derivative = if dual.value > value {
        dual.derivative
    } else if dual.value < value {
        0.
    } else {
        f64::NAN
    };

    Dual {
        value: dual.value.max(value),
        derivative,
    }
}
