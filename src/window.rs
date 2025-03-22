           /*      =^..^=       */
/////////////////////////////////////////////////
//                                             //
//    T H E   G R E A T   W I N D O W I N G    //
//               H A N D L E R                 //
//                                             //
/////////////////////////////////////////////////

// References:
// [2] The Signalsmith crossfade curve functions were derived and retrieved from here:
//  - https://signalsmith-audio.co.uk/writing/2021/cheap-energy-crossfade/#fractional-power-law

// All the window / fade functions here use scalars with a range of 0 to 1 for simplicity's sake.
// We need to make sure that they are scaled correctly when we apply them to our signal though...
// We also have some parameters specific to each category of window shape that could change.

use std::f32::consts::{E, TAU, PI, FRAC_PI_2};

// Duration here is seen as sd - 1, as we wish to guarantee that the initial and final sample
// scalars are the actual intended endpoint values (0 or 1) of our functions. If we don't do this,
// the functions may reach their target value 1 sample "after" the end of the output, which will
// cause issues in a variety of scenarios.
pub fn match_window(window: &str, k: f32, sd: u32, x: f32) -> f32 {
    let d: f32 = sd as f32 - 1.0;
    match window {
        "default" | "def" | "flat" | "unity" | "full" | "none" | "constant" | "const" => 1.0,
        "linear_out" | "lin_out" => linear_out(d, x),
        "linear_in" | "lin_in" => linear_in(d, x),
        "linear_io" | "lin_io" => linear_io(d, x),
        "linear_oi" | "lin_oi" => linear_oi(d, x),
        "exp1_out" | "exp_out" => exp1_out(d, x, k),
        "exp1_in" | "exp_in" => exp1_in(d, x, k),
        "exp1_io" | "exp_io" => exp1_io(d, x, k),
        "exp1_oi" | "exp_oi" => exp1_oi(d, x, k),
        "exp2_out" => exp2_out(d, x),
        "exp2_in" => exp2_in(d, x),
        "exp2_io" => exp2_io(d, x),
        "exp2_oi" => exp2_oi(d, x),
        "exp3_out" => exp3_out(d, x, k),
        "exp3_in" => exp3_in(d, x, k),
        "exp3_io" => exp3_io(d, x, k),
        "exp3_oi" => exp3_oi(d, x, k),
        "exp4_out" => exp4_out(d, x, k),
        "exp4_in" => exp4_in(d, x, k),
        "exp4_io" => exp4_io(d, x, k),
        "exp4_oi" => exp4_oi(d, x, k),
        "exp5_out" => exp5_out(d, x),
        "exp5_in" => exp5_in(d, x),
        "exp5_io" => exp5_io(d, x),
        "exp5_oi" => exp5_oi(d, x),
        "log1_out" | "log_out" => log1_out(d, x),
        "log1_in" | "log_in" => log1_in(d, x),
        "log1_io" | "log_io" => log1_io(d, x),
        "log1_oi" | "log_oi" => log1_oi(d, x),
        "log2_out" => log2_out(d, x),
        "log2_in" => log2_in(d, x),
        "log2_io" => log2_io(d, x),
        "log2_oi" => log2_oi(d, x),
        "eqp1_out" | "eqp_out" => eqp1_out(d, x),
        "eqp1_in" | "eqp_in" => eqp1_in(d, x),
        "eqp1_io" | "eqp_io" => eqp1_io(d, x),
        "eqp1_oi" | "eqp_oi" => eqp1_oi(d, x),
        "eqp2_out" => eqp2_out(d, x),
        "eqp2_in" => eqp2_in(d, x),
        "eqp2_io" => eqp2_io(d, x),
        "eqp2_oi" => eqp2_oi(d, x),
        "sc1_out" | "sc_out" | "s1_out" | "s_out" => sc1_out(d, x),
        "sc1_in" | "sc_in" | "s1_in" | "s_in" => sc1_in(d, x),
        "sc1_io" | "sc_io" | "s1_io" | "s_io" => sc1_io(d, x),
        "sc1_oi" | "sc_oi" | "s1_oi" | "s_oi" => sc1_oi(d, x),
        "sc2_out" | "s2_out" => sc2_out(d, x, k),
        "sc2_in" | "s2_in" => sc2_in(d, x, k),
        "sc2_io" | "s2_io" => sc2_io(d, x, k),
        "sc2_oi" | "s2_oi" => sc2_oi(d, x, k),
        "sc3_out" | "s3_out" => sc3_out(d, x, k),
        "sc3_in" | "s3_in" => sc3_in(d, x, k),
        "sc3_io" | "s3_io" => sc3_io(d, x, k),
        "sc3_oi" | "s3_oi" => sc3_oi(d, x, k),
        "sc4_out" | "s4_out" => sc4_out(d, x),
        "sc4_in" | "s4_in" => sc4_in(d, x),
        "sc4_io" | "s4_io" => sc4_io(d, x),
        "sc4_oi" | "s4_oi" => sc4_oi(d, x),
        "chs_out" | "smoothstep_out" => chs_out(d, x),
        "chs_in" | "smoothstep_in" => chs_in(d, x),
        "chs_io" | "smoothstep_io" => chs_io(d, x),
        "chs_oi" | "smoothstep_oi" => chs_oi(d, x),
        "chsg_out" => chsg_out(d, x, k),
        "chsg_in" => chsg_in(d, x, k),
        "chsg_io" => chsg_io(d, x, k),
        "chsg_oi" => chsg_oi(d, x, k),
        "sscf_out" => sscf_out(d, x),
        "sscf_in" => sscf_in(d, x),
        "sscf_io" => sscf_io(d, x),
        "sscf_oi" => sscf_oi(d, x),
        "tet_out" => tet_out(d, x),
        "tet_in" => tet_in(d, x),
        "tet_io" => tet_io(d, x),
        "tet_oi" => tet_oi(d, x),
        "slg_out" => slg_out(d, x),
        "slg_in" => slg_in(d, x),
        "slg_io" => slg_io(d, x),
        "slg_oi" => slg_oi(d, x),
        _ => panic!("\n\n\tError:\tUnrecognized window type.\n\n")
    }
}


           /*      =^..^=       */
