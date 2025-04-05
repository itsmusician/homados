           /*      =^..^=       */
/////////////////////////////////////////////////
//                                             //
//    T H E   G R E A T   G E N E R A T O R    //
//              O F   N O I S E                //
//                                             //
/////////////////////////////////////////////////

// References:
// [1] The Paul Kellet filtering methods used for our pink (and blue) noise were retrieved from here:
//  - https://www.firstpr.com.au/dsp/pink-noise/

use std::{f32::consts::TAU, path::Path};
use rand::prelude::*;
use rand_distr::{Bernoulli, Normal, Triangular};
use hound;
use crate::window;

// Generic container to store information about our sound type
pub struct Sound {
    pub sv: [f32; 8],
    pub freq: f32,
    pub freq_min: f32,
    pub freq_max: f32,
    pub offset: f32,
    pub p1: f32,
}

pub fn create_sound(mut path: String, spec: hound::WavSpec, mut sound: Sound, sound_type: &str, duration: f32, window_type: &str,
     window_k: f32, scalar: f32, verbose: bool) {
    // Before creating the wav we need to ensure the given sound and window types are valid.
    // This uses short dummy values to make sure we don't panic once we start writing the file.
    match_sound(sound_type, 44100.0,2, 1.0, &mut sound);
    window::match_window(window_type, window_k, 1, 1.0);

    // Converting our duration to total samples will make our math much nicer.
    let duration_samples: u32 = (spec.sample_rate as f32 * duration) as u32;

    // This will be a scalar to adjust the output level depending on the bit depth. This value is
    // effectively our "1.0" signal value for the given bit depth when writing our output.
    let b: f32 = 2.0f32.powf(spec.bits_per_sample as f32 - 1.0) - 1.0;

    // Before we create the wav, let's make sure there's not an existing file with the same name.
    // If there is, increment on the name until a suitable alternative is found. 
    if Path::new((path.clone() + ".wav").as_str()).exists() == true {
        let mut i = 1;
        let mut exists = true;
        while exists == true {
            if Path::new((path.clone() + " (" + &i.to_string() + ").wav").as_str()).exists() == true {
                i += 1;
            } else {
                path = path.clone() + " (" + &i.to_string() + ")";
                exists = false;
            }
        }
    }

    // Here we calculate the output from our window, generator, and gain values (multiply them),
    // then clip the output if the magnitude > 1.0 to avoid any errors on writing.
    let mut writer = hound::WavWriter::create({path.clone()}+".wav", spec).unwrap(); 
    let mut sample: f32;
    for t in 0 .. duration_samples {
        let source: f32 = match_sound(sound_type, spec.sample_rate as f32, duration_samples, t as f32, &mut sound);
        let amplitude: f32 = window::match_window(window_type, window_k, duration_samples, t as f32);
        sample = scalar * amplitude * source * b;
        if sample.abs() > b {sample = sample.signum() * b}
        writer.write_sample(sample as i32).unwrap();
    }

    // Output information about the newly created file for verbose output
    if verbose == true {
        println!("\n\u{1F388}\u{1F388}\u{1F388} !!!!! YAY !!!!! \u{1F388}\u{1F388}\u{1F388}\n");
        println!("Sound Type: \t{}", sound_type);
        println!("Channels: \t{}", spec.channels);
        println!("Sample Rate: \t{}", spec.sample_rate);
        println!("Bit Depth: \t{}", spec.bits_per_sample);
        println!("Duration:\n    Seconds:    {}\n    Samples:    {}", duration, duration_samples);
        println!("Gain Window: \t{}", window_type);
        println!("Gain Scalar: \t{:.1}", scalar);
        println!("\nFile Successfully created at:\nfile://{}.wav", path.clone());
    }
}

