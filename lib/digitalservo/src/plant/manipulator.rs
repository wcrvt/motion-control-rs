use crate::algebra::*;
use num_traits::Float;

pub const JOINTSPACE_DIM: usize = 3;
pub const WORKSPACE_DIM: usize = 2;

pub struct SeriesLinkManipulator<T: Float> {
    pub d0theta: Vector<T, JOINTSPACE_DIM>,
    pub d1theta: Vector<T, JOINTSPACE_DIM>,
    pub d2theta: Vector<T, JOINTSPACE_DIM>,
    pub d0x: Vector<T, WORKSPACE_DIM>,
    pub d1x: Vector<T, WORKSPACE_DIM>,
    pub d2x: Vector<T, WORKSPACE_DIM>,
    jacobian: Matrix<T, WORKSPACE_DIM, JOINTSPACE_DIM>,
    djacobian: Matrix<T, WORKSPACE_DIM, JOINTSPACE_DIM>,
    pub kt: Vector<T, JOINTSPACE_DIM>,
    pub jm: Vector<T, JOINTSPACE_DIM>,
    pub link: Vector<T, JOINTSPACE_DIM>,
    ts: T,
}

impl<T: Float + Default + std::ops::AddAssign> SeriesLinkManipulator<T> {
    pub fn new(
        kt: [T; JOINTSPACE_DIM],
        jm: [T; JOINTSPACE_DIM],
        link: [T; JOINTSPACE_DIM],
        ts: T,
    ) -> Self {
        Self {
            d0theta: Vector::<T, JOINTSPACE_DIM>::new(),
            d1theta: Vector::<T, JOINTSPACE_DIM>::new(),
            d2theta: Vector::<T, JOINTSPACE_DIM>::new(),
            d0x: Vector::<T, WORKSPACE_DIM>::new(),
            d1x: Vector::<T, WORKSPACE_DIM>::new(),
            d2x: Vector::<T, WORKSPACE_DIM>::new(),
            jacobian: Matrix::<T, WORKSPACE_DIM, JOINTSPACE_DIM>::new(),
            djacobian: Matrix::<T, WORKSPACE_DIM, JOINTSPACE_DIM>::new(),
            kt: Vector::from(kt),
            jm: Vector::from(jm),
            link: Vector::from(link),
            ts,
        }
    }

    fn update_position(&mut self) {
        let theta: [T; JOINTSPACE_DIM] = [
            self.d0theta[0],
            self.d0theta[0] + self.d0theta[1],
            self.d0theta[0] + self.d0theta[1] + self.d0theta[2],
        ];

        self.d0x[0] = self.link[0] * theta[0].sin()
            + self.link[1] * theta[1].sin()
            + self.link[2] * theta[2].sin();
        self.d0x[1] = self.link[0] * theta[0].cos()
            + self.link[1] * theta[1].cos()
            + self.link[2] * theta[2].cos();
    }

    fn update_jacobian(&mut self) {
        let theta: [T; JOINTSPACE_DIM] = [
            self.d0theta[0],
            self.d0theta[0] + self.d0theta[1],
            self.d0theta[0] + self.d0theta[1] + self.d0theta[2],
        ];

        let dtheta: [T; JOINTSPACE_DIM] = [
            self.d1theta[0],
            self.d1theta[0] + self.d1theta[1],
            self.d1theta[0] + self.d1theta[1] + self.d1theta[2],
        ];

        self.jacobian = Matrix::from([
            [
                self.link[0] * theta[0].cos()
                    + self.link[1] * theta[1].cos()
                    + self.link[2] * theta[2].cos(),
                self.link[1] * theta[1].cos() + self.link[2] * theta[2].cos(),
                self.link[2] * theta[2].cos(),
            ],
            [
                -self.link[0] * theta[0].sin()
                    - self.link[1] * theta[1].sin()
                    - self.link[2] * theta[2].sin(),
                -self.link[1] * theta[1].sin() - self.link[2] * theta[2].sin(),
                -self.link[2] * theta[2].sin(),
            ],
        ]);

        self.djacobian = Matrix::from([
            [
                -self.link[0] * dtheta[0] * theta[0].sin()
                    - self.link[1] * dtheta[1] * theta[1].sin()
                    - self.link[2] * dtheta[2] * theta[2].sin(),
                -self.link[1] * dtheta[1] * theta[1].sin()
                    - self.link[2] * dtheta[2] * theta[2].sin(),
                -self.link[2] * dtheta[2] * theta[2].sin(),
            ],
            [
                -self.link[0] * dtheta[0] * theta[0].cos()
                    - self.link[1] * dtheta[1] * theta[1].cos()
                    - self.link[2] * dtheta[2] * theta[2].cos(),
                -self.link[1] * dtheta[1] * theta[1].cos()
                    - self.link[2] * dtheta[2] * theta[2].cos(),
                -self.link[2] * dtheta[2] * theta[2].cos(),
            ],
        ]);
    }

    #[must_use]
    pub fn set_init_theta(mut self, theta: [T; 3]) -> Self {
        self.d0theta = Vector::from(theta);
        self.update_position();
        self.update_jacobian();
        self
    }

    pub fn update(&mut self, iq: [T; JOINTSPACE_DIM], dis: [T; JOINTSPACE_DIM]) {
        for i in 0..JOINTSPACE_DIM {
            self.d0theta[i] += self.d1theta[i] * self.ts;
            self.d1theta[i] += self.d2theta[i] * self.ts;
            self.d2theta[i] = (self.kt[i] * iq[i] - dis[i]) / self.jm[i];
        }

        self.update_position();
        self.update_jacobian();
        self.d1x = &self.jacobian * &self.d1theta;
    }

    pub fn computed_torque_method(
        &mut self,
        ddx_ref: &[T; WORKSPACE_DIM],
        ddx_null: &[T; JOINTSPACE_DIM],
    ) -> [T; JOINTSPACE_DIM] {
        let djaco_dq: Vector<T, 2> = &self.djacobian * &self.d1theta;
        let inv_jacobian: Matrix<T, 3, 2> = self.jacobian.inverse_underdetermined().unwrap();

        //Null space
        let identity_matrix: Matrix<T, 3, 3> = Matrix::diag(T::one());
        let jacobian_kernel: Matrix<T, 3, 3> = &identity_matrix - &inv_jacobian * &self.jacobian;

        let ddtheta_ref: Vector<T, 3> = &inv_jacobian * (Vector::from(ddx_ref) - djaco_dq);
        let ddtheta_null: Vector<T, 3> = &jacobian_kernel * Vector::from(ddx_null);

        (ddtheta_ref + ddtheta_null).data
    }
}