/////////////////////////////////////////////////
//       W I N D O W   F U N C T I O N S       //
/////////////////////////////////////////////////

//////-------------------------------------------
//////      Linear
//////-------------------------------------------
pub fn linear_out(duration: f32, sample: f32) -> f32 {
    1.0 - (sample / duration)
}

pub fn linear_in(duration: f32, sample: f32) -> f32 {
    sample / duration
}

pub fn linear_io(duration: f32, sample: f32) -> f32 {
    1.0 - (((2.0 * sample) - duration).abs() / duration)
}

pub fn linear_oi(duration: f32, sample: f32) -> f32 {
    ((2.0 * sample) - duration).abs() / duration
}
// - - - - - - - - - - - - - - - - - - - - - - -


//////-------------------------------------------
//////      Exponential Curves (Default: Exp Curve 1)
//////-------------------------------------------
// Exp Curve 1: Logistic curve-inspired, S-Curve 1 functions scaled and cut to use half the "s"
pub fn exp1_in(duration: f32, sample: f32, k: f32) -> f32 {
    2.0 / (1.0 + (((2.0 * duration) - sample) / sample).powf(k))
}

pub fn exp1_out(duration: f32, sample: f32, k: f32) -> f32 {
    2.0 / (1.0 + ((duration + sample) / (duration - sample)).powf(k))
}

pub fn exp1_io(duration: f32, sample: f32, k: f32) -> f32 {
    let diff = duration - sample;
    if sample <= (duration * 0.5) {
        2.0 / (1.0 + ((diff) / sample).powf(k))
    } else {
        2.0 / (1.0 + (sample / (diff)).powf(k))
    }
}

