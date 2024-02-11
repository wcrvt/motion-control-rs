use num_traits::Float;
use rustfft::{num_complex::Complex, Fft, FftNum, FftPlanner};

#[derive(Debug, Copy, Clone)]
pub struct FFTData<T> {
    pub nfreq: T,
    pub re: T,
    pub im: T,
    pub power: T,
    pub phase: T,
}

pub fn power<T: FftNum + Float>(data: &[T], logarithmic: bool) -> Vec<T> {
    let t_0: T = T::zero();
    let t_1: T = T::one();
    let t_2: T = T::from(2.0).unwrap();
    let t_20: T = T::from(20.0).unwrap();

    let data_len: usize = data.len();
    let inv_data_len = match T::from_usize(data.len()) {
        Some(i) => t_1 / i,
        None => return vec![],
    };

    let mut planner: FftPlanner<T> = FftPlanner::new();
    let fft: std::sync::Arc<dyn Fft<T>> = planner.plan_fft_forward(data_len);

    let mut buffer: Vec<Complex<T>> = data
        .iter()
        .map(|x| Complex::new(*x * inv_data_len, t_0))
        .collect();
    fft.process(&mut buffer);

    let data_len_half: usize = buffer.len() / 2;
    buffer[0..data_len_half]
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let c: T = if i == 0 { t_1 } else { t_2 };
            let power: T = c * (x.re * x.re + x.im * x.im).sqrt();
            match logarithmic {
                true => t_20 * power.log10(),
                _ => power,
            }
        })
        .collect()
}

pub fn fft<T: FftNum + Float>(data: &[T], logarithmic: bool) -> Vec<FFTData<T>> {
    let t_0: T = T::zero();
    let t_1: T = T::one();
    let t_2: T = T::from(2.0).unwrap();
    let t_20: T = T::from(20.0).unwrap();

    let data_len: usize = data.len();
    let inv_data_len = match T::from_usize(data.len()) {
        Some(i) => t_1 / i,
        None => return vec![],
    };

    let mut planner: FftPlanner<T> = FftPlanner::new();
    let fft: std::sync::Arc<dyn Fft<T>> = planner.plan_fft_forward(data_len);

    let mut buffer: Vec<Complex<T>> = data
        .iter()
        .map(|x| Complex::new(*x * inv_data_len, t_0))
        .collect();

    fft.process(&mut buffer);

    let data_len_half: usize = buffer.len() / 2;
    buffer[0..data_len_half]
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let c: T = if i == 0 { t_1 } else { t_2 };
            let power: T = c * (x.re * x.re + x.im * x.im).sqrt();
            match logarithmic {
                true => t_20 * power.log10(),
                _ => power,
            };
            let phase: T = x.im.atan2(x.re);
            FFTData {
                nfreq: T::from(i).unwrap(),
                re: x.re,
                im: x.im,
                power,
                phase,
            }
        })
        .collect()
}

pub fn ideal_hpf<T: FftNum + Float>(data: &[T], nf_cutoff: usize) -> Vec<T> {
    let t_0: T = T::zero();
    let t_1: T = T::one();

    let data_len: usize = data.len();
    let inv_data_len = match T::from_usize(data.len()) {
        Some(i) => t_1 / i,
        None => return vec![],
    };

    let mut planner_forward: FftPlanner<T> = FftPlanner::new();
    let fft_forward: std::sync::Arc<dyn Fft<T>> = planner_forward.plan_fft_forward(data_len);

    let mut buffer: Vec<Complex<T>> = data
        .iter()
        .map(|x| Complex::new(*x * inv_data_len, t_0))
        .collect();

    fft_forward.process(&mut buffer);

    for i in 0..nf_cutoff {
        buffer[i].re = t_0;
        buffer[i].im = t_0;
        buffer[data_len - 1 - i].re = t_0;
        buffer[data_len - 1 - i].re = t_0;
    }

    let mut planner_backward: FftPlanner<T> = FftPlanner::new();
    let fft_backward: std::sync::Arc<dyn Fft<T>> = planner_backward.plan_fft_inverse(data_len);

    fft_backward.process(&mut buffer);

    buffer.iter().map(|x| x.re).collect()
}
