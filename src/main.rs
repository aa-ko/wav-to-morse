use hound;

use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;

pub mod parser;

fn main() {
    println!("Moin!");

    try_find_beeps();
}

fn try_find_beeps() {
    const chuck_size: usize = 256;
    
    let samples = get_indexed_samples("samples/marc01.wav", 1);
    println!("Min: {}", samples.iter().map(|(_, s)| s).min().unwrap());
    println!("Max: {}", samples.iter().map(|(_, s)| s).max().unwrap());

    let amplitudes = samples.chunks(chuck_size).map(|c| avg_abs_amp(c));
    let mut i = 0;
    for a in amplitudes {
        println!("Chunk no. {} has avg amp of: {}", i, a);
        i += 1;
    }
}

fn avg_abs_amp(frame: &[Sample]) -> u32 {
    let sum: u128 = frame.iter().map(|(_, s)| s.abs() as u128).sum();
    (sum / frame.len() as u128) as u32
}

fn run_fft() {
    let mut reader = hound::WavReader::open("samples/marc01.wav").unwrap();
    let all_samples = reader.samples::<i16>().map(|s| s.unwrap() as f32);
    println!("Number of samples in file: {}", all_samples.len());

    let mut input: Vec<Complex<f32>> = all_samples.map(|f| Complex::new(f, 0.)).collect();
    let mut output: Vec<Complex<f32>> = vec![Complex::zero(); 249824];

    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(249824);
    fft.process(&mut input, &mut output);

    println!("Foo!");
}

fn render_scatter() {
    let all_samples = get_indexed_samples("samples/marc01.wav", 10);

    let num_of_samples = all_samples.len();

    let indexed_samples = all_samples
        .iter()
        .map(|(i, s)| (*i as f64, *s as f64))
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
        //.y_range(-262144., 262144.)
        .y_range(-128., 128.)
        .x_label("sample")
        .y_label("value");

    Page::single(&v).save("scatter.svg").unwrap();
}

type Sample = (u32, i32);
fn get_indexed_samples<'a>(filename: &str, resolution: u32) -> Vec<Sample> {
    let mut reader = hound::WavReader::open(filename).unwrap();
    let all_samples = reader.samples::<i32>();
    let number = all_samples.len() as u32;

    println!("Actual number of samples in file: {}", number);
    println!(
        "Filtered number of samples returned: {}",
        number / resolution
    );

    let temp = all_samples
        .enumerate()
        .map(|(i, s)| (i as u32, s.unwrap() as i32));

    if resolution > 1 {
        temp.filter(|(i, _)| i % resolution == 0).collect()
    } else {
        temp.collect()
    }
}
