use crate::{add, div, mul, sub};

struct Adadelta<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> {
    x: [f32; N],
    grad_f: Grad,
    s: [f32; N],
    u: [f32; N],
    gamma_s: f32,
    gamma_u: f32,
    epsilon: f32,
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> Adadelta<N, Grad> {
    fn new(x: [f32; N], grad_f: Grad, gamma_s: f32, gamma_u: f32, epsilon: f32) -> Self {
        Self {
            x,
            grad_f,
            s: [0.; N],
            u: [0.; N],
            gamma_s,
            gamma_u,
            epsilon,
        }
    }
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> Iterator for Adadelta<N, Grad> {
    type Item = [f32; N];

    fn next(&mut self) -> Option<Self::Item> {
        let g = (self.grad_f)(self.x);
        self.s = add(
            self.s.map(|s| self.gamma_s * s),
            g.map(|g| (1. - self.gamma_s) * g.powi(2)),
        );
        let delta = mul(
            div(
                self.u.map(|u| self.epsilon + u.sqrt()),
                self.s.map(|s| self.epsilon + s.sqrt()),
            ),
            g,
        );
        self.u = add(
            self.u.map(|u| self.gamma_u * u),
            delta.map(|d| (1. - self.gamma_u) * d.powi(2)),
        );
        self.x = sub(self.x, delta);
        Some(self.x)
    }
}
