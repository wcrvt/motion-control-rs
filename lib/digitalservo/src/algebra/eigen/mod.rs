use super::*;

pub mod jacobi_eigenvalue;
pub mod lu_decomposition;
pub mod qr_decomposition;

#[derive(Debug, Clone, Copy)]
pub struct Eigen<T, const N: usize> {
    pub value: [T; N],
    pub vector: [[T; N]; N],
}
