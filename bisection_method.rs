struct MethodConfig {
    interval: Interval,
    epsilon: f32,
}

struct Interval {
    left: f32,
    right: f32,
}

impl From<(f32, f32)> for Interval {
    fn from((left, right): (f32, f32)) -> Self {
        Self { left, right }
    }
}

fn bisection_method(f_prime: impl Fn(f32) -> f32, config: MethodConfig) -> Interval {
    let MethodConfig {
        interval: Interval {
            left: mut a,
            right: mut b,
        },
        epsilon,
    } = config;

    if a > b {
        (a, b) = (b, a);
    }

    let (ya, yb) = (f_prime(a), f_prime(b));
    if ya == 0. {
        b = a;
    }
    if yb == 0. {
        a = b;
    }

    while b - a > epsilon {
        let x = (a + b) / 2.;
        let y = f_prime(x);
        if y == 0. {
            (a, b) = (x, x);
        } else if y.signum() == ya.signum() {
            a = x;
        } else {
            b = x;
        }
    }

    (a, b).into()
}

fn bracket_sign_change(
    f_prime: impl Fn(f32) -> f32,
    interval: Interval,
    growth_fctr: f32,
) -> Interval {
    let Interval {
        left: mut a,
        right: mut b,
    } = interval;

    if a > b {
        (a, b) = (b, a);
    }

    let (center, mut half_width) = ((b + a) / 2., (b - a) / 2.);
    while f_prime(a) * f_prime(b) > 0. {
        half_width *= growth_fctr;
        a = center - half_width;
        b = center + half_width;
    }

    (a, b).into()
}
