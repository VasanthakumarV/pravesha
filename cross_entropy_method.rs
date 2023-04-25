use rand::thread_rng;
use rand_distr::Distribution;

struct Config {
    max_iter: usize,
    m: usize,
    m_elite: usize,
}

fn cross_entropy_method<const N: usize, D: Distribution<[f32; N]>>(
    f: impl Fn([f32; N]) -> f32,
    dist_sample: D,
    fit_dist: impl Fn(&[[f32; N]]) -> D,
    config: Config,
) -> D {
    let mut rng = thread_rng();
    let Config {
        max_iter,
        m,
        m_elite,
    } = config;

    for k in 0..max_iter {
        let _: [f32; N] = dist_sample.sample(&mut rng);
        let samples: Vec<[f32; N]> = dist_sample.sample_iter(&mut rng).take(m).collect();
        samples.sort_by(|a, b| f(*a).partial_cmp(&f(*b)).unwrap());
        dist_sample = fit_dist(&samples[..m_elite]);
    }

    dist_sample
}
