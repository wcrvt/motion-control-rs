use super::*;
use num_traits::Float;

impl<T: Float + Default, const N: usize> Matrix<T, N, N> {
    pub fn inverse(&self) -> Option<Matrix<T, N, N>> {
        let mut m1: Matrix<T, N, N> = self.clone();
        let mut m2: Matrix<T, N, N> = Matrix::diag(T::one());

        for i in 0..N {
            let mut max_row_option: usize = 0;
            let mut max_value_option: T = T::zero();
            for j in i..N {
                if m1[j][i].abs() > max_value_option {
                    max_value_option = m1[j][i].abs();
                    max_row_option = j;
                }
            }

            /*Error Handling */
            if max_value_option == T::zero() {
                return None;
            }

            let m1_buffer: [T; N] = m1[i];
            let m2_buffer: [T; N] = m2[i];
            m1[i] = m1[max_row_option];
            m2[i] = m2[max_row_option];
            m1[max_row_option] = m1_buffer;
            m2[max_row_option] = m2_buffer;

            let scaler: T = T::one() / m1[i][i];
            for j in 0..N {
                m1[i][j] = m1[i][j] * scaler;
                m2[i][j] = m2[i][j] * scaler;
            }

            for j in 0..N {
                if i != j {
                    let scaler: T = m1[j][i];
                    for k in 0..N {
                        m1[j][k] = m1[j][k] - scaler * m1[i][k];
                        m2[j][k] = m2[j][k] - scaler * m2[i][k];
                    }
                }
            }
        }

        Some(m2)
    }
}

impl<T: Float + Default, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub fn inverse_underdetermined(&self) -> Option<Matrix<T, COLS, ROWS>> {
        if ROWS > COLS {
            return None;
        }
        let mt: Matrix<T, COLS, ROWS> = self.transpose();
        let m_mt: Matrix<T, ROWS, ROWS> = self * &mt;
        let inv_m_mt: Matrix<T, ROWS, ROWS> = if let Some(x) = m_mt.inverse() {
            x
        } else {
            return None;
        };
        Some(&mt * &inv_m_mt)
    }

    pub fn inverse_overdetermined(&self) -> Option<Matrix<T, COLS, ROWS>> {
        if ROWS < COLS {
            return None;
        }
        let mt: Matrix<T, COLS, ROWS> = self.transpose();
        let m_mt: Matrix<T, COLS, COLS> = &mt * self;
        let inv_m_mt: Matrix<T, COLS, COLS> = if let Some(x) = m_mt.inverse() {
            x
        } else {
            return None;
        };
        Some(&inv_m_mt * &mt)
    }

    pub fn generalized_inverse(&self) -> Option<Matrix<T, COLS, ROWS>> {
        if ROWS < COLS {
            self.inverse_underdetermined()
        } else {
            self.inverse_overdetermined()
        }
    }
}