pub fn exp1_oi(duration: f32, sample: f32, k: f32) -> f32 {
    let x2 = 2.0 * sample;
    if sample <= (duration * 0.5) {
        2.0 / (1.0 + ((duration + x2) / (duration - x2)).powf(k))
    } else {
        2.0 / (1.0 + (((3.0 * duration) - x2) / (x2 - duration)).powf(k))
    }
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Exp Curve 2: Gaussian "Bell" Curve Function, "magic numbers" here normalize the output gain
pub fn exp2_out(duration: f32, sample: f32) -> f32 {
    (1.00637003594226 / ((E as f32).powf((2.25 * sample / duration).powf(2.0)))) - 0.00637003594226
}

pub fn exp2_in(duration: f32, sample: f32) -> f32 {
    (1.00637003594226 / ((E as f32).powf((2.25 * ((sample / duration) - 1.0)).powf(2.0)))) - 0.00637003594226
}

pub fn exp2_io(duration: f32, sample: f32) -> f32 {
    (1.00637003594226 / ((E as f32).powf(((4.5 * sample / duration) - 2.25).powf(2.0)))) - 0.00637003594226
}

pub fn exp2_oi(duration: f32, sample: f32) -> f32 {
    1.0 - exp2_io(duration, sample)
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Exp Curve 3: Exponential function -- base e with controllable contour
pub fn exp3_out(duration: f32, sample: f32, k: f32) -> f32 {
    1.0 - exp3_in(duration, sample, -1.0 * k)
}

pub fn exp3_in(duration: f32, sample: f32, k: f32) -> f32 {
    (E.powf(k * sample / duration) - 1.0) / (E.powf(k) - 1.0)
}

pub fn exp3_io(duration: f32, sample: f32, k: f32) -> f32 {
    let x2 = 2.0 * sample;
    let k2 = -1.0 * k;
    if sample <= (duration * 0.5) {
        (E.powf(k * x2 / duration) - 1.0) / (E.powf(k) - 1.0)
    } else {
        1.0 - ((E.powf(k2 * (x2 - duration) / duration) - 1.0) / (E.powf(k2) - 1.0))
    }
}

pub fn exp3_oi(duration: f32, sample: f32, k: f32) -> f32 {
    let x2 = 2.0 * sample;
    let k2 = -1.0 * k;
    if sample <= (duration * 0.5) {
        1.0 - ((E.powf(x2 * k2 / duration) - 1.0) / (E.powf(k2) - 1.0))
    } else {
        (E.powf(k * (x2 - duration) / duration) - 1.0) / (E.powf(k) - 1.0)
    }
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Exp Curve 4: Power function with controllable contour
pub fn exp4_out(duration: f32, sample: f32, k: f32) -> f32 {
    ((sample - duration) / duration).powf(k)
}

pub fn exp4_in(duration: f32, sample: f32, k: f32) -> f32 {
    (sample / duration).powf(k)
}

pub fn exp4_io(duration: f32, sample: f32, k: f32) -> f32 {
    if sample <= (duration * 0.5) {
        (2.0 * sample / duration).powf(k)
    } else {
        (2.0 * ((sample - duration) / duration)).powf(k)
    }
}

pub fn exp4_oi(duration: f32, sample: f32, k: f32) -> f32 {
    (((2.0 * sample) - duration) / duration).powf(k)
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Exp Curve 5: Audio Potentiometer "Log Taper", Piecewise Linear w/ no knee
pub fn exp5_out(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    if sample <= (duration * 0.5) {
        1.0 - (1.8 * quot)
    } else {
        0.2 - (0.2 * quot)
    }
}

pub fn exp5_in(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    if sample <= (duration * 0.5) {
        0.2 * quot
    } else {
        (1.8 * quot) - 0.8
    }
}

pub fn exp5_io(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    if sample <= (duration * 0.25) {
        0.4 * quot
    } else if sample > (duration * 0.25) && sample <= (duration * 0.5) {
        (3.6 * quot) - 0.8
    } else if sample > (duration * 0.5) && sample <= (duration * 0.75) {
        2.8 - (3.6 * quot)
    } else {
        0.4 - (0.4 * quot)
    }
}

pub fn exp5_oi(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    if sample <= (duration * 0.25) {
        1.0 - (3.6 * quot)
    } else if sample > (duration * 0.25) && sample <= (duration * 0.5) {
        0.2 - (0.4 * quot)
    } else if sample > (duration * 0.5) && sample <= (duration * 0.75) {
        (0.4 * quot) - 0.2
    } else {
        (3.6 * quot) - 2.6
    }
}
// - - - - - - - - - - - - - - - - - - - - - - -


//////-------------------------------------------
//////      Logarithmic Curves (Default: Log Curve 1)
//////-------------------------------------------
// Log Curve 1: Standard Log (base 10, scaled)
pub fn log1_out(duration: f32, sample: f32) -> f32 {
    (10.0 - (9.0 * sample / duration)).log10()
}

pub fn log1_in(duration: f32, sample: f32) -> f32 {
    (1.0 + (9.0 * sample / duration)).log10()
}

pub fn log1_io(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    if sample <= (duration * 0.5) {
        (1.0 + (18.0 * quot)).log10()
    } else {
        (19.0 - (18.0 * quot)).log10()
    }
}

pub fn log1_oi(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    if sample <= (duration * 0.5) {
        (10.0 - (18.0 * quot)).log10()
    } else {
        ((18.0 * quot) - 8.0).log10()
    }
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Log Curve 2: Audio Potentiometer "Anti-Log (or Inverse Log) Taper", Piecewise Linear w/ no knee
pub fn log2_out(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    if sample <= (duration * 0.5) {
        1.0 - (0.2 * quot)
    } else {
        1.8 - (1.8 * quot)
    }
}

pub fn log2_in(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    if sample <= (duration * 0.5) {
        1.8 * quot
    } else {
        0.8 + (0.2 * quot)
    }
}

pub fn log2_io(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    if sample <= (duration * 0.25) {
        3.6 * quot
    } else if sample > (duration * 0.25) && sample <= (duration * 0.5) {
        0.8 + (0.4 * quot)
    } else if sample > (duration * 0.5) && sample <= (duration * 0.75) {
        1.2 - (0.4 * quot)
    } else {
        3.6 - (3.6 * quot)
    }
}

pub fn log2_oi(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    if sample <= (duration * 0.25) {
        1.0 - (0.4 * quot)
    } else if sample > (duration * 0.25) && sample <= (duration * 0.5) {
        1.8 - (3.6 * quot)
    } else if sample > (duration * 0.5) && sample <= (duration * 0.75) {
        (3.6 * quot) - 1.8
    } else {
        (0.4 * quot) + 0.6
    }
}
// - - - - - - - - - - - - - - - - - - - - - - -


//////-------------------------------------------
//////      Equal Power Curves (Default: Equal Power 1)
//////-------------------------------------------
// Equal Power 1: Sinusoidal -- opposing sin and cos function slices
pub fn eqp1_out(duration: f32, sample: f32) -> f32 {
    (sample * FRAC_PI_2 / duration).cos()
}

pub fn eqp1_in(duration: f32, sample: f32) -> f32 {
    (sample * FRAC_PI_2 / duration).sin()
}

pub fn eqp1_io(duration: f32, sample: f32) -> f32 {
    (sample * PI / duration).sin()
}

pub fn eqp1_oi(duration: f32, sample: f32) -> f32 {
    (sample * PI / duration).cos()
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Equal Power 2: Square Root -- scaled square root function
pub fn eqp2_out(duration: f32, sample: f32) -> f32 {
   ((duration - sample) / duration).sqrt()
}

pub fn eqp2_in(duration: f32, sample: f32) -> f32 {
    (sample / duration).sqrt()
}

pub fn eqp2_io(duration: f32, sample: f32) -> f32 {
    let x2 = 2.0 * sample / duration;
    if sample <= (duration * 0.5) {
        x2.sqrt()
    } else {
        (2.0 - x2).sqrt()
    }
}

pub fn eqp2_oi(duration: f32, sample: f32) -> f32 {
    let x2 = 2.0 * sample / duration;
    if sample <= (duration * 0.5) {
        (1.0 - x2).sqrt()
    } else {
        (x2 - 1.0).sqrt()
    }
}
// - - - - - - - - - - - - - - - - - - - - - - -


//////-------------------------------------------
//////      S-Curves (Default: S-Curve 1)
//////-------------------------------------------
// S-Curve 1: Sinusoidal -- basic cos function curves scaled
pub fn sc1_out(duration: f32, sample: f32) -> f32 {
    0.5 * (1.0 + (PI * sample / duration).cos())
}

pub fn sc1_in(duration: f32, sample: f32) -> f32 {
    0.5 * (1.0 - (PI * sample / duration).cos())
}

pub fn sc1_io(duration: f32, sample: f32) -> f32 {
    0.5 * (1.0 - (TAU * sample / duration).cos())
}

pub fn sc1_oi(duration: f32, sample: f32) -> f32 {
    0.5 * (1.0 + (TAU * sample / duration).cos())
}
// - - - - - - - - - - - - - - - - - - - - - - -


// S-Curve 2: Logistic curve-inspired, Piecewise Sigmoid-like that I enjoy
pub fn sc2_in(duration: f32, sample: f32, k: f32) -> f32 {
    1.0 - (1.0 / (1.0 + (sample / (duration - sample)).powf(k)))
}

pub fn sc2_out(duration: f32, sample: f32, k: f32) -> f32 {
    1.0 / (1.0 + (sample / (duration - sample)).powf(k))
}

pub fn sc2_io(duration: f32, sample: f32, k: f32) -> f32 {
    if sample <= (duration * 0.5) {
        1.0 / (1.0 + ((duration / (2.0 * sample)) - 1.0).powf(k))
    } else {
        1.0 - (1.0 / (1.0 + ((duration - sample) / (sample - (duration / 2.0))).powf(k)))
    }
}

pub fn sc2_oi(duration: f32, sample: f32, k: f32) -> f32 {
    if sample <= (duration * 0.5) {
        1.0 - (1.0 / (1.0 + ((duration / (2.0 * sample)) - 1.0).powf(k)))
    } else {
        1.0 / (1.0 + ((duration - sample) / (sample - (duration / 2.0))).powf(k))
    }
}
// - - - - - - - - - - - - - - - - - - - - - - -


// S-Curve 3: Power function curves spliced to our desired inflection points
pub fn sc3_in(duration: f32, sample: f32, k: f32) -> f32 {
    let quot = 2.0 / duration;
    if sample <= (duration * 0.5) {
        0.5 * (quot * sample).powf(k)
    } else {
        1.0 - (0.5 * (quot * (sample - duration).abs()).powf(k))
    }
}

pub fn sc3_out(duration: f32, sample: f32, k: f32) -> f32 {
    let quot = 2.0 / duration;
    if sample <= (duration * 0.5) {
        1.0 - (0.5 * (quot * sample).powf(k))
    } else {
        0.5 * (quot * (sample - duration).abs()).powf(k)
    }
}

pub fn sc3_io(duration: f32, sample: f32, k: f32) -> f32 {
    let quot = 2.0 / duration;
    let quot2 = 2.0 * quot;
    if sample <= (duration * 0.25) {
        0.5 * (quot2 * sample).powf(k)
    } else if sample > (duration * 0.25) && sample <= (duration * 0.75) {
        1.0 - (0.5 * (quot * ((2.0 * sample) - duration).abs()).powf(k))
    } else {
        0.5 * (quot2 * (sample - duration).abs()).powf(k)
    }
}

pub fn sc3_oi(duration: f32, sample: f32, k: f32) -> f32 {
    let quot = 2.0 / duration;
    let quot2 = 2.0 * quot;
    if sample <= (duration * 0.25) {
        1.0 - (0.5 * (quot2 * sample).powf(k))
    } else if sample > (duration * 0.25) && sample <= (duration * 0.75) {
        0.5 * (quot * ((2.0 * sample) - duration).abs()).powf(k)
    } else {
        1.0 - (0.5 * (quot2 * (sample - duration).abs()).powf(k))
    }
}
// - - - - - - - - - - - - - - - - - - - - - - -


// S-Curve 4: Ellipse quadrants spliced to our desired inflection points
pub fn sc4_out(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    if sample <= (duration * 0.5) {
        0.5 * (1.0 + (1.0 - (2.0 * quot).powf(2.0)).sqrt())
    } else {
        0.5 * (1.0 - (1.0 - (4.0 * (quot - 1.0).powf(2.0))).sqrt())
    }
}

pub fn sc4_in(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    if sample <= (duration * 0.5) {
        0.5 * (1.0 - (1.0 - (2.0 * quot).powf(2.0)).sqrt())
    } else {
        0.5 * (1.0 + (1.0 - (4.0 * (quot - 1.0).powf(2.0))).sqrt())
    }
}

pub fn sc4_io(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    let quot2 = 2.0 * quot;
    if sample <= (duration * 0.25) {
        0.5 * (1.0 - (1.0 - (2.0 * quot2).powf(2.0)).sqrt())
    } else if sample > (duration * 0.25) && sample <= (duration * 0.75) {
        0.5 * (1.0 + (1.0 - (4.0 * (quot2 - 1.0).powf(2.0))).sqrt())
    } else {
        0.5 * (1.0 - (1.0 - (16.0 * (quot - 1.0).powf(2.0))).sqrt())
    }
}

pub fn sc4_oi(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    let quot2 = 2.0 * quot;
    if sample <= (duration * 0.25) {
        0.5 * (1.0 + (1.0 - (2.0 * quot2).powf(2.0)).sqrt())
    } else if sample > (duration * 0.25) && sample <= (duration * 0.75) {
        0.5 * (1.0 - (1.0 - (4.0 * (quot2 - 1.0).powf(2.0))).sqrt())
    } else {
        0.5 * (1.0 + (1.0 - (16.0 * (quot - 1.0).powf(2.0))).sqrt())
    }
}
// - - - - - - - - - - - - - - - - - - - - - - -


//////-------------------------------------------
//////      Special Curves (*No Default*)
//////-------------------------------------------
// Cubic Hermite Spline: "The Classic" cubic hermite spline -- 3x^2 - 2x^3, scaled
pub fn chs_out(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    1.0 - (quot.powf(2.0) * (3.0 - (2.0 * quot)))
}

pub fn chs_in(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    quot.powf(2.0) * (3.0 - (2.0 * quot))
}

pub fn chs_io(duration: f32, sample: f32) -> f32 {
    let quot = sample / duration;
    let exp = ((2.0 * sample) - duration) / duration;
    if sample <= (duration * 0.5) {
        (2.0 * quot).powf(2.0) * (3.0 - (4.0 * quot))
    } else {
        1.0 - (exp.powf(2.0) * (3.0 - (2.0 * exp)))
    }
}

pub fn chs_oi(duration: f32, sample: f32) -> f32 {
    let quot2 = 2.0 * sample / duration;
    let exp = ((2.0 * sample) - duration) / duration;
    if sample <= (duration * 0.5) {
        1.0 - (quot2.powf(2.0) * (3.0 - (2.0 * quot2)))
    } else {
        exp.powf(2.0) * (3.0 - (2.0 * exp))
    }
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Cubic Hermite Spline Generalized: Above function adapted to allow any power for contouring
pub fn chsg_out(duration: f32, sample: f32, k: f32) -> f32 {
    let quot = sample / duration;
    1.0 - (quot.powf(k) * (k + 1.0 - (k * quot)))
}

pub fn chsg_in(duration: f32, sample: f32, k: f32) -> f32 {
    let quot = sample / duration;
    quot.powf(k) * (k + 1.0 - (k * quot))
}

pub fn chsg_io(duration: f32, sample: f32, k: f32) -> f32 {
    let quot = sample / duration;
    let exp = -2.0 * (sample - duration) / duration;
    if sample <= (duration * 0.5) {
        (2.0 * quot).powf(k) * (k + 1.0 - (2.0 * k * quot))
    } else {
        exp.powf(k) * (k + 1.0 - (k * exp))
    }
}

pub fn chsg_oi(duration: f32, sample: f32, k: f32) -> f32 {
    let quot2 = 2.0 * sample / duration;
    let exp = -2.0 * (sample - duration) / duration;
    if sample <= (duration * 0.5) {
       1.0 - (quot2.powf(k) * (k + 1.0 - (k * quot2)))
    } else {
       1.0 - (exp.powf(k) * (k + 1.0 - (k * exp)))
    }
}
// - - - - - - - - - - - - - - - - - - - - - - -


// [2]
// Signalsmith Crossfade -- cheap polynomial crossfade curve with near constant energy
pub fn sscf_out(duration: f32, sample: f32) -> f32 {
    let x: f32 = sample / duration;
    let x2: f32 = 1.0 - x;
    let a: f32 = x * x2;
    (x2 + (a * (1.0 + (1.4186 * a)))).powf(2.0)
}

pub fn sscf_in(duration: f32, sample: f32) -> f32 {
    let x: f32 = sample / duration;
    let x2: f32 = 1.0 - x;
    let a: f32 = x * x2;
    (x + (a * (1.0 + (1.4186 * a)))).powf(2.0)
}

pub fn sscf_io(duration: f32, sample: f32) -> f32 {
    let x: f32 = 2.0 * sample / duration;
    let x2: f32 = 1.0 - x;
    let x3: f32 = ((2.0 * sample) - duration) / duration;
    let x4: f32 = 1.0 - x3;
    let a: f32 = x * x2;
    let a2: f32 = x3 * x4;
    if sample <= (duration * 0.5) {
        (x + (a * (1.0 + (1.4186 * a)))).powf(2.0)
    } else {
        (x4 + (a2 * (1.0 + (1.4186 * a2)))).powf(2.0)
    }
}

pub fn sscf_oi(duration: f32, sample: f32) -> f32 {
    let x: f32 = 2.0 * sample / duration;
    let x2: f32 = 1.0 - x;
    let x3: f32 = ((2.0 * sample) - duration) / duration;
    let x4: f32 = 1.0 - x3;
    let a: f32 = x * x2;
    let a2: f32 = x3 * x4;
    if sample <= (duration * 0.5) {
        (x2 + (a * (1.0 + (1.4186 * a)))).powf(2.0)
    } else {
        (x3 + (a2 * (1.0 + (1.4186 * a2)))).powf(2.0)
    }
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Tetrational: First-order tetration of our scaled current sample
pub fn tet_out(duration: f32, sample: f32) -> f32 {
    duration / (sample + duration).powf((sample / duration) + 1.0)
}

pub fn tet_in(duration: f32, sample: f32) -> f32 {
    sample.powf(sample / duration) / duration
}

pub fn tet_io(duration: f32, sample: f32) -> f32 {
    let quot2 = 2.0 * sample / duration;
    if sample <= (duration * 0.5) {
        2.0 * sample.powf(quot2) / duration
    } else {
        duration / (2.0 * sample).powf(quot2)
    }
}

pub fn tet_oi(duration: f32, sample: f32) -> f32 {
    1.0 - tet_io(duration, sample)
}
// - - - - - - - - - - - - - - - - - - - - - - -


// Super Log
pub fn slg_out(duration: f32, sample: f32) -> f32 {
    1.0 - (sample.powf(sample / duration) / duration)
}

pub fn slg_in(duration: f32, sample: f32) -> f32 {
    1.0 - (duration / (sample + duration).powf(1.0 + (sample / duration)))
}

pub fn slg_io(duration: f32, sample: f32) -> f32 {
    let x2 = 2.0 * sample;
    let quot2 = x2 / duration;
    if sample <= (duration * 0.5) {
        1.0 - (duration / (x2 + duration).powf(1.0 + quot2))
    } else {
        1.0 - ((x2 - duration).powf(quot2 - 1.0) / duration)
    }
}

pub fn slg_oi(duration: f32, sample: f32) -> f32 {
    let x2 = 2.0 * sample;
    let quot2 = x2 / duration;
    if sample <= (duration * 0.5) {
        duration / (x2 + duration).powf(1.0 + quot2)
    } else {
        (x2 - duration).powf(quot2 - 1.0) / duration
    }
}
// - - - - - - - - - - - - - - - - - - - - - - -