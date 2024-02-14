use crate::algebra::*;
use num_traits::Float;

#[derive(Debug, Clone, Copy)]
pub struct Differentiator<T, const DIFF_ORDER: usize, const FILT_ORDER: usize>
where
    [(); DIFF_ORDER + FILT_ORDER]:,
{
    ts: T,
    g: Vector<T, { DIFF_ORDER + FILT_ORDER }>,
    ty: Vector<T, { DIFF_ORDER + FILT_ORDER }>,
    tz: Matrix<T, { DIFF_ORDER + FILT_ORDER }, { DIFF_ORDER + FILT_ORDER }>,
    py: Vector<T, { DIFF_ORDER + FILT_ORDER }>,
    py_z1: Vector<T, { DIFF_ORDER + FILT_ORDER }>,
}

impl<T: Float + Default, const DIFF_ORDER: usize, const FILT_ORDER: usize>
    Differentiator<T, DIFF_ORDER, FILT_ORDER>
where
    [(); DIFF_ORDER + FILT_ORDER]:,
    [(); DIFF_ORDER + FILT_ORDER + 1]:,
{
    pub fn new(ts: T, bandwidth: T) -> Self {
        /* gain vector which realizes multiple root */
        let pascal_coeff: [T; DIFF_ORDER + FILT_ORDER + 1] =
            pascal_triangle::<T, { DIFF_ORDER + FILT_ORDER + 1 }>();
        let mut g: Vector<T, { DIFF_ORDER + FILT_ORDER }> = Vector::new();
        for i in 0..(DIFF_ORDER + FILT_ORDER) {
            g[i] = pascal_coeff[i + 1] * bandwidth.powi(i as i32 + 1);
        }

        /* minimum-order state observer */
        /* see https://digitalservo.jp/library/linear-control-design/observer-design/minimal-order-observer/ */

        //system matrix
        let a_11: Matrix<T, { DIFF_ORDER + FILT_ORDER }, { DIFF_ORDER + FILT_ORDER }> = jordan_block(T::zero());
        let a_12: Vector<T, { DIFF_ORDER + FILT_ORDER }> = Vector::new();
        let mut a_21: Vector<T, { DIFF_ORDER + FILT_ORDER }> = Vector::new();
        a_21[0] = T::one();
        let a_22: T = T::zero();

        //matrices for state observer
        let ty: Vector<T, { DIFF_ORDER + FILT_ORDER }> = a_12 - g * a_22;
        let tz: Matrix<T, { DIFF_ORDER + FILT_ORDER }, { DIFF_ORDER + FILT_ORDER }> = a_11 - g.outer(a_21);

        //initialize
        let py: Vector<T, { DIFF_ORDER + FILT_ORDER }> = Vector::new();
        let py_z1: Vector<T, { DIFF_ORDER + FILT_ORDER }> = Vector::new();

        Self {
            ts,
            g,
            ty,
            tz,
            py,
            py_z1,
        }
    }

    pub fn update(&mut self, x: T) -> T {
        let u: Vector<T, { DIFF_ORDER + FILT_ORDER }> = (self.tz * self.g + self.ty) * x;
        self.py += (u + self.tz * self.py) * self.ts;
        let out: Vector<T, { DIFF_ORDER + FILT_ORDER }> = self.py_z1 + (self.g * x);
        self.py_z1 = self.py;
        out[DIFF_ORDER - 1]
    }
}

fn jordan_block<T: Float + Default, const ORDER: usize>(lambda: T) -> Matrix<T, ORDER, ORDER> {
    let mut ret: Matrix<T, ORDER, ORDER> = Matrix::<T, ORDER, ORDER>::new();
    for i in 0..ORDER - 1 {
        ret[i][i + 1] = T::one();
    }
    for i in 0..ORDER {
        ret[i][i] = lambda;
    }
    ret
}

pub fn pascal_triangle<T: Float + Default, const ORDER: usize>() -> [T; ORDER] {
    let mut ret: [T; ORDER] = [T::one(); ORDER];
    for i in 1..ORDER {
        for j in 1..(ORDER - i) {
            ret[j] = ret[j] + ret[j - 1];
        }
    }
    ret
}
