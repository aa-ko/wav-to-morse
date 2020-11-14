#[macro_use]
extern crate clap;
extern crate num_cpus;

mod amp;
mod fft;
mod config;
mod preprocess;
mod ascii;

fn main() {
    let config = config::parse_cli_arguments();
    println!("{}", config);

    match config.mode {
        config::Subcommand::Amp => {
            amp::try_find_beeps(&config);
        }
        config::Subcommand::Fft => {
            fft::run_fft(&config);
        }
        config::Subcommand::Render => {
            todo!("Not implemented yet.");
        }
    }
}
