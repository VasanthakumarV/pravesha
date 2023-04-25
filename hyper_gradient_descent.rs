use crate::{dot, sub};

struct HyperGradientDescent<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> {
    x: [f32; N],
    grad_f: Grad,
    alpha: f32,
    mu: f32,
    g_prev: [f32; N],
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> HyperGradientDescent<N, Grad> {
    fn new(x: [f32; N], grad_f: Grad, alpha: f32, mu: f32) -> Self {
        Self {
            x,
            grad_f,
            alpha,
            mu,
            g_prev: [0.; N],
        }
    }
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> Iterator for HyperGradientDescent<N, Grad> {
    type Item = [f32; N];

    fn next(&mut self) -> Option<Self::Item> {
        let g = (self.grad_f)(self.x);
        self.alpha += self.mu * dot(g, self.g_prev);
        self.g_prev = g;
        self.x = sub(self.x, g.map(|g| g * self.alpha));
        Some(self.x)
    }
}
