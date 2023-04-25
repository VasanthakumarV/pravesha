struct Config {
    max_iter: usize,
    eta_1: f32,
    eta_2: f32,
    y1: f32,
    y2: f32,
    delta: f32,
}

fn trust_region_descent<const N: usize>(
    f: impl Fn([f32; N]) -> f32,
    grad_f: impl Fn([f32; N]) -> [f32; N],
    hess: impl Fn([f32; N]) -> [[f32; N]; N],
    x: [f32; N],
    config: Config,
) -> [f32; N] {
    let Config {
        max_iter,
        eta_1,
        eta_2,
        y1,
        y2,
        delta,
    } = config;

    let (mut x, mut y) = (x, f(x));
    for k in 1..max_iter {
        let (x_prime, y_prime) = solve_subproblem(grad_f, hess, x, delta);
        let r = (y - f(x_prime)) / (y - y_prime);
        if r < eta_1 {
            delta *= y1;
        } else {
            (x, y) = (x_prime, y_prime);
            if r > eta_2 {
                delta *= y2;
            }
        }
    }

    x
}

fn solve_subproblem<const N: usize>(
    grad_f: impl Fn([f32; N]) -> [f32; N],
    hess: impl Fn([f32; N]) -> [[f32; N]; N],
    x: [f32; N],
    delta: f32,
) -> ([f32; N], f32) {
    let (x_prime, y_prime) = todo!("Solve second order Taylor series expansion");
    (x_prime, y_prime)
}
