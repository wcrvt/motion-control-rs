use super::*;
use num_traits::Float;

impl<T, const ROWS: usize> Vector<T, ROWS>
where
    T: Float + Default,
{
    pub fn norm(&self) -> T {
        (self * self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let zero_vec: Vector<T, ROWS> = Vector::<T, ROWS>::new();
        if self == zero_vec {
            zero_vec
        } else {
            self / (self * self).sqrt()
        }
    }

    pub fn projection<S: Borrow<Self>>(self, u: S) -> Vector<T, ROWS> {
        let u = u.borrow();
        u * (self * u) / (u * u)
    }
}
