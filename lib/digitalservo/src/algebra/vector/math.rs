use num_traits::Float;
use super::*;

impl<T, const ROWS: usize> Vector<T, ROWS>
where
    T: Float + Default
{
    pub fn norm(&self) -> T {
        (self * self).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / (self * self).sqrt()
    }

    pub fn projection<S: Borrow<Self>>(self, u: S) -> Vector<T, ROWS> {
        let u = u.borrow();
        u * (self * u) / (u * u)
    }
}