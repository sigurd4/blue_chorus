use std::{fmt::Display, f64::consts::{FRAC_PI_2, TAU}};

#[derive(Clone, Copy)]
pub enum Waveform
{
    Triangle = 0,
    Triangle2 = 1,
    Sawtooth = 2,
    Sawtooth2 = 3,
    Square = 4,
    Square2 = 5
}

impl Waveform
{
    pub const VARIANT_COUNT: usize = core::mem::variant_count::<Self>();
    pub const VARIANTS: [Self; Self::VARIANT_COUNT] = [
        Self::Triangle,
        Self::Triangle2,
        Self::Sawtooth,
        Self::Sawtooth2,
        Self::Square,
        Self::Square2
    ];
}

impl Waveform
{
    pub fn triangle(phase: f64, duty_cycle: f64) -> f64
    {
        if phase < duty_cycle {2.0*phase/duty_cycle - 1.0} else {1.0 - 2.0*(phase - duty_cycle)/(TAU - duty_cycle)}
    }

    pub fn sawtooth2(phase: f64, duty_cycle: f64) -> f64
    {
        if phase < duty_cycle {2.0*phase/duty_cycle - 1.0} else {2.0*(phase - duty_cycle)/(TAU - duty_cycle) - 1.0}
    }

    pub fn sawtooth(phase: f64, duty_cycle: f64) -> f64
    {
        (Self::square(phase, duty_cycle) + Self::sawtooth2(phase, duty_cycle))*0.5
    }

    pub fn triangle2(phase: f64, duty_cycle: f64) -> f64
    {
        1.0 - Self::sawtooth2(phase, duty_cycle).abs()*2.0
    }

    pub fn square(phase: f64, duty_cycle: f64) -> f64
    {
        if phase < duty_cycle {-1.0} else {1.0}
    }

    fn square2(phase: f64, duty_cycle: f64) -> f64
    {
        if Self::sawtooth2(phase, duty_cycle) < 0.0 {-1.0} else {1.0}
    }
    
    pub fn triangle_to_sin(x: f64) -> f64
    {
        (FRAC_PI_2*x).sin()
    }

    pub fn waveform(&self, phase: f64, duty_cycle: f64) -> f64
    {
        match self
        {
            Waveform::Triangle => Self::triangle(phase, duty_cycle),
            Waveform::Triangle2 => Self::triangle2(phase, duty_cycle),
            Waveform::Sawtooth => Self::sawtooth(phase, duty_cycle),
            Waveform::Sawtooth2 => Self::sawtooth2(phase, duty_cycle),
            Waveform::Square => Self::square(phase, duty_cycle),
            Waveform::Square2 => Self::square2(phase, duty_cycle),
        }
    }
}

impl Display for Waveform
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Waveform::Triangle => write!(f, "Triangle"),
            Waveform::Triangle2 => write!(f, "Triangle2"),
            Waveform::Sawtooth => write!(f, "Sawtooth"),
            Waveform::Sawtooth2 => write!(f, "Sawtooth2"),
            Waveform::Square => write!(f, "Square"),
            Waveform::Square2 => write!(f, "Square2"),
        }
    }
}