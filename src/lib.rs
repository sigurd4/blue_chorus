#![feature(adt_const_params)]
#![feature(split_array)]
#![feature(const_for)]
#![feature(variant_count)]
#![feature(array_chunks)]
#![feature(iter_array_chunks)]
#![feature(generic_const_exprs)]

use std::f64::consts::TAU;
use std::sync::Arc;

use cache::BlueChorusCache;
use channel::BlueChorusChannel;
use num::Float;
use real_time_fir_iir_filters::change::Change;
use real_time_fir_iir_filters::conf::LowPass;
use real_time_fir_iir_filters::filters::iir::first::{FirstOrderFilter, FirstOrderRCFilter};
use real_time_fir_iir_filters::param::{Omega, RC};
use real_time_fir_iir_filters::rtf::Rtf;
use vst::{plugin_main, prelude::*};

use self::lfo::LFO;
use self::parameters::{BlueChorusParameters, Parameter};
use self::waveform::Waveform;

moddef::moddef!(
    mod {
        bank,
        cache,
        channel,
        parameters,
        tapped_delay_line,
        filter,
        lfo,
        waveform
    }
);

const DELAY: f64 = 0.1;
const CHANGE: f64 = 1.0;
const F_DELAY: f64 = 1.0;

struct BlueChorusPlugin
{
    pub param: Arc<BlueChorusParameters>,
    channel: [BlueChorusChannel; CHANNEL_COUNT],
    lfo: LFO,
    filter_lfo: FirstOrderRCFilter<LowPass, f64>,
    filter_delay: FirstOrderFilter<LowPass, f64>,
    cache: BlueChorusCache
}

const CHANNEL_COUNT: usize = 2;

impl BlueChorusPlugin
{
    fn stages(&self) -> usize
    {
        (self.cache.rate * DELAY) as usize
    }

    fn process<F: Float>(&mut self, buffer: &mut AudioBuffer<F>)
    {
        self.lfo.omega = self.param.frequency.get() as f64 * TAU;
        self.lfo.waveform = Waveform::Triangle; //Waveform::VARIANTS[self.param.waveform.load(Ordering::Relaxed) as usize];

        self.cache.depth.change(self.param.depth.get() as f64, CHANGE);
        self.cache.length.change(self.param.length.get() as f64, CHANGE);

        let sine = self.param.sine.get() as f64;
        let feedback = self.param.feedback.get() as f64;
        let duty_cycle = self.param.duty_cycle.get() as f64;
        let mix = self.param.mix.get() as f64;
        let stages = self.stages();
        let tap = self.cache.length * stages as f64;

        self.lfo.duty_cycle = duty_cycle;

        let mut channels = buffer.zip().map(|(i, o)| i.iter().zip(o.iter_mut())).array_chunks::<CHANNEL_COUNT>().next().unwrap();

        'lp: loop
        {
            let lfo = self.lfo.next(self.cache.rate);
            let lfo = Waveform::triangle_to_sin(lfo).mul_add(sine, lfo * (1.0 - sine));
            let [lfo] = self.filter_lfo.filter(self.cache.rate, lfo * self.cache.depth);
            [self.cache.tap] = self.filter_delay.filter(self.cache.rate, tap * lfo.mul_add(0.5, 0.5));

            for (xy, channel) in channels.iter_mut().map(Iterator::next).zip(self.channel.iter_mut())
            {
                match xy
                {
                    Some((&x, y)) => {
                        let x = x.to_f64().unwrap();
                        *y = F::from(channel.process(&self.cache, feedback, mix, stages, x)).unwrap()
                    },
                    _ => break 'lp
                }
            }
        }
    }
}

#[allow(deprecated)]
impl Plugin for BlueChorusPlugin
{
    fn new(_host: HostCallback) -> Self
    where
        Self: Sized
    {
        let param = BlueChorusParameters::default();
        let cache = (&param).into();
        BlueChorusPlugin {
            param: Arc::new(param),
            channel: Default::default(),
            lfo: LFO::new(TAU * 1.0, 0.5, Waveform::Triangle),
            filter_lfo: FirstOrderRCFilter::new::<LowPass>(RC { r: 220e3, c: 10e-9 }),
            filter_delay: FirstOrderFilter::new::<LowPass>(Omega { omega: F_DELAY * TAU }),
            cache
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
        self.cache.rate = rate as f64;
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
