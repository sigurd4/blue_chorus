use real_time_fir_iir_filters::iir::IIRFilter;
use real_time_fir_iir_filters::iir::rc::FirstOrderRCFilter;
use real_time_fir_iir_filters::iir::sallen_key::SecondOrderSallenKeyFilter;

const R9: f32 = 330000.0;
const R10: f32 = 10000.0;

const C5: f32 = 0.000000033;
const C6: f32 = 0.0000000033;
const C7: f32 = 0.0000000082;
const C8: f32 = 0.000000000470;

#[derive(Clone, Copy)]
pub struct FilterChorus
{
    h0: FirstOrderRCFilter,
    h1: FirstOrderRCFilter,
    h2: SecondOrderSallenKeyFilter,
}

impl FilterChorus
{
    pub fn new() -> Self
    {
        Self {
            h0: FirstOrderRCFilter::new(R9, C5),
            h1: FirstOrderRCFilter::new(R10, C6),
            h2: SecondOrderSallenKeyFilter::new(R10, R10, C7, C8),
        }
    }

    pub fn filter(&mut self, rate: f32, x: f32) -> f32
    {
        let x0 = self.h0.filter(rate, x)[1];
        let x1 = self.h1.filter(rate, x0)[0];
        self.h2.filter(rate, x1)[0]
    }
}