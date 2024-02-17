use num_traits::{float::FloatConst, Float, NumAssignOps, One};
use num_complex::Complex;
use super::*;


pub fn get_frequency_characteristics_from_s<T: Float + FloatConst + Default + NumAssignOps, const N: usize, const M: usize>(
    coefficients_numer: [T; N],
    coefficients_denom: [T; M],
    freq_from: T,
    freq_to: T,
    datalen: usize
) -> Vec<FrequencyResponse<T>>{

    let mut ret: Vec<FrequencyResponse<T>> = Vec::with_capacity(datalen);

    let mut s: Complex<T>;
    let mut sv: Complex<T>;
    let mut freq_res: Complex<T>;

    let mut freq: T = freq_from;
    let dfreq: T = (freq_to - freq_from) / T::from(datalen).unwrap();
    let conv_f_to_omega: T = T::from(2.0).unwrap() * FloatConst::PI();

    let mut numer: Complex<T>;
    let mut denom: Complex<T>;

    for _ in 0..datalen {
        numer = Complex::default();
        denom = Complex::default();

        let omega: T = conv_f_to_omega * freq;
        s = Complex::new(T::zero(), omega);

        sv = Complex::one();
        for i in 0..N {
            numer += sv * coefficients_numer[N - 1 - i];
            sv *= s;
        }

        sv = Complex::one();
        for i in 0..M {
            denom +=  sv * coefficients_denom[M - 1 - i];
            sv *= s;
        }

        freq_res = numer / denom;

        ret.push(
            FrequencyResponse {
                freq,
                re: freq_res.re,
                im: freq_res.im,
                gain: freq_res.norm(),
                phase: freq_res.im.atan2(freq_res.re),
            }
        );

        freq += dfreq;
    }

    ret
}



pub fn get_frequency_characteristics_from_z<T: Float + FloatConst + Default + NumAssignOps, const N: usize, const M: usize>(
    coefficients_numer: [T; N],
    coefficients_denom: [T; M],
    ts: T,
    freq_from: T,
    freq_to: T,
    datalen: usize
) -> Vec<FrequencyResponse<T>>{

    let mut ret: Vec<FrequencyResponse<T>> = Vec::with_capacity(datalen);

    let mut z: Complex<T>;
    let mut zv: Complex<T>;
    let mut freq_res: Complex<T>;

    let mut freq: T = freq_from;
    let dfreq: T = (freq_to - freq_from) / T::from(datalen).unwrap();
    let conv_f_to_omega: T = T::from(2.0).unwrap() * FloatConst::PI();

    let mut numer: Complex<T>;
    let mut denom: Complex<T>;

    for _ in 0..datalen {
        numer = Complex::default();
        denom = Complex::default();

        let omega: T = conv_f_to_omega * freq;
        z = Complex::new(T::zero(), -omega * ts).exp();

        zv = Complex::one();
        for i in 0..N {
            numer += zv * coefficients_numer[i];
            zv *= z;
        }

        zv = Complex::one();
        for i in 0..M {
            denom +=  zv * coefficients_denom[i];
            zv *= z;
        }

        freq_res = numer / denom;

        ret.push(
            FrequencyResponse {
                freq,
                re: freq_res.re,
                im: freq_res.im,
                gain: freq_res.norm(),
                phase: freq_res.im.atan2(freq_res.re),
            }
        );

        freq += dfreq;
    }

    ret
}