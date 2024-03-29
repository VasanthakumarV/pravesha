pub struct GradientDescent<const N: usize, F: Fn([f32; N]) -> [f32; N]> {
    x: [f32; N],
    grad_f: F,
    alpha: f32,
}

impl<const N: usize, F: Fn([f32; N]) -> [f32; N]> GradientDescent<N, F> {
    pub fn new(x: [f32; N], grad_f: F, alpha: f32) -> Self {
        Self { x, grad_f, alpha }
    }

    // Helpter method for use in NoisyDescent
    pub(crate) fn set_x(&mut self, x: [f32; N]) {
        self.x = x;
    }
}

impl<const N: usize, F: Fn([f32; N]) -> [f32; N]> Iterator for GradientDescent<N, F> {
    type Item = [f32; N];

    fn next(&mut self) -> Option<Self::Item> {
        let g = (self.grad_f)(self.x);
        self.x = self.x.zip(g).map(|(x, g)| x - self.alpha * g);
        Some(self.x)
    }
}