fn match_sound(sound_type: &str, s: f32, sd: u32, x: f32, sound: &mut Sound) -> f32 {
    let d: f32 = sd as f32 - 1.0;
    match sound_type {
        "silence" | "silent" | "zero" | "null" => 0.0,
        "dc" | "dc_offset" | "offset" | "constant" | "const" => 1.0,
        "sin" | "sine" | "sine_wave" | "sinusoid" => sine(s, sound),
        "cos" | "cosine" | "cosine_wave" => cosine(s, sound),
        "sweep_lin" | "sweep_lin_sin" | "sweep_linear" | "chirp_lin" 
        | "chirp_linear" | "lss" => sweep_lin_sin(x, s, d, sound),
        "sweep" | "sweep_log" | "sweep_sin" | "sweep_log_sin" | "chirp" | "chirp_log" | 
        "chirp_exp" | "ess" | "sweep_exp" | "sweep_exp_sin" | "log_sweep" 
        | "exp_sweep" => sweep_exp_sin(x, s, d, sound),
        "clip_sine" | "clipped_sine" | "hardclip_sine" | "hard_clip_sine" | 
        "hardclipped_sine" | "hard_clipped_sine" => clip_sine(s, sound),
        "quantized_sine" | "quantized_sin" | "quant_sine"
        | "quant_sin" => quant_sine(s, sound),
        "saw" | "sawtooth" | "saw_wave" => saw(s, sound),
        "sqr" | "square" | "square_wave" => square(s, sound),
        "tri" | "triangle" | "triangle_wave" => triangle(s, sound),
        "pwm" | "pulse" | "pulse_wave" => pulse(s, sound),
        "shark" | "sharktooth" | "sharktooth_wave" => sharktooth(s, sound),
        "unit_impulse" | "dirac" | "delta" | "kronecker"
        | "dirac_delta" | "kronecker_delta" | "click" => unit_impulse(x, sound),
        "dirac_comb" | "impulse_train" | "needle" | "comb" 
        | "needle_pulse" | "sha" => dirac_comb(s, sound),
        "random" | "noise" | "random_noise" => random_uniform(),
        "white" | "white_random" | "white_uniform" | "white_noise" => white_random_uniform(),
        "white_normal" | "white_random_normal" | "white_gaussian" | "white_random_gaussian" 
        | "white_gauss" | "gaussian_noise" => white_random_normal(sound),
        "white_tri" | "white_triangle" | "white_triangular" 
        | "triangular_noise" => white_random_tri(),
        "white_binary" | "white_bin" | "white_bernoulli" 
        | "binary_noise" | "bernoulli_noise" => white_random_bin(),
        "pink" | "pink_kellet_econ" | "pke" | "pink_noise" => pink_kellet_econ(sound),
        "pink_ref" | "pink_kellet_ref" | "pk3" => pink_kellet_ref(sound),
        "brown" | "red" | "brownian" | "brown_ema" | "brown_noise" => brown_ema(s, sound),
        "blue" | "azure" | "blue_pke" | "blue_pke_ema" | "blue_noise" => blue_pke_ema(s, sound),
        "blue_pk3" | "blue_pk3_ema" | "blue_ref" | "blue_ref_ema" => blue_pk3_ema(s, sound),
        "violet" | "purple" | "violet_ema" | "violet_noise" 
        | "purple_noise" => violet_ema(s, sound),
        "pseudo_velvet" | "pseudo_velvet_noise" 
        | "pseudo_velvet_consecutive" => pseudo_velvet_consecutive(sound),
        _ => panic!("\n\n\tError:\tUnrecognized sound type.\n\n")
    }
}



           /*      =^..^=       */
/////////////////////////////////////////////////
//       S I G N A L   F U N C T I O N S       //
/////////////////////////////////////////////////

//////-------------------------------------------
//////     Sinusoidal
//////-------------------------------------------
// Sine (Constant frequency)
fn sine(s: f32, sound: &mut Sound) -> f32 {
    let phase = sound.sv[0];
    sound.sv[0] = (sound.sv[0] + (sound.freq / s)) % 1.0;
    (phase * TAU).sin()
}

// Cosine (Constant frequency)
fn cosine(s: f32, sound: &mut Sound) -> f32 {
    let phase = sound.sv[0];
    sound.sv[0] = (sound.sv[0] + (sound.freq / s)) % 1.0;
    (phase * TAU).cos()
}

// Sweep Linear (Sine)
fn sweep_lin_sin(x: f32, s: f32, d: f32, sound: &mut Sound) -> f32 {
    let phase = sound.sv[0];
    sound.sv[0] = (sound.sv[0] + ((sound.freq_min + ((sound.freq_max - sound.freq_min) * x / d)) / s)) % 1.0;
    (phase * TAU).sin()
}

// Sweep Exponential / Log (Sine)
fn sweep_exp_sin(x: f32, s: f32, d: f32, sound: &mut Sound) -> f32 {
    let phase = sound.sv[0];
    sound.sv[0] = (sound.sv[0] + (10.0f32.powf(sound.freq_min.log10() + ((sound.freq_max.log10() - sound.freq_min.log10()) * x / d)) / s)) % 1.0;
    (phase * TAU).sin()
}

// Clipped Sine
fn clip_sine(s: f32, sound: &mut Sound) -> f32 {
    let phase = sound.sv[0];
    sound.sv[0] = (sound.sv[0] + (sound.freq / s)) % 1.0;
    let out = (phase * TAU).sin();
    if out.abs() > sound.p1 { sound.p1 * out.signum() } else { out }
}

