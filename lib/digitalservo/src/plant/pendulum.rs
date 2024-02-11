use num_traits::Float;

const G: f64 = 9.80619920;

#[derive(Debug)]
pub struct Pendulum<T: Float> {
    pub d0xm: T,
    pub d1xm: T,
    pub d2xm: T,
    pub d0xp: [T; 2],
    pub d1xp: [T; 2],
    pub d2xp: [T; 2],
    pub d0theta: T,
    pub d1theta: T,
    pub d2theta: T,
    kt: T,
    mm: T,
    lp: T,
    mp: T,
    jp: T,
    ts: T,
}

impl<T: Float + std::ops::AddAssign + std::fmt::Debug> Pendulum<T> {
    pub fn new(ts: T) -> Self {
        let tz = T::zero();
        Self {
            d0xm: tz,
            d1xm: tz,
            d2xm: tz,
            d0xp: [tz; 2],
            d1xp: [tz; 2],
            d2xp: [tz; 2],
            d0theta: tz,
            d1theta: tz,
            d2theta: tz,
            kt: tz,
            mm: tz,
            lp: tz,
            mp: tz,
            jp: tz,
            ts,
        }
    }

    #[must_use]
    pub fn set_motor_param(mut self, kt: T, mm: T) -> Self {
        self.kt = kt;
        self.mm = mm;
        self
    }

    #[must_use]
    pub fn set_pendulum_param(mut self, lp: T, mp: T) -> Self {
        self.lp = lp;
        self.mp = mp;
        self.jp = mp * lp.powi(2) / T::from(12.0).unwrap();
        self
    }

    pub fn set_init_theta(mut self, theta: T) -> Self {
        self.d0theta = theta;
        self
    }

    pub fn update(&mut self, iq_ref: T, tau_p: T) {
        let lp_h: T = self.lp * T::from(0.5).unwrap();

        self.d0xp[0] = -lp_h * self.d0theta.sin() + self.d0xm;
        self.d0xp[1] = lp_h * self.d0theta.cos();

        self.d1xp[0] = -lp_h * self.d1theta * self.d0theta.cos() + self.d1xm;
        self.d1xp[1] = -lp_h * self.d1theta * self.d0theta.sin();

        self.d2xp[0] = self.d2xm
            - lp_h
                * (self.d2theta * self.d0theta.cos() - self.d1theta.powi(2) * self.d0theta.sin());
        self.d2xp[1] =
            -lp_h * (self.d2theta * self.d0theta.sin() + self.d1theta.powi(2) * self.d0theta.cos());

        self.d0xm += self.d1xm * self.ts;
        self.d1xm += self.d2xm * self.ts;

        self.d0theta += self.d1theta * self.ts;
        self.d1theta += self.d2theta * self.ts;

        let f_resultant_m: T = self.mp
            * lp_h
            * (self.d2theta * self.d0theta.cos() - self.d1theta.powi(2) * self.d0theta.sin());
        let moment_p: T = self.mp
            * lp_h
            * (self.d2xm * self.d0theta.cos() + T::from(G).unwrap() * self.d0theta.sin());

        self.d2xm = (iq_ref * self.kt + f_resultant_m) / (self.mm + self.mp);
        self.d2theta = (moment_p + tau_p) / (self.jp + self.mp * lp_h.powi(2));

        self.d0theta = self.d0theta % (T::from(2.0 * std::f64::consts::PI).unwrap());
    }
}
