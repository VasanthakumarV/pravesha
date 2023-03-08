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
