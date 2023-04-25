use crate::sub;

fn newtons_method<const N: usize>(
    grad_f: impl Fn([f32; N]) -> [f32; N],
    hess: impl Fn([f32; N]) -> [[f32; N]; N],
    x: [f32; N],
    epsilon: f32,
    k_max: usize,
) -> [f32; N] {
    let (k, mut delta) = (1, [f32::INFINITY; N]);

    while norm(delta) > epsilon && k <= k_max {
        delta = solve(hess(x), grad_f(x));
        x = sub(x, delta);
        k += 1;
    }

    x
}

fn solve<const N: usize>(x_1: [[f32; N]; N], x_2: [f32; N]) -> [f32; N] {
    todo!("lu factorization")
}

fn norm<const N: usize>(delta: [f32; N]) -> f32 {
    delta.map(|d| d.powi(2)).iter().sum::<f32>().sqrt()
}
