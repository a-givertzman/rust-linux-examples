use std::{env, f64::consts::PI, sync::Arc};
use charming::{component::{Axis, Legend}, element::AxisType, series::Line, Chart, ImageRenderer};
// use charts_rs::{ChildChart, LineChart, MultiChart};
use fft_buf::FftBuf;
use rustfft::{num_complex::ComplexFloat, Fft, FftPlanner};
mod fft_buf;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();


    let fft_size = 30_000;
    let sampl_freq = 30_000;
    let fft: Arc<dyn Fft<f64>> = FftPlanner::new().plan_fft_forward(fft_size);
    let mut fft_buf = FftBuf::new(fft_size, sampl_freq);
    let mut x_axis = vec![];
    let mut y = vec![];
    let mut ffts: Vec< Vec<f32> > = vec![];
    for _ in 0..fft_size * 2 {
        let t = fft_buf.time();
        let value = 
             50. * (2. * PI *  100.0 * t).sin() + 
            200. * (2. * PI *  4000.0 * t).sin() + 
            150. * (2. * PI * 400.0 * t).sin();
        x_axis.push(format!("{:.3}", t));
        y.push(value as f32);
        // let (_, fft_buf_add) = fft_buf.add(value as f64);
        match fft_buf.add(value as f64) {
            (_, Some(buf)) => {
                // log::debug!("main | t: {:.4},  buf: {:?}", t, buf);
                fft.process(buf);
                // log::debug!("main | t: {:.4},  fft: {:?}", t, buf);
                let fft_scalar: Vec<f32> = buf.iter().take(fft_size / 2).map(|val| (val.abs() / ((fft_size / 2) as f64)) as f32).collect();
                log::debug!("main | t: {:.4},  fft: {:?}", t, fft_scalar.iter().map(|v| format!("{:.3}", v)).collect::<Vec<String>>());
                ffts.push(fft_scalar);
            }
            (_, None) => {
                log::debug!("main | t: {:.4}", t);
            },
        };
    }
    log::debug!("main | ffts: {}", ffts.len());
    for (i, fft) in ffts.into_iter().enumerate() {
        let len = fft.len();
        let freq = fft_buf.freq_of(i);
        log::debug!("main | fft_{}: {}", i, len);
        let chart = Chart::new()
            .legend(Legend::new().top("chart"))
            .x_axis(Axis::new().type_(AxisType::Category).interval(50.0).max_interval(50.0).data(
                (0..len).map(|_| format!("{:.1}", freq)).collect())
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Line::new()
                .name(format!("fft-{}", i))
                .data(fft),
            );
            let mut renderer = ImageRenderer::new(2800, 800);
            renderer.save(&chart, format!("src/fft/chart-{}.svg", i)).unwrap();
        
    }
}

