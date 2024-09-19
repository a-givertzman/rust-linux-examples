use std::{env, f64::consts::PI, sync::Arc};
use charming::{component::{Axis, Legend}, element::{AxisTick, AxisType, MarkLine}, series::Line, Chart, ImageRenderer};
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
    let freq_factor = (sampl_freq as f64) / (fft_size as f64);
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
        log::debug!("main | fft_{}: {}", i, len);
        let chart = Chart::new()
            .legend(Legend::new().top("chart"))
            .x_axis(Axis::new().type_(AxisType::Category).interval(50.0).max_interval(50.0).data(
                (0..len).map(|i| format!("{:.1}", (i as f64) * freq_factor)).collect())
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