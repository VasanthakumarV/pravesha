struct Config {
    alpha: f32,
    beta: f32,
    shrink_fctr: f32,
}

fn bracktracking_line_search<const N: usize>(
    f: impl Fn([f32; N]) -> f32,
    grad_f: impl Fn([f32; N]) -> [f32; N],
    x: [f32; N],
    dir: [f32; N],
    config: Config,
) -> f32 {
    let Config {
        mut alpha,
        beta,
        shrink_fctr: p,
    } = config;

    let (y, g) = (f(x), grad_f(x));
    loop {
        if f(sum(x, dir.map(|d| alpha * d))) <= y + (beta * alpha * dot(g, dir)) {
            break;
        }
        alpha *= p;
    }

    alpha
}

struct StrongConfig {
    alpha: f32,
    beta: f32,
    sigma: f32,
}

fn strong_bracktracking<const N: usize>(
    f: impl Fn([f32; N]) -> f32,
    grad_f: impl Fn([f32; N]) -> [f32; N],
    x: [f32; N],
    d: [f32; N],
    config: StrongConfig,
) -> f32 {
    let StrongConfig {
        mut alpha,
        beta,
        sigma,
    } = config;

    let (y0, g0, mut y_prev, mut alpha_prev) = (f(x), dot(grad_f(x), d), None, 0.);

    let (mut alpha_lo, mut alpha_hi) = loop {
        let y = f(sum(x, d.map(|d| alpha * d)));
        if y > y0 + beta * alpha * g0 || (y_prev.is_some() && y >= y_prev.unwrap()) {
            break (alpha_prev, alpha);
        }
        let g = dot(grad_f(sum(x, d.map(|d| alpha * d))), d);
        if g.abs() <= -sigma * g0 {
            return alpha;
        } else if g >= 0. {
            break (alpha, alpha_prev);
        }
        (y_prev, alpha_prev, alpha) = (Some(y), alpha, 2. * alpha);
    };

    let y_lo = f(sum(x, d.map(|d| alpha_lo * d)));
    loop {
        let alpha = (alpha_lo + alpha_hi) / 2.;
        let y = f(sum(x, d.map(|d| alpha * d)));
        if y > y0 + beta * alpha * g0 || y >= y_lo {
            alpha_hi = alpha;
        } else {
            let g = dot(grad_f(sum(x, d.map(|d| alpha * d))), d);
            if g.abs() <= -sigma * g0 {
                return alpha;
            } else if g * (alpha_hi - alpha_lo) >= 0. {
                alpha_hi = alpha_lo;
            }
            alpha_lo = alpha;
        }
    }
}

fn dot<const N: usize>(a: [f32; N], b: [f32; N]) -> f32 {
    a.zip(b).map(|(a, b)| a * b).iter().sum()
}

fn sum<const N: usize>(a: [f32; N], b: [f32; N]) -> [f32; N] {
    a.zip(b).map(|(a, b)| a + b)
}
