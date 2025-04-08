use real_time_fir_iir_filters::{conf::HighPass, filters::iir::first::FirstOrderRCFilter, param::RC, rtf::Rtf};
use saturation::{LinMoid, Saturation};

use crate::{cache::BlueChorusCache, filter::FilterChorus, tapped_delay_line::TappedDelayLine};

#[derive(Clone, Debug)]
pub struct BlueChorusChannel
{
    delay_line: TappedDelayLine,
    filter_input: FirstOrderRCFilter<HighPass, f64>,
    filter_chorus: [FilterChorus; 2],
    filter_feedback: FirstOrderRCFilter<HighPass, f64>
}

impl BlueChorusChannel
{
    pub fn process(&mut self, cache: &BlueChorusCache, feedback: f64, mix: f64, stages: usize, x: f64) -> f64
    {
        let BlueChorusChannel { delay_line, filter_input, filter_chorus: [filter_chorus1, filter_chorus2], filter_feedback } = self;
        delay_line.tap = cache.tap;

        let [x] = filter_input.filter(cache.rate, x);

        let y = filter_chorus2.filter(cache.rate, delay_line.delay(filter_chorus1.filter(cache.rate, x), stages));

        let [x_f] = filter_feedback.filter(cache.rate, y*feedback);

        if let Some(w) = delay_line.w.front_mut()
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
            filter_input: FirstOrderRCFilter::new::<HighPass>(RC {r: 471e3, c: 47e-9}),
            filter_chorus: Default::default(),
            filter_feedback: FirstOrderRCFilter::new::<HighPass>(RC {r: 39e3 + 50e3*0.5, c: 47e-9}),
            delay_line: Default::default(),
        }
    }
}