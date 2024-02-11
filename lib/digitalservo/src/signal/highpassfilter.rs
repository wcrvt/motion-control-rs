use num_traits;

#[derive(Debug, Copy, Clone)]
pub struct FirstOrder<T> {
    y_z1: T,
    u_z1: T,
    pub ts: T,
    coef: [T; 2],
}

impl<T: num_traits::Float> FirstOrder<T> {
    pub fn new(ts: T, bandwidth: T) -> Self {
        let t2: T = T::from(2.0).unwrap();
        let divider: T = t2 + bandwidth * ts;
        Self {
            y_z1: T::zero(),
            u_z1: T::zero(),
            ts,
            coef: [t2 / divider, (t2 - bandwidth * ts) / divider],
        }
    }

    pub fn update(&mut self, u: T) -> T {
        let out = self.coef[0] * (u - self.u_z1) + self.coef[1] * self.y_z1;
        self.y_z1 = out;
        self.u_z1 = u;
        out
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SecondOrder<T> {
    y_z1: T,
    y_z2: T,
    u_z1: T,
    u_z2: T,
    pub ts: T,
    coef: [T; 3],
}

impl<T: num_traits::Float> SecondOrder<T> {
    pub fn new(ts: T, bandwidth: T) -> Self {
        let t2: T = T::from(2.0).unwrap();
        let t4: T = T::from(4.0).unwrap();

        let divider = (t2 + bandwidth * ts) * (t2 + bandwidth * ts);
        Self {
            y_z1: T::zero(),
            y_z2: T::zero(),
            u_z1: T::zero(),
            u_z2: T::zero(),
            ts,
            coef: [
                t4 / divider,
                t2 * (t4 - bandwidth * bandwidth * ts * ts) / divider,
                (-t4 + t4 * bandwidth * ts - bandwidth * bandwidth * ts * ts) / divider,
            ],
        }
    }

    pub fn update(&mut self, u: T) -> T {
        let t2: T = T::from(2.0).unwrap();
        let out: T = self.coef[0] * (u - t2 * self.u_z1 + self.u_z2)
            + self.coef[1] * self.y_z1
            + self.coef[2] * self.y_z2;
        self.y_z2 = self.y_z1;
        self.u_z2 = self.u_z1;
        self.y_z1 = out;
        self.u_z1 = u;
        out
    }
}
