struct ConjugateGradDescent<
    const N: usize,
    Func: Fn([f32; N]) -> f32,
    Grad: Fn([f32; N]) -> [f32; N],
> {
    x: [f32; N],
    f: Func,
    grad_f: Grad,
    d: [f32; N],
    g: [f32; N],
}

impl<const N: usize, Func: Fn([f32; N]) -> f32, Grad: Fn([f32; N]) -> [f32; N]>
    ConjugateGradDescent<N, Func, Grad>
{
    fn new(x: [f32; N], f: Func, grad_f: Grad) -> Self {
        let g = grad_f(x);
        Self {
            x,
            f,
            grad_f,
            g,
            d: g.map(|g| -g),
        }
    }
}

impl<const N: usize, Func: Fn([f32; N]) -> f32, Grad: Fn([f32; N]) -> [f32; N]> Iterator
    for ConjugateGradDescent<N, Func, Grad>
{
    type Item = [f32; N];

    fn next(&mut self) -> Option<Self::Item> {
        let g_prime = (self.grad_f)(self.x);
        let beta = (dot(g_prime, sub(g_prime, self.g)) / dot(self.g, self.g)).max(0.);
        let d_prime = sub(self.d.map(|d| beta * d), g_prime);
        self.x = todo!("line_search(self.f, self.x, d_prime)");
        (self.d, self.g) = (d_prime, g_prime);
        Some(self.x)
    }
}

fn dot<const N: usize>(a: [f32; N], b: [f32; N]) -> f32 {
    a.zip(b).map(|(a, b)| a * b).iter().sum()
}

fn sub<const N: usize>(a: [f32; N], b: [f32; N]) -> [f32; N] {
    a.zip(b).map(|(a, b)| a - b)
}
