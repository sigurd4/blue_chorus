use std::f64::consts::{PI, TAU};

use crate::waveform::Waveform;


#[derive(Clone, Copy)]
pub struct LFO
{
    pub omega: f64,
    pub duty_cycle: f64,
    pub waveform: Waveform,
    pub theta: f64
}

impl LFO
{
    pub fn new(omega: f64, duty_cycle: f64, waveform: Waveform) -> Self
    {
        Self {
            omega,
            duty_cycle,
            waveform,
            theta: 0.0
        }
    }

    pub fn step(&mut self, rate: f64)
    {
        let omega_norm = self.omega/rate;
        self.theta = (self.theta + omega_norm) % TAU;
    }

    pub fn next(&mut self, rate: f64) -> f64
    {
        let omega_norm = self.omega/rate;
        let y = self.waveform(omega_norm);
        self.step(rate);
        return y
    }

    fn waveform(&self, omega_norm: f64) -> f64
    {
        if omega_norm < PI
        {
            self.waveform.waveform(self.theta, self.duty_cycle*TAU)
        }
        else
        {
            0.0
        }
    }
}