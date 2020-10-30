use hound;

use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

pub mod parser;

fn main() {
    println!("Moin!");

    //render_scatter();

    morse_to_ascii();
}

fn morse_to_ascii() {
    println!("A: {}", parser::to_ascii(vec![parser::MorseToken::Short, parser::MorseToken::Long]).unwrap());
}

fn render_scatter() {
    let mut reader = hound::WavReader::open("samples/marc01.wav").unwrap();
    let all_samples = reader.samples::<i16>();
    println!("Number of samples in file: {}", all_samples.len());

    let indexed_samples = all_samples
        .enumerate()
        .map(|(i, s)| (i as f64, s.unwrap() as f64))
        .collect();
    
    let s1: Plot = Plot::new(indexed_samples).point_style(
        PointStyle::new()
            .marker(PointMarker::Circle)
            .size(0.1)
            .colour("#DD3355"),
    );

    let v = ContinuousView::new()
        .add(s1)
        .x_range(0., 250000.)
        .y_range(-130., 130.)
        .x_label("index")
        .y_label("value");

    Page::single(&v).save("scatter.svg").unwrap();
}