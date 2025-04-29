use delay_line::DelayLine;
use real_time_fir_iir_filters::{conf::HighPass, filters::iir::first::FirstOrderRCFilter, param::RC, rtf::Rtf};
use saturation::{LinMoid, Saturation};

use crate::{cache::BlueChorusCache, filter::FilterChorus};

#[derive(Clone, Debug)]
pub struct BlueChorusChannel
{
    delay_line: DelayLine<f64>,
    filter_input: FirstOrderRCFilter<HighPass, f64>,
    filter_chorus: [FilterChorus; 2],
    filter_feedback: FirstOrderRCFilter<HighPass, f64>
}

impl BlueChorusChannel
{
    pub fn process(&mut self, cache: &BlueChorusCache, feedback: f64, mix: f64, stages: usize, mut x: f64) -> f64
    {
        let BlueChorusChannel { delay_line, filter_input, filter_chorus: [filter_chorus1, filter_chorus2], filter_feedback } = self;

        [x] = filter_input.filter(cache.rate, x);
        let mut z = filter_chorus1.filter(cache.rate, x);

        delay_line.stretch(stages);
        z = delay_line.delay(z);
        z = delay_line.read_tap(cache.tap).unwrap_or(z);

        let y = filter_chorus2.filter(cache.rate, z);

        let [x_f] = filter_feedback.filter(cache.rate, y*feedback);

        if let Some(w) = delay_line.input_mut()
        {
            *w += x_f;
            *w = LinMoid.saturate(*w, ..);
        }

        x.mul_add(1.0 - mix, y*mix)
    }
}

impl Default for  BlueChorusChannel
{
    fn default() -> Self
    {
        Self {
            delay_line: Default::default(),
            filter_input: FirstOrderRCFilter::new(RC {
                r: 471e3,
                c: 47e-9
            }),
            filter_chorus: Default::default(),
            filter_feedback: FirstOrderRCFilter::new(RC {
                r: 39e3 + 50e3*0.5,
                c: 47e-9
            }),
        }
    }
}