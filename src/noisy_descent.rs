use rand::Rng;
use rand_distr::StandardNormal;

use crate::{add, gradient_descent::GradientDescent};

struct NoisyDescent<const N: usize, G: Fn([f32; N]) -> [f32; N], S: Fn(usize) -> f32> {
    descent: GradientDescent<N, G>,
    sigma: S,
    k: usize,
}

impl<const N: usize, G: Fn([f32; N]) -> [f32; N], S: Fn(usize) -> f32> NoisyDescent<N, G, S> {
    fn new(x: [f32; N], grad_f: G, alpha: f32, sigma: S) -> Self {
        Self {
            descent: GradientDescent::new(x, grad_f, alpha),
            sigma,
            k: 1,
        }
    }
}

impl<const N: usize, G: Fn([f32; N]) -> [f32; N], S: Fn(usize) -> f32> Iterator
    for NoisyDescent<N, G, S>
{
    type Item = [f32; N];

    fn next(&mut self) -> Option<Self::Item> {
        // The iterator will never end, so unwrap is fine
        let mut x = self.descent.next().unwrap();
        let sigma = (self.sigma)(self.k);
        x = add(x, randn::<N>().map(|r| r * sigma));
        self.k += 1;
        // Since x has changed, we update the x stored in GradientDescent
        self.descent.set_x(x);
        Some(x)
    }
}

fn randn<const N: usize>() -> [f32; N] {
    let mut rng = rand::thread_rng();
    [0.; N].map(|_| rng.sample(StandardNormal))
}
