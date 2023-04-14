use std::f32::consts::{PI, TAU};

use crate::waveform::Waveform;


#[derive(Clone, Copy)]
pub struct LFO
{
    pub omega: f32,
    pub waveform: Waveform,
    pub theta: f32
}

impl LFO
{
    pub fn new(omega: f32, waveform: Waveform) -> Self
    {
        Self {
            omega,
            waveform,
            theta: 0.0
        }
    }

    pub fn step(&mut self, rate: f32)
    {
        let omega_norm = self.omega/rate;
        self.theta = (self.theta + omega_norm) % TAU;
    }

    pub fn next(&mut self, rate: f32) -> f32
    {
        let omega_norm = self.omega/rate;
        let y = self.waveform(omega_norm);
        self.step(rate);
        return y
    }

    fn waveform(&self, omega_norm: f32) -> f32
    {
        if omega_norm < PI
        {
            self.waveform.waveform_direct(self.theta)
        }
        else
        {
            0.0
        }
    }
}