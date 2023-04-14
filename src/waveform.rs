use std::f32::consts::{PI, FRAC_2_PI, TAU};

use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::thread_rng;

pub const MAX_SERIES: usize = 2048;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Waveform
{
    Sine,
    Triangle,
    Sawtooth,
    InverseSawtooth,
    Square,
    Noise
}

impl Waveform
{
    pub const WAVEFORMS: [Self; 6] = [Self::Sine, Self::Triangle, Self::Sawtooth, Self::InverseSawtooth, Self::Square, Self::Noise];

    pub fn from(i: u8) -> Self
    {
        Self::WAVEFORMS[i as usize]
    }

    pub fn wavetable(&self) -> [(f32, f32); MAX_SERIES]
    {
        match self
        {
            Waveform::Sine => {
                let mut w = [(0.0, 0.0); MAX_SERIES];
                w[0] = (1.0, 1.0);
                return w
            },
            Waveform::Triangle => array_init::array_init(|m| {
                let n = (m*2 + 1) as f32;
                (8.0/PI/PI*(-1.0f32).powi(m as i32)/n/n, n)
            }),
            Waveform::Sawtooth => array_init::array_init(|n| {
                let n = n + 1;
                (-2.0/PI/(n as f32), n as f32)
            }),
            Waveform::InverseSawtooth => array_init::array_init(|n| {
                let n = n + 1;
                (2.0/PI/(n as f32), n as f32)
            }),
            Waveform::Square => array_init::array_init(|m| {
                let n = (m*2 + 1) as f32;
                (4.0/PI/n, n)
            }),
            Waveform::Noise => [(0.0, 0.0); MAX_SERIES]
        }
    }

    pub fn waveform_direct(&self, theta: f32) -> f32
    {
        match self
        {
            Waveform::Sine => {
                theta.sin()
            },
            Waveform::Triangle => {
                FRAC_2_PI*theta.sin().asin()
            },
            Waveform::Sawtooth => {
                return theta/TAU - 0.5
            },
            Waveform::InverseSawtooth => {
                return 0.5 - theta/TAU
            },
            Waveform::Square => {
                return if theta < PI {1.0} else {-1.0}
            },
            Waveform::Noise => {
                let y: f32 = Standard.sample(&mut thread_rng());
                return y.max(-1.0).min(1.0)
            },
        }
    }

    pub fn waveform_wavetable(wavetable: &[(f32, f32); MAX_SERIES], theta: f32, max_series: usize) -> f32
    {
        let mut y = 0.0;
        for m in 0..MAX_SERIES.min(max_series)
        {
            let (a, n) = wavetable[m];
            y += a*(n*theta).sin()
        }
        return y
    }
}