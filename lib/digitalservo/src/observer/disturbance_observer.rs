use crate::algebra::*;
use num_traits::Float;

#[derive(Debug, Clone, Copy)]
pub struct VelocityBased<T, const ORDER: usize>
where
    [(); ORDER + 1]:,
{
    ts: T,
    pub kt: T,
    pub jm: T,
    g: Vector<T, { ORDER + 1 }>,
    tu: Vector<T, { ORDER + 1 }>,
    ty: Vector<T, { ORDER + 1 }>,
    tz: Matrix<T, { ORDER + 1 }, { ORDER + 1 }>,
    py: Vector<T, { ORDER + 1 }>,
    py0_z1: T,
}

impl<T: Float + Default, const ORDER: usize> VelocityBased<T, ORDER>
where
    [(); ORDER + 1]:,
    [(); ORDER + 2]:,
{
    pub fn new(ts: T, kt: T, jm: T, bandwidth: T) -> Self {
        /* gain vector which realizes multiple root */
        let pascal_coeff: [T; ORDER + 2] = pascal_triangle::<T, { ORDER + 2 }>();
        let mut coeff: Vector<T, { ORDER + 1 }> = Vector::new();
        for i in 0..(ORDER + 1) {
            coeff[i] = pascal_coeff[i + 1] * bandwidth.powi(i as i32 + 1);
        }
        let g: Vector<T, { ORDER + 1 }> = &coeff * jm;

        /* minimum-order state observer */
        /* see https://digitalservo.jp/library/linear-control-design/observer-design/minimal-order-observer/ */

        //system matrix
        let a_11: Matrix<T, { ORDER + 1 }, { ORDER + 1 }> =
            jordan_block::<T, { ORDER + 1 }>(T::zero());
        let a_12: Vector<T, { ORDER + 1 }> = Vector::new();
        let mut a_21: Vector<T, { ORDER + 1 }> = Vector::new();
        a_21[0] = T::one() / jm;
        let a_22: T = T::zero();

        //input vector
        let b1: Vector<T, { ORDER + 1 }> = Vector::new();
        let b2: T = kt / jm;

        //matrices for state observer
        let tu: Vector<T, { ORDER + 1 }> = b1 - g * b2;
        let ty: Vector<T, { ORDER + 1 }> = a_12 - g * a_22;
        let tz: Matrix<T, { ORDER + 1 }, { ORDER + 1 }> = a_11 - g.outer(a_21);

        //initialize
        let py: Vector<T, { ORDER + 1 }> = Vector::new();
        let py0_z1: T = T::zero();

        Self {
            ts,
            kt,
            jm,
            g,
            tu,
            ty,
            tz,
            py,
            py0_z1,
        }
    }

    pub fn set_kt(mut self, kt: T) -> Self {
        self.kt = kt;
        self.tu = self.g * (-kt / self.jm);
        self
    }

    pub fn jm(mut self, jm: T) -> Self {
        let a_11: Matrix<T, { ORDER + 1 }, { ORDER + 1 }> =
            jordan_block::<T, { ORDER + 1 }>(T::zero());
        let mut a_21: Vector<T, { ORDER + 1 }> = Vector::new();
        a_21[0] = -T::one() / jm;

        self.jm = jm;
        self.tu = self.g * (-self.kt / jm);
        self.tz = a_11 + self.g.outer(a_21);

        self
    }

    pub fn update(&mut self, i: T, v: T) -> T {
        let u: Vector<T, { ORDER + 1 }> = self.tu * i + (self.tz * self.g + self.ty) * v;
        self.py += (u + self.tz * self.py) * self.ts;
        let out: T = self.py0_z1 + (self.g * v)[0];
        self.py0_z1 = self.py[0];
        -out
    }
}

fn jordan_block<T: Float + Default, const ORDER: usize>(lambda: T) -> Matrix<T, ORDER, ORDER> {
    let mut ret: Matrix<T, ORDER, ORDER> = Matrix::<T, ORDER, ORDER>::new();
    for i in 0..ORDER - 1 {
        ret[i][i + 1] = T::one();
    }
    for i in 0..ORDER {
        ret[i][i] = lambda
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
