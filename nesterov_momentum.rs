struct NesterovMomentum<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> {
    x: [f32; N],
    grad_f: Grad,
    alpha: f32,
    beta: f32,
    v: [f32; N],
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> NesterovMomentum<N, Grad> {
    fn new(x: [f32; N], grad_f: Grad, alpha: f32, beta: f32) -> Self {
        Self {
            x,
            grad_f,
            alpha,
            beta,
            v: [0.; N],
        }
    }
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> Iterator for NesterovMomentum<N, Grad> {
    type Item = [f32; N];

    fn next(&mut self) -> Option<Self::Item> {
        let next_pt = add(self.x, self.v.map(|v| self.beta * v));
        self.v = add(
            self.v.map(|v| self.beta * v),
            next_pt.map(|n| self.alpha * n),
        );
        self.x = add(self.x, self.v);
        Some(self.x)
    }
}

fn add<const N: usize>(a: [f32; N], b: [f32; N]) -> [f32; N] {
    a.zip(b).map(|(a, b)| a + b)
}
