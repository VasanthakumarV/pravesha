use crate::{basis, sub};

fn powells_method<const N: usize, F: Fn([f32; N]) -> f32>(
    f: F,
    mut x: [f32; N],
    epsilon: f32,
) -> [f32; N] {
    let u: Vec<[f32; N]> = (0..N).map(|i| basis::<N>(i)).collect();
    let mut delta = f32::INFINITY;
    while delta > epsilon {
        let mut x_prime = x;
        for i in 0..N {
            x_prime = todo!("line_search(f, x_prime, u[i])");
        }
        for i in 0..(N - 1) {
            u[i] = u[i + 1];
        }
        u[N] = sub(x_prime, x);
        x_prime = todo!("line_search(f, x, u[N])");
        delta = sub(x_prime, x).map(|x| x.powi(2)).iter().sum();
        x = x_prime;
    }
    x
}
