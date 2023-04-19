use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Uniform};

use crate::{add, basis};

struct Config<const N: usize> {
    ns: usize,
    ne: usize,
    nt: usize,
    gamma: f32,
    c: [f32; N],
}

impl<const N: usize> Config<N> {
    fn new() -> Self {
        Self {
            ns: 20,
            ne: 4,
            nt: 100.max(5 * N),
            gamma: 0.85,
            c: [2.; N],
        }
    }
}

fn adaptive_simulated_annealing<const N: usize>(
    f: impl Fn([f32; N]) -> f32,
    x: [f32; N],
    v: [f32; N],
    t: f32,
    epsilon: f32,
    config: Config<N>,
) -> [f32; N] {
    let Config {
        ns,
        ne,
        nt,
        gamma,
        c,
    } = config;

    let y = f(x);
    let (mut x_best, mut y_best) = (x, y);
    let y_arr: Vec<f32> = vec![];
    let (mut rng, uniform) = (thread_rng(), Uniform::from(-1f32..1f32));
    let (a, count_cycles, count_resets) = ([0.; N], 0, 0);

    loop {
        for i in 0..N {
            let x_prime = add(
                x,
                basis::<N>(i).map(|b| b * uniform.sample(&mut rng) * v[i]),
            );
            let y_prime = f(x_prime);
            let delta_y = y_prime - y;
            if delta_y < 0. || rng.gen::<f32>() < (-delta_y / t).exp() {
                (x, y) = (x_prime, y_prime);
                a[i] += 1.;
                if y_prime < y_best {
                    (x_best, y_best) = (x_prime, y_prime);
                }
            }
        }

        count_cycles += 1;
        if count_cycles >= ns {
            continue;
        }

        count_cycles = 0;
        corona_update(&mut v, a, c, ns);
        a = [0.; N];
        count_resets += 1;
        if count_resets >= nt {
            continue;
        }

        t *= gamma;
        count_resets = 0;
        y_arr.push(y);

        if !(y_arr.len() > ne
            && y_arr.last().unwrap() - y_best <= epsilon
            && (0..ne)
                .into_iter()
                .all(|u| (y_arr.last().unwrap() - y_arr[y_arr.len() - 2 - u]).abs() <= epsilon))
        {
            (x, y) = (x_best, y_best);
        } else {
            break;
        }
    }
    x_best
}

fn corona_update<const N: usize>(v: &mut [f32; N], a: [f32; N], c: [f32; N], ns: usize) {
    for i in 0..v.len() {
        let (ai, ci) = (a[i], c[i]);
        if ai > 0.6 * ns as f32 {
            v[i] *= 1. + ci * (ai / ns as f32 - 0.6) / 0.4;
        } else if ai < 0.4 * ns as f32 {
            v[i] /= 1. + ci * (0.4 - ai / ns as f32) / 0.4;
        }
    }
}
