#![feature(adt_const_params)]
#![feature(split_array)]
#![feature(const_for)]
#![feature(variant_count)]
#![feature(array_chunks)]

use std::f64::consts::{TAU, PI};
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

use num::Float;
use real_time_fir_iir_filters::{iir::first::{FirstOrderFilter, FirstOrderRCFilter}, Filter};
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

const DELAY: f64 = 0.1;
const CHANGE: f64 = 0.2;
const DIST: f64 = 0.000001;

const F_DELAY: f64 = 1.0;

struct BlueChorusPlugin
{
    pub param: Arc<BlueChorusParameters>,
    lfo: LFO,
    depth: f64,
    length: f64,
    delay_line: [TappedDelayLine; CHANNEL_COUNT],
    filter_delay: [FirstOrderFilter<f64>; 2],
    filter_input: [FirstOrderRCFilter<f64>; CHANNEL_COUNT],
    filter_chorus: [[FilterChorus; 2]; CHANNEL_COUNT],
    filter_lfo: FirstOrderRCFilter<f64>,
    filter_feedback: [FirstOrderRCFilter<f64>; CHANNEL_COUNT],
    rate: f64
}

const CHANNEL_COUNT: usize = 2;

impl BlueChorusPlugin
{
    fn stages(&self) -> usize
    {
        (self.rate*DELAY) as usize
    }

    fn process<F: Float>(&mut self, buffer: &mut AudioBuffer<F>)
    {
        self.lfo.omega = self.param.frequency.get() as f64*TAU;
        self.lfo.waveform = Waveform::Triangle;//Waveform::VARIANTS[self.param.waveform.load(Ordering::Relaxed) as usize];
        let sine = self.param.sine.get() as f64;
        self.depth = CHANGE*self.param.depth.get() as f64 + (1.0 - CHANGE)*self.depth;
        self.length = CHANGE*self.param.length.get() as f64 + (1.0 - CHANGE)*self.length;
        let feedback = self.param.feedback.get() as f64;
        let duty_cycle = self.param.duty_cycle.get() as f64;
        let mix = self.param.mix.get() as f64;

        let stages = self.stages();

        let theta0 = self.lfo.theta;
        self.lfo.duty_cycle = duty_cycle;
        let w_lfo = self.filter_lfo.w;

        for ((((((input_channel, output_channel), delay_line), filter_input), [filter_chorus1, filter_chorus2]), filter_feedback), filter_delay) in buffer.zip()
            .zip(self.delay_line.iter_mut())
            .zip(self.filter_input.iter_mut())
            .zip(self.filter_chorus.iter_mut())
            .zip(self.filter_feedback.iter_mut())
            .zip(self.filter_delay.iter_mut())
        {
            self.filter_lfo.w = w_lfo;
            self.lfo.theta = theta0;
            for (input_sample, output_sample) in input_channel.into_iter()
                .zip(output_channel.into_iter())
            {
                let lfo = self.lfo.next(self.rate);
                let lfo = Waveform::triangle_to_sin(lfo)*sine + lfo*(1.0 - sine);
                let lfo = 0.5*self.filter_lfo.filter(self.rate, lfo*self.depth)[0] + 0.5;
                delay_line.tap = lfo*filter_delay.filter(self.rate, self.length*(stages - 1) as f64)[0];

                let x = filter_input.filter(self.rate, input_sample.to_f64().unwrap())[1];
                
                let x_ = filter_chorus1.filter(self.rate, x);

                let y = filter_chorus2.filter(self.rate, delay_line.siso(x_));
                let x_ = x + filter_feedback.filter(self.rate, y*feedback)[1];
                delay_line.w[0] = (x_*DIST)/(1.0 + (x_*DIST).abs())/DIST;

                *output_sample = F::from(y*mix + x*(1.0 - mix)).unwrap();
            }
        }
    }
}

impl Plugin for BlueChorusPlugin
{
    fn new(_host: HostCallback) -> Self
    where
        Self: Sized
    {
        BlueChorusPlugin {
            param: Arc::new(Default::default()),
            depth: 1.0,
            length: 0.01/DELAY,
            lfo: LFO::new(TAU*1.0, 0.5, Waveform::Triangle),
            filter_delay: [FirstOrderFilter::new(F_DELAY); CHANNEL_COUNT],
            filter_input: [FirstOrderRCFilter::new(471000.0, 0.000000047); CHANNEL_COUNT],
            filter_chorus: [[FilterChorus::new(); 2]; CHANNEL_COUNT],
            filter_lfo: FirstOrderRCFilter::new(220000.0, 0.000000010),
            filter_feedback: [FirstOrderRCFilter::new(39000.0 + 50000.0*0.5, 0.000000047); CHANNEL_COUNT],
            delay_line: [(); CHANNEL_COUNT].map(|()| TappedDelayLine::new()),
            rate: 44100.0
        }
    }

    fn get_info(&self) -> Info
    {
        Info {
            name: "Blue Chorus".to_string(),
            vendor: "Soma FX".to_string(),
            presets: 0,
            parameters: Parameter::VARIANTS.len() as i32,
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
        self.rate = rate as f64;
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters>
    {
        self.param.clone()
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>)
    {
        self.process(buffer)
    }

    fn process_f64(&mut self, buffer: &mut AudioBuffer<f64>)
    {
        self.process(buffer)
    }
}

plugin_main!(BlueChorusPlugin);