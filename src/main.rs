           /*      =^..^=       */
/////////////////////////////////////////////////
//                                             //
//                h o m a d o s                //
//          Signal should be simple.           //
//                                             //
/////////////////////////////////////////////////

// Welcome to homados :)

use std::fs;
use std::path::Path;
use clap::Parser;
use hound;

mod generator;
mod window;

// Convert dBFS value to amplitude
fn dbfs_to_amp(input: f64) -> f64 {
    10.0f64.powf(input / 20.0)
}

// We'll use clap to handle all our command line input logistics and set up some opinionated
// default values to make creating sound as simple as one wants it to be.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Output file path destination
    #[arg(default_value = "./homados Output", hide_default_value = true)]
    path: String,

    /// Output file name
    #[arg(default_value = "homados_output", hide_default_value = true)]
    name: String,

    /// Sample Rate
    #[arg(short = 's', long = "SampleRate", required = false, value_name = "Positive Int", 
            default_value = "48000", hide_default_value = true)]
    rate: u32,

    /// Bit Depth
    #[arg(short = 'b', long = "BitDepth", required = false, value_name = "Positive Int", 
            default_value = "24", hide_default_value = true)]
    bitdepth: u16,

    /// Channel Count
    #[arg(short = 'c', long = "ChannelCount", required = false, value_name = "Positive Int", 
            default_value = "1", hide_default_value = true)]
    channels: u16,
    
    /// Type of sound
    #[arg(short = 't', long = "SoundType", required = false, value_name = "String", 
            default_value = "white", hide_default_value = true)]
    sound_type: String,

    /// Duration of sound in seconds
    #[arg(short = 'd', long = "DurationSeconds", required = false, value_name = "Positive Float", 
            default_value = "10.0", hide_default_value = true)]
    duration_seconds: f64,

    /// Duration of sound in samples
    #[arg(short = 'D', long = "DurationSamples", required = false, value_name = "Positive Float", 
            default_value = "480000", hide_default_value = true)]
    duration_samples: f64,

    /// Base frequency
    #[arg(short = 'f', long = "BaseFrequency", required = false, value_name = "Float", 
            default_value = "440.0", hide_default_value = true)]
    freq: f64,

    /// Minimum frequency
    #[arg(long = "MinFrequency", required = false, value_name = "Float", 
            default_value = "20.0", hide_default_value = true)]
    freq_min: f64,

    /// Maximum frequency
    #[arg(long = "MaxFrequency", required = false, value_name = "Float", 
            default_value = "20000.0", hide_default_value = true)]
    freq_max: f64,

    /// Time offset in seconds
    #[arg(short = 'o', long = "Offset", required = false, value_name = "Float",
            allow_hyphen_values = true, number_of_values = 1, default_value = "0.0", 
            hide_default_value = true)]
    offset: f64,

    /// Generator-Specific Parameter 1
    #[arg(short = 'p', long = "p1", required = false, value_name = "Float",  
            default_value = "1.0", hide_default_value = true)]
    param_1: f64,

    /// Generator-Specific Parameter 1 as dBFS value
    #[arg(long = "p1dB", required = false, value_name = "Float", allow_hyphen_values = true,  
            number_of_values = 1, default_value = "0.0", hide_default_value = true)]
    param_1_db: f64,

    /// Generator-Specific Parameter 2
    #[arg(long = "p2", required = false, value_name = "Float",  
            default_value = "1.0", hide_default_value = true)]
    param_2: f64,

    /// Generator-Specific Parameter 2 as dBFS value
    #[arg(long = "p2dB", required = false, value_name = "Float", allow_hyphen_values = true,  
            number_of_values = 1, default_value = "0.0", hide_default_value = true)]
    param_2_db: f64,

    /// Shape of the gain envelope / "fade window"
    #[arg(short = 'w', long = "WindowShape", required = false, value_name = "String", 
            default_value = "def", hide_default_value = true)]
    window: String,

    /// Modifier for fade window curve shape
    #[arg(long = "WindowCurve", required = false, value_name = "Float", 
            default_value = "2.0", hide_default_value = true)]
    window_k: f64,

    /// Gain scalar
    #[arg(short = 'g', long = "Gain", required = false, value_name = "Float", 
            default_value = "1.0", hide_default_value = true, allow_hyphen_values = true)]
    gain: f64,

    /// Gain scalar as dBFS value
    #[arg(short = 'G', long = "GaindB", required = false, value_name = "Float", 
            default_value = "0.0", allow_hyphen_values = true, number_of_values = 1, 
            hide_default_value = true)]
    gain_db: f64,
    
    /// Display verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut spec = hound::WavSpec {
        channels: 1,
        sample_rate: 48000,
        bits_per_sample: 24,
        sample_format: hound::SampleFormat::Int,
    };

    let sound = generator::Sound {
        sv: [0.0; 8],
        freq: cli.freq.clone(),
        freq_min: cli.freq_min.clone(),
        freq_max: cli.freq_max.clone(),
        offset: cli.rate.clone() as f64 * cli.offset.clone(),
        p1: 
            if cli.param_1_db.clone() != 0.0 {
                dbfs_to_amp(cli.param_1_db.clone())
            } else {
                cli.param_1.clone()
            },
        p2: 
            if cli.param_2_db.clone() != 0.0 {
                dbfs_to_amp(cli.param_2_db.clone())
            } else {
                cli.param_2.clone()
            },
    };

    // We only need to pass in one gain scalar. If both unit types are provided, dB will triumph.
    let mut gain = cli.gain;
    if cli.gain_db != 0.0 {gain = dbfs_to_amp(cli.gain_db)};

    // Scaling the volume should always be possible -- regardless of the window shape.
    // Warn the user if the given gain value may cause any issues.
    if gain.abs() > 1.0 && cli.gain_db == 0.0
        {println!("\nWARNING: |Scalar| > 1.0 Amplitude\nThis may cause the output to clip.\n\n")}
    if gain < 0.0 && cli.gain_db == 0.0
        {println!("\nWARNING: Scalar < 0.0 Amplitude\nThis will flip the signal polarity\n\n")}
    if cli.gain_db > 0.0 
        {println!("\nWARNING: Scalar > 0.0dBFS\nThis may cause the output to clip.\n\n")}

    // We only need to pass in one duration. If both unit types are provided, samples triumphs.
    let mut duration = cli.duration_samples;
    if cli.duration_samples == 480000.0 && cli.duration_seconds != 10.0 {
        duration = cli.duration_seconds * cli.rate as f64;
    }

    // We will create the directory for our output in case it does not already exist.
    if Path::new(cli.path.clone().as_str()).exists() == false {
        fs::create_dir_all(cli.path.clone()).expect("\n\nError occurred creating output\n\n");
    }

    // Ensure that our output path distinguishes the file name from its target directory
    let mut output_path: String = cli.path.clone();
    if cli.path.chars().last().unwrap() == '/' {output_path += &cli.name}
    else {output_path += &("/".to_owned() + &cli.name)}

    // Populate the sound specifications to pass to the generator
    spec.channels = cli.channels;
    spec.bits_per_sample = cli.bitdepth;
    spec.sample_rate = cli.rate;
    
    // Now call the appropriate sound generating function.
    generator::create_sound(output_path, spec, sound, &cli.sound_type, duration, &cli.window, cli.window_k, gain, cli.verbose);
}