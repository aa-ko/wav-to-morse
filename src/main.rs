use hound;

use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;

#[macro_use]
extern crate clap;
use clap::App;

extern crate num_cpus;

pub mod parser;

fn main() {
    let config = parse_cli_arguments();
    println!("{}", config);

    match config.mode {
        Subcommand::Amp => {
            try_find_beeps(&config);
        }
        Subcommand::Fft => {
            run_fft(&config);
        }
        Subcommand::Render => {
            todo!("Not implemented yet.");
        }
    }
}

fn parse_cli_arguments() -> ComputationArguments {
    const DEFAULT_FRAMESIZE: usize = 128;
    const DEFAULT_QUANTIZATION_THRESHOLD: f64 = 0.5;
    const DEFAULT_SAMPLE_RESOLUTION: usize = 1;
    let default_threadcount: usize = num_cpus::get();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    ComputationArguments {
        mode: Subcommand::from_str(matches.subcommand_name()),
        input_file: matches.value_of("INPUT").unwrap().to_owned(),
        framesize: parse_argument(&matches, "framesize", DEFAULT_FRAMESIZE),
        quantization_threshold: parse_argument(
            &matches,
            "quantization-threshold",
            DEFAULT_QUANTIZATION_THRESHOLD,
        ),
        sample_resolution: parse_argument(&matches, "sample-resolution", DEFAULT_SAMPLE_RESOLUTION),
        threadcount: parse_argument(&matches, "threadcount", default_threadcount),
    }
}

fn parse_argument<T>(matches: &clap::ArgMatches, arg: &str, default: T) -> T
where
    T: std::str::FromStr,
{
    match matches.value_of(arg) {
        Some(arg) => match arg.parse() {
            Ok(res) => res,
            Err(_) => {
                println!("Cannot parse argument string '{}'.", arg);
                default
            }
        },
        None => default,
    }
}

// TODO: Split this into base arguments and subcommand arguments?
struct ComputationArguments {
    mode: Subcommand,
    input_file: String,
    framesize: usize,
    quantization_threshold: f64,
    sample_resolution: usize,
    threadcount: usize,
}

impl std::fmt::Display for ComputationArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComputationArguments")
            .field("Input file", &self.input_file)
            .field("Frame size", &self.framesize)
            .field("Quantization threshold", &self.quantization_threshold)
            .field("Sample resolution", &self.sample_resolution)
            .field("Thread count", &self.threadcount)
            .finish()
    }
}

// TODO: Include cli arguments?
enum Subcommand {
    Amp,
    Fft,
    Render
}

impl Subcommand {
    fn from_str(s: Option<&str>) -> Subcommand {
        if let Some(a) = s {
            match a {
                "amp" => Subcommand::Amp,
                "fft" => Subcommand::Fft,
                "render" => Subcommand::Render,
                _ => panic!("Unable to determine subcommand")
            }
        }
        else {
            panic!("Unable to determine subcommand")
        }
    }
}

fn try_find_beeps(config: &ComputationArguments) {
    let samples = get_indexed_samples(config.input_file.as_str(), config.sample_resolution);
    println!("Min: {}", samples.iter().map(|(_, s)| s).min().unwrap());
    println!("Max: {}", samples.iter().map(|(_, s)| s).max().unwrap());

    let amplitudes: Vec<Frame> = samples
        .chunks(config.framesize)
        .map(|c| avg_abs_amp(c))
        .enumerate()
        .collect();

    let normalized = normalize(amplitudes);
    let quantized_frames: Vec<bool> = normalized
        .iter()
        .map(|(_, v)| {
            if *v > config.quantization_threshold {
                true
            } else {
                false
            }
        })
        .collect();

    parser::translate(quantized_frames);
}

type Frame = (usize, u32);
fn avg_abs_amp(frame: &[Sample]) -> u32 {
    let sum: u128 = frame.iter().map(|(_, s)| s.abs() as u128).sum();
    (sum / frame.len() as u128) as u32
}

fn run_fft(config: &ComputationArguments) {
    let raw_samples = get_indexed_samples(config.input_file.as_str(), config.sample_resolution);
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

type NormalizedFrame = (usize, f64);
fn normalize(frames: Vec<Frame>) -> Vec<NormalizedFrame> {
    let min = frames.iter().map(|(_, s)| s).min().unwrap();
    let max = frames.iter().map(|(_, s)| s).max().unwrap();
    let div = (max - min) as f64;
    frames
        .iter()
        .map(|(i, v)| (*i, (v - min) as f64 / div))
        .collect()
}

fn render_frames(indexed_frames: Vec<NormalizedFrame>) {
    let num_of_samples = indexed_frames.len();

    let indexed_samples = indexed_frames
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
        .y_range(0., 1.)
        .x_label("frame_index")
        .y_label("normalized_value");

    Page::single(&v).save("scatter-frames.svg").unwrap();
}

fn render_samples(indexed_samples: Vec<Sample>) {
    let num_of_samples = indexed_samples.len();

    let indexed_samples = indexed_samples
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

    Page::single(&v).save("scatter-samples.svg").unwrap();
}

type Sample = (usize, i32);
fn get_indexed_samples(filename: &str, resolution: usize) -> Vec<Sample> {
    let mut reader = hound::WavReader::open(filename).unwrap();
    let all_samples = reader.samples::<i32>();
    let number = all_samples.len();

    println!("Actual number of samples in file: {}", number);
    println!(
        "Filtered number of samples returned: {}",
        number / resolution
    );

    let temp = all_samples.enumerate().map(|(i, s)| (i, s.unwrap() as i32));

    if resolution > 1 {
        temp.filter(|(i, _)| i % resolution == 0).collect()
    } else {
        temp.collect()
    }
}
