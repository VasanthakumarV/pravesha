struct Adagrad<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> {
    x: [f32; N],
    grad_f: Grad,
    alpha: f32,
    epsilon: f32,
    s: [f32; N],
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> Adagrad<N, Grad> {
    fn new(x: [f32; N], grad_f: Grad, alpha: f32, epsilon: f32) -> Self {
        Self {
            x,
            grad_f,
            alpha,
            epsilon,
            s: [0.; N],
        }
    }
}

impl<const N: usize, Grad: Fn([f32; N]) -> [f32; N]> Iterator for Adagrad<N, Grad> {
    type Item = [f32; N];

    fn next(&mut self) -> Option<Self::Item> {
        let g = (self.grad_f)(self.x);
        self.s = add(self.s, g.map(|g| g.powi(2)));
        let adap_grad = div(
            g.map(|g| self.alpha * g),
            self.s.map(|s| s.sqrt() + self.epsilon),
        );
        self.x = add(self.x, adap_grad.map(|a| -a));
        Some(self.x)
    }
}

fn div<const N: usize>(a: [f32; N], b: [f32; N]) -> [f32; N] {
    a.zip(b).map(|(a, b)| a / b)
}

fn add<const N: usize>(a: [f32; N], b: [f32; N]) -> [f32; N] {
    a.zip(b).map(|(a, b)| a + b)
}