#[test]
fn fft_buf() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let test_data = [
        // sampl_freq   fft_size    ffts    target
        (     12,            12,    1,      vec![(  2.0, 50.0), (  3.0, 150.0), (   4.0, 200.0)]),
        (     16,            12,    1,      vec![(  2.0, 50.0), (  3.0, 150.0), (   4.0, 200.0)]),
        (     16,            16,    2,      vec![(  3.0, 50.0), (  5.0, 150.0), (   6.0, 200.0)]),
        (    128,           128,    2,      vec![( 16.0, 50.0), ( 36.0, 150.0), (  62.0, 200.0)]),
        (    256,           256,    2,      vec![(  2.0, 50.0), (  4.0, 150.0), (  12.0, 200.0), (  37.0, 20.0), (  112.0, 12.0), (  126.0, 15.0)]),
        ( 10_000,        10_000,    2,      vec![(100.0, 50.0), (400.0, 150.0), (4000.0, 200.0)]),
        // ( 30_000,        30_000,    2,      vec![(100.0, 50.0), (400.0, 150.0), (4000.0, 200.0)]),
        // (300_000,       300_000,    2,      vec![(100.0, 50.0), (400.0, 150.0), (4000.0, 200.0)]),
    ];

    for (sampl_freq, fft_size, target_ffts, target_freqs) in test_data {
        let fft: Arc<dyn Fft<f64>> = FftPlanner::new().plan_fft_forward(fft_size);
        let mut fft_buf = FftBuf::new(fft_size, sampl_freq);
        log::debug!("main | fft_buf.sampling_freq: {}", fft_buf.sampl_freq());
        assert!(fft_buf.sampl_freq() == sampl_freq, "\nresult: {:?}\ntarget: {:?}", fft_buf.sampl_freq(), sampl_freq);
        let fft_amp_factor = fft_buf.amp_factor();
        log::debug!("main | fft_buf.amp_factor: {}", fft_amp_factor);
        assert!(fft_amp_factor == 1.0 / ((fft_size as f64) / 2.0), "\nresult: {:?}\ntarget: {:?}", fft_amp_factor, 1.0 / ((fft_size as f64) / 2.0));
        let mut ffts: Vec< Vec<f64> > = vec![];
        for _ in 0..fft_size * target_ffts {
            let t = fft_buf.time();
            let value = target_freqs.iter().fold(0.0, |val, (freq, amp)| {
                val + amp * (2. * PI *  freq * t).sin()
            });
            match fft_buf.add(value) {
                (_, Some(buf)) => {
                    // log::debug!("main | t: {:.4},  buf: {:?}", t, buf);
                    fft.process(buf);
                    // log::debug!("main | t: {:.4},  fft: {:?}", t, buf);
                    let fft_scalar: Vec<f64> = buf.iter().take(fft_size / 2).map(|val| val.abs() * fft_amp_factor).collect();
                    log::trace!("main | t: {:.4},  fft_scalar: {:?}", t, fft_scalar.iter().map(|v| format!("{:.3}", v)).collect::<Vec<String>>());
                    ffts.push(fft_scalar);
                }
                (_, None) => {
                    log::trace!("main | t: {:.4}", t);
                },
            };
        }
        log::trace!("main | ffts: {}", ffts.len());
        let result = ffts.len();
        let target = target_ffts;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        for fft in ffts {
            for (i, amp) in fft.into_iter().skip(1).enumerate() {
                let mut error_limit = ErrorLimit::new(3);
                let freq = fft_buf.freq_of(i);
                log::trace!("main | fft.freq[{}]: {}", i, freq);
                if amp > 1.0 && freq > 0.0 {
                    match nierest_freq(freq, &target_freqs) {
                        Some((target_freq, target_amp)) => {
                            let freq_err = (100.0 * (target_freq - freq) / target_freq).abs();
                            let amp_err = (100.0 * (target_amp - amp) / target_amp).abs();
                            if freq_err < 25.0 && amp_err < 25.0 {
                                log::debug!("main | fft.freq[{}]: {:.3} ({:.3} %), amp: {:.3} ({:.3} %)", i, freq, freq_err, amp, amp_err);
                            } else {
                                log::warn!("main | fft.freq[{}]: {:.3} ({:.3} %), amp: {:.3} ({:.3} %)", i, freq, freq_err, amp, amp_err);
                                if let Err(_) = error_limit.add() {
                                    panic!("main | error limit ({}) exceeded", error_limit.limit);
                                }
                            }
                        },
                        None => {
                            log::warn!("main | fft.freq[{}]: {:.3} - not found", i, freq);
                        },
                    }
                }
            }
        }
    }
    ///
    /// Returns nierest `freq` and coresponding Amp in `freqs`
    fn nierest_freq(freq: f64, freqs: &Vec<(f64, f64)>) -> Option<(f64, f64)> {
        let mut min_delta = f64::MAX;
        let mut delta;
        let mut result = None;
        for (f, amp) in freqs {
            delta = ((*f as f64) - freq).abs();
            if delta < min_delta {
                min_delta = delta;
                result = Some((*f, *amp));
            }
        }
        result
    }
}
///
/// Counts errors, returns Err if errors exceeded `limit`
#[allow(unused)]
struct ErrorLimit {
    errors: usize,
    limit: usize,
}
//
//
impl ErrorLimit {
    #[allow(unused)]
    pub fn new(limit: usize) -> Self {
        Self { errors: 0, limit }
    }
    ///
    /// Counts errors, returns Err if `limit` is exceeded
    #[allow(unused)]
    pub fn add(&mut self) -> Result<(), ()> {
        self.errors += 1;
        if self.errors < self.limit {
            Ok(())
        } else {
            Err(())
        }
    }
    ///
    /// Returns current number of errors
    #[allow(unused)]
    pub fn errors(&self) -> usize {
        self.errors
    }
}