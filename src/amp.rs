pub fn try_find_beeps(config: &super::config::ComputationArguments) {
    let samples = super::preprocess::get_indexed_samples(config.input_file.as_str(), config.sample_resolution);
    println!("Min: {}", samples.iter().map(|(_, s)| s).min().unwrap());
    println!("Max: {}", samples.iter().map(|(_, s)| s).max().unwrap());

    let amplitudes: Vec<super::preprocess::Frame> = samples
        .chunks(config.framesize)
        .map(|c| avg_abs_amp(c))
        .enumerate()
        .collect();

    let normalized = super::preprocess::normalize_frames(amplitudes);
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

        super::ascii::translate(quantized_frames);
}

pub fn avg_abs_amp(frame: &[super::preprocess::Sample]) -> u32 {
    let sum: u128 = frame.iter().map(|(_, s)| s.abs() as u128).sum();
    (sum / frame.len() as u128) as u32
}