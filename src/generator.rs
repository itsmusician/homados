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

use std::{f32::consts::{PI, TAU}, path::Path};
use rand::prelude::*;
use rand_distr::{Bernoulli, Normal, Triangular};
use hound;
use crate::window;

// Container to hold state variables as needed
struct State {
    sv: [f32; 8]
}

pub fn create_sound(mut path: String, spec: hound::WavSpec, sound_type: &str, duration: f32, window_type: &str,
     window_k: f32, p1: f32, freq_0: f32, scalar: f32, verbose: bool) {
    // Let's initialize a state struct and set our spec parameters for hound.
    let mut state: State = State {sv: [0.0; 8]};

    // Before creating the wav we need to ensure the given sound and window types are valid.
    // This uses short dummy values to make sure we don't panic once we start writing the file.
    match_sound(sound_type, 44100.0,1, 1.0, p1, freq_0, &mut state);
    window::match_window(window_type, window_k, 1, 1.0);

    // Converting our duration to total samples will make our math much nicer.
    let duration_samples: u32 = (spec.sample_rate as f32 * duration) as u32;

    // This will be a scalar to adjust the output level depending on the bit depth. This value is
    // effectively our "1.0" signal value for the given bit depth when writing our output.
    let b: f32 = 2.0_f32.powf(spec.bits_per_sample as f32 - 1.0) - 1.0;

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
        let sound: f32 = match_sound(sound_type, spec.sample_rate as f32, duration_samples, t as f32, p1, freq_0, &mut state);
        let amplitude: f32 = window::match_window(window_type, window_k, duration_samples, t as f32);
        sample = scalar * amplitude * sound * b;
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

fn match_sound(sound_type: &str, s: f32, _sd: u32, x: f32, p1: f32, f: f32, state: &mut State) -> f32 {
    match sound_type {
        "silence" | "silent" | "zero" | "null" => 0.0,
        "dc" | "dc_offset" | "offset" | "constant" | "const" => 1.0,
        "sin" | "sine" | "sine_wave" | "sinusoid" => sine(x, s, f),
        "cos" | "cosine" | "cosine_wave" => cosine(x, s, f),
        /*
        !!! DISABLED -- SEE BELOW -- REENABLE SD ONCE UPDATED
        "sweep_linear" | "sweep_lin" | "sweep_linear_sin" | "sweep_lin_sin"
        | "sweep_linear_sine" | "sweep_lin_sine" => sweep_lin_sin(x, s, sd as f32),
        "sweep" | "sweep_log" | "sweep_sin" | "sweep_log_sin"
        | "sweep_sine" | "sweep_log_sine" => sweep_log_sin(x, sd as f32),
        */
        "quantized_sine" | "quantized_sin" | "quant_sine"
        | "quant_sin" => quant_sine(x, s, f),
        "saw" | "sawtooth" | "saw_wave" => saw(x, s, f),
        "sqr" | "square" | "square_wave" => square(x, s, f),
        "tri" | "triangle" | "triangle_wave" => triangle(x, s, f),
        "unit_impulse" | "dirac" | "delta" | "kronecker"
        | "dirac_delta" | "kronecker_delta" | "click" => unit_impulse(x),
        "dirac_comb" | "impulse_train" | "needle" 
        | "needle_pulse" | "sha" => dirac_comb(x, s, f),
        "random" | "noise" | "random_noise" => random_uniform(),
        "white" | "white_random" | "white_uniform" | "white_noise" => white_random_uniform(),
        "white_normal" | "white_random_normal" | "white_gaussian" | "white_random_gaussian" 
        | "white_gauss" | "gaussian_noise" => white_random_normal(p1),
        "white_tri" | "white_triangle" | "white_triangular" 
        | "triangular_noise" => white_random_tri(),
        "white_binary" | "white_bin" | "white_bernoulli" 
        | "binary_noise" | "bernoulli_noise" => white_random_bin(),
        "pink" | "pink_kellet_econ" | "pke" | "pink_noise" => pink_kellet_econ(state),
        "pink_ref" | "pink_kellet_ref" | "pk3" => pink_kellet_ref(state),
        "brown" | "red" | "brownian" | "brown_ema" | "brown_noise" => brown_ema(state, s),
        "blue" | "azure" | "blue_pke" | "blue_pke_ema" | "blue_noise" => blue_pke_ema(state, s),
        "blue_pk3" | "blue_pk3_ema" | "blue_ref" | "blue_ref_ema" => blue_pk3_ema(state, s),
        "violet" | "purple" | "violet_ema" | "violet_noise" 
        | "purple_noise" => violet_ema(state, s),
        "pseudo_velvet" | "pseudo_velvet_noise" 
        | "pseudo_velvet_consecutive" => pseudo_velvet_consecutive(p1),
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
fn sine(x: f32, s: f32, f: f32) -> f32 {
    (TAU * f * x / s).sin()
}

// Cosine (Constant frequency)
fn cosine(x: f32, s: f32, f: f32) -> f32 {
    (TAU * f * x / s).cos()
}

//
// !!! SUBOPTIMAL -- Needs phase accumulator implementations
//                   currently unstable when modulating freq

/*
// Sweep Linear (Sine)
fn sweep_lin_sin(x: f32, s: f32, sd: f32) -> f32 {
    let lo: f32 = 1234.0;
    let hi: f32 = 5678.0;
    (PI * (((hi - lo) * x / sd) + 2.0 * lo) * x / s).sin()
}

// Sweep Logarithmic (Sine)
fn sweep_log_sin(x: f32, sd: f32) -> f32 {
    let lo: f32 = 1234.0;
    let hi: f32 = 5678.0;
    let temp: f32 = (hi - lo + 1.0).powf(x / sd) + 2.0 * (lo - 1.0);
    (2.0 * PI * temp).sin()
}
*/

// Quantized Sine (Constant Frequency)
fn quant_sine(x: f32, s: f32, f: f32) -> f32 {
   let a: f32 = (2.0 * PI * f * x / s).sin();
   let b: f32 = 2.4;
   (a * (2.0 as f32).powf(b - 1.0)).round() / (2.0 as f32).powf(b - 1.0).round()
}
// - - - - - - - - - - - - - - - - - - - - - - -


//////-------------------------------------------
//////      Simple Waveshapes
//////-------------------------------------------
// Sawtooth (Constant frequency)
fn saw(x: f32, s: f32, f: f32) -> f32 {
    let a = x * f / s;
    2.0 * (a - (0.5 + a).floor())
}

// Square (Constant frequency)
fn square(x: f32, s: f32, f: f32) -> f32 {
    (-1.0 as f32).powf((2.0 * f * x / s).floor())
}

// Triangle (Constant frequency)
fn triangle(x: f32, s: f32, f: f32) -> f32 {
    let a = 0.25 + (x * f / s);
    (4.0 * (a - (a + 0.5).floor()).abs()) - 1.0
}
// - - - - - - - - - - - - - - - - - - - - - - -


//////-------------------------------------------
//////      Impulses
//////-------------------------------------------
// Unit Impulse //
// Unit Impulse
fn unit_impulse(x: f32) -> f32 {
    // a is a parameter which will offset the click in time.
    // There is no infrastructure yet to make this happen in a sensible way from the CLI.
    // Once that's there this will be nice and easy to implement :)
    let a = 0.0;
    if x == a { 1.0 } else { 0.0 }
}

// Dirac Comb (Constant frequency)
fn dirac_comb(x: f32, s: f32, f: f32) -> f32 {
    let a = x * f / s;
    ((a - a.floor()) < (f / s)) as i32 as f32
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
fn white_random_normal(a: f32) -> f32 {
    0.12499856588 * Normal::new(0.0, a).unwrap().sample(&mut rand::rng()) as f32
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
fn pink_kellet_econ(state: &mut State) -> f32 {
    let white: f32 = random_uniform();
    state.sv[0] = 0.99765 * state.sv[0] + white * 0.0990460;
    state.sv[1] = 0.96300 * state.sv[1] + white * 0.2965164;
    state.sv[2] = 0.57000 * state.sv[2] + white * 1.0526913;
    0.07263870048 * (state.sv[0] + state.sv[1] + state.sv[2] + (white * 0.1848))
}

// Pink Kellet Refined "pk3" (Rust rand crate default, filtered with Kellet refined method)
fn pink_kellet_ref(state: &mut State) -> f32 {
    let white: f32 = random_uniform();
    state.sv[0] = 0.99886 * state.sv[0] + white * 0.0555179;
    state.sv[1] = 0.99332 * state.sv[1] + white * 0.0750759;
    state.sv[2] = 0.96900 * state.sv[2] + white * 0.1538520;
    state.sv[3] = 0.86650 * state.sv[3] + white * 0.3104856;
    state.sv[4] = 0.55000 * state.sv[4] + white * 0.5329522;
    state.sv[5] = -0.7616 * state.sv[5] - white * 0.0168980;
    let pk3: f32 = state.sv[0] + state.sv[1] + state.sv[2] + state.sv[3]
                    + state.sv[4] + state.sv[5] + state.sv[6] + white * 0.5362;
    state.sv[6] = white * 0.115926;
    0.07093071735 * pk3
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Brown Noise //
// Brown Noise (Rust rand crate default, filtered 6dB/oct EMA low pass)
fn brown_ema(state: &mut State, s: f32) -> f32 {
    let a: f32 = 20.0 / (s * 0.5);
    state.sv[0] = a * random_uniform() + (1.0 - a) * state.sv[0];
    10.6143507417 * state.sv[0]
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Blue Noise //
// Blue PKE_EMA (Rust rand crate default, filtered w/ Kellet econ, then 6dB/oct EMA high pass)
fn blue_pke_ema(state: &mut State, s: f32) -> f32 {
    let a: f32 = 20000.0 / (s * 0.5);
    let white: f32 = random_uniform();

    // Kellet Econ
    state.sv[0] = 0.99765 * state.sv[0] + white * 0.0990460;
    state.sv[1] = 0.96300 * state.sv[1] + white * 0.2965164;
    state.sv[2] = 0.57000 * state.sv[2] + white * 1.0526913;
    let pke = 0.14 * (state.sv[0] + state.sv[1] + state.sv[2] + white * 0.1848);

    // EMA high pass
    state.sv[4] = a * pke + (1.0 - a) * state.sv[4];
    5.36339784311 * (pke - state.sv[4])
}

// Blue PK3_EMA (Rust rand crate default, filtered with Kellet ref, then 6dB/oct EMA high pass)
fn blue_pk3_ema(state: &mut State, s: f32) -> f32 {
    let a: f32 = 20000.0 / (s * 0.5);
    let white: f32 = random_uniform();

    // Kellet Ref
    state.sv[0] = 0.99886 * state.sv[0] + white * 0.0555179;
    state.sv[1] = 0.99332 * state.sv[1] + white * 0.0750759;
    state.sv[2] = 0.96900 * state.sv[2] + white * 0.1538520;
    state.sv[3] = 0.86650 * state.sv[3] + white * 0.3104856;
    state.sv[4] = 0.55000 * state.sv[4] + white * 0.5329522;
    state.sv[5] = -0.7616 * state.sv[5] - white * 0.0168980;
    let pk3: f32 = state.sv[0] + state.sv[1] + state.sv[2] + state.sv[3]
                    + state.sv[4] + state.sv[5] + state.sv[6] + white * 0.5362;
    state.sv[6] = white * 0.115926;

    // EMA high pass
    state.sv[7] = a * pk3 + (1.0 - a) * state.sv[7];
    0.74195281187 * (pk3 - state.sv[7])
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Violet Noise //
// Violet Noise (Rust rand crate default, filtered 6dB/oct EMA high pass)
fn violet_ema(state: &mut State, s: f32) -> f32 {
    let a: f32 = 20000.0 / (s * 0.5);
    let white: f32 = random_uniform();
    state.sv[0] = a * white + (1.0 - a) * state.sv[0];
    0.99206475709 * (white - state.sv[0])
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Velvet Noise //
// Velvet Round (Rust rand crate default, threshold to 0 between abs() our sparsity parameter)
fn pseudo_velvet_consecutive(a: f32) -> f32 {
    let rand = random_uniform();
    if rand.abs() < a {
        0.0
    }
    else {
        rand.signum()
    }
}
// - - - - - - - - - - - - - - - - - - - - - - -