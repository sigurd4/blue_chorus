use serde::{Deserialize, Serialize};

use crate::{parameters::BlueChorusParameters, DELAY};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlueChorusBank
{
    #[serde(default = "BlueChorusBank::default_sine")]
    pub sine: f32,
    #[serde(default = "BlueChorusBank::default_frequency")]
    pub frequency: f32,
    #[serde(default = "BlueChorusBank::default_duty_cycle")]
    pub duty_cycle: f32,
    #[serde(default = "BlueChorusBank::default_length")]
    pub length: f32,
    #[serde(default = "BlueChorusBank::default_depth")]
    pub depth: f32,
    #[serde(default = "BlueChorusBank::default_feedback")]
    pub feedback: f32,
    #[serde(default = "BlueChorusBank::default_mix")]
    pub mix: f32
}

impl BlueChorusBank
{
    fn default_sine() -> f32
    {
        0.0
    }
    fn default_frequency() -> f32
    {
        1.0
    }
    fn default_duty_cycle() -> f32
    {
        0.5
    }
    fn default_length() -> f32
    {
        0.005/DELAY as f32
    }
    fn default_depth() -> f32
    {
        1.0
    }
    fn default_feedback() -> f32
    {
        0.0
    }
    fn default_mix() -> f32
    {
        0.50
    }
}
impl From<&BlueChorusParameters> for BlueChorusBank
{
    fn from(param: &BlueChorusParameters) -> Self
    {
        Self {
            sine: param.sine.get(),
            frequency: param.frequency.get(),
            duty_cycle: param.duty_cycle.get(),
            length: param.length.get(),
            depth: param.depth.get(),
            feedback: param.feedback.get(),
            mix: param.mix.get()
        }
    }
}
impl From<BlueChorusParameters> for  BlueChorusBank
{
    fn from(value: BlueChorusParameters) -> Self
    {
        (&value).into()
    }
}
impl Default for BlueChorusBank
{
    fn default() -> Self
    {
        Self {
            sine: Self::default_sine(),
            frequency: Self::default_frequency(),
            duty_cycle: Self::default_duty_cycle(),
            length: Self::default_length(),
            depth: Self::default_depth(),
            feedback: Self::default_feedback(),
            mix: Self::default_mix()
        }
    }
}