use crate::{identity_mat, sub};

struct BfgsMethod<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> {
    grad_f: Grad,
    x: [f32; N],
    q: [[f32; N]; N],
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> BfgsMethod<N, Grad> {
    fn new(grad_f: Grad, x: [f32; N]) -> Self {
        let q = identity_mat::<N>();
        Self { grad_f, x, q }
    }
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> Iterator for BfgsMethod<N, Grad> {
    type Item = [f32; N];

    fn next(&mut self) -> Option<Self::Item> {
        let g = (self.grad_f)(self.x);
        let x_prime = todo!("line_search(f, x, -Q*g)");
        let g_prime = self.grad_f(x_prime);
        let delta = sub(x_prime, self.x);
        let gamma = sub(g_prime, g);
        self.q = todo!("Q - (δ*γ'*Q + Q*γ*δ')/(δ'*γ) + (1 + (γ'*Q*γ)/(δ'*γ))[1]*(δ*δ')/(δ'*γ)");
        self.x = x_prime;
        Some(self.x)
    }
}
