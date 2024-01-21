use real_time_fir_iir_filters::{iir::{first::FirstOrderRCFilter, second::SecondOrderSallenKeyFilter}, Filter};

const R9: f64 = 330000.0;
const R10: f64 = 10000.0;

const C5: f64 = 0.000000033;
const C6: f64 = 0.0000000033;
const C7: f64 = 0.0000000082;
const C8: f64 = 0.000000000470;

#[derive(Clone, Copy)]
pub struct FilterChorus
{
    h0: FirstOrderRCFilter<f64>,
    h1: FirstOrderRCFilter<f64>,
    h2: SecondOrderSallenKeyFilter<f64>,
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

    pub fn filter(&mut self, rate: f64, x: f64) -> f64
    {
        let x0 = self.h0.filter(rate, x)[1];
        let x1 = self.h1.filter(rate, x0)[0];
        self.h2.filter(rate, x1)[0]
    }
}