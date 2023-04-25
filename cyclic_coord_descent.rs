use crate::{basis, sub};

fn cyclic_coord_descent<const N: usize, F: Fn([f32; N]) -> f32>(
    mut x: [f32; N],
    f: F,
    epsilon: f32,
) {
    let mut delta = f32::INFINITY;
    while delta.abs() > epsilon {
        let x_init = x;
        for i in 1..N {
            let d = basis::<N>(i);
            x = todo!("line_search(f, x, d)");
        }
        delta = sub(x, x_init).map(|x| x.powi(2)).iter().sum();
    }
    x
}

fn cyclic_coord_descent_w_acc<const N: usize, F: Fn([f32; N]) -> f32>(
    mut x: [f32; N],
    f: F,
    epsilon: f32,
) {
    let mut delta = f32::INFINITY;
    while delta.abs() > epsilon {
        let x_init = x;
        for i in 1..N {
            let d = basis::<N>(i);
            x = todo!("line_search(f, x, d)");
        }
        x = todo!("line_search(f, x, x - x_init)");
        delta = sub(x, x_init).map(|x| x.powi(2)).iter().sum();
    }
    x
}
