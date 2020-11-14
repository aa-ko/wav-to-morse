use clap::App;

pub fn parse_cli_arguments() -> ComputationArguments {
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
pub struct ComputationArguments {
    pub mode: Subcommand,
    pub input_file: String,
    pub framesize: usize,
    pub quantization_threshold: f64,
    pub sample_resolution: usize,
    pub threadcount: usize,
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
pub enum Subcommand {
    Amp,
    Fft,
    Render,
}

impl Subcommand {
    fn from_str(s: Option<&str>) -> Subcommand {
        if let Some(a) = s {
            match a {
                "amp" => Subcommand::Amp,
                "fft" => Subcommand::Fft,
                "render" => Subcommand::Render,
                _ => panic!("Invalid subcommand."),
            }
        } else {
            panic!("No subcommand found.")
        }
    }
}