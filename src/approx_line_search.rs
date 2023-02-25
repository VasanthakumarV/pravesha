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
        let suff_decr = y + (beta * alpha * g.zip(dir).map(|(g, d)| g * d).iter().sum::<f32>());
        if f(x.zip(dir).map(|(x, d)| x + alpha * d)) <= suff_decr {
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

    let (y0, g0, mut y_prev, mut alpha_prev) = (
        f(x),
        grad_f(x).zip(d).map(|(g, d)| g * d).iter().sum::<f32>(),
        None,
        0.,
    );

    let (mut alpha_lo, mut alpha_hi) = loop {
        let y = f(x.zip(d).map(|(x, d)| x + alpha * d));
        if y > y0 + beta * alpha * g0 || (y_prev.is_some() && y >= y_prev.unwrap()) {
            break (alpha_prev, alpha);
        }
        let g: f32 = grad_f(x.zip(d).map(|(x, d)| x + alpha * d))
            .zip(d)
            .map(|(g, d)| g * d)
            .iter()
            .sum();
        if g.abs() <= -sigma * g0 {
            return alpha;
        } else if g >= 0. {
            break (alpha, alpha_prev);
        }
        (y_prev, alpha_prev, alpha) = (Some(y), alpha, 2. * alpha);
    };

    let y_lo = f(x.zip(d).map(|(x, d)| x + alpha_lo * d));
    loop {
        let alpha = (alpha_lo + alpha_hi) / 2.;
        let y = f(x.zip(d).map(|(x, d)| x + alpha * d));
        if y > y0 + beta * alpha * g0 || y >= y_lo {
            alpha_hi = alpha;
        } else {
            let g: f32 = grad_f(x.zip(d).map(|(x, d)| x + alpha * d))
                .zip(d)
                .map(|(g, d)| g * d)
                .iter()
                .sum();
            if g.abs() <= -sigma * g0 {
                return alpha;
            } else if g * (alpha_hi - alpha_lo) >= 0. {
                alpha_hi = alpha_lo;
            }
            alpha_lo = alpha;
        }
    }
}
