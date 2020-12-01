use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;

use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

pub fn run_fft(config: &super::config::ComputationArguments) {
    let raw_samples = super::preprocess::get_indexed_samples(config.input_file.as_str(), config.sample_resolution);
    let samples = raw_samples.iter().map(|(_, s)| *s as f32);
    let samples_num = samples.len();

    println!("Number of samples in file: {}", samples_num);

    let mut input: Vec<Complex<f32>> = samples.map(|f| Complex::new(f, 0.)).collect();
    let mut output: Vec<Complex<f32>> = vec![Complex::zero(); samples_num];

    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(samples_num);
    fft.process(&mut input, &mut output);

    // for r in output {
    //     println!("{}", r);
    // }

    render_real(&output);
    //render_imag(&output);
}

fn render_real(fft_result: &Vec<Complex<f32>>) {
    let num_of_samples = fft_result.len();

    let indexed_samples = fft_result
        .iter()
        .enumerate()
        .map(|(i, s)| (i as f64, s.re as f64))
        .collect();

    let s1: Plot = Plot::new(indexed_samples).point_style(
        PointStyle::new()
            .marker(PointMarker::Circle)
            .size(0.3)
            .colour("#DD3355"),
    );

    let v = ContinuousView::new()
        .add(s1)
        .x_range(0., num_of_samples as f64)
        .y_range(-20000., 20000.)
        .x_label("number")
        .y_label("real portion");

    Page::single(&v).save("scatter-real.svg").unwrap();
}

fn render_imag(fft_result: &Vec<Complex<f32>>) {
    let num_of_samples = fft_result.len();

    let indexed_samples = fft_result
        .iter()
        .enumerate()
        .map(|(i, s)| (i as f64, s.im as f64))
        .collect();

    let s1: Plot = Plot::new(indexed_samples).point_style(
        PointStyle::new()
            .marker(PointMarker::Circle)
            .size(0.3)
            .colour("#DD3355"),
    );

    let v = ContinuousView::new()
        .add(s1)
        .x_range(0., num_of_samples as f64)
        .y_range(-20000., 20000.)
        .x_label("number")
        .y_label("imaginary portion");

    Page::single(&v).save("scatter-imag.svg").unwrap();
}
