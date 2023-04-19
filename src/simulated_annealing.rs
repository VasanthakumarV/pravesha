use rand::{thread_rng, Rng};

use crate::add;

fn simulated_annealing<const N: usize>(
    f: impl Fn([f32; N]) -> f32,
    mut x: [f32; N],
    dist: impl Fn() -> [f32; N],
    t: impl Fn(usize) -> f32,
    max_iter: usize,
) -> [f32; N] {
    let mut rng = thread_rng();
    let mut y = f(x);
    let (mut x_best, mut y_best) = (x, y);
    for k in 0..max_iter {
        let x_prime = add(x, dist());
        let y_prime = f(x_prime);
        let delta_y = y_prime - y;
        if delta_y <= 0. || rng.gen::<f32>() < (-delta_y / t(k)).exp() {
            (x, y) = (x_prime, y_prime);
        }
        if y_prime < y_best {
            (x_best, y_best) = (x_prime, y_prime);
        }
    }
    x_best
}
