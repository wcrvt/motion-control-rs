use nalgebra::{DVector, DMatrix};
use num_traits::Float;

//Use vector dataset
pub fn identify<T>(x: &Vec<T>, y: &Vec<T>, degree: usize) -> Option<Vec<T>>
    where T: Float + std::fmt::Debug + nalgebra::ComplexField
{
    let x_len: usize = x.len();
    let y_len: usize = y.len();
    let n: usize = std::cmp::min(x_len, y_len);

    let degree: usize = degree + 1;
    let mut psi_sum = DVector::from_element(degree, T::zero());
    let mut phi_sum = DMatrix::from_element(degree, degree, T::zero());

    for i in 0..n {
        let phi: Vec<T> = vec![T::zero(); degree].iter().enumerate().map(|(j, _ )| Float::powi(x[i], j as i32)).collect();
        let phi = DVector::from(phi);
        phi_sum += &phi * &phi.transpose();
        psi_sum += &phi * y[i];
    }

    match phi_sum.clone().try_inverse() {
        Some(res) => {
            let theta = res * &psi_sum;
            let mut theta: Vec<T> = theta.data.as_vec().clone();
            theta.reverse();
            Some(theta)
        },
        None => None
    }
}

