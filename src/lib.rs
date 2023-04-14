#![feature(adt_const_params)]
#![feature(split_array)]

use std::f32::consts::{TAU, PI};
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

use real_time_fir_iir_filters::iir::rc::FirstOrderRCFilter;
use real_time_fir_iir_filters::iir::{FirstOrderFilter, IIRFilter};
use vst::{prelude::*, plugin_main};

use self::filter::FilterChorus;
use self::lfo::LFO;
use self::parameters::{BlueChorusParameters, Parameter};
use self::tapped_delay_line::TappedDelayLine;
use self::waveform::Waveform;

pub mod parameters;
pub mod tapped_delay_line;
pub mod filter;
pub mod lfo;
pub mod waveform;

const DELAY: f32 = 0.1;
const CHANGE: f32 = 0.2;
const DIST: f32 = 0.000001;

struct BlueChorusPlugin
{
    pub param: Arc<BlueChorusParameters>,
    lfo: LFO,
    depth: f32,
    length: f32,
    delay_line: [TappedDelayLine; CHANNEL_COUNT],
    filter_input: [FirstOrderRCFilter; CHANNEL_COUNT],
    filter_chorus: [[FilterChorus; 2]; CHANNEL_COUNT],
    filter_lfo: FirstOrderRCFilter,
    filter_feedback: [FirstOrderRCFilter; CHANNEL_COUNT],
    rate: f32
}

const CHANNEL_COUNT: usize = 2;

impl BlueChorusPlugin
{
    fn triangle(phase: f32) -> f32
    {
        (1.0 - phase/PI).abs() - 1.0
    }

    fn stages(&self) -> usize
    {
        (self.rate*DELAY) as usize
    }
}

impl Plugin for BlueChorusPlugin
{
    fn new(_host: HostCallback) -> Self
    where
        Self: Sized
    {
        BlueChorusPlugin {
            param: Arc::new(BlueChorusParameters {
                waveform: AtomicU8::from(Waveform::Triangle as u8),
                frequency: AtomicFloat::from(1.0),
                length: AtomicFloat::from(0.005/DELAY),
                depth: AtomicFloat::from(1.0),
                feedback: AtomicFloat::from(0.0),
                mix: AtomicFloat::from(0.50)
            }),
            depth: 1.0,
            length: 0.01/DELAY,
            lfo: LFO::new(TAU*1.0, Waveform::Triangle),
            filter_input: [FirstOrderRCFilter::new(471000.0, 0.000000047); CHANNEL_COUNT],
            filter_chorus: [[FilterChorus::new(); 2]; CHANNEL_COUNT],
            filter_lfo: FirstOrderRCFilter::new(220000.0, 0.000000010),
            filter_feedback: [FirstOrderRCFilter::new(39000.0 + 50000.0*0.5, 0.000000047); CHANNEL_COUNT],
            delay_line: array_init::array_init(|_| TappedDelayLine::new()),
            rate: 44100.0
        }
    }

    fn get_info(&self) -> Info
    {
        Info {
            name: "Blue Chorus".to_string(),
            vendor: "Soma FX".to_string(),
            presets: 0,
            parameters: Parameter::PARAMETERS.len() as i32,
            inputs: CHANNEL_COUNT as i32,
            outputs: CHANNEL_COUNT as i32,
            midi_inputs: 0,
            midi_outputs: 0,
            unique_id: 5436354,
            version: 1,
            category: Category::Effect,
            initial_delay: 0,
            preset_chunks: false,
            f64_precision: true,
            silent_when_stopped: true,
            ..Default::default()
        }
    }

    fn set_sample_rate(&mut self, rate: f32)
    {
        self.rate = rate;
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>)
    {
        self.lfo.omega = TAU*self.param.frequency.get();
        self.lfo.waveform = Waveform::from(self.param.waveform.load(Ordering::Relaxed));
        self.depth = CHANGE*self.param.depth.get() + (1.0 - CHANGE)*self.depth;
        self.length = CHANGE*self.param.length.get() + (1.0 - CHANGE)*self.length;
        let feedback = self.param.feedback.get();
        let mix = self.param.mix.get();

        let stages = self.stages();

        let theta0 = self.lfo.theta;
        let w_lfo = self.filter_lfo.w;

        for (((((input_channel, output_channel), delay_line), filter_input), filter_chorus), filter_feedback) in buffer.zip()
            .zip(self.delay_line.iter_mut())
            .zip(self.filter_input.iter_mut())
            .zip(self.filter_chorus.iter_mut())
            .zip(self.filter_feedback.iter_mut())
        {
            delay_line.stages = stages;
            self.filter_lfo.w = w_lfo;
            self.lfo.theta = theta0;
            for (input_sample, output_sample) in input_channel.into_iter()
                .zip(output_channel.into_iter())
            {
                let lfo = 0.5*self.filter_lfo.filter(self.rate, self.lfo.next(self.rate)*self.depth)[0] + 0.5;
                delay_line.tap = lfo*self.length*(stages - 1) as f32;

                let x = filter_input.filter(self.rate, *input_sample)[1];
                
                let x_ = filter_chorus[0].filter(self.rate, x);

                let y = filter_chorus[1].filter(self.rate, delay_line.siso(x_));
                let x_ = x + filter_feedback.filter(self.rate, y*feedback)[1];
                delay_line.w[0] = (x_*DIST)/(1.0 + (x_*DIST).abs())/DIST;

                *output_sample = y*mix + x*(1.0 - mix);
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters>
    {
        self.param.clone()
    }
}

plugin_main!(BlueChorusPlugin);