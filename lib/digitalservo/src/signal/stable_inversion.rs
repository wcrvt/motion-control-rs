use num_traits::Float;

pub struct StableInverter <T, F1, const N: usize> {
    reference: F1,
    f_stable: Option<fn(T) -> T>,
    f_unstable: Option<fn(T) -> T>,
    t_max: T,
    dt: T,
}

impl <T: Float + std::ops::AddAssign, F1: Fn(T) -> [T; N], const N: usize> StableInverter<T, F1, N> {

    pub fn new(reference: F1, f_stable: Option<fn(T) -> T>, f_unstable: Option<fn(T) -> T>, t_max: T) -> Self {
        Self {
            reference,
            f_stable,
            f_unstable,
            t_max,
            dt: T::from(1e-5).unwrap()
        }
    }

    pub fn set_dt(mut self, dt: T) -> Self {
        self.dt = dt;
        self
    }

    pub fn output(&mut self, t: T) -> [T; N] {
        let mut x: [T; N] = [T::zero(); N];

        if self.f_stable.is_none() && self.f_unstable.is_none() {
            return (&self.reference)(t);
        }

        //stable part
        if let Some(f) = &self.f_stable {
            let mut x_stable: [T; N] = [T::zero(); N];
            let mut tau: T = T::zero();
            let iter: usize = (t / self.dt).to_usize().unwrap();
            for _ in 0..iter {
                let fu_tau: T = f(t - tau);
                let ref_tau: [T; N] = (&self.reference)(tau);
                for i in 0..N {
                    x_stable[i] += fu_tau * ref_tau[i] * self.dt;
                }
                tau += self.dt;
            }
            for i in 0..N {
                x[i] += x_stable[i];
            }
        }

        //unstable part
        if let Some(f) = &self.f_unstable {
            let mut x_unstable: [T; N] = [T::zero(); N];
            let mut tau: T = t;
            let iter: usize = ((self.t_max - t) / self.dt).to_usize().unwrap();
            for _ in 0..iter {
                let fu_tau: T = f(tau - t);
                let ref_tau: [T; N] = (&self.reference)(tau);
                for i in 0..N {
                    x_unstable[i] += fu_tau * ref_tau[i] * self.dt;
                }
                tau += self.dt;
            }
            for i in 0..N {
                x[i] += x_unstable[i];
            }
        }

        x
    }
}