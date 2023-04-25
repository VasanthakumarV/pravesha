use rand::{seq::SliceRandom, thread_rng};
use rand_distr::uniform::SampleRange;

use crate::add;

fn mesh_adaptive_search<const N: usize>(
    f: impl Fn([f32; N]) -> f32,
    mut x: [f32; N],
    epsilon: f32,
) -> [f32; N]
where
    [(); N + 1]:,
{
    let mut alpha = 1.;
    let mut y = f(x);

    while alpha > epsilon {
        let mut improved = false;
        for (i, d) in rand_positive_spanning_set::<N>(alpha)
            .into_iter()
            .enumerate()
        {
            let mut x_prime = add(x, d.map(|d| d * alpha));
            let mut y_prime = f(x_prime);
            if y_prime < y {
                (x, y, improved) = (x_prime, y_prime, true);
                x_prime = add(x, d.map(|d| 3. * alpha * d));
                y_prime = f(x_prime);
                if y_prime < y {
                    (x, y) = (x_prime, y_prime);
                }
                break;
            }
        }
        alpha = if improved {
            1f32.min(4. * alpha)
        } else {
            alpha / 4.
        };
    }

    x
}

fn rand_positive_spanning_set<const N: usize>(alpha: f32) -> [[f32; N]; N + 1] {
    let mut rng = thread_rng();

    let delta = (1. / alpha.sqrt()).round() as i32;

    let mut upper = [[0.; N + 1]; N];
    for i in 0..N {
        for j in i..N {
            upper[i][j] = if i == j {
                delta * [1, -1].choose(&mut rng).unwrap()
            } else {
                (-delta + 1..delta - 1).sample_single(&mut rng)
            } as f32;
            upper[i][N] -= upper[i][j];
        }
    }

    let mut out = [[0.; N]; N + 1];
    let mut row_perm = (0..N).collect::<Vec<_>>();
    row_perm.shuffle(&mut rng);
    let mut col_perm = (0..N).collect::<Vec<_>>();
    col_perm.shuffle(&mut rng);
    for (i, col) in col_perm.into_iter().enumerate() {
        for (j, row) in row_perm.into_iter().enumerate() {
            out[i][j] = upper[row][col];
        }
    }

    out
}
