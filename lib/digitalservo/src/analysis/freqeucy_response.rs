use num_traits::{float::FloatConst, Float, NumAssignOps, One};
use num_complex::Complex;

use crate::data_storage::DataStorage;
use std::fmt::Debug;
use std::path::Path;

pub struct FrequencyAnalyzer<T, P> {
    freq_from: T,
    freq_to: T,
    dfreq: T,
    storage: DataStorage<T, P, 5>,
    datalen: usize
}

impl <T, P> FrequencyAnalyzer<T, P>
where
    T: Float + Default + Copy + Debug + FloatConst + NumAssignOps,
    P: AsRef<Path>
{
    pub fn new(path: P, separator: &str, datalen: usize) -> Self {
        Self {
            freq_from: T::zero(),
            freq_to: T::from(1000.0).unwrap(),
            dfreq: T::from(1000.0).unwrap() / T::from(datalen).unwrap(),
            storage: DataStorage::new(path, separator, datalen),
            datalen
        }
    }

    pub fn set_frequency_range(mut self, from: T, to: T) -> Self {
        self.freq_from = from;
        self.freq_to = to;
        self.dfreq = (to - from) / T::from(self.datalen).unwrap();
        self
    }

    pub fn frequency_response_s<const N: usize, const M: usize>( &mut self, coefficients_numer: [T; N], coefficients_denom: [T; M]) {
    
        let mut s: Complex<T>;
        let mut sv: Complex<T>;
        let mut freq_res: Complex<T>;
    
        let mut freq: T = self.freq_from;
        let conv_f_to_omega: T = T::from(2.0).unwrap() * FloatConst::PI();
    
        let mut numer: Complex<T>;
        let mut denom: Complex<T>;
    
        for _ in 0..self.datalen {
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

            let gain: T = T::from(20.0).unwrap() * freq_res.norm().log10();
            let phase: T = freq_res.im.atan2(freq_res.re);
            self.storage.add([freq, gain, phase, freq_res.re, freq_res.im]);
    
            freq += self.dfreq;
        }

        self.storage.write_file().unwrap();
    }

    pub fn frequency_response_z<const N: usize, const M: usize>( &mut self, coefficients_numer: [T; N], coefficients_denom: [T; M], ts: T) {
    
        let mut z: Complex<T>;
        let mut zv: Complex<T>;
        let mut freq_res: Complex<T>;
    
        let mut freq: T = self.freq_from;
        let conv_f_to_omega: T = T::from(2.0).unwrap() * FloatConst::PI();
    
        let mut numer: Complex<T>;
        let mut denom: Complex<T>;
    
        for _ in 0..self.datalen {
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
            
            let gain: T = T::from(20.0).unwrap() * freq_res.norm().log10();
            let phase: T = freq_res.im.atan2(freq_res.re);
            self.storage.add([freq, gain, phase, freq_res.re, freq_res.im]);
    
            freq += self.dfreq;
        }

        self.storage.write_file().unwrap();
    }

}