// Quantized Sine (Constant Frequency)
fn quant_sine(s: f32, sound: &mut Sound) -> f32 {
    let phase = sound.sv[0];
    sound.sv[0] = (sound.sv[0] + (sound.freq / s)) % 1.0;
    let a = (phase * TAU).sin();
    (a * (2.0f32).powf(sound.p1 - 1.0)).round() / (2.0f32).powf(sound.p1 - 1.0).round()
}
// - - - - - - - - - - - - - - - - - - - - - - -


//////-------------------------------------------
//////      Simple Waveshapes
//////-------------------------------------------
// Sawtooth (Constant frequency)
fn saw(s: f32, sound: &mut Sound) -> f32 {
    let phase = sound.sv[0];
    sound.sv[0] = (sound.sv[0] + (sound.freq / s)) % 1.0;
    2.0 * (phase - (0.5 + phase).floor())
}

// Square (Constant frequency)
fn square(s: f32, sound: &mut Sound) -> f32 {
    let phase = sound.sv[0];
    sound.sv[0] = (sound.sv[0] + (sound.freq / s)) % 1.0;
    (-1.0 as f32).powf((2.0 * phase).floor())
}

// Triangle (Constant frequency)
fn triangle(s: f32, sound: &mut Sound) -> f32 {
    let phase = sound.sv[0];
    let a = 0.25 + phase;
    sound.sv[0] = (sound.sv[0] + (sound.freq / s)) % 1.0;
    (4.0 * (a - (a + 0.5).floor()).abs()) - 1.0
}

// Pulse (Constant frequency, width)
fn pulse(s: f32, sound: &mut Sound) -> f32 {
    let phase = sound.sv[0];
    sound.sv[0] = (sound.sv[0] + (sound.freq / s)) % 1.0;
    2.0 * (((phase - phase.floor()) < sound.p1) as i32 as f32) - 1.0
}

// Sharktooth (Constant frequency)
fn sharktooth(s: f32, sound: &mut Sound) -> f32 {
    let phase = sound.sv[0];
    let phase_2 = phase * 2.0;
    let a = 0.25 + phase;
    sound.sv[0] = (sound.sv[0] + (sound.freq / s)) % 1.0;
    (3.0 * (a - (a + 0.5).floor()).abs()) + (0.5 * (phase_2 + 0.5).floor()) - phase - 0.75
}
// - - - - - - - - - - - - - - - - - - - - - - -


//////-------------------------------------------
//////      Impulses
//////-------------------------------------------
// Unit Impulse //
// Unit Impulse
fn unit_impulse(x: f32, sound: &mut Sound) -> f32 {
    if x == sound.offset { 1.0 } else { 0.0 }
}

// Dirac Comb (Constant frequency)
fn dirac_comb(s: f32, sound: &mut Sound) -> f32 {
    let phase = sound.sv[0];
    sound.sv[0] = (sound.sv[0] + (sound.freq / s)) % 1.0;
    ((phase - phase.floor()) < (sound.freq / s)) as i32 as f32
}
// - - - - - - - - - - - - - - - - - - - - - - -


//////-------------------------------------------
//////      Noise
//////-------------------------------------------
// Random Noise //
// Random Uniform (Rust rand crate default -- ChaCha 12 round, StandardUniform distribution)
fn random_uniform() -> f32 {
    rand::random_range(-1.0..=1.0)
}

// White Noise //
// White Random Uniform (Rust rand crate Uniform distribution, scaled)
fn white_random_uniform() -> f32 {
    0.21646117788 * random_uniform()
}

// White Random Normal / Gaussian (Rust rand_distr crate Normal distribution, scaled)
fn white_random_normal(sound: &mut Sound) -> f32 {
    0.12499856588 * Normal::new(0.0, sound.p1).unwrap().sample(&mut rand::rng()) as f32
}

// White Random Triangular (Rust rand_distr crate Triangular distribution, scaled)
fn white_random_tri() -> f32 {
    0.30616465062 * Triangular::new(-1.0, 1.0, 0.0).unwrap().sample(&mut rand::rng()) as f32
}

