use num_traits::Float;
use crate::algebra::*;

use super::{continuous, discrete};


#[derive(Debug, Copy, Clone)]
pub struct BlockedSSR<T, const N: usize> {
    pub a: Matrix::<T, N, N>,
    pub b: Matrix::<T, N, N>,
    pub c: Vector<T, N>,
    pub ts: T,
}

impl <T: Float + Default, const N: usize> BlockedSSR<T, N> {

    pub fn from_discrete_ssr(m: &discrete::SSR<T, N>) -> BlockedSSR<T, N> {
        let mut a: Matrix<T, N, N> = Matrix::<T, N, N>::diag(T::one());
        let mut b: [[T; N]; N] = [[T::zero(); N]; N];

        for i in 0..N {
            b[N - 1 - i] = (&a * &m.b).data;
            a = &a * &m.a;
        }

        let b: Matrix<T, N, N> = Matrix::from(b).transpose();
        let c: Vector<T, N> = m.c;

        BlockedSSR { a, b, c, ts: m.ts }
    }

    pub fn from_continuous_ssr(m: &continuous::SSR<T, N>, ts: T) -> BlockedSSR<T, N> {
        let discrete_ssr = discrete::SSR::<T, N>::from_continuous_ssr(m, ts);
        Self::from_discrete_ssr(&discrete_ssr)
    }

    pub fn calculate_input(&mut self, current: &[T; N], next: &[T; N]) -> [T; N] {
        let next: Vector<T, N> = Vector::from(next);
        let current: Vector<T, N> = Vector::from(current);
        let u: Vector<T, N> = &self.b.inverse().unwrap() * &(&next - &(&self.a * &current));
        u.data
    }
}