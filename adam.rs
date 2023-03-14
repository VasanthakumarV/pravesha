use crate::{add, div, sub};

struct Adam<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> {
    x: [f32; N],
    grad_f: Grad,
    alpha: f32,
    gamma_v: f32,
    gamma_s: f32,
    epsilon: f32,
    k: usize,
    v: [f32; N],
    s: [f32; N],
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> Adam<N, Grad> {
    fn new(
        x: [f32; N],
        grad_f: Grad,
        alpha: f32,
        gamma_v: f32,
        gamma_s: f32,
        epsilon: f32,
    ) -> Self {
        Self {
            x,
            grad_f,
            alpha,
            gamma_v,
            gamma_s,
            epsilon,
            k: 0,
            v: [0.; N],
            s: [0.; N],
        }
    }
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> Iterator for Adam<N, Grad> {
    type Item = [f32; N];

    fn next(&mut self) -> Option<Self::Item> {
        let g = (self.grad_f)(self.x);
        self.v = add(
            self.v.map(|v| v * self.gamma_v),
            g.map(|g| g * (1. - self.gamma_v)),
        );
        self.s = add(
            self.s.map(|s| s * self.gamma_s),
            g.map(|g| g.powi(2) * (1. - self.gamma_s)),
        );
        self.k += 1;
        let v_hat = self.v.map(|v| v / (1. - self.gamma_v.powi(self.k as i32)));
        let s_hat = self.s.map(|s| s / (1. - self.gamma_s.powi(self.k as i32)));
        self.x = sub(
            self.x,
            div(
                v_hat.map(|v| v * self.alpha),
                s_hat.map(|s| s.sqrt() + self.epsilon),
            ),
        );
        Some(self.x)
    }
}
