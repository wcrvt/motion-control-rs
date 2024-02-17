use crate::algebra::*;
use num_traits::Float;

#[derive(Debug, Copy, Clone)]
pub struct SSR<T, const N: usize> {
    pub a: Matrix<T, N, N>,
    pub b: Vector<T, N>,
    pub c: Vector<T, N>,
}

impl<T: Float + Default, const N: usize> SSR<T, N> {
    pub fn new(a: &[[T; N]; N], b: &[T; N], c: &[T; N]) -> Self {
        Self {
            a: Matrix::from(a),
            b: Vector::from(b),
            c: Vector::from(c),
        }
    }
}

pub struct Plant<T, const N: usize> {
    ssr: SSR<T, N>,
    x: Vector<T, N>,
    pub y: T,
    ts: T,
}

impl<T: Float + Default, const N: usize> Plant<T, N> {
    pub fn new(ssr: &SSR<T, N>, ts: T) -> Self {
        Self {
            ssr: ssr.clone(),
            x: Vector::new(),
            y: T::zero(),
            ts,
        }
    }

    pub fn update(&mut self, u: T) {
        let dx: Vector<T, N> = self.ssr.a * self.x + self.ssr.b * u;
        self.x += dx * self.ts;
        self.y = self.ssr.c.dot(self.x);
    }
}
