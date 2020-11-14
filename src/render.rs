use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

pub fn render_frames(indexed_frames: Vec<super::preprocess::NormalizedFrame>) {
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

pub fn render_samples(indexed_samples: Vec<super::preprocess::Sample>) {
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