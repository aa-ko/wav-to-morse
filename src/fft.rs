use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;

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

    for r in output {
        println!("{}", r);
    }
}