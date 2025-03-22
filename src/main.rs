           /*      =^..^=       */
/////////////////////////////////////////////////
//                                             //
//                h o m a d o s                //
//       sound generator and test suite!       //
//                                             //
/////////////////////////////////////////////////

// Welcome to homados :)
//
// homados likes to make noise, and is always finding new ways to do it!
// Check out the README.md file for more information on that matter. From this point forward, 
// any information in code comments will aim to help you understand how homados works under the
// hood -- what it's doing and when, and some very brief remarks about creative decisions that
// were decided upon in its implementation.

use std::fs;
use std::path::Path;
use clap::Parser;
use hound;

mod generator;
mod window;

// We'll use clap to handle all our command line input logistics and set up some opinionated
// default values to make creating sound as simple as one wants it to be.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Output file path destination
    path: String,

    /// Output file name
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
    #[arg(short = 'd', long = "SoundDuration", required = false, value_name = "Positive Float", 
            default_value = "10.0", hide_default_value = true)]
    duration_seconds: f32,

    /// Base generator frequency
    #[arg(short = 'f', long = "BaseFrequency", required = false, value_name = "Float", 
            default_value = "440.0", hide_default_value = true)]
    freq_0: f32,

    /// Generator-Specific Parameter 1
    #[arg(long = "Param1", required = false, value_name = "Float", 
            default_value = "1.0", hide_default_value = true)]
    param_1: f32,

    /// Shape of the gain envelope / "fade window"
    #[arg(short = 'w', long = "WindowShape", required = false, value_name = "String", 
            default_value = "def", hide_default_value = true)]
    window: String,

    /// Modifier for fade window curve shape
    #[arg(long = "WindowCurve", required = false, value_name = "Float", 
            default_value = "2.0", hide_default_value = true)]
    window_k: f32,

    /// Gain scalar value
    #[arg(short = 'g', long = "Gain", required = false, value_name = "Float", 
            default_value = "1.0", hide_default_value = true, allow_hyphen_values = true)]
    gain: f32,
    
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

    // Scaling the volume should always be possible -- regardless of the window shape.
    // If the user doesn't give us a value though, we'll just assume no change in gain.
    if cli.gain.abs() > 1.0 
        {println!("\nWARNING: |Scalar| > 1.0\nThis may cause the output to clip.\n\n")}
    if cli.gain < 0.0
        {println!("\nWARNING: Scalar < 0.0\nThis will flip the signal polarity\n\n")}

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
    generator::create_sound(output_path, spec, &cli.sound_type, cli.duration_seconds, &cli.window, cli.window_k, cli.param_1, cli.freq_0, cli.gain, cli.verbose);
}