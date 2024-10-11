use std::f64::consts::PI;
use rustfft::{num_complex::Complex, num_traits::Zero};
///
/// Holds a buffer of samples ready to FFT processing
pub struct FftBuf {
    fft_size: usize,
    sampl_freq: usize,
    /// Used for restoring the frequency by it's index withing 0..`fft_size`
    freq_factor: f64,
    amp_factor: f64,
    delta_t: f64,
    /// continuous index
    time_i: usize,
    unit_complex: Vec<Complex<f64>>,
    /// circular index
    index: usize,
    index_last: usize,
    complex: Vec<Complex<f64>>,

}
//
//
impl FftBuf {
    ///
    /// Returns new instance of `FftBuf`
    /// - `fft_size` - length of the FFT input buffer as well as length of the FFT out buffer
    /// - `sampl_freq` - frequency of the sampling of the input signal, Hz
    pub fn new(fft_size: usize, sampl_freq: usize) -> Self {
        let sampling_period = 1.0 / (sampl_freq as f64);
        let delta_t = sampling_period;  // / (fft_size as f64);
        let unit_complex: Vec<Complex<f64>> = (0..fft_size).into_iter().map(|i| {
            let angle = PI * 2.0 * (i as f64) / (fft_size as f64);
            Complex {
                re: angle.cos(), 
                im: angle.sin()
            }
        }).collect();
        log::trace!("FftBuf.new | unit_complex: {:?}", unit_complex);
        Self {
            fft_size,
            sampl_freq,
            freq_factor: (sampl_freq as f64) / (fft_size as f64),
            amp_factor: 2.0 / (fft_size as f64),
            delta_t,
            time_i: 0,
            unit_complex,
            index: 0,
            index_last: fft_size - 1,
            complex: vec![Complex::zero(); fft_size],
        }
    }
    ///
    /// Returns sampling frequency
    pub fn sampl_freq(&self) -> usize {
        self.sampl_freq
    }
    ///
    /// Returns factor to restore the amplitude from FFT results
    pub fn amp_factor(&self) -> f64 {
        self.amp_factor
    }
    ///
    /// Pushing new value into FFT buffer
    /// - Projecting `value` onto complex circle
    /// - Returns mutable ref to FFT buffer if it is full, else None
    /// - If buffer is full, next time call `add` will first clear the buffer for new sequence
    pub fn add(&mut self, value: f64) -> (f64, Option<&mut Vec<Complex<f64> >>) {
        if self.index == 0 {
            self.complex = vec![Complex::zero(); self.fft_size];
        }
        self.complex[self.index].re = value * self.unit_complex[self.index].re;
        self.complex[self.index].im = value * self.unit_complex[self.index].im;
        log::trace!("FftBuf.add | index: {}", self.index);
        if self.index < self.index_last {
            self.index = (self.index  + 1) % self.fft_size;
            self.time_i += 1;
            (self.time(), None)
        } else {
            self.index = (self.index  + 1) % self.fft_size;
            self.time_i += 1;
            (self.time(), Some(&mut self.complex))
        }
    }
    ///
    /// Returns current timestamp as f64
    pub fn time(&self) -> f64 {
        (self.time_i as f64) * self.delta_t
    }
    ///
    /// Retirns freq corresponding to freq `index` withing `0..fft_size`
    pub fn freq_of(&self, index: usize) -> f64 {
        (index as f64) * self.freq_factor
    }
}
