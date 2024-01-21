use std::sync::atomic::{AtomicU8, Ordering};

use vst::prelude::PluginParameters;
use vst::util::AtomicFloat;

use crate::DELAY;
use crate::waveform::Waveform;

const MIN_FREQ: f32 = 0.12742313;
const MAX_FREQ: f32 = 58.354592;
const LENGTH_CURVE: f32 = 4.0;

#[derive(Clone, Copy)]
pub enum Parameter
{
    //Waveform,
    Sine,
    Frequency,
    DutyCycle,
    Length,
    Depth,
    Feedback,
    Mix
}

impl Parameter
{
    pub const VARIANT_COUNT: usize = core::mem::variant_count::<Self>();
    pub const VARIANTS: [Self; Self::VARIANT_COUNT] = [
        //Self::Waveform,
        Self::Sine,
        Self::Frequency,
        Self::DutyCycle,
        Self::Length,
        Self::Depth,
        Self::Feedback,
        Self::Mix
    ];

    pub fn from(i: i32) -> Self
    {
        Self::VARIANTS[i as usize]
    }
}

pub struct BlueChorusParameters
{
    //pub waveform: AtomicU8,
    pub sine: AtomicFloat,
    pub frequency: AtomicFloat,
    pub duty_cycle: AtomicFloat,
    pub length: AtomicFloat,
    pub depth: AtomicFloat,
    pub feedback: AtomicFloat,
    pub mix: AtomicFloat
}

impl Default for BlueChorusParameters
{
    fn default() -> Self
    {
        Self {
            //waveform: AtomicU8::from(Waveform::Triangle as u8),
            sine: AtomicFloat::from(0.0),
            frequency: AtomicFloat::from(1.0),
            duty_cycle: AtomicFloat::from(0.5),
            length: AtomicFloat::from(0.005/DELAY as f32),
            depth: AtomicFloat::from(1.0),
            feedback: AtomicFloat::from(0.0),
            mix: AtomicFloat::from(0.50)
        }
    }
}

impl PluginParameters for BlueChorusParameters
{
    fn get_parameter_label(&self, index: i32) -> String
    {
        match Parameter::from(index)
        {
            //Parameter::Waveform => "",
            Parameter::Sine => "%",
            Parameter::Frequency => "Hz",
            Parameter::DutyCycle => "%",
            Parameter::Length => "ms",
            Parameter::Depth => "%",
            Parameter::Feedback => "%",
            Parameter::Mix => "%"
        }.to_string()
    }

    fn get_parameter_text(&self, index: i32) -> String
    {
        match Parameter::from(index)
        {
            /*Parameter::Waveform => match Waveform::VARIANTS[self.waveform.load(Ordering::Relaxed) as usize]
            {
                Waveform::Triangle => "Triangle",
                Waveform::Triangle2 => "Triangle 2",
                Waveform::Sawtooth => "Sawtooth",
                Waveform::Sawtooth2 => "Sawtooth 2",
                Waveform::Square => "Square",
                Waveform::Square2 => "Square 2",
            }.to_string(),*/
            Parameter::Sine => format!("{:.3}", 100.0*self.sine.get()),
            Parameter::Frequency => format!("{:.3}", self.frequency.get()),
            Parameter::DutyCycle => format!("{:.3}", 100.0*self.duty_cycle.get()),
            Parameter::Length => format!("{:.3}", 1000.0*DELAY as f32*self.length.get()),
            Parameter::Depth => format!("{:.3}", 100.0*self.depth.get()),
            Parameter::Feedback => format!("{:.3}", 100.0*self.feedback.get()),
            Parameter::Mix => format!("{:.3}", 100.0*self.mix.get())
        }
    }

    fn get_parameter_name(&self, index: i32) -> String
    {
        match Parameter::from(index)
        {
            //Parameter::Waveform => "Waveform",
            Parameter::Sine => "Sine",
            Parameter::Frequency => "Frequency",
            Parameter::DutyCycle => "Duty Cycle",
            Parameter::Length => "Length",
            Parameter::Depth => "Depth",
            Parameter::Feedback => "Feedback",
            Parameter::Mix => "Mix",
        }.to_string()
    }

    /// Get the value of parameter at `index`. Should be value between 0.0 and 1.0.
    fn get_parameter(&self, index: i32) -> f32
    {
        match Parameter::from(index)
        {
            //Parameter::Waveform => self.waveform.load(Ordering::Relaxed) as f32/(Waveform::VARIANTS.len() - 1) as f32,
            Parameter::Sine => self.sine.get(),
            Parameter::Frequency => (self.frequency.get().log2() - MIN_FREQ.log2())/(MAX_FREQ.log2() - MIN_FREQ.log2()),
            Parameter::DutyCycle => self.duty_cycle.get(),
            Parameter::Length => self.length.get().powf(1.0/LENGTH_CURVE),
            Parameter::Depth => self.depth.get(),
            Parameter::Feedback => self.feedback.get()*0.5 + 0.5,
            Parameter::Mix => self.mix.get()
        }
    }
    
    fn set_parameter(&self, index: i32, value: f32)
    {
        match Parameter::from(index)
        {
            //Parameter::Waveform => self.waveform.store((value*(Waveform::VARIANTS.len() - 1) as f32).round() as u8, Ordering::Relaxed),
            Parameter::Sine => self.sine.set(value),
            Parameter::Frequency => self.frequency.set((value*(MAX_FREQ.log2() - MIN_FREQ.log2()) + MIN_FREQ.log2()).exp2()),
            Parameter::DutyCycle => self.duty_cycle.set(value),
            Parameter::Length => self.length.set(value.powf(LENGTH_CURVE)),
            Parameter::Depth => self.depth.set(value),
            Parameter::Feedback => self.feedback.set(value*2.0 - 1.0),
            Parameter::Mix => self.mix.set(value)
        }
    }

    fn change_preset(&self, _preset: i32) {}

    fn get_preset_num(&self) -> i32
    {
        0
    }

    fn set_preset_name(&self, _name: String) {}

    fn get_preset_name(&self, _preset: i32) -> String
    {
        "".to_string()
    }

    fn can_be_automated(&self, index: i32) -> bool
    {
        index < Parameter::VARIANTS.len() as i32
    }


    fn get_preset_data(&self) -> Vec<u8>
    {
        Parameter::VARIANTS.map(|v| self.get_parameter(v as i32).to_le_bytes())
            .concat()
    }

    fn get_bank_data(&self) -> Vec<u8>
    {
        Parameter::VARIANTS.map(|v| self.get_parameter(v as i32).to_le_bytes())
            .concat()
    }

    fn load_preset_data(&self, data: &[u8])
    {
        for (v, &b) in Parameter::VARIANTS.into_iter()
            .zip(data.array_chunks())
        {
            self.set_parameter(v as i32, f32::from_le_bytes(b));
        }
    }

    fn load_bank_data(&self, data: &[u8])
    {
        for (v, &b) in Parameter::VARIANTS.into_iter()
            .zip(data.array_chunks())
        {
            self.set_parameter(v as i32, f32::from_le_bytes(b));
        }
    }
}