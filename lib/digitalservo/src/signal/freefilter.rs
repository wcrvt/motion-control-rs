use crate::algebra::*;
use num_traits::Float;

#[derive(Debug)]
pub struct FreeFilter<T, const P: usize, const Z: usize>
where
    [(); P - 1]:,
{
    a: Matrix<T, { P - 1 }, { P - 1 }>,
    b: Vector<T, { P - 1 }>,
    c: Vector<T, { P - 1 }>,
    x: Vector<T, { P - 1 }>,
    pz1: Vector<T, { P - 1 }>,
    ts_h: T,
}

impl<T: Float + Default + std::fmt::Debug, const P: usize, const Z: usize> FreeFilter<T, P, Z>
where
    [(); P - 1]:,
{
    pub fn new(numer: &[T; Z], denom: &[T; P], ts: T) -> Self {
        if Z > P - 1 {
            panic!("filter setting error: improper system.")
        }
        if denom[0] == T::zero() {
            panic!("filter setting error: invalid characteristic polynomial.")
        }

        let mut a: Matrix<T, { P - 1 }, { P - 1 }> = Matrix::new();
        let mut b: Vector<T, { P - 1 }> = Vector::new();
        let mut c: Vector<T, { P - 1 }> = Vector::new();
        let pz1: Vector<T, { P - 1 }> = Vector::new();
        let x: Vector<T, { P - 1 }> = Vector::new();
        let ts_h = ts * T::from(0.5).unwrap();

        let numer: Vector<T, Z> = Vector::from(numer) / denom[0];
        let denom: Vector<T, P> = Vector::from(denom) / denom[0];

        for i in 0..(P - 2) {
            a[i][i + 1] = T::one();
        }

        for i in 0..(P - 1) {
            a[P - 2][(P - 2) - i] = -denom[i + 1];
            c[(P - 2) - i] = if i < Z { numer[i] } else { T::zero() };
        }

        b[P - 2] = T::one();

        Self {
            a,
            b,
            c,
            pz1,
            x,
            ts_h,
        }
    }

    pub fn update(&mut self, u: T) -> T {
        let p: Vector<T, { P - 1 }> = &self.a * &self.x + &self.b * u;
        self.x += (&p + &self.pz1) * self.ts_h;
        let y: T = self.c.dot(&self.x);
        self.pz1 = p;
        y
    }
}
