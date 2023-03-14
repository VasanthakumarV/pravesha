use crate::{add, dot, sub};

struct HyperNesterovMomentum<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> {
    x: [f32; N],
    grad_f: Grad,
    alpha: f32,
    mu: f32,
    beta: f32,
    v: [f32; N],
    g_prev: [f32; N],
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> HyperNesterovMomentum<N, Grad> {
    fn new(x: [f32; N], grad_f: Grad, alpha: f32, mu: f32, beta: f32) -> Self {
        Self {
            x,
            grad_f,
            alpha,
            mu,
            beta,
            v: [0.; N],
            g_prev: [0.; N],
        }
    }
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> Iterator for HyperNesterovMomentum<N, Grad> {
    type Item = [f32; N];

    fn next(&mut self) -> Option<Self::Item> {
        let g = (self.grad_f)(self.x);
        self.alpha -= self.mu
            * dot(
                g,
                sub(self.g_prev.map(|g| -g), self.v.map(|v| self.beta * v)),
            );
        self.v = add(self.v.map(|v| self.beta * v), g);
        self.g_prev = g;
        self.x = sub(
            self.x,
            add(g, self.v.map(|v| self.beta * v)).map(|d| d * self.alpha),
        );
        Some(self.x)
    }
}
