use num_complex::Complex;
use num_traits::Float;
use std::borrow::Borrow;

pub fn dka_method<T: Float + Default, S: Borrow<[T; N]>, const N: usize>(coefficients: S) -> Option<[Complex<T>; N - 1]>
where [(); N - 1]:
{    
    if N <= 1 { return None; }

    let coefficients: &[T; N] = coefficients.borrow();
    let mut z: [Complex<T>; N - 1] = [Complex::default(); N - 1];

    let c1n: T = coefficients[1] / T::from(N - 1).unwrap();
    if N == 2 {
        z[0] = Complex {re: -c1n / coefficients[0], im: T::zero()};
        return Some(z);
    }

    /* Horner's rule (synthetic division) */
    let mut c_origin: [T; N] = coefficients.clone();
    let mut c_horner: [T; N] = [T::zero(); N];

    for i in (2..N).rev() {
        c_horner[i] = c_origin[0];
        for j in 1..i + 1 {
            c_origin[j - 1] = c_horner[i];
            c_horner[i] = c_horner[i] * c1n + c_origin[j];
        }
    }

    c_horner[0] = coefficients[0];
    c_horner[1] = c_origin[1] + c1n;

    /* Aberth method */
    let mut c_radius: [T; N] = [T::one(); N];
    for i in 1..N {
        c_radius[i] = -c_horner[i];
    }

    let mut r_0: T = T::zero();
    for i in 0..N {
        let x: T = (coefficients[i] / coefficients[0]).abs().powf(T::from(1.0 / ((i as f64) + 1.0)).unwrap());
        r_0 = if r_0 > x { r_0 } else { x };
    }

    let r: Complex<T> = Complex {re: newton_method(c_radius, r_0), im: T::zero()};

    for i in 0..(N - 1) {
        let phase_f64: f64 = (2.0 * (i as f64) - 1.5) * std::f64::consts::PI / ((N - 1) as f64);
        let phase: Complex<T> = Complex {re: T::zero(), im: T::from(phase_f64).unwrap()};
        z[i] = Complex{re: c1n / coefficients[0], im: T::zero()} + r * phase.exp();
    }

    /* Durand-Kerner method */
    const MAX_ITERATION: usize = 10000;
    let mut iteration: usize = 0;
    let mut h_sum: T;

    loop {
        h_sum = T::zero();
        
        for i in 0..(N - 1) {
            let h: Complex<T> = mapping_h(coefficients, z[i]);
            let dh: Complex<T> = mapping_dh(z, i);
            if (dh == Complex{re: T::zero(), im: T::zero()}) { break; }
            z[i] = z[i] - h / dh;
            h_sum = h_sum + h.norm();
        }

        iteration += 1;

        if h_sum < T::from(1e-15).unwrap() || iteration == MAX_ITERATION { break; }
    }

    Some(z)
}

fn mapping_h<T: Float, S: Borrow<[T; N]>, const N: usize>(coefficients: S, x: Complex<T>) -> Complex<T> {
    let coefficients = coefficients.borrow();
    let mut ret: Complex<T> = Complex::new(T::zero(), T::zero());
    for i in 0..N {
        ret = ret + Complex {re: coefficients[i], im: T::zero()} * x.powf(T::from(N - 1 - i).unwrap());
    }
    ret
}

fn mapping_dh<T: Float, S: Borrow<[Complex<T>; N]>, const N: usize>(z: S, j: usize) -> Complex<T> {
    let z = z.borrow();
    let z_j: Complex<T> = z[j];
    let mut ret: Complex<T> = Complex::new(T::one(), T::zero());
    for i in 0..N {
        let x: Complex<T> = if i == j { Complex {re: T::one(), im: T::zero()} } else { z_j - z[i] };
        ret = ret * x;
    }
    ret
}

fn newton_method<T: Float, S: Borrow<[T; N]>, const N: usize>(c: S, x0: T) -> T {

    let c: &[T; N] = &c.borrow();

    const MAX_ITERATION: usize = 100000;
    let mut iteration: usize = 0;
    let mut x: T = x0;

    loop {
        let y: T = mapping_f(c, x);
        let dy: T = mapping_df(c, x);
        if y.abs() < T::from(1e-15).unwrap() || iteration == MAX_ITERATION || dy == T::zero() { break; }
        x = x - y / dy;
        iteration += 1;
    }

    x
}

fn mapping_f<T: Float, S: Borrow<[T; N]>, const N: usize>(conefficients: S, x: T) -> T {
    let conefficients = conefficients.borrow();
    let mut ret: T = T::zero();
    for i in 0..N {
        ret = ret + conefficients[i] * x.powf(T::from(N - 1 - i).unwrap());
    }
    ret
}

fn mapping_df<T: Float, S: Borrow<[T; N]>, const N: usize>(conefficients: S, x: T) -> T {
    let conefficients = conefficients.borrow();
    let mut ret: T = T::zero();
    for i in 0..(N - 1) {
        ret = ret + conefficients[i] * x.powf(T::from(N - 2 - i).unwrap()) * T::from(N - 1 - i).unwrap();
    }
    ret

}