// White Random Bernoulli / Binary (Rust rand crate bernoulli distribution, scaled)
fn white_random_bin() -> f32 {
    0.25 * (Bernoulli::new(0.5).unwrap().sample(&mut rand::rng()) as i32 as f32 - 0.5)
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Pink Noise //
// Pink Kellet Econ "pke" (Rust rand crate default, filtered with Kellet econ method)
fn pink_kellet_econ(sound: &mut Sound) -> f32 {
    let white: f32 = random_uniform();
    sound.sv[0] = 0.99765 * sound.sv[0] + white * 0.0990460;
    sound.sv[1] = 0.96300 * sound.sv[1] + white * 0.2965164;
    sound.sv[2] = 0.57000 * sound.sv[2] + white * 1.0526913;
    0.07263870048 * (sound.sv[0] + sound.sv[1] + sound.sv[2] + (white * 0.1848))
}

// Pink Kellet Refined "pk3" (Rust rand crate default, filtered with Kellet refined method)
fn pink_kellet_ref(sound: &mut Sound) -> f32 {
    let white: f32 = random_uniform();
    sound.sv[0] = 0.99886 * sound.sv[0] + white * 0.0555179;
    sound.sv[1] = 0.99332 * sound.sv[1] + white * 0.0750759;
    sound.sv[2] = 0.96900 * sound.sv[2] + white * 0.1538520;
    sound.sv[3] = 0.86650 * sound.sv[3] + white * 0.3104856;
    sound.sv[4] = 0.55000 * sound.sv[4] + white * 0.5329522;
    sound.sv[5] = -0.7616 * sound.sv[5] - white * 0.0168980;
    let pk3: f32 = sound.sv[0] + sound.sv[1] + sound.sv[2] + sound.sv[3]
                    + sound.sv[4] + sound.sv[5] + sound.sv[6] + white * 0.5362;
    sound.sv[6] = white * 0.115926;
    0.07093071735 * pk3
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Brown Noise //
// Brown Noise (Rust rand crate default, filtered 6dB/oct EMA low pass)
fn brown_ema(s: f32, sound: &mut Sound) -> f32 {
    let a: f32 = 20.0 / (s * 0.5);
    sound.sv[0] = a * random_uniform() + (1.0 - a) * sound.sv[0];
    10.6143507417 * sound.sv[0]
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Blue Noise //
// Blue PKE_EMA (Rust rand crate default, filtered w/ Kellet econ, then 6dB/oct EMA high pass)
fn blue_pke_ema(s: f32, sound: &mut Sound) -> f32 {
    let a: f32 = 20000.0 / (s * 0.5);
    let white: f32 = random_uniform();

    // Kellet Econ
    sound.sv[0] = 0.99765 * sound.sv[0] + white * 0.0990460;
    sound.sv[1] = 0.96300 * sound.sv[1] + white * 0.2965164;
    sound.sv[2] = 0.57000 * sound.sv[2] + white * 1.0526913;
    let pke = 0.14 * (sound.sv[0] + sound.sv[1] + sound.sv[2] + white * 0.1848);

    // EMA high pass
    sound.sv[4] = a * pke + (1.0 - a) * sound.sv[4];
    5.36339784311 * (pke - sound.sv[4])
}

// Blue PK3_EMA (Rust rand crate default, filtered with Kellet ref, then 6dB/oct EMA high pass)
fn blue_pk3_ema(s: f32, sound: &mut Sound) -> f32 {
    let a: f32 = 20000.0 / (s * 0.5);
    let white: f32 = random_uniform();

    // Kellet Ref
    sound.sv[0] = 0.99886 * sound.sv[0] + white * 0.0555179;
    sound.sv[1] = 0.99332 * sound.sv[1] + white * 0.0750759;
    sound.sv[2] = 0.96900 * sound.sv[2] + white * 0.1538520;
    sound.sv[3] = 0.86650 * sound.sv[3] + white * 0.3104856;
    sound.sv[4] = 0.55000 * sound.sv[4] + white * 0.5329522;
    sound.sv[5] = -0.7616 * sound.sv[5] - white * 0.0168980;
    let pk3: f32 = sound.sv[0] + sound.sv[1] + sound.sv[2] + sound.sv[3]
                    + sound.sv[4] + sound.sv[5] + sound.sv[6] + white * 0.5362;
    sound.sv[6] = white * 0.115926;

    // EMA high pass
    sound.sv[7] = a * pk3 + (1.0 - a) * sound.sv[7];
    0.74195281187 * (pk3 - sound.sv[7])
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Violet Noise //
// Violet Noise (Rust rand crate default, filtered 6dB/oct EMA high pass)
fn violet_ema(s: f32, sound: &mut Sound) -> f32 {
    let a: f32 = 20000.0 / (s * 0.5);
    let white: f32 = random_uniform();
    sound.sv[0] = a * white + (1.0 - a) * sound.sv[0];
    0.99206475709 * (white - sound.sv[0])
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Velvet Noise //
// Velvet Round (Rust rand crate default, threshold to 0 between abs() our sparsity parameter)
fn pseudo_velvet_consecutive(sound: &mut Sound) -> f32 {
    let rand = random_uniform();
    if rand.abs() < sound.p1 {
        0.0
    }
    else {
        rand.signum()
    }
}
// - - - - - - - - - - - - - - - - - - - - - - -
