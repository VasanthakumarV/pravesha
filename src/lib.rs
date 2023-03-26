#![feature(array_zip)]

mod bisection_method;
mod bracket_minimum;
mod fibonacci_search;
mod forward_accumulation;
mod golden_section_search;
mod quadratic_fit_search;
mod shubert_piyavskii_method;

mod adadelta;
mod adagrad;
mod adam;
mod approx_line_search;
mod conjugate_gradient_descent;
mod gradient_descent;
mod hyper_gradient_descent;
mod hyper_nesterov_momentum;
mod line_search;
mod momentum;
mod nesterov_momentum;
mod rms_prop;
mod trust_region_descent;

mod bfgs_method;
mod dfp_method;
mod newtons_method;
mod secant_method;

mod cyclic_coord_descent;
mod generalized_pattern_search;
mod hooke_jeeves;
mod nelder_mead;
mod powells_method;

fn div<const N: usize>(a: [f32; N], b: [f32; N]) -> [f32; N] {
    a.zip(b).map(|(a, b)| a / b)
}

fn mul<const N: usize>(a: [f32; N], b: [f32; N]) -> [f32; N] {
    a.zip(b).map(|(a, b)| a * b)
}

fn add<const N: usize>(a: [f32; N], b: [f32; N]) -> [f32; N] {
    a.zip(b).map(|(a, b)| a + b)
}

fn sub<const N: usize>(a: [f32; N], b: [f32; N]) -> [f32; N] {
    a.zip(b).map(|(a, b)| a - b)
}

fn dot<const N: usize>(a: [f32; N], b: [f32; N]) -> f32 {
    a.zip(b).map(|(a, b)| a * b).iter().sum()
}

fn identity_mat<const N: usize>() -> [[f32; N]; N] {
    let matrix = [[0.; N]; N];
    for i in 0..N {
        matrix[i][i] = 1.0;
    }
    matrix
}

fn basis<const N: usize>(i: usize) -> [f32; N] {
    let basis = [0.0; N];
    basis[i] = 1.;
    basis
}
