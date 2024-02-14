use num_traits::Float;
use super::*;

impl<T, const N: usize> Matrix<T, N, N>
where
    T: Float + Default
{
    pub fn exp(self) -> Self {
        pub const P: usize = 1000;
        let identity: Matrix<T, N, N> = Matrix::<T, N, N>::diag(T::one());
        let mut ret: Matrix<T, N, N>  = identity + self / T::from(P).unwrap();
        for i in (1..P).rev() {
            ret = identity + self / T::from(i).unwrap() * ret;
        }
        ret
    }
}