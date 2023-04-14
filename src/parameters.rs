use std::sync::atomic::{AtomicU8, Ordering};

use vst::prelude::PluginParameters;
use vst::util::AtomicFloat;

use crate::DELAY;
use crate::waveform::Waveform;

const MIN_FREQ: f32 = 0.01;
const MAX_FREQ: f32 = 10.0;
const LENGTH_CURVE: f32 = 4.0;

#[derive(Clone, Copy)]
pub enum Parameter
{
    Waveform,
    Frequency,
    Length,
    Depth,
    Feedback,
    Mix
}

impl Parameter
{
    pub const PARAMETERS: [Self; 6] = [
        Self::Waveform,
        Self::Frequency,
        Self::Length,
        Self::Depth,
        Self::Feedback,
        Self::Mix
    ];

    pub fn from(i: i32) -> Self
    {
        Self::PARAMETERS[i as usize]
    }
}

pub struct BlueChorusParameters
{
    pub waveform: AtomicU8,
    pub frequency: AtomicFloat,
    pub length: AtomicFloat,
    pub depth: AtomicFloat,
    pub feedback: AtomicFloat,
    pub mix: AtomicFloat
}

impl PluginParameters for BlueChorusParameters
{
    fn get_parameter_label(&self, index: i32) -> String
    {
        match Parameter::from(index)
        {
            Parameter::Waveform => "".to_string(),
            Parameter::Frequency => "Hz".to_string(),
            Parameter::Length => "ms".to_string(),
            Parameter::Depth => "%".to_string(),
            Parameter::Feedback => "%".to_string(),
            Parameter::Mix => "%".to_string()
        }
    }

    fn get_parameter_text(&self, index: i32) -> String
    {
        match Parameter::from(index)
        {
            Parameter::Waveform => match Waveform::from(self.waveform.load(Ordering::Relaxed))
            {
                Waveform::Sine => "Sine".to_string(),
                Waveform::Triangle => "Triangle".to_string(),
                Waveform::Sawtooth => "Sawtooth 1".to_string(),
                Waveform::InverseSawtooth => "Sawtooth 2".to_string(),
                Waveform::Square => "Square".to_string(),
                Waveform::Noise => "Noise".to_string()
            },
            Parameter::Frequency => format!("{:.3}", self.frequency.get()),
            Parameter::Length => format!("{:.3}", 1000.0*DELAY*self.length.get()),
            Parameter::Depth => format!("{:.3}", 100.0*self.depth.get()),
            Parameter::Feedback => format!("{:.3}", 100.0*self.feedback.get()),
            Parameter::Mix => format!("{:.3}", 100.0*self.mix.get())
        }
    }

    fn get_parameter_name(&self, index: i32) -> String
    {
        match Parameter::from(index)
        {
            Parameter::Waveform => "Waveform".to_string(),
            Parameter::Frequency => "Frequency".to_string(),
            Parameter::Length => "Length".to_string(),
            Parameter::Depth => "Depth".to_string(),
            Parameter::Feedback => "Feedback".to_string(),
            Parameter::Mix => "Mix".to_string(),
        }
    }

    /// Get the value of parameter at `index`. Should be value between 0.0 and 1.0.
    fn get_parameter(&self, index: i32) -> f32
    {
        match Parameter::from(index)
        {
            Parameter::Waveform => self.waveform.load(Ordering::Relaxed) as f32/(Waveform::WAVEFORMS.len() - 1) as f32,
            Parameter::Frequency => (self.frequency.get().log2() - MIN_FREQ.log2())/(MAX_FREQ.log2() - MIN_FREQ.log2()),
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
            Parameter::Waveform => self.waveform.store((value*(Waveform::WAVEFORMS.len() - 1) as f32).round() as u8, Ordering::Relaxed),
            Parameter::Frequency => self.frequency.set((value*(MAX_FREQ.log2() - MIN_FREQ.log2()) + MIN_FREQ.log2()).exp2()),
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
        index < Parameter::PARAMETERS.len() as i32
    }

    fn get_preset_data(&self) -> Vec<u8>
    {
        [
            vec![self.waveform.load(Ordering::Relaxed)],
            self.frequency.get().to_le_bytes().to_vec(),
            self.length.get().to_le_bytes().to_vec(),
            self.depth.get().to_le_bytes().to_vec(),
            self.feedback.get().to_le_bytes().to_vec(),
            self.mix.get().to_le_bytes().to_vec(),
        ].concat()
    }

    fn get_bank_data(&self) -> Vec<u8>
    {
        self.get_preset_data()
    }

    fn load_preset_data(&self, data: &[u8])
    {
        let mut i = 0;
        self.waveform.store(data[i], Ordering::Relaxed); i += 1;
        self.frequency.set(f32::from_le_bytes(*data[i..].split_array_ref().0)); i += 4;
        self.length.set(f32::from_le_bytes(*data[i..].split_array_ref().0)); i += 4;
        self.depth.set(f32::from_le_bytes(*data[i..].split_array_ref().0)); i += 4;
        self.feedback.set(f32::from_le_bytes(*data[i..].split_array_ref().0)); i += 4;
        self.mix.set(f32::from_le_bytes(*data[i..].split_array_ref().0)); i += 4;
    }

    fn load_bank_data(&self, data: &[u8])
    {
        self.load_preset_data(data)
    }
}