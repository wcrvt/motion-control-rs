use num_traits::Float;
use crate::algebra::*;

use super::continuous;

#[derive(Debug, Copy, Clone)]
pub struct SSR<T, const N: usize> {
    pub a: Matrix::<T, N, N>,
    pub b: Vector<T, N>,
    pub c: Vector<T, N>,
    pub ts: T,
}

impl <T: Float + Default, const N: usize> SSR<T, N> {
    pub fn new(ts: T) -> Self {
        Self {
            a: Matrix::new(),
            b: Vector::new(),
            c: Vector::new(),
            ts
        }
    }

    pub fn from_discrete_ssr(a: &[[T; N]; N], b: &[T; N], c: &[T; N], ts: T) -> Self {
        Self {
            a: Matrix::from(a),
            b: Vector::from(b),
            c: Vector::from(c),
            ts
        }
    }

    pub fn from_continuous_ssr(c_ssr: &continuous::SSR<T, N>, ts: T) -> Self {
        const INT_DIV: usize = 100;
        let mut a_int: Matrix<T, N, N> = Matrix::<T, N, N>::new();
        let mut t: T = T::zero();
        let tp: T = ts / T::from(INT_DIV).unwrap();
        for _ in 0..INT_DIV {
            let integrand: Matrix<T, N, N> = get_matrix_exponential(&(c_ssr.a * t));
            a_int += integrand * tp;
            t = t + tp;
        }

        let a: Matrix<T, N, N> = get_matrix_exponential(&(c_ssr.a * ts));
        let b: Vector<T, N> = a_int * c_ssr.b;
        let c: Vector<T, N> = c_ssr.c;
        Self { a, b, c, ts }
    }

}


pub struct Plant<T, const N: usize> {
    ssr: SSR<T, N>,
    x: Vector<T, N>,
    pub y: T,
}

impl <T: Float + Default, const N: usize> Plant<T, N> {
    pub fn new(ssr: &SSR<T, N>) -> Self {
        Self {
            ssr: ssr.clone(),
            x: Vector::new(),
            y: T::zero(),
        }
    }

    pub fn update(&mut self, u: T) {
        self.x =  &self.ssr.a * &self.x + &self.ssr.b * u; 
        self.y = self.ssr.c.dot(&self.x);
    }
}

fn get_matrix_exponential<T: Float + Default, const N: usize>(m: &Matrix<T, N, N>) -> Matrix<T, N, N> {
    let identity: Matrix<T, N, N> = Matrix::<T, N, N>::diag(T::one());
    let mut a_operand: Matrix<T, N, N> = identity;
    let mut divider: T = T::one();
    let mut ret: Matrix<T, N, N>  = identity;

    for i in 0..1000 {
        a_operand = a_operand * m;
        ret += a_operand / divider;
        divider = divider * T::from(i + 2).unwrap();
    }

    ret
}
