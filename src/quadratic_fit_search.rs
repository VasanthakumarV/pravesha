struct SearchConfig {
    bracket: Bracket,
    n_queries: usize,
}

struct Bracket {
    start: f32,
    middle: f32,
    end: f32,
}

impl From<(f32, f32, f32)> for Bracket {
    fn from((start, middle, end): (f32, f32, f32)) -> Self {
        Self { start, middle, end }
    }
}

fn quadratic_fit_search<F: Fn(f32) -> f32>(f: F, config: SearchConfig) -> Bracket {
    let SearchConfig {
        bracket:
            Bracket {
                start: mut a,
                middle: mut b,
                end: mut c,
            },
        n_queries,
    } = config;

    let (mut ya, mut yb, mut yc) = (f(a), f(b), f(c));

    for _ in 1..n_queries - 3 {
        let x = 0.5
            * (ya * (b.powi(2) - c.powi(2))
                + yb * (c.powi(2) - a.powi(2))
                + yc * (a.powi(2) - b.powi(2)))
            / (ya * (b - c) + yb * (c - a) + yc * (a - b));
        let yx = f(x);

        if x > b {
            if yx > yb {
                (c, yc) = (x, yx);
            } else {
                (a, ya, b, yb) = (b, yb, x, yx);
            }
        } else if x < b {
            if yx > yb {
                (a, ya) = (x, yx);
            } else {
                (c, yc, b, yb) = (b, yb, x, yx);
            }
        }
    }

    (a, b, c).into()
}
