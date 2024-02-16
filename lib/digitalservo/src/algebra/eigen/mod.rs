use super::*;

pub mod lu_decomposition;
pub mod qr_decomposition;
pub mod jacobi_eigenvalue;

#[derive(Debug, Clone, Copy)]
pub struct Eigen<T, const N: usize> {
    pub value: [T; N],
    pub vector: [[T; N]; N]
}