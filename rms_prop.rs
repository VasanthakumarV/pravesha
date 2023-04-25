use crate::{add, div, sub};

struct RmsProp<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> {
    x: [f32; N],
    grad_f: Grad,
    s: [f32; N],
    alpha: f32,
    gamma: f32,
    epsilon: f32,
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> RmsProp<N, Grad> {
    fn new(x: [f32; N], grad_f: Grad, alpha: f32, gamma: f32, epsilon: f32) -> Self {
        Self {
            x,
            grad_f,
            s: [0.; N],
            alpha,
            gamma,
            epsilon,
        }
    }
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> Iterator for RmsProp<N, Grad> {
    type Item = [f32; N];

    fn next(&mut self) -> Option<Self::Item> {
        let g = (self.grad_f)(self.x);
        self.s = add(
            self.s.map(|s| self.gamma * s),
            g.map(|g| (1. - self.gamma) * g.powi(2)),
        );
        let delta = div(
            g.map(|g| self.alpha * g),
            self.s.map(|s| self.epsilon + s.sqrt()),
        );
        self.x = sub(self.x, delta);
        Some(self.x)
    }
}